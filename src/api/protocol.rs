// src/api/protocol.rs

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Envelope {
    pub id: String,
    pub payload: Payload,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Payload {
    Request(Request),
    Response(Response),
    Event(Event),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Request {
    Ping,
    GetSpread { coin: String },
    SubscribeWallet { address: String },
    Unsubscribe { subscription_id: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Response {
    Pong,
    Spread {
        coin: String,
        bid: String,
        ask: String,
        spread_abs: String,
        spread_pct: String,
    },
    Subscribed {
        subscription_id: String,
    },
    Unsubscribed,
    Error {
        message: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Event {
    WalletBookDiff {
        address: String,
        coin: String,
        side: String,
        price: String,
        action: String,
        oid: u64,
    },
    WalletTrade {
        address: String,
        coin: String,
        side: String,
        price: String,
        size: String,
        role: String,
    },
    WalletOrderStatus {
        address: String,
        coin: String,
        side: String,
        status: String,
        oid: u64,
    },
    WalletFill {
        address: String,
        coin: String,
        side: String,
        price: String,
        size: String,
        dir: String,
        closed_pnl: String,
        fee: String,
    },
    WalletTwapStatus {
        address: String,
        coin: String,
        side: String,
        status: String,
        twap_id: u64,
    },
    WalletMiscEvent {
        address: String,
        event_type: String,
        raw: String,
    },
    WalletSystemAction {
        address: String,
        action_type: String,
        raw: String,
    },
}

impl Envelope {
    pub fn response(id: String, response: Response) -> Self {
        Self {
            id,
            payload: Payload::Response(response),
        }
    }

    pub fn event(event: Event) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            payload: Payload::Event(event),
        }
    }

    pub fn serialize(&self) -> Result<Vec<u8>, rmp_serde::encode::Error> {
        rmp_serde::to_vec_named(self)
    }

    pub fn deserialize(bytes: &[u8]) -> Result<Self, rmp_serde::decode::Error> {
        rmp_serde::from_slice(bytes)
    }
}