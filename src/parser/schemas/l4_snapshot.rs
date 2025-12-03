// parser/schemas/l4_snapshot.rs

use sonic_rs::{Deserialize, Serialize};
use super::common::Order;

#[derive(Debug, Deserialize, Serialize)]
pub struct L4Snapshot(pub u64, pub Vec<CoinSnapshot>);

impl L4Snapshot {
    pub fn block_height(&self) -> u64 {
        self.0
    }

    pub fn coins(&self) -> &[CoinSnapshot] {
        &self.1
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CoinSnapshot(pub String, pub BookSnapshot);

impl CoinSnapshot {
    pub fn coin(&self) -> &str {
        &self.0
    }

    pub fn book(&self) -> &BookSnapshot {
        &self.1
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BookSnapshot(pub Vec<UserOrder>, pub Vec<UserOrder>);

impl BookSnapshot {
    pub fn bids(&self) -> &[UserOrder] {
        &self.0
    }

    pub fn asks(&self) -> &[UserOrder] {
        &self.1
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserOrder(pub String, pub Order);

impl UserOrder {
    pub fn user(&self) -> &str {
        &self.0
    }

    pub fn order(&self) -> &Order {
        &self.1
    }
}