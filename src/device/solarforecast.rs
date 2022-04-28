// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc //
// SPDX-License-Identifier: Apache-2.0

use crate::prelude::*;
use openfmb_messages::{
    solarforecastmodule::{
      SolarForecastProfile,
    },
    Module, Profile,
};


use uuid::Uuid;

/// Provide functionality to publish and accept messages a switch device
/// needs.
#[derive(Debug, Clone)]
pub struct SolarForecast<MB>
where
    MB: std::fmt::Debug
        + Clone
        + Publisher<SolarForecastProfile>,
{
    bus: MB,
    mrid: Uuid,
    forecast_topic: ProfileTopic,
}

fn topic(profile: Profile, mrid: &Uuid) -> ProfileTopic {
    ProfileTopic::new(Module::SolarForecastModule, profile, mrid.clone())
}

impl<MB> SolarForecast<MB>
where
    MB: std::fmt::Debug
        + Clone
        + Publisher<SolarForecastProfile>,
{
    /// Create a new solar forecast client instance
    pub fn new(bus: MB, mrid: Uuid) -> SolarForecast<MB> {
        SolarForecast {
            bus,
            mrid,
            forecast_topic: topic(Profile::SolarForecastProfile, &mrid),
        }
    }

    /// publish solar forecast messages
    pub async fn forecast(&mut self, msg: SolarForecastProfile) -> PublishResult<()> {
        Ok(self.bus.publish(self.forecast_topic.iter(), msg).await?)
    }

}
