use sonic_rs::{Deserialize, Serialize};
use super::common::Order;

#[derive(Debug, Deserialize, Serialize)]
pub struct OrderStatus {
    pub time: String,
    pub user: String,
    pub status: String,
    pub order: Order,
}