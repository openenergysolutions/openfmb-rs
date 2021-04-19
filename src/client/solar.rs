// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

use crate::prelude::*;
use async_trait::async_trait;

pub type SolarResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// Control and wait on updates from a solar device
#[async_trait]
pub trait Solar {
    /// On the next status update checks if the solar is enabled
    /// returning true if so.
    async fn is_enabled(&self) -> SolarResult<bool>;

    /// On the next status update checks if the solar is disabled
    /// returning true if so.
    async fn is_disabled(&self) -> SolarResult<bool> {
        Ok(self.is_enabled().await?)
    }

    /// Enable solar device
    async fn enable(&self) -> SolarResult<()>;

    /// Disable solar device
    async fn disable(&self) -> SolarResult<()>;
}
