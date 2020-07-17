use crate::prelude::*;
use async_trait::async_trait;
use std::error::Error;
use std::fmt;
use std::marker::PhantomData;
use std::thread;

/// Nats Message Bus
#[derive(Debug, Clone)]
pub struct NatsBus<M: MessageEncoding> {
    conn: nats::Connection,
    encoding: PhantomData<M>,
}

#[derive(Debug)]
pub enum SubscribeError {
    IoError(std::io::Error),
}

impl fmt::Display for SubscribeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for SubscribeError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

#[derive(Debug)]
pub enum SubscriptionError<M: MessageEncoding> {
    IoError(std::io::Error),
    DecodeError(M::DecodeError),
}

impl<M> fmt::Display for SubscriptionError<M>
where
    M: MessageEncoding,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl<M> Error for SubscriptionError<M>
where
    M: MessageEncoding,
{
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

impl<M, T> Subscriber<T> for NatsBus<M>
where
    T: 'static + Message<M> + Send,
    M: 'static + MessageEncoding + Send,
{
    type SubscribeError = SubscribeError;
    type SubscriptionError = SubscriptionError<M>;

    fn subscribe(
        &mut self,
        subject: &str,
    ) -> Result<Subscription<T, SubscriptionError<M>>, SubscribeError> {
        dbg!("subscribing to {:?}", subject);
        let sub = self
            .conn
            .subscribe(subject)
            .map_err(SubscribeError::IoError)?;
        let (mut tx, rx) = futures::channel::mpsc::channel(128);
        thread::spawn(move || {
            dbg!("waiting on message");
            for msg in sub {
                let data: &[u8] = &msg.data;
                let res = T::decode(data).map_err(SubscriptionError::DecodeError);
                dbg!("decoded message, sending");
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

impl<M> fmt::Display for PublishError<M>
where
    M: MessageEncoding,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl<M> Error for PublishError<M>
where
    M: MessageEncoding,
{
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

#[async_trait]
impl<M, T> Publisher<T> for NatsBus<M>
where
    T: 'static + Message<M> + Send,
    M: 'static + MessageEncoding + Send,
{
    type PublishError = PublishError<M>;

    async fn publish(&mut self, subject: &str, msg: T) -> Result<(), Self::PublishError> {
        let mut buf = Vec::new();
        msg.encode(&mut buf).map_err(PublishError::EncodeError)?;
        Ok(self
            .conn
            .publish(subject, buf)
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
    pub fn new(conn: nats::Connection) -> NatsBus<M> {
        NatsBus {
            conn,
            encoding: PhantomData,
        }
    }
}
