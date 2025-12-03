// src/parser/schemas/fill.rs

use sonic_rs::{Deserialize, Serialize};
use super::common::Side;

#[derive(Debug, Deserialize, Serialize)]
pub struct Fill(pub String, pub FillData);

impl Fill {
    pub fn user(&self) -> &str {
        &self.0
    }

    pub fn data(&self) -> &FillData {
        &self.1
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FillData {
    pub coin: String,
    pub px: String,
    pub sz: String,
    pub side: Side,
    pub time: u64,
    #[serde(rename = "startPosition")]
    pub start_position: String,
    pub dir: String,
    #[serde(rename = "closedPnl")]
    pub closed_pnl: String,
    pub hash: String,
    pub oid: u64,
    pub crossed: bool,
    pub fee: String,
    pub tid: u64,
    #[serde(rename = "feeToken")]
    pub fee_token: String,
    pub cloid: Option<String>,
    #[serde(rename = "twapId")]
    pub twap_id: Option<u64>,
    #[serde(rename = "builderFee")]
    pub builder_fee: Option<String>,
    pub builder: Option<String>,
}