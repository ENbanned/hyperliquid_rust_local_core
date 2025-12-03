use sonic_rs::{Deserialize, Serialize};
use super::common::Side;

#[derive(Debug, Deserialize, Serialize)]
pub struct Trade {
    pub coin: String,
    pub side: Side,
    pub time: String,
    pub px: String,
    pub sz: String,
    pub hash: String,
    pub trade_dir_override: String,
    pub side_info: [SideInfo; 2],
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SideInfo {
    pub user: String,
    pub start_pos: String,
    pub oid: u64,
    pub twap_id: Option<u64>,
    pub cloid: Option<String>,
}