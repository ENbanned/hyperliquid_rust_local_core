// parser/stream.rs
use crate::reader::Reader;
use anyhow::Result;
use serde::de::DeserializeOwned;
use std::marker::PhantomData;
use std::path::PathBuf;

pub struct StreamReader<T> {
    reader: Reader,
    _marker: PhantomData<T>,
}

impl<T: DeserializeOwned> StreamReader<T> {
    pub async fn new(path: PathBuf) -> Result<Self> {
        Ok(Self {
            reader: Reader::new(path).await?,
            _marker: PhantomData,
        })
    }

    pub async fn next(&mut self) -> Result<T> {
        let event = self.reader.next_event().await?;
        sonic_rs::from_str(&event.line).map_err(Into::into)
    }
}