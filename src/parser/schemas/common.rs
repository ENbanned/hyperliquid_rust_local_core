use sonic_rs::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Side {
    #[serde(rename = "B")]
    Buy,
    #[serde(rename = "A")]
    Sell,
    Bid,
    Ask,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Order {
    pub coin: String,
    pub side: Side,
    #[serde(rename = "limitPx")]
    pub limit_px: String,
    pub sz: String,
    pub oid: u64,
    pub timestamp: u64,
    #[serde(rename = "triggerCondition")]
    pub trigger_condition: String,
    #[serde(rename = "isTrigger")]
    pub is_trigger: bool,
    #[serde(rename = "triggerPx")]
    pub trigger_px: String,
    pub children: Vec<String>,
    #[serde(rename = "isPositionTpsl")]
    pub is_position_tpsl: bool,
    #[serde(rename = "reduceOnly")]
    pub reduce_only: bool,
    #[serde(rename = "orderType")]
    pub order_type: String,
    #[serde(rename = "origSz")]
    pub orig_sz: String,
    pub tif: Option<String>,
    pub cloid: Option<String>,
}