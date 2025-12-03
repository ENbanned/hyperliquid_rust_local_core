// src/parser/schemas/order_status.rs

use sonic_rs::{Deserialize, Serialize};
use super::common::Order;

#[derive(Debug, Deserialize, Serialize)]
pub struct OrderStatus {
    pub time: String,
    pub user: String,
    pub hash: Option<String>,
    pub builder: Option<OrderBuilder>,
    pub status: String,
    pub order: Order,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OrderBuilder {
    pub b: String,
    pub f: u64,
}