use crate::prelude::*;
use async_trait::async_trait;

/// Position of Switch
pub enum SwitchPosition {
    Open,
    Closed,
    Transient,
}

pub type SwitchResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// Control and read status from a switch
#[async_trait]
pub trait Switch<M: MessageEncoding, MB: MessageBus<M>>: Device<M, MB> {
    async fn is_open(&self) -> SwitchResult<bool>;
    async fn is_closed(&self) -> SwitchResult<bool>;
    async fn is_transient(&self) -> SwitchResult<bool>;
    async fn position(&self) -> SwitchResult<SwitchPosition>;

    /// Open a switch, errors if the switch does not open or takes too long
    async fn open(&self) -> SwitchResult<()>;

    /// Close a switch, errors if the switch does not open or takes too long
    async fn close(&self) -> SwitchResult<()>;
}
