use std::path::PathBuf;

use hl_rust_core::reader::Reader;

#[tokio::main]
async fn main() -> anyhow::Result<()> {

    let node_twap_statuses_path = PathBuf::from("/var/lib/docker/volumes/hyperliquid_node-data/_data/hl/data/node_twap_statuses");

    let mut reader = Reader::new(node_twap_statuses_path).await?;

    loop {
        let event = reader.next_event().await?;
        println!("{}: {}", event.source.display(), event.line);
    }
}
