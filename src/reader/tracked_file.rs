use std::path::PathBuf;

use anyhow::Result;
use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, AsyncSeekExt, BufReader, SeekFrom};

pub struct TrackedFile {
    path: PathBuf,
    reader: BufReader<File>,
    partial: String,
}

impl TrackedFile {
    pub async fn open(path: PathBuf) -> Result<Self> {
        let file = File::open(&path).await?;
        Ok(Self {
            path,
            reader: BufReader::new(file),
            partial: String::new(),
        })
    }

    pub async fn open_at_end(path: PathBuf) -> Result<Self> {
        let mut file = File::open(&path).await?;
        file.seek(SeekFrom::End(0)).await?;
        Ok(Self {
            path,
            reader: BufReader::new(file),
            partial: String::new(),
        })
    }

    pub async fn read_lines(&mut self) -> Result<Vec<String>> {
        let mut lines = Vec::new();
        let mut buf = String::new();

        loop {
            buf.clear();
            let bytes_read = self.reader.read_line(&mut buf).await?;

            if bytes_read == 0 {
                break;
            }

            let has_newline = buf.ends_with('\n');
            let content = buf.trim_end();

            if content.is_empty() {
                continue;
            }

            let full_line = if self.partial.is_empty() {
                content.to_owned()
            } else {
                let combined = std::mem::take(&mut self.partial) + content;
                combined
            };

            if has_newline {
                lines.push(full_line);
            } else {
                self.partial = full_line;
            }
        }

        Ok(lines)
    }

    pub fn discard_partial(&mut self) {
        if !self.partial.is_empty() {
            tracing::debug!("discarding {} bytes partial line", self.partial.len());
            self.partial.clear();
        }
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }
}