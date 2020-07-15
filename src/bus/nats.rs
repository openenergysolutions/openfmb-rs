use crate::prelude::*;
use async_trait::async_trait;
use std::marker::PhantomData;
use std::thread;

/// Nats Message Bus
pub struct NatsBus<M: MessageEncoding> {
    conn: nats::Connection,
    encoding: PhantomData<M>,
}

#[derive(Debug)]
pub enum SubscribeError {
    IoError(std::io::Error),
}

#[derive(Debug)]
pub enum SubscriptionError<M: MessageEncoding> {
    IoError(std::io::Error),
    DecodeError(M::DecodeError),
}

impl<M> Subscriber<M> for NatsBus<M>
where
    M: 'static + MessageEncoding + Send,
{
    type SubscribeError = SubscribeError;
    type SubscriptionError = SubscriptionError<M>;

    fn subscribe<T: 'static + Message<M> + Send>(
        &mut self,
        subject: &str,
    ) -> Result<Subscription<T, SubscriptionError<M>>, SubscribeError> {
        let sub = self
            .conn
            .subscribe(subject)
            .map_err(SubscribeError::IoError)?;
        let (mut tx, rx) = futures::channel::mpsc::channel(128);
        thread::spawn(move || {
            for msg in sub {
                let data: &[u8] = &msg.data;
                let res = T::decode(data).map_err(SubscriptionError::DecodeError);
                tx.try_send(res).unwrap();
            }
        });
        Ok(Box::pin(rx))
    }
}

#[derive(Debug)]
pub enum PublishError<M: MessageEncoding> {
    IoError(std::io::Error),
    EncodeError(M::EncodeError),
}

#[async_trait]
impl<M> Publisher<M> for NatsBus<M>
where
    M: 'static + MessageEncoding + Send,
{
    type PublishError = PublishError<M>;

    async fn publish<T: Message<M> + Send>(
        &mut self,
        subject: &str,
        msg: T,
    ) -> Result<(), Self::PublishError> {
        let mut buf = Vec::new();
        msg.encode(&mut buf).map_err(PublishError::EncodeError)?;
        Ok(self
            .conn
            .publish(subject, buf)
            .map_err(PublishError::IoError)?)
    }
}

/// A nats connection implements the MessageBus trait
impl<M> MessageBus<M> for NatsBus<M> where M: 'static + MessageEncoding + Send {}
