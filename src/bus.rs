// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

use crate::error::{PublishResult, SubscribeResult};
use crate::topic::Topic;
use async_trait::async_trait;

/// A publisher provides the functionality needed to publish encodeable
/// messages.
#[async_trait]
pub trait Publisher<M> {
    /// Publish a message of type M to a topic of type T
    async fn publish<S: AsRef<str>, T: Topic<S>>(&mut self, topic: T, msg: M) -> PublishResult<()>;
}

/// Subscriber provides the functionality to subscribe to topics and messages
/// of a specific type on the bus.
#[async_trait]
pub trait Subscriber<M> {
    /// Subscribe to a topic and return a stream of Messages
    async fn subscribe<S: AsRef<str>, T: Topic<S>>(&mut self, topic: T) -> SubscribeResult<M>;
}

/// A message bus provides functionality to publish messages to a topic
/// and subscribe to messages on a topic. The publish and subscribe are
/// typed
pub trait MessageBus<M>: Publisher<M> + Subscriber<M> {}

#[cfg(feature = "nats")]
mod nats;
#[cfg(feature = "nats")]
pub use self::nats::NatsBus;

#[cfg(feature = "zenoh")]
mod zenoh;
#[cfg(feature = "zenoh")]
pub use self::zenoh::ZenohBus;
