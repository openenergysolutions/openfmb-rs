use crate::prelude::*;
use async_trait::async_trait;

pub type EssResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// Control and read status from an energy storage (ESS) device
#[async_trait]
pub trait Ess {
    /// On the next status update checks if ess is enabled
    /// returning true if so.
    async fn is_enabled(&self) -> SwitchResult<bool>;

    /// On the next status update checks if the switch is closed
    /// returning true if so.
    async fn is_disabled(&self) -> SwitchResult<bool> {
        Ok(!self.is_enabled().await?)
    }

    /// Enable ESS, errors if the ESS does not enable
    async fn enable(&self) -> SwitchResult<()>;

    /// Disable ESS, errors if ESS does not disable
    async fn disable(&self) -> SwitchResult<()>;
}
