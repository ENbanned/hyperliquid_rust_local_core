// src/bin/server.rs

use std::path::PathBuf;
use std::sync::Arc;

use anyhow::Result;
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;
use tracing::{error, info, warn};

use hl_rust_core::api::{self, Envelope, Event, Router};
use hl_rust_core::orderbook::{OrderBookService, Sync, SyncConfig};
use hl_rust_core::parser::schemas::{BookDiff, Fill, MiscEvent, OrderStatus, SystemAction, Trade, TwapStatus};
use hl_rust_core::parser::StreamReader;
use hl_rust_core::transport::ZmqServer;

const VOLUME_PATH: &str = "/var/lib/docker/volumes/hyperliquid_node-data/_data";
const DATA_PATH: &str = "/var/lib/docker/volumes/hyperliquid_node-data/_data/hl/data";

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let cancel = CancellationToken::new();
    let orderbook = Arc::new(OrderBookService::new());

    let (event_tx, mut event_rx) = mpsc::unbounded_channel::<Event>();
    let (diff_tx, diff_rx) = mpsc::channel::<BookDiff>(1_000_000);

    spawn_sync(orderbook.clone(), diff_rx, cancel.clone());
    spawn_book_diff_reader(diff_tx, event_tx.clone(), cancel.clone());
    spawn_readers(event_tx, cancel.clone());

    let mut router = Router::new(orderbook);
    let mut server = ZmqServer::bind("tcp://127.0.0.1:5555", "tcp://127.0.0.1:5556").await?;

    info!("server listening on :5555 (req/rep) and :5556 (pub/sub)");

    loop {
        tokio::select! {
            biased;

            _ = cancel.cancelled() => {
                info!("server shutting down");
                break;
            }

            result = server.recv() => {
                match result {
                    Ok((identity, envelope)) => {
                        let response = router.handle(envelope);
                        if let Err(e) = server.send(identity, response).await {
                            warn!("send error: {}", e);
                        }
                    }
                    Err(e) => {
                        warn!("recv error: {}", e);
                    }
                }
            }

            Some(event) = event_rx.recv() => {
                let topics = router.streams().matching_topics(&event);
                for topic in topics {
                    if let Err(e) = server.publish(&topic, Envelope::event(event.clone())).await {
                        warn!("publish error: {}", e);
                    }
                }
            }

            _ = tokio::signal::ctrl_c() => {
                info!("shutdown signal received");
                cancel.cancel();
            }
        }
    }

    info!("shutdown complete");
    Ok(())
}

fn spawn_sync(
    orderbook: Arc<OrderBookService>,
    diff_rx: mpsc::Receiver<BookDiff>,
    cancel: CancellationToken,
) {
    tokio::spawn(async move {
        let config = SyncConfig::docker_volume(VOLUME_PATH);
        let sync = Sync::new(config, orderbook, diff_rx);

        tokio::select! {
            result = sync.run() => {
                if let Err(e) = result {
                    error!("sync error: {}", e);
                }
            }
            _ = cancel.cancelled() => {
                info!("sync shutting down");
            }
        }
    });
}

fn spawn_book_diff_reader(
    diff_tx: mpsc::Sender<BookDiff>,
    event_tx: mpsc::UnboundedSender<Event>,
    cancel: CancellationToken,
) {
    tokio::spawn(async move {
        let path = PathBuf::from(format!("{}/node_raw_book_diffs", DATA_PATH));

        let mut reader = match StreamReader::<BookDiff>::new(path).await {
            Ok(r) => r,
            Err(e) => {
                error!("failed to create book_diff reader: {}", e);
                return;
            }
        };

        loop {
            tokio::select! {
                biased;

                _ = cancel.cancelled() => {
                    info!("book_diff reader shutting down");
                    break;
                }

                result = reader.next() => {
                    match result {
                        Ok(diff) => {
                            let event = api::events::from_book_diff(&diff);
                            let _ = event_tx.send(event);

                            if diff_tx.send(diff).await.is_err() {
                                break;
                            }
                        }
                        Err(e) => {
                            warn!("book_diff reader error: {}", e);
                        }
                    }
                }
            }
        }
    });
}

fn spawn_readers(tx: mpsc::UnboundedSender<Event>, cancel: CancellationToken) {
    spawn_reader::<Trade>("node_trades", tx.clone(), cancel.clone(), |item| {
        api::events::from_trade(&item)
    });

    spawn_reader::<OrderStatus>("node_order_statuses", tx.clone(), cancel.clone(), |item| {
        vec![api::events::from_order_status(&item)]
    });

    spawn_reader::<Fill>("node_fills", tx.clone(), cancel.clone(), |item| {
        vec![api::events::from_fill(&item)]
    });

    spawn_reader::<TwapStatus>("node_twap_statuses", tx.clone(), cancel.clone(), |item| {
        vec![api::events::from_twap_status(&item)]
    });

    spawn_reader::<MiscEvent>("misc_events", tx.clone(), cancel.clone(), |item| {
        api::events::from_misc_event(&item)
    });

    spawn_reader::<SystemAction>("system_and_core_writer_actions", tx, cancel, |item| {
        vec![api::events::from_system_action(&item)]
    });
}

fn spawn_reader<T>(
    dir: &'static str,
    tx: mpsc::UnboundedSender<Event>,
    cancel: CancellationToken,
    convert: fn(&T) -> Vec<Event>,
)
where
    T: serde::de::DeserializeOwned + Send + 'static,
{
    tokio::spawn(async move {
        let path = PathBuf::from(format!("{}/{}", DATA_PATH, dir));

        let mut reader = match StreamReader::<T>::new(path).await {
            Ok(r) => r,
            Err(e) => {
                error!("failed to create reader for {}: {}", dir, e);
                return;
            }
        };

        loop {
            tokio::select! {
                biased;

                _ = cancel.cancelled() => {
                    info!("{} reader shutting down", dir);
                    break;
                }

                result = reader.next() => {
                    match result {
                        Ok(item) => {
                            for event in convert(&item) {
                                let _ = tx.send(event);
                            }
                        }
                        Err(e) => {
                            warn!("{} reader error: {}", dir, e);
                        }
                    }
                }
            }
        }
    });
}