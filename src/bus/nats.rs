use crate::prelude::*;
use futures::StreamExt;
use async_trait::async_trait;
use log::debug;
use std::marker::PhantomData;

/// Nats Message Bus
#[derive(Debug, Clone)]
pub struct NatsBus<M: MessageEncoding> {
    conn: nats::asynk::Connection,
    encoding: PhantomData<M>,
}

#[async_trait]
impl<M, T> Subscriber<T> for NatsBus<M>
where
    T: 'static + Message<M> + Send,
    M: 'static + MessageEncoding + Send,
{
    async fn subscribe(&mut self, subject: &str) -> Result<Subscription<T>, SubscribeError> {
        debug!("subscribing to {:?}", subject);
        Ok(Box::pin(self.conn
            .subscribe(subject)
            .await
            .map_err(SubscribeError::IoError)?
        .map(|msg| {
            let data: &[u8] = &msg.data;
            T::decode(data).map_err(|err| SubscriptionError::DecodeError(Box::new(err)))
        })))
    }
}

#[async_trait]
impl<M, T> Publisher<T> for NatsBus<M>
where
    T: 'static + Message<M> + Send,
    M: 'static + MessageEncoding + Send,
{
    async fn publish(&mut self, subject: &str, msg: T) -> PublishResult<()> {
        let mut buf = Vec::new();
        msg.encode(&mut buf)
            .map_err(|err| PublishError::EncodeError(Box::new(err)))?;
        Ok(self
            .conn
            .publish(subject, buf).await
            .map_err(PublishError::IoError)?)
    }
}

/// A nats connection implements the MessageBus trait
impl<M, T> MessageBus<T> for NatsBus<M>
where
    M: 'static + MessageEncoding + Send,
    T: 'static + Send + Message<M>,
{
}

impl<M> NatsBus<M>
where
    M: 'static + MessageEncoding + Send,
{
    pub fn new(conn: nats::asynk::Connection) -> NatsBus<M> {
        NatsBus {
            conn,
            encoding: PhantomData,
        }
    }
}
