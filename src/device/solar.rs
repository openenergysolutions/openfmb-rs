// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc //
// SPDX-License-Identifier: Apache-2.0

use crate::prelude::*;
use openfmb_messages::{
    solarmodule::{
        SolarControlProfile, SolarEventProfile, SolarReadingProfile, SolarStatusProfile,
    },
    Module, Profile,
};


use uuid::Uuid;

/// Provide functionality to publish and accept messages a switch device
/// needs.
#[derive(Debug, Clone)]
pub struct Solar<MB>
where
    MB: std::fmt::Debug
        + Clone
        + Publisher<SolarStatusProfile>
        + Publisher<SolarEventProfile>
        + Publisher<SolarReadingProfile>
        + Subscriber<SolarControlProfile>,
{
    bus: MB,
    mrid: Uuid,
    status_topic: ProfileTopic,
    event_topic: ProfileTopic,
    reading_topic: ProfileTopic,
    discrete_control_topic: ProfileTopic,
}

fn topic(profile: Profile, mrid: &Uuid) -> ProfileTopic {
    ProfileTopic::new(Module::SolarModule, profile, mrid.clone())
}

impl<MB> Solar<MB>
where
    MB: std::fmt::Debug
        + Clone
        + Publisher<SolarStatusProfile>
        + Publisher<SolarEventProfile>
        + Publisher<SolarReadingProfile>
        + Subscriber<SolarControlProfile>,
{
    /// Create a new Solar client instance
    pub fn new(bus: MB, mrid: Uuid) -> Solar<MB> {
        Solar {
            bus,
            mrid,
            status_topic: topic(Profile::SolarStatusProfile, &mrid),
            event_topic: topic(Profile::SolarEventProfile, &mrid),
            reading_topic: topic(Profile::SolarReadingProfile, &mrid),
            discrete_control_topic: topic(Profile::SolarControlProfile, &mrid),
        }
    }

    /// publish Solar status messages
    pub async fn status(&mut self, msg: SolarStatusProfile) -> PublishResult<()> {
        Ok(self.bus.publish(self.status_topic.iter(), msg).await?)
    }

    /// publish Solar event messages
    pub async fn event(&mut self, msg: SolarEventProfile) -> PublishResult<()> {
        Ok(self.bus.publish(self.event_topic.iter(), msg).await?)
    }

    /// publish Solar reading messages
    pub async fn reading(&mut self, msg: SolarReadingProfile) -> PublishResult<()> {
        Ok(self.bus.publish(self.reading_topic.iter(), msg).await?)
    }

    /// Subscribe to control messages
    pub async fn control(&mut self) -> SubscribeResult<SolarControlProfile> {
        self.bus.subscribe(self.discrete_control_topic.iter()).await
    }
}
