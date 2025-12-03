// src/parser/schemas/twap_status.rs

use sonic_rs::{Deserialize, Serialize};
use super::common::Side;

#[derive(Debug, Deserialize, Serialize)]
pub struct TwapStatus {
    pub time: String,
    pub twap_id: u64,
    pub state: TwapState,
    pub status: TwapStatusValue,
}

impl TwapStatus {
    pub fn user(&self) -> &str {
        &self.state.user
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TwapState {
    pub coin: String,
    pub user: String,
    pub side: Side,
    pub sz: String,
    #[serde(rename = "executedSz")]
    pub executed_sz: String,
    #[serde(rename = "executedNtl")]
    pub executed_ntl: String,
    pub minutes: u64,
    #[serde(rename = "reduceOnly")]
    pub reduce_only: bool,
    pub randomize: bool,
    pub timestamp: u64,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum TwapStatusValue {
    Simple(String),
    Error { error: String },
}