// src/api/queries/mod.rs

mod spread;

use std::sync::Arc;

use crate::orderbook::OrderBookService;
use super::protocol::{Request, Response};

pub struct QueryContext {
    pub orderbook: Arc<OrderBookService>,
}

pub struct QueryRegistry {
    ctx: QueryContext,
}

impl QueryRegistry {
    pub fn new(orderbook: Arc<OrderBookService>) -> Self {
        Self {
            ctx: QueryContext { orderbook },
        }
    }

    pub fn handle(&self, request: Request) -> Response {
        match request {
            Request::Ping => Response::Pong,
            Request::GetSpread { coin } => spread::handle(&self.ctx, &coin),
            _ => Response::Error {
                message: "unknown query".into(),
            },
        }
    }
}