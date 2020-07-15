use crate::message::MessageEncoding;
use crate::message_bus::MessageBus;
use chrono::{DateTime, Utc};
use uuid::Uuid;

/// Every device on the grid implements a minimum set of functionality
pub trait Device<M: MessageEncoding, MB: MessageBus<M>> {
    fn mrid(&self) -> Uuid;
    fn last_update(&self) -> Option<DateTime<Utc>>;
}

pub mod switch;
