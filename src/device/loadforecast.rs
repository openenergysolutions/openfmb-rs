// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc //
// SPDX-License-Identifier: Apache-2.0

use crate::prelude::*;
use openfmb_messages::{
    loadforecastmodule::{
      LoadForecastProfile,
    },
    Module, Profile,
};


use uuid::Uuid;

/// Provide functionality to publish and accept messages a switch device
/// needs.
#[derive(Debug, Clone)]
pub struct LoadForecast<MB>
where
    MB: std::fmt::Debug
        + Clone
        + Publisher<LoadForecastProfile>,
{
    bus: MB,
    mrid: Uuid,
    forecast_topic: ProfileTopic,
}

fn topic(profile: Profile, mrid: &Uuid) -> ProfileTopic {
    ProfileTopic::new(Module::LoadForecastModule, profile, mrid.clone())
}

impl<MB> LoadForecast<MB>
where
    MB: std::fmt::Debug
        + Clone
        + Publisher<LoadForecastProfile>,
{
    /// Create a new load forecast client instance
    pub fn new(bus: MB, mrid: Uuid) -> LoadForecast<MB> {
        LoadForecast {
            bus,
            mrid,
            forecast_topic: topic(Profile::LoadForecastProfile, &mrid),
        }
    }

    /// publish load forecast messages
    pub async fn forecast(&mut self, msg: LoadForecastProfile) -> PublishResult<()> {
        Ok(self.bus.publish(self.forecast_topic.iter(), msg).await?)
    }

}
