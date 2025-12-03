// src/api/queries/spread.rs

use crate::api::protocol::Response;
use super::QueryContext;

pub fn handle(ctx: &QueryContext, coin: &str) -> Response {
    let Some(book) = ctx.orderbook.get(coin) else {
        return Response::Error {
            message: format!("coin {} not found", coin),
        };
    };

    let Some((bid, ask, spread_abs)) = book.spread() else {
        return Response::Error {
            message: "no spread available".into(),
        };
    };

    let mid = (bid.as_decimal() + ask.as_decimal()) / rust_decimal::Decimal::TWO;
    let spread_pct = if mid.is_zero() {
        rust_decimal::Decimal::ZERO
    } else {
        (spread_abs / mid) * rust_decimal::Decimal::ONE_HUNDRED
    };

    Response::Spread {
        coin: coin.to_string(),
        bid: bid.to_string(),
        ask: ask.to_string(),
        spread_abs: spread_abs.to_string(),
        spread_pct: format!("{:.4}", spread_pct),
    }
}