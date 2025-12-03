use anyhow::Result;
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::time::{Duration, Instant, interval};
use tracing::{debug, error, info, warn};

use crate::parser::schemas::book_diff::BookDiff;

use super::diff::ApplyResult;
use super::loader::SnapshotLoader;
use super::service::OrderBookService;

pub struct SyncConfig {
    pub info_url: String,
    pub container_snapshot_path: String,
    pub host_snapshot_path: String,
    pub snapshot_timeout: Duration,
    pub resync_interval: Duration,
}

impl SyncConfig {
    pub fn docker_volume(volume_path: &str) -> Self {
        Self {
            info_url: "http://127.0.0.1:3001/info".to_string(),
            container_snapshot_path: "/data/l4_snapshot.json".to_string(),
            host_snapshot_path: format!("{}/l4_snapshot.json", volume_path),
            snapshot_timeout: Duration::from_secs(120),
            resync_interval: Duration::from_secs(10),
        }
    }
}

impl Default for SyncConfig {
    fn default() -> Self {
        Self::docker_volume("/var/lib/docker/volumes/hyperliquid_node-data/_data")
    }
}

pub struct Sync {
    config: SyncConfig,
    service: Arc<OrderBookService>,
    rx: mpsc::Receiver<BookDiff>,
}

impl Sync {
    pub fn new(
        config: SyncConfig,
        service: Arc<OrderBookService>,
        rx: mpsc::Receiver<BookDiff>,
    ) -> Self {
        Self { config, service, rx }
    }

    pub async fn run(mut self) -> Result<()> {
        info!("orderbook sync starting");

        self.initial_sync().await?;

        info!("entering live mode with periodic resync every {:?}", self.config.resync_interval);

        let mut resync_ticker = interval(self.config.resync_interval);
        resync_ticker.reset();

        let mut live_applied = 0u64;
        let mut live_skipped = 0u64;
        let mut last_stats_log = Instant::now();

        loop {
            tokio::select! {
                biased;

                Some(diff) = self.rx.recv() => {
                    match self.service.apply_diff(diff) {
                        ApplyResult::Applied => live_applied += 1,
                        ApplyResult::Skipped => live_skipped += 1,
                    }

                    if last_stats_log.elapsed() > Duration::from_secs(10) {
                        let stats = self.service.stats();
                        info!(
                            "live: {}/s applied, {}/s skipped | {} books, {} orders",
                            live_applied / 10,
                            live_skipped / 10,
                            stats.books,
                            stats.total_orders
                        );
                        live_applied = 0;
                        live_skipped = 0;
                        last_stats_log = Instant::now();
                    }
                }

                _ = resync_ticker.tick() => {
                    if let Err(e) = self.periodic_resync().await {
                        warn!("periodic resync failed: {}, will retry next interval", e);
                    }
                }

                else => {
                    warn!("diff channel closed");
                    break;
                }
            }
        }

        Ok(())
    }

    async fn initial_sync(&mut self) -> Result<()> {
        let loader = SnapshotLoader::new(
            &self.config.info_url,
            &self.config.container_snapshot_path,
            &self.config.host_snapshot_path,
        );

        loader.cleanup().await.ok();
        loader.request().await?;

        info!("snapshot requested, buffering diffs");

        let mut buffer = Vec::with_capacity(500_000);
        let start = Instant::now();

        loop {
            tokio::select! {
                biased;

                Some(diff) = self.rx.recv() => {
                    buffer.push(diff);
                    if buffer.len() % 50_000 == 0 {
                        debug!("buffered {} diffs", buffer.len());
                    }
                }

                _ = tokio::time::sleep(Duration::from_millis(100)) => {
                    if loader.wait(Duration::from_millis(10)).await.is_ok() {
                        break;
                    }
                    if start.elapsed() > self.config.snapshot_timeout {
                        error!("snapshot timeout");
                        anyhow::bail!("snapshot timeout");
                    }
                }
            }
        }

        info!("snapshot ready, buffered {} diffs", buffer.len());

        let height = loader.load_into(&self.service).await?;
        let stats = self.service.stats();

        info!(
            "loaded snapshot: height={}, books={}, orders={}",
            height, stats.books, stats.total_orders
        );

        loader.cleanup().await.ok();

        info!("draining buffer");
        let drain_start = Instant::now();
        let mut applied = 0usize;
        let mut skipped = 0usize;

        for diff in buffer {
            match self.service.apply_diff(diff) {
                ApplyResult::Applied => applied += 1,
                ApplyResult::Skipped => skipped += 1,
            }
        }

        info!(
            "buffer drained in {:?}: applied={}, skipped={}",
            drain_start.elapsed(),
            applied,
            skipped
        );

        Ok(())
    }

    async fn periodic_resync(&self) -> Result<()> {
        debug!("starting periodic resync");

        let loader = SnapshotLoader::new(
            &self.config.info_url,
            &self.config.container_snapshot_path,
            &self.config.host_snapshot_path,
        );

        loader.cleanup().await.ok();
        loader.request().await?;
        loader.wait(self.config.snapshot_timeout).await?;

        let height = loader.load_into(&self.service).await?;
        let stats = self.service.stats();

        loader.cleanup().await.ok();

        info!(
            "resync complete: height={}, books={}, orders={}",
            height, stats.books, stats.total_orders
        );

        Ok(())
    }
}