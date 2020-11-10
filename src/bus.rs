use async_trait::async_trait;

use crate::error::{PublishResult, SubscribeResult};

/// A publisher provides the functionality needed to publish encodeable
/// messages.
#[async_trait]
pub trait Publisher<T> {
    /// Publish a message of type T to a topics
    async fn publish(&mut self, topic: &str, msg: T) -> PublishResult<()>;
}

/// Subscriber provides the functionality to subscribe to topics and messages
/// of a specific type on the bus.
#[async_trait]
pub trait Subscriber<T> {
    /// Subscribe to a topic string and return a Stream of results
    //
    /// The idea is that the result type is a stream of decoded typed messages
    /// but that is up to the implemention.
    async fn subscribe(&mut self, topic: &str) -> SubscribeResult<T>;
}

/// A message bus provides functionality to publish messages to a topic
/// and subscribe to messages on a topic. The publish and subscribe are
/// typed
pub trait MessageBus<T>: Publisher<T> + Subscriber<T> {}

#[cfg(feature = "nats")]
mod nats;

#[cfg(feature = "nats")]
pub use self::nats::NatsBus;
