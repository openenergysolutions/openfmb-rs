#![deny(clippy::all)]

pub mod message;
pub mod message_bus;

/// Commonly used types
pub mod prelude {
    pub use crate::device::Device;
    pub use crate::message::{Message, MessageEncoding};
    pub use crate::message_bus::{MessageBus, Publisher, Subscriber, Subscription};
}

pub mod bus;
pub mod device;
pub mod encoding;
