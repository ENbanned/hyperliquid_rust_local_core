use anyhow::Result;
use hl_rust_core::parser::schemas::BookDiff;
use hl_rust_core::parser::StreamReader;
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<()> {
    let book_diff_path = PathBuf::from("/var/lib/docker/volumes/hyperliquid_node-data/_data/hl/data/node_raw_book_diffs/");

    let mut book_diff_stream = StreamReader::<BookDiff>::new(book_diff_path).await?;


    tokio::spawn(async move {
        loop {
            match book_diff_stream.next().await {
                Ok(diff) => println!("BookDiff: {} {:?} @ {}", diff.coin, diff.side, diff.px),
                Err(e) => eprintln!("BookDiff error: {}", e),
            }
        }
    });

    tokio::signal::ctrl_c().await?;
    Ok(())
}