use crate::message::{Message, MessageEncoding};
use async_trait::async_trait;
use futures::prelude::*;
use std::pin::Pin;

/// Generalized MessageBus interface
#[async_trait]
pub trait Publisher<M: 'static + MessageEncoding> {
    type PublishError;

    /// Publish a message of type T to a topics
    async fn publish<T: Message<M> + Send>(
        &mut self,
        topic: &str,
        msg: T,
    ) -> Result<(), Self::PublishError>;
}

/// Type alias for the subscription stream
pub type Subscription<T, E> = Pin<Box<dyn Stream<Item = Result<T, E>>>>;

/// Subscriber provides the functionality to subscribe to topics and message
/// of a specific type on the bus.
pub trait Subscriber<M: 'static + MessageEncoding> {
    type SubscribeError;
    type SubscriptionError;

    /// Subscribe to a topic string and return a Stream of results
    //
    /// The idea is that the result type is a stream of decoded typed messages
    /// but that is up to the implemention.
    fn subscribe<T: 'static + Message<M> + Send>(
        &mut self,
        subject: &str,
    ) -> Result<Subscription<T, Self::SubscriptionError>, Self::SubscribeError>;
}

/// A message bus provides functionality to publish messages to a topic
/// and subscribe to messages on a topic. The publish and subscribe are
/// typed and require a MessageEncoding to encode/decode.
pub trait MessageBus<M: 'static + MessageEncoding>: Publisher<M> + Subscriber<M> {}
