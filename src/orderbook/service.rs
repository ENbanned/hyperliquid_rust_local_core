// orderbook/service.rs

use arc_swap::ArcSwap;
use dashmap::DashMap;
use std::sync::Arc;

use crate::parser::schemas::book_diff::BookDiff;

use super::book::CoinBook;
use super::diff::{apply, ApplyResult};

pub struct OrderBookService {
    books: DashMap<String, ArcSwap<CoinBook>>,
}

impl OrderBookService {
    pub fn new() -> Self {
        Self {
            books: DashMap::new(),
        }
    }

    pub fn get(&self, coin: &str) -> Option<Arc<CoinBook>> {
        self.books.get(coin).map(|e| e.load_full())
    }

    pub fn set(&self, book: CoinBook) {
        let coin = book.coin().to_string();
        self.books
            .entry(coin)
            .and_modify(|swap| swap.store(Arc::new(book.clone())))
            .or_insert_with(|| ArcSwap::from_pointee(book));
    }

    pub fn apply_diff(&self, diff: BookDiff) -> ApplyResult {
        let coin = diff.coin.clone();

        let swap = self
            .books
            .entry(coin.clone())
            .or_insert_with(|| ArcSwap::from_pointee(CoinBook::new(coin)));

        let current = swap.load();
        let mut updated = (**current).clone();

        let result = apply(&mut updated, diff);

        if result == ApplyResult::Applied {
            swap.store(Arc::new(updated));
        }

        result
    }

    pub fn coins(&self) -> Vec<String> {
        self.books.iter().map(|r| r.key().clone()).collect()
    }

    pub fn len(&self) -> usize {
        self.books.len()
    }

    pub fn is_empty(&self) -> bool {
        self.books.is_empty()
    }

    pub fn stats(&self) -> Stats {
        let mut total_orders = 0;
        let mut bid_levels = 0;
        let mut ask_levels = 0;

        for entry in self.books.iter() {
            let book = entry.load();
            total_orders += book.total_orders();
            bid_levels += book.bid_levels();
            ask_levels += book.ask_levels();
        }

        Stats {
            books: self.books.len(),
            total_orders,
            bid_levels,
            ask_levels,
        }
    }
}

impl Default for OrderBookService {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct Stats {
    pub books: usize,
    pub total_orders: usize,
    pub bid_levels: usize,
    pub ask_levels: usize,
}