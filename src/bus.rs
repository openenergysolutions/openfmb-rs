use async_trait::async_trait;
use futures::prelude::*;
use std::pin::Pin;

/// A publisher provides the functionality needed to publish encodeable
/// messages.
#[async_trait]
pub trait Publisher<T> {
    type PublishError: std::error::Error;

    /// Publish a message of type T to a topics
    async fn publish(&mut self, topic: &str, msg: T) -> Result<(), Self::PublishError>;
}

/// Type alias for the subscription stream
pub type Subscription<T, E> = Pin<Box<dyn Stream<Item = Result<T, E>>>>;

/// Subscriber provides the functionality to subscribe to topics and messages
/// of a specific type on the bus.
pub trait Subscriber<T> {
    type SubscribeError: std::error::Error;
    type SubscriptionError: std::error::Error;

    /// Subscribe to a topic string and return a Stream of results
    //
    /// The idea is that the result type is a stream of decoded typed messages
    /// but that is up to the implemention.
    fn subscribe(
        &mut self,
        subject: &str,
    ) -> Result<Subscription<T, Self::SubscriptionError>, Self::SubscribeError>;
}

/// A message bus provides functionality to publish messages to a topic
/// and subscribe to messages on a topic. The publish and subscribe are
/// typed and require a MessageEncoding to encode/decode.
pub trait MessageBus<T>: Publisher<T> + Subscriber<T> {}

#[cfg(feature = "nats")]
mod nats;

#[cfg(feature = "nats")]
pub use self::nats::NatsBus;
