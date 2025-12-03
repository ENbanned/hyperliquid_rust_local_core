use sonic_rs::{Deserialize, Serialize};
use super::common::Order;

pub type L4Snapshot = Vec<CoinSnapshot>;

#[derive(Debug, Deserialize, Serialize)]
pub struct CoinSnapshot(pub String, pub BookSnapshot);

#[derive(Debug, Deserialize, Serialize)]
pub struct BookSnapshot(pub Vec<Order>, pub Vec<Order>);