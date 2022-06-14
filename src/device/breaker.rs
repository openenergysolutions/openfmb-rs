// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc //
// SPDX-License-Identifier: Apache-2.0

use crate::prelude::*;
use openfmb_messages::{
    breakermodule::{
        BreakerDiscreteControlProfile, BreakerEventProfile,
        BreakerReadingProfile, BreakerStatusProfile,
    },
    Module, Profile,
};
use uuid::Uuid;

/// Provide functionality to publish and accept messages a capbank device
/// needs.
#[derive(Debug, Clone)]
pub struct Breaker<MB>
where
    MB: std::fmt::Debug
        + Clone
        + Publisher<BreakerStatusProfile>
        + Publisher<BreakerEventProfile>
        + Publisher<BreakerReadingProfile>
        + Subscriber<BreakerDiscreteControlProfile>,
{
    bus: MB,
    mrid: Uuid,
    status_topic: ProfileTopic,
    event_topic: ProfileTopic,
    reading_topic: ProfileTopic,
    discrete_control_topic: ProfileTopic,
}

fn topic(profile: Profile, mrid: &Uuid) -> ProfileTopic {
    ProfileTopic::new(Module::BreakerModule, profile, mrid.clone())
}

impl<MB> Breaker<MB>
where
    MB: std::fmt::Debug
        + Clone
        + Publisher<BreakerStatusProfile>
        + Publisher<BreakerEventProfile>
        + Publisher<BreakerReadingProfile>
        + Subscriber<BreakerDiscreteControlProfile>,
{
    /// Create a new capbank client instance
    pub fn new(bus: MB, mrid: Uuid) -> Breaker<MB> {
        Breaker {
            bus,
            mrid,
            status_topic: topic(Profile::BreakerStatusProfile, &mrid),
            event_topic: topic(Profile::BreakerEventProfile, &mrid),
            reading_topic: topic(Profile::BreakerReadingProfile, &mrid),
            discrete_control_topic: topic(Profile::BreakerDiscreteControlProfile, &mrid),
        }
    }

    /// publish capbank status messages
    pub async fn status(&mut self, msg: BreakerStatusProfile) -> PublishResult<()> {
        Ok(self.bus.publish(self.status_topic.iter(), msg).await?)
    }

    /// publish capbank event messages
    pub async fn event(&mut self, msg: BreakerEventProfile) -> PublishResult<()> {
        Ok(self.bus.publish(self.event_topic.iter(), msg).await?)
    }

    /// publish capbank reading messages
    pub async fn reading(&mut self, msg: BreakerReadingProfile) -> PublishResult<()> {
        Ok(self.bus.publish(self.reading_topic.iter(), msg).await?)
    }

    /// subscribe to discrete control messages
    pub async fn discrete_control(&mut self) -> SubscribeResult<BreakerDiscreteControlProfile> {
        self.bus.subscribe(self.discrete_control_topic.iter()).await
    }
}
