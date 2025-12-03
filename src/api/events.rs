// src/api/events.rs

use sonic_rs::JsonValueTrait;

use crate::parser::schemas::{BookDiff, Fill, MiscEvent, OrderStatus, SystemAction, Trade, TwapStatus};
use crate::parser::schemas::book_diff::RawBookDiff;
use crate::parser::schemas::misc_events::MiscEventInner;

use super::protocol::Event;

pub fn from_book_diff(diff: &BookDiff) -> Event {
    let action = match &diff.raw_book_diff {
        RawBookDiff::New { .. } => "new",
        RawBookDiff::Update { .. } => "update",
        RawBookDiff::Remove(_) => "remove",
    };

    Event::WalletBookDiff {
        address: diff.user.clone(),
        coin: diff.coin.clone(),
        side: format!("{:?}", diff.side),
        price: diff.px.clone(),
        action: action.to_string(),
        oid: diff.oid,
    }
}

pub fn from_trade(trade: &Trade) -> Vec<Event> {
    let mut events = Vec::with_capacity(2);

    for (i, info) in trade.side_info.iter().enumerate() {
        let role = if i == 0 { "buyer" } else { "seller" };
        events.push(Event::WalletTrade {
            address: info.user.clone(),
            coin: trade.coin.clone(),
            side: format!("{:?}", trade.side),
            price: trade.px.clone(),
            size: trade.sz.clone(),
            role: role.to_string(),
        });
    }

    events
}

pub fn from_order_status(status: &OrderStatus) -> Event {
    Event::WalletOrderStatus {
        address: status.user.clone(),
        coin: status.order.coin.clone(),
        side: format!("{:?}", status.order.side),
        status: status.status.clone(),
        oid: status.order.oid,
    }
}

pub fn from_fill(fill: &Fill) -> Event {
    let data = fill.data();
    Event::WalletFill {
        address: fill.user().to_string(),
        coin: data.coin.clone(),
        side: format!("{:?}", data.side),
        price: data.px.clone(),
        size: data.sz.clone(),
        dir: data.dir.clone(),
        closed_pnl: data.closed_pnl.clone(),
        fee: data.fee.clone(),
    }
}

pub fn from_twap_status(twap: &TwapStatus) -> Event {
    let status = match &twap.status {
        crate::parser::schemas::twap_status::TwapStatusValue::Simple(s) => s.clone(),
        crate::parser::schemas::twap_status::TwapStatusValue::Error { error } => format!("error: {}", error),
    };

    Event::WalletTwapStatus {
        address: twap.user().to_string(),
        coin: twap.state.coin.clone(),
        side: format!("{:?}", twap.state.side),
        status,
        twap_id: twap.twap_id,
    }
}

pub fn from_misc_event(event: &MiscEvent) -> Vec<Event> {
    let mut events = Vec::new();
    let raw = sonic_rs::to_string(event).unwrap_or_default();

    match &event.inner {
        MiscEventInner::CDeposit(d) => {
            events.push(Event::WalletMiscEvent {
                address: d.user.clone(),
                event_type: "deposit".to_string(),
                raw: raw.clone(),
            });
        }
        MiscEventInner::Delegation(d) => {
            events.push(Event::WalletMiscEvent {
                address: d.user.clone(),
                event_type: "delegation".to_string(),
                raw: raw.clone(),
            });
        }
        MiscEventInner::CWithdrawal(w) => {
            events.push(Event::WalletMiscEvent {
                address: w.user.clone(),
                event_type: "withdrawal".to_string(),
                raw: raw.clone(),
            });
        }
        MiscEventInner::ValidatorRewards(r) => {
            for (validator, _) in &r.validator_to_reward {
                events.push(Event::WalletMiscEvent {
                    address: validator.clone(),
                    event_type: "validator_reward".to_string(),
                    raw: raw.clone(),
                });
            }
        }
        MiscEventInner::Funding(f) => {
            for delta in &f.deltas {
                events.push(Event::WalletMiscEvent {
                    address: delta.user.clone(),
                    event_type: "funding".to_string(),
                    raw: raw.clone(),
                });
            }
        }
        MiscEventInner::LedgerUpdate(l) => {
            for user in &l.users {
                events.push(Event::WalletMiscEvent {
                    address: user.clone(),
                    event_type: "ledger_update".to_string(),
                    raw: raw.clone(),
                });
            }
        }
    }

    events
}

pub fn from_system_action(action: &SystemAction) -> Event {
    let action_type = action.action
        .get("type")
        .and_then(|v| v.as_str())
        .unwrap_or("unknown")
        .to_string();

    let raw = sonic_rs::to_string(action).unwrap_or_default();

    Event::WalletSystemAction {
        address: action.user.clone(),
        action_type,
        raw,
    }
}