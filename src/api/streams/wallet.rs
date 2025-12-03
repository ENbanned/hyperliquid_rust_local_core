// src/api/streams/wallet.rs

use crate::api::protocol::Event;

#[derive(Clone)]
pub struct WalletStream {
    address_lower: String,
}

impl WalletStream {
    pub fn new(address: String) -> Self {
        Self {
            address_lower: address.to_lowercase(),
        }
    }

    pub fn topic(&self) -> String {
        format!("wallet:{}", self.address_lower)
    }

    pub fn matches(&self, event: &Event) -> bool {
        let addr = match event {
            Event::WalletBookDiff { address, .. } => address,
            Event::WalletTrade { address, .. } => address,
            Event::WalletOrderStatus { address, .. } => address,
            Event::WalletFill { address, .. } => address,
            Event::WalletTwapStatus { address, .. } => address,
            Event::WalletMiscEvent { address, .. } => address,
            Event::WalletSystemAction { address, .. } => address,
        };
        addr.to_lowercase() == self.address_lower
    }
}