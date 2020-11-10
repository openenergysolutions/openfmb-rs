#![deny(clippy::all)]

//! A library for OpenFMB in Rust that provides traits and implementations
//! for common devices and device implementations.
//!
//! Given a typical NATS message bus using protocol buffers a Switch on a bus
//! may be monitored and controlled by creating an appropriate Switch instance
//! as an OpenFMB client
//!
//! The goal API is as follows
//!
//! ```ignore
//! use tokio::main;
//! use openfmb::{prelude::*, encoding::ProtobufEncoding};
//! use futures::StreamExt;
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let nats_url = "localhost:4222";
//!     let mrid = uuid::Uuid::parse_str("06fb668d-f87a-4b1b-8d99-0949513126ff")?;
//!     let nc = nats::asynk::connect(&nats_url).await?;
//!     let bus = openfmb::bus::NatsBus::<ProtobufEncoding>::new(nc);
//!     let mut switch = openfmb::client::Switch::new(bus, mrid);
//!     //
//!     if switch.is_closed().await?.next().await? {
//!         switch.open().await?;
//!     }
//! }
//! ```
//!
//! And also may act as an OpenFMB device which periodically publishes Readings
//! and Statuses
//!
//! ```ignore
//! use openfmb::prelude::*;
//!
//! use tokio::time::{interval_at, Duration, Instant};
//
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let nc = nats::asynk::connect(&nats_url).await?;
//!     let bus = openfmb::bus::NatsBus::<ProtobufEncoding>::new(nc);
//!     let bus = bus::nats::connect::<encoding::protobufs::MessageEncoding>("127.0.0.1").await?;
//!
//!     // initial conditions of the switch to begin publishing status updates
//!     let switch = service::Switch::build(bus)
//!         .mrid("06fb668d-f87a-4b1b-8d99-0949513126ff")
//!         .position(SwitchState::Closed);
//!
//!     // periodically publish SwitchStatusProfile with tokio (could be any
//!     // future executer)
//!     let mut interval = interval_at(Instant::now(), Duration::from_secs(1));
//!     loop {
//!         interval.tick().await;
//!         switch.publish().await?;
//!     }
//! }
//! ```

pub mod error;
pub mod message;
pub mod prelude {
    //! Exports very commonly used types and traits
    pub use crate::bus::{MessageBus, Publisher, Subscriber};
    pub use crate::encoding::MessageEncoding;
    pub use crate::error::{
        ControlError, ControlResult, PublishError, PublishResult, SubscribeError, SubscribeResult,
        Subscription, SubscriptionError,
    };
    pub use crate::message::Message;
}
pub mod bus;
pub mod client;
pub mod device;
pub mod encoding;
