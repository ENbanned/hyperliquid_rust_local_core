// src/api/streams/mod.rs

mod wallet;

use std::collections::{HashMap, HashSet};

use super::protocol::Event;

pub use wallet::WalletStream;

#[derive(Clone)]
pub struct Subscription {
    pub id: String,
    pub topic: String,
    pub kind: SubscriptionKind,
}

#[derive(Clone)]
pub enum SubscriptionKind {
    Wallet(WalletStream),
}

impl Subscription {
    pub fn matches(&self, event: &Event) -> bool {
        match &self.kind {
            SubscriptionKind::Wallet(s) => s.matches(event),
        }
    }
}

pub struct StreamManager {
    subscriptions: HashMap<String, Subscription>,
    by_topic: HashMap<String, HashSet<String>>,
}

impl StreamManager {
    pub fn new() -> Self {
        Self {
            subscriptions: HashMap::new(),
            by_topic: HashMap::new(),
        }
    }

    pub fn subscribe_wallet(&mut self, address: String) -> Subscription {
        let stream = WalletStream::new(address);
        self.add(stream.topic(), SubscriptionKind::Wallet(stream))
    }

    pub fn unsubscribe(&mut self, id: &str) -> bool {
        let Some(sub) = self.subscriptions.remove(id) else {
            return false;
        };

        if let Some(set) = self.by_topic.get_mut(&sub.topic) {
            set.remove(id);
        }

        true
    }

    pub fn matching_topics(&self, event: &Event) -> Vec<String> {
        self.subscriptions
            .values()
            .filter(|sub| sub.matches(event))
            .map(|sub| sub.topic.clone())
            .collect()
    }

    fn add(&mut self, topic: String, kind: SubscriptionKind) -> Subscription {
        let id = uuid::Uuid::new_v4().to_string();

        let sub = Subscription {
            id: id.clone(),
            topic: topic.clone(),
            kind,
        };

        self.subscriptions.insert(id.clone(), sub.clone());
        self.by_topic.entry(topic).or_default().insert(id);

        sub
    }
}

impl Default for StreamManager {
    fn default() -> Self {
        Self::new()
    }
}