// src/api/router.rs

use std::sync::Arc;

use crate::orderbook::OrderBookService;

use super::protocol::{Envelope, Payload, Request, Response};
use super::queries::QueryRegistry;
use super::streams::StreamManager;

pub struct Router {
    queries: QueryRegistry,
    streams: StreamManager,
}

impl Router {
    pub fn new(orderbook: Arc<OrderBookService>) -> Self {
        Self {
            queries: QueryRegistry::new(orderbook),
            streams: StreamManager::new(),
        }
    }

    pub fn handle(&mut self, envelope: Envelope) -> Envelope {
        let response = match envelope.payload {
            Payload::Request(req) => self.dispatch(req),
            _ => Response::Error {
                message: "expected request".into(),
            },
        };

        Envelope::response(envelope.id, response)
    }

    fn dispatch(&mut self, request: Request) -> Response {
        match &request {
            Request::SubscribeWallet { address } => {
                let sub = self.streams.subscribe_wallet(address.clone());
                Response::Subscribed { subscription_id: sub.id }
            }

            Request::Unsubscribe { subscription_id } => {
                self.streams.unsubscribe(subscription_id);
                Response::Unsubscribed
            }

            _ => self.queries.handle(request),
        }
    }

    pub fn streams(&self) -> &StreamManager {
        &self.streams
    }
}