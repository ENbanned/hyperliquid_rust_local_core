use std::path::PathBuf;

use anyhow::Result;
use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, AsyncSeekExt, BufReader, SeekFrom};

pub struct TrackedFile {
    path: PathBuf,
    reader: BufReader<File>,
}

impl TrackedFile {
    pub async fn open(path: PathBuf) -> Result<Self> {
        let file = File::open(&path).await?;
        Ok(Self {
            path,
            reader: BufReader::new(file),
        })
    }

    pub async fn open_at_end(path: PathBuf) -> Result<Self> {
        let mut file = File::open(&path).await?;
        file.seek(SeekFrom::End(0)).await?;
        Ok(Self {
            path,
            reader: BufReader::new(file),
        })
    }

    pub async fn read_lines(&mut self) -> Result<Vec<String>> {
        let mut lines = Vec::new();
        let mut buf = String::new();

        loop {
            buf.clear();
            if self.reader.read_line(&mut buf).await? == 0 {
                break;
            }
            let trimmed = buf.trim_end();
            if !trimmed.is_empty() {
                lines.push(trimmed.to_owned());
            }
        }

        Ok(lines)
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }
}