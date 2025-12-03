// orderbook/diff.rs

use crate::parser::schemas::book_diff::{BookDiff, RawBookDiff};
use crate::parser::schemas::common::Order;

use super::book::CoinBook;
use super::entry::OrderEntry;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApplyResult {
    Applied,
    Skipped,
}

pub fn apply(book: &mut CoinBook, diff: BookDiff) -> ApplyResult {
    match diff.raw_book_diff {
        RawBookDiff::New { new } => {
            if book.contains(diff.oid) {
                return ApplyResult::Skipped;
            }

            let order = Order {
                coin: diff.coin,
                side: diff.side,
                limit_px: diff.px,
                sz: new.sz,
                oid: diff.oid,
                timestamp: 0,
                trigger_condition: "N/A".to_string(),
                is_trigger: false,
                trigger_px: "0.0".to_string(),
                children: vec![],
                is_position_tpsl: false,
                reduce_only: false,
                order_type: "Limit".to_string(),
                orig_sz: String::new(),
                tif: None,
                cloid: None,
            };

            book.insert(OrderEntry::new(diff.user, order));
            ApplyResult::Applied
        }

        RawBookDiff::Update { update } => {
            if book.update(diff.oid, &update.orig_sz, update.new_sz) {
                ApplyResult::Applied
            } else {
                ApplyResult::Skipped
            }
        }

        RawBookDiff::Remove(_) => {
            if book.remove(diff.oid).is_some() {
                ApplyResult::Applied
            } else {
                ApplyResult::Skipped
            }
        }
    }
}