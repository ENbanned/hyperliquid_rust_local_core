use std::path::{Path, PathBuf};

use anyhow::{Result, anyhow};
use tokio::fs;

pub async fn find_latest_file(base_path: &Path) -> Result<PathBuf> {
    let hourly_path = base_path.join("hourly");
    let mut latest: Option<(u64, PathBuf)> = None;

    let mut date_dirs = fs::read_dir(&hourly_path).await?;
    while let Some(date_entry) = date_dirs.next_entry().await? {
        if !date_entry.file_type().await?.is_dir() {
            continue;
        }

        let date_path = date_entry.path();
        let Some(date) = parse_component(date_path.file_name()) else {
            continue;
        };

        let mut hour_files = fs::read_dir(&date_path).await?;
        while let Some(hour_entry) = hour_files.next_entry().await? {
            if !hour_entry.file_type().await?.is_file() {
                continue;
            }

            let hour_path = hour_entry.path();
            let Some(hour) = parse_component(hour_path.file_name()) else {
                continue;
            };

            if hour >= 24 {
                continue;
            }

            let ts = date * 100 + hour;
            if latest.as_ref().is_none_or(|(best, _)| ts > *best) {
                latest = Some((ts, hour_path));
            }
        }
    }

    latest
        .map(|(_, path)| path)
        .ok_or_else(|| anyhow!("No hourly files in {}", hourly_path.display()))
}

pub fn extract_timestamp(path: &Path) -> Option<u64> {
    let hour = parse_component(path.file_name())?;
    let date = parse_component(path.parent()?.file_name())?;
    (hour < 24).then_some(date * 100 + hour)
}

pub fn is_valid_hourly_file(path: &Path) -> bool {
    extract_timestamp(path).is_some()
}

fn parse_component(s: Option<&std::ffi::OsStr>) -> Option<u64> {
    s?.to_str()?.parse().ok()
}