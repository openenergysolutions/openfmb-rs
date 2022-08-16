// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc //
// SPDX-License-Identifier: Apache-2.0

use crate::prelude::*;
use openfmb_messages::{
    generationmodule::{
        GenerationControlProfile, GenerationEventProfile, GenerationReadingProfile,
        GenerationStatusProfile,
    },
    Module, Profile,
};

use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Generation<MB>
where
    MB: std::fmt::Debug
        + Clone
        + Publisher<GenerationStatusProfile>
        + Publisher<GenerationEventProfile>
        + Publisher<GenerationReadingProfile>
        + Subscriber<GenerationControlProfile>,
{
    bus: MB,
    _mrid: Uuid,
    status_topic: ProfileTopic,
    event_topic: ProfileTopic,
    reading_topic: ProfileTopic,
    discrete_control_topic: ProfileTopic,
}

fn topic(profile: Profile, mrid: &Uuid) -> ProfileTopic {
    ProfileTopic::new(Module::GenerationModule, profile, mrid.clone())
}

impl<MB> Generation<MB>
where
    MB: std::fmt::Debug
        + Clone
        + Publisher<GenerationStatusProfile>
        + Publisher<GenerationEventProfile>
        + Publisher<GenerationReadingProfile>
        + Subscriber<GenerationControlProfile>,
{
    /// Create a new Generation client instance
    pub fn new(bus: MB, mrid: Uuid) -> Generation<MB> {
        Generation {
            bus,
            _mrid: mrid,
            status_topic: topic(Profile::GenerationStatusProfile, &mrid),
            event_topic: topic(Profile::GenerationEventProfile, &mrid),
            reading_topic: topic(Profile::GenerationReadingProfile, &mrid),
            discrete_control_topic: topic(Profile::GenerationControlProfile, &mrid),
        }
    }

    /// publish Generation status messages
    pub async fn status(&mut self, msg: GenerationStatusProfile) -> PublishResult<()> {
        Ok(self.bus.publish(self.status_topic.iter(), msg).await?)
    }

    /// publish Generation event messages
    pub async fn event(&mut self, msg: GenerationEventProfile) -> PublishResult<()> {
        Ok(self.bus.publish(self.event_topic.iter(), msg).await?)
    }

    /// publish Generation reading messages
    pub async fn reading(&mut self, msg: GenerationReadingProfile) -> PublishResult<()> {
        Ok(self.bus.publish(self.reading_topic.iter(), msg).await?)
    }

    /// Subscribe to control messages
    pub async fn control(&mut self) -> SubscribeResult<GenerationControlProfile> {
        self.bus.subscribe(self.discrete_control_topic.iter()).await
    }
}
