// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc //
// SPDX-License-Identifier: Apache-2.0

use crate::prelude::*;
use openfmb_messages::{
    priceforecastmodule::{
      PriceForecastProfile,
    },
    Module, Profile,
};


use uuid::Uuid;

/// Provide functionality to publish and accept messages a switch device
/// needs.
#[derive(Debug, Clone)]
pub struct PriceForecast<MB>
where
    MB: std::fmt::Debug
        + Clone
        + Publisher<PriceForecastProfile>,
{
    bus: MB,
    mrid: Uuid,
    forecast_topic: ProfileTopic,
}

fn topic(profile: Profile, mrid: &Uuid) -> ProfileTopic {
    ProfileTopic::new(Module::PriceForecastModule, profile, mrid.clone())
}

impl<MB> PriceForecast<MB>
where
    MB: std::fmt::Debug
        + Clone
        + Publisher<PriceForecastProfile>,
{
    /// Create a new price forecast client instance
    pub fn new(bus: MB, mrid: Uuid) -> PriceForecast<MB> {
        PriceForecast {
            bus,
            mrid,
            forecast_topic: topic(Profile::PriceForecastProfile, &mrid),
        }
    }

    /// publish price forecast messages
    pub async fn forecast(&mut self, msg: PriceForecastProfile) -> PublishResult<()> {
        Ok(self.bus.publish(self.forecast_topic.iter(), msg).await?)
    }

}
