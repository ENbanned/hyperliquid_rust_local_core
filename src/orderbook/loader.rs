use anyhow::{Context, Result};
use reqwest::Client;
use serde::Serialize;
use std::path::Path;
use tokio::fs;
use tokio::time::{sleep, Duration};

use crate::parser::schemas::l4_snapshot::L4Snapshot;

use super::book::CoinBook;
use super::entry::OrderEntry;
use super::service::OrderBookService;

#[derive(Serialize)]
struct SnapshotRequest {
    #[serde(rename = "type")]
    req_type: &'static str,
    request: L4Request,
    #[serde(rename = "outPath")]
    out_path: String,
    #[serde(rename = "includeHeightInOutput")]
    include_height: bool,
}

#[derive(Serialize)]
struct L4Request {
    #[serde(rename = "type")]
    req_type: &'static str,
    #[serde(rename = "includeUsers")]
    include_users: bool,
    #[serde(rename = "includeTriggerOrders")]
    include_trigger_orders: bool,
}

pub struct SnapshotLoader {
    client: Client,
    info_url: String,
    container_path: String,
    host_path: String,
}

impl SnapshotLoader {
    pub fn new(
        info_url: impl Into<String>,
        container_path: impl Into<String>,
        host_path: impl Into<String>,
    ) -> Self {
        Self {
            client: Client::new(),
            info_url: info_url.into(),
            container_path: container_path.into(),
            host_path: host_path.into(),
        }
    }

    pub async fn request(&self) -> Result<()> {
        let payload = SnapshotRequest {
            req_type: "fileSnapshot",
            request: L4Request {
                req_type: "l4Snapshots",
                include_users: true,
                include_trigger_orders: false,
            },
            out_path: self.container_path.clone(),
            include_height: true,
        };

        self.client
            .post(&self.info_url)
            .json(&payload)
            .send()
            .await
            .context("snapshot request failed")?
            .error_for_status()
            .context("snapshot request returned error status")?;

        Ok(())
    }

    pub async fn wait(&self, timeout: Duration) -> Result<()> {
        let path = Path::new(&self.host_path);
        let start = std::time::Instant::now();

        loop {
            if path.exists() {
                let meta = fs::metadata(path).await?;
                if meta.len() > 1000 {
                    sleep(Duration::from_millis(200)).await;
                    return Ok(());
                }
            }

            if start.elapsed() > timeout {
                anyhow::bail!("snapshot timeout after {:?}", timeout);
            }

            sleep(Duration::from_millis(50)).await;
        }
    }

    pub async fn load_into(&self, service: &OrderBookService) -> Result<u64> {
        let bytes = fs::read(&self.host_path)
            .await
            .with_context(|| format!("failed to read snapshot from {}", self.host_path))?;

        let snapshot: L4Snapshot =
            sonic_rs::from_slice(&bytes).context("failed to parse snapshot")?;

        let height = snapshot.block_height();

        for coin_snap in snapshot.coins() {
            let coin = coin_snap.coin().to_string();
            let mut book = CoinBook::new(coin);

            for user_order in coin_snap.book().bids() {
                let entry = OrderEntry::new(
                    user_order.user().to_string(),
                    user_order.order().clone(),
                );
                book.insert(entry);
            }

            for user_order in coin_snap.book().asks() {
                let entry = OrderEntry::new(
                    user_order.user().to_string(),
                    user_order.order().clone(),
                );
                book.insert(entry);
            }

            service.set(book);
        }

        Ok(height)
    }

    pub async fn cleanup(&self) -> Result<()> {
        let path = Path::new(&self.host_path);
        if path.exists() {
            fs::remove_file(path).await?;
        }
        Ok(())
    }
}