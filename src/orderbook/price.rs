// orderbook/price.rs

use rust_decimal::Decimal;
use std::cmp::Ordering;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Price(Decimal);

impl Price {
    pub fn new(value: Decimal) -> Self {
        Self(value)
    }

    pub fn parse(s: &str) -> Option<Self> {
        Decimal::from_str(s).ok().map(Self)
    }

    pub fn as_decimal(&self) -> Decimal {
        self.0
    }
}

impl Ord for Price {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl PartialOrd for Price {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl From<Decimal> for Price {
    fn from(value: Decimal) -> Self {
        Self(value)
    }
}

impl std::fmt::Display for Price {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}