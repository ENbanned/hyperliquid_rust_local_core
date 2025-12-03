use anyhow::Result;
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;
use tracing::{error, info};

use hl_rust_core::orderbook::{OrderBookService, Sync, SyncConfig};
use hl_rust_core::parser::schemas::BookDiff;
use hl_rust_core::parser::StreamReader;

const VOLUME_PATH: &str = "/var/lib/docker/volumes/hyperliquid_node-data/_data";

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let cancel = CancellationToken::new();
    let service = Arc::new(OrderBookService::new());
    let (tx, rx) = mpsc::channel::<BookDiff>(1_000_000);

    let diff_path = format!("{}/hl/data/node_raw_book_diffs", VOLUME_PATH);
    let mut reader = StreamReader::<BookDiff>::new(diff_path.into()).await?;

    let reader_cancel = cancel.clone();
    let reader_handle = tokio::spawn(async move {
        loop {
            tokio::select! {
                biased;
                _ = reader_cancel.cancelled() => {
                    info!("reader shutting down");
                    break;
                }
                result = reader.next() => {
                    match result {
                        Ok(diff) => {
                            if tx.send(diff).await.is_err() {
                                break;
                            }
                        }
                        Err(e) => error!("reader error: {}", e),
                    }
                }
            }
        }
    });

    let sync_service = service.clone();
    let sync_cancel = cancel.clone();
    let sync_handle = tokio::spawn(async move {
        let config = SyncConfig::docker_volume(VOLUME_PATH);
        let sync = Sync::new(config, sync_service, rx);
        
        tokio::select! {
            result = sync.run() => {
                if let Err(e) = result {
                    error!("sync error: {}", e);
                }
            }
            _ = sync_cancel.cancelled() => {
                info!("sync shutting down");
            }
        }
    });

    let api = service.clone();
    let api_cancel = cancel.clone();
    let api_handle = tokio::spawn(async move {
        loop {
            tokio::select! {
                biased;
                _ = api_cancel.cancelled() => {
                    info!("api logger shutting down");
                    break;
                }
                _ = tokio::time::sleep(tokio::time::Duration::from_secs(5)) => {
                    if let Some(book) = api.get("BTC") {
                        if let Some((bid, ask, spread)) = book.spread() {
                            info!("BTC: {} / {} (spread: {})", bid, ask, spread);
                        }
                    }
                }
            }
        }
    });

    tokio::signal::ctrl_c().await?;
    info!("shutdown signal received");
    cancel.cancel();

    let _ = tokio::time::timeout(
        tokio::time::Duration::from_secs(5),
        futures::future::join_all([reader_handle, sync_handle, api_handle]),
    )
    .await;

    info!("shutdown complete");
    Ok(())
}