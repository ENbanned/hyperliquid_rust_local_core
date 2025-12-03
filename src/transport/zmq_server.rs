// src/transport/zmq_server.rs

use anyhow::Result;
use bytes::Bytes;
use zeromq::{PubSocket, RouterSocket, Socket, SocketRecv, SocketSend, ZmqMessage};

use crate::api::Envelope;

pub struct ZmqServer {
    router: RouterSocket,
    publisher: PubSocket,
}

impl ZmqServer {
    pub async fn bind(router_addr: &str, pub_addr: &str) -> Result<Self> {
        let mut router = RouterSocket::new();
        router.bind(router_addr).await?;

        let mut publisher = PubSocket::new();
        publisher.bind(pub_addr).await?;

        Ok(Self { router, publisher })
    }

    pub async fn recv(&mut self) -> Result<(Vec<u8>, Envelope)> {
        let msg = self.router.recv().await?;
        let frames: Vec<Bytes> = msg.into_vec();

        if frames.len() < 2 {
            anyhow::bail!("invalid message format");
        }

        let identity = frames[0].to_vec();
        let data = frames.last().unwrap();
        let envelope = Envelope::deserialize(data)?;

        Ok((identity, envelope))
    }

    pub async fn send(&mut self, identity: Vec<u8>, envelope: Envelope) -> Result<()> {
        let data = envelope.serialize()?;
        let frames: Vec<Bytes> = vec![Bytes::from(identity), Bytes::from(data)];
        let msg = ZmqMessage::try_from(frames).map_err(|e| anyhow::anyhow!("{:?}", e))?;
        self.router.send(msg).await?;
        Ok(())
    }

    pub async fn publish(&mut self, topic: &str, envelope: Envelope) -> Result<()> {
        let data = envelope.serialize()?;
        let mut payload = topic.as_bytes().to_vec();
        payload.push(0);
        payload.extend(data);
        let frames: Vec<Bytes> = vec![Bytes::from(payload)];
        let msg = ZmqMessage::try_from(frames).map_err(|e| anyhow::anyhow!("{:?}", e))?;
        self.publisher.send(msg).await?;
        Ok(())
    }
}