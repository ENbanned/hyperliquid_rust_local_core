use std::collections::VecDeque;
use std::path::PathBuf;

use anyhow::{Result, anyhow};
use notify::{Event, RecommendedWatcher, RecursiveMode, Watcher};
use tokio::sync::mpsc::{UnboundedReceiver, unbounded_channel};

use crate::reader::file_rotation::{extract_timestamp, find_latest_file, is_valid_hourly_file};
use crate::reader::tracked_file::TrackedFile;

pub struct FileEvent {
    pub source: PathBuf,
    pub line: String,
}

pub struct Reader {
    base_path: PathBuf,
    file: Option<TrackedFile>,
    fs_rx: UnboundedReceiver<notify::Result<Event>>,
    pending: VecDeque<FileEvent>,
    _watcher: RecommendedWatcher,
}

impl Reader {
    pub async fn new(base_path: PathBuf) -> Result<Self> {
        let (tx, rx) = unbounded_channel();
        let mut watcher = notify::recommended_watcher(move |res| {
            let _ = tx.send(res);
        })?;

        let hourly_dir = base_path.join("hourly");
        let file = match find_latest_file(&base_path).await {
            Ok(latest) => Some(TrackedFile::open_at_end(latest).await?),
            Err(_) => None,
        };

        if hourly_dir.exists() {
            let canonical = hourly_dir.canonicalize()?;
            watcher.watch(&canonical, RecursiveMode::Recursive)?;
        }

        Ok(Self {
            base_path,
            file,
            fs_rx: rx,
            pending: VecDeque::new(),
            _watcher: watcher,
        })
    }

    pub async fn next_event(&mut self) -> Result<FileEvent> {
        loop {
            if let Some(event) = self.pending.pop_front() {
                return Ok(event);
            }

            match self.fs_rx.recv().await {
                Some(Ok(event)) => self.handle_fs_event(event).await?,
                Some(Err(e)) => return Err(e.into()),
                None => return Err(anyhow!("File watcher channel closed")),
            }
        }
    }

    pub fn try_next_event(&mut self) -> Option<FileEvent> {
        self.pending.pop_front()
    }

    async fn handle_fs_event(&mut self, event: Event) -> Result<()> {
        if !event.kind.is_create() && !event.kind.is_modify() {
            return Ok(());
        }

        for path in event.paths {
            if !path.is_file() {
                continue;
            }

            if !path.starts_with(&self.base_path) {
                continue;
            }

            let current_path = self.file.as_ref().map(|f| f.path().clone());

            if event.kind.is_create() && is_valid_hourly_file(&path) && self.should_rotate(&path) {
                self.rotate(path).await?;
            } else if event.kind.is_modify() {
                match &current_path {
                    Some(p) if *p == path => {
                        self.read_into_pending().await?;
                    }
                    None if is_valid_hourly_file(&path) => {
                        let tracked = TrackedFile::open_at_end(path).await?;
                        self.file = Some(tracked);
                    }
                    _ => {}
                }
            }
        }

        Ok(())
    }

    fn should_rotate(&self, new_path: &PathBuf) -> bool {
        let Some(current) = &self.file else {
            return true;
        };
        match (extract_timestamp(current.path()), extract_timestamp(new_path)) {
            (Some(cur), Some(new)) => new > cur,
            (None, Some(_)) => true,
            _ => false,
        }
    }

    async fn rotate(&mut self, new_path: PathBuf) -> Result<()> {
        self.read_into_pending().await?;

        let tracked = TrackedFile::open(new_path).await?;
        self.file = Some(tracked);

        self.read_into_pending().await?;
        Ok(())
    }

    async fn read_into_pending(&mut self) -> Result<()> {
        let Some(file) = &mut self.file else {
            return Ok(());
        };

        let source = file.path().clone();
        for line in file.read_lines().await? {
            self.pending.push_back(FileEvent {
                source: source.clone(),
                line,
            });
        }

        Ok(())
    }
}