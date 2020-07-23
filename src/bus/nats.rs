use crate::prelude::*;
use async_trait::async_trait;
use std::marker::PhantomData;
use std::thread;

/// Nats Message Bus
#[derive(Debug, Clone)]
pub struct NatsBus<M: MessageEncoding> {
    conn: nats::Connection,
    encoding: PhantomData<M>,
}

impl<M, T> Subscriber<T> for NatsBus<M>
where
    T: 'static + Message<M> + Send,
    M: 'static + MessageEncoding + Send,
{
    fn subscribe(
        &mut self,
        subject: &str,
    ) -> Result<Subscription<T>, SubscribeError> {
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
                let res = T::decode(data).map_err(|err| SubscriptionError::DecodeError(Box::new(err)));
                dbg!("decoded message, sending");
                tx.try_send(res).unwrap();
            }
        });
        Ok(Box::pin(rx))
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
        msg.encode(&mut buf).map_err(|err| PublishError::EncodeError(Box::new(err)))?;
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
