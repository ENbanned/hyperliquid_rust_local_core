// orderbook/book.rs

use imbl::{HashMap as ImHashMap, OrdMap, Vector};

use crate::parser::schemas::common::Side;

use super::entry::OrderEntry;
use super::price::Price;

#[derive(Debug, Clone, Default)]
pub struct PriceLevel {
    orders: Vector<OrderEntry>,
}

impl PriceLevel {
    pub fn new() -> Self {
        Self {
            orders: Vector::new(),
        }
    }

    pub fn push(&mut self, entry: OrderEntry) {
        self.orders.push_back(entry);
    }

    pub fn remove_by_oid(&mut self, oid: u64) -> Option<OrderEntry> {
        let idx = self.orders.iter().position(|e| e.oid() == oid)?;
        Some(self.orders.remove(idx))
    }

    pub fn find_by_oid(&self, oid: u64) -> Option<&OrderEntry> {
        self.orders.iter().find(|e| e.oid() == oid)
    }

    pub fn update_size(&mut self, oid: u64, new_sz: String) -> bool {
        for entry in self.orders.iter_mut() {
            if entry.oid() == oid {
                entry.order.sz = new_sz;
                return true;
            }
        }
        false
    }

    pub fn is_empty(&self) -> bool {
        self.orders.is_empty()
    }

    pub fn orders(&self) -> &Vector<OrderEntry> {
        &self.orders
    }

    pub fn len(&self) -> usize {
        self.orders.len()
    }
}

#[derive(Debug, Clone)]
struct OidLocation {
    side: Side,
    price: Price,
    user: String,
}

#[derive(Debug, Clone)]
pub struct CoinBook {
    coin: String,
    bids: OrdMap<Price, PriceLevel>,
    asks: OrdMap<Price, PriceLevel>,
    oid_index: ImHashMap<u64, OidLocation>,
}

impl CoinBook {
    pub fn new(coin: String) -> Self {
        Self {
            coin,
            bids: OrdMap::new(),
            asks: OrdMap::new(),
            oid_index: ImHashMap::new(),
        }
    }

    pub fn coin(&self) -> &str {
        &self.coin
    }

    pub fn insert(&mut self, entry: OrderEntry) {
        let oid = entry.oid();

        if self.oid_index.contains_key(&oid) {
            return;
        }

        let price = match Price::parse(entry.price_str()) {
            Some(p) => p,
            None => return,
        };

        let side = entry.side();
        let user = entry.user.clone();

        let levels = match side {
            Side::Bid => &mut self.bids,
            Side::Ask => &mut self.asks,
        };

        levels
            .entry(price)
            .or_insert_with(PriceLevel::new)
            .push(entry);

        self.oid_index.insert(oid, OidLocation { side, price, user });
    }

    pub fn update(&mut self, oid: u64, orig_sz: &str, new_sz: String) -> bool {
        let loc = match self.oid_index.get(&oid) {
            Some(l) => l.clone(),
            None => return false,
        };

        let levels = match loc.side {
            Side::Bid => &mut self.bids,
            Side::Ask => &mut self.asks,
        };

        let level = match levels.get_mut(&loc.price) {
            Some(l) => l,
            None => return false,
        };

        let entry = match level.find_by_oid(oid) {
            Some(e) => e,
            None => return false,
        };

        if entry.order.sz != orig_sz {
            return false;
        }

        level.update_size(oid, new_sz)
    }

    pub fn remove(&mut self, oid: u64) -> Option<OrderEntry> {
        let loc = self.oid_index.remove(&oid)?;

        let levels = match loc.side {
            Side::Bid => &mut self.bids,
            Side::Ask => &mut self.asks,
        };

        let level = levels.get_mut(&loc.price)?;
        let entry = level.remove_by_oid(oid)?;

        if level.is_empty() {
            levels.remove(&loc.price);
        }

        Some(entry)
    }

    pub fn contains(&self, oid: u64) -> bool {
        self.oid_index.contains_key(&oid)
    }

    pub fn get_user(&self, oid: u64) -> Option<&str> {
        self.oid_index.get(&oid).map(|loc| loc.user.as_str())
    }

    pub fn best_bid(&self) -> Option<(&Price, &PriceLevel)> {
        self.bids.get_max().map(|(p, l)| (p, l))
    }

    pub fn best_ask(&self) -> Option<(&Price, &PriceLevel)> {
        self.asks.get_min().map(|(p, l)| (p, l))
    }

    pub fn bids(&self) -> &OrdMap<Price, PriceLevel> {
        &self.bids
    }

    pub fn asks(&self) -> &OrdMap<Price, PriceLevel> {
        &self.asks
    }

    pub fn bids_desc(&self) -> impl Iterator<Item = (&Price, &PriceLevel)> {
        self.bids.iter().rev()
    }

    pub fn asks_asc(&self) -> impl Iterator<Item = (&Price, &PriceLevel)> {
        self.asks.iter()
    }

    pub fn total_orders(&self) -> usize {
        self.oid_index.len()
    }

    pub fn bid_levels(&self) -> usize {
        self.bids.len()
    }

    pub fn ask_levels(&self) -> usize {
        self.asks.len()
    }

    pub fn spread(&self) -> Option<(Price, Price, rust_decimal::Decimal)> {
        let (bid_px, _) = self.best_bid()?;
        let (ask_px, _) = self.best_ask()?;
        let spread = ask_px.as_decimal() - bid_px.as_decimal();
        Some((*bid_px, *ask_px, spread))
    }
}