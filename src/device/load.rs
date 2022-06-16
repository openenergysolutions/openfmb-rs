// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc //
// SPDX-License-Identifier: Apache-2.0

use crate::prelude::*;
use openfmb_messages::{
    loadmodule::{LoadControlProfile, LoadEventProfile, LoadReadingProfile, LoadStatusProfile},
    Module, Profile,
};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Load<MB>
where
    MB: std::fmt::Debug
        + Clone
        + Publisher<LoadStatusProfile>
        + Publisher<LoadEventProfile>
        + Publisher<LoadReadingProfile>
        + Subscriber<LoadControlProfile>,
{
    bus: MB,
    _mrid: Uuid,
    status_topic: ProfileTopic,
    event_topic: ProfileTopic,
    reading_topic: ProfileTopic,
    control_topic: ProfileTopic,
}

fn topic(profile: Profile, mrid: &Uuid) -> ProfileTopic {
    ProfileTopic::new(Module::LoadModule, profile, mrid.clone())
}

impl<MB> Load<MB>
where
    MB: std::fmt::Debug
        + Clone
        + Publisher<LoadStatusProfile>
        + Publisher<LoadEventProfile>
        + Publisher<LoadReadingProfile>
        + Subscriber<LoadControlProfile>,
{
    /// Create a new load device instance
    pub fn new(bus: MB, mrid: Uuid) -> Load<MB> {
        Load {
            bus,
            _mrid: mrid,
            status_topic: topic(Profile::LoadStatusProfile, &mrid),
            event_topic: topic(Profile::LoadEventProfile, &mrid),
            reading_topic: topic(Profile::LoadReadingProfile, &mrid),
            control_topic: topic(Profile::LoadControlProfile, &mrid),
        }
    }

    /// publish load status messages
    pub async fn status(&mut self, msg: LoadStatusProfile) -> PublishResult<()> {
        Ok(self.bus.publish(self.status_topic.iter(), msg).await?)
    }

    /// publish load event messages
    pub async fn event(&mut self, msg: LoadEventProfile) -> PublishResult<()> {
        Ok(self.bus.publish(self.event_topic.iter(), msg).await?)
    }

    /// publish load reading messages
    pub async fn reading(&mut self, msg: LoadReadingProfile) -> PublishResult<()> {
        Ok(self.bus.publish(self.reading_topic.iter(), msg).await?)
    }

    /// subscribe to control messages
    pub async fn control(&mut self) -> SubscribeResult<LoadControlProfile> {
        self.bus.subscribe(self.control_topic.iter()).await
    }
}
