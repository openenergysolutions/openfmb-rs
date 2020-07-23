use std::error::Error;
use std::fmt;
use async_trait::async_trait;
use futures::prelude::*;
use std::pin::Pin;


/// A common publish error type with erased enclosed error types
#[derive(Debug)]
pub enum PublishError {
    IoError(std::io::Error),
    EncodeError(Box<dyn std::error::Error>),
    BusError(Box<dyn std::error::Error>),
}

impl fmt::Display for PublishError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for PublishError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            PublishError::IoError(ref err) => Some(err),
            _ => None,
        }
    }
}

/// Type alias for a publish result
pub type PublishResult<T, E=PublishError> = Result<T, E>;

/// A publisher provides the functionality needed to publish encodeable
/// messages.
#[async_trait]
pub trait Publisher<T> {
    /// Publish a message of type T to a topics
    async fn publish(&mut self, topic: &str, msg: T) -> PublishResult<()>;
}

/// A common publish error type with erased enclosed error types, useful
/// for matching on and correcting issues potentially.
#[derive(Debug)]
pub enum SubscriptionError {
    IoError(std::io::Error),
    Unsubscribed(Box<(dyn std::error::Error + Send)>),
    DecodeError(Box<(dyn std::error::Error + Send)>),
    BusError(Box<(dyn std::error::Error + Send)>),
}

impl fmt::Display for SubscriptionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for SubscriptionError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            SubscriptionError::IoError(ref err) => Some(err),
            _ => None,
        }
    }
}


/// A common publish error type with erased enclosed error types, useful
/// for matching on and correcting issues potentially.
#[derive(Debug)]
pub enum SubscribeError {
    IoError(std::io::Error),
    BusError(Box<dyn std::error::Error>),
    InvalidTopic(String),
}

impl fmt::Display for SubscribeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for SubscribeError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            SubscribeError::IoError(ref err) => Some(err),
            _ => None,
        }
    }
}

/// Type alias for the subscription stream
pub type Subscription<T> = Pin<Box<dyn Stream<Item = Result<T, SubscriptionError>>>>;

/// Type alias for a publish result
pub type SubscribeResult<T, E=SubscribeError> = Result<Subscription<T>, E>;

/// Subscriber provides the functionality to subscribe to topics and messages
/// of a specific type on the bus.
pub trait Subscriber<T> {
    /// Subscribe to a topic string and return a Stream of results
    //
    /// The idea is that the result type is a stream of decoded typed messages
    /// but that is up to the implemention.
    fn subscribe(
        &mut self,
        topic: &str,
    ) -> SubscribeResult<T>;
}

/// A message bus provides functionality to publish messages to a topic
/// and subscribe to messages on a topic. The publish and subscribe are
/// typed and require a MessageEncoding to encode/decode.
pub trait MessageBus<T>: Publisher<T> + Subscriber<T> {}

#[cfg(feature = "nats")]
mod nats;

#[cfg(feature = "nats")]
pub use self::nats::NatsBus;
