// orderbook/entry.rs

use crate::parser::schemas::common::{Order, Side};

#[derive(Debug, Clone)]
pub struct OrderEntry {
    pub user: String,
    pub order: Order,
}

impl OrderEntry {
    pub fn new(user: String, order: Order) -> Self {
        Self { user, order }
    }

    pub fn oid(&self) -> u64 {
        self.order.oid
    }

    pub fn side(&self) -> Side {
        self.order.side
    }

    pub fn price_str(&self) -> &str {
        &self.order.limit_px
    }

    pub fn size_str(&self) -> &str {
        &self.order.sz
    }
}