// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc //
// SPDX-License-Identifier: Apache-2.0

use crate::prelude::*;
use openfmb_messages::{
    reclosermodule::{
        RecloserDiscreteControlProfile, RecloserEventProfile, RecloserReadingProfile,
        RecloserStatusProfile,
    },
    Module, Profile,
};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Recloser<MB>
where
    MB: std::fmt::Debug
        + Clone
        + Publisher<RecloserStatusProfile>
        + Publisher<RecloserEventProfile>
        + Publisher<RecloserReadingProfile>
        + Subscriber<RecloserDiscreteControlProfile>,
{
    bus: MB,
    mrid: Uuid,
    status_topic: ProfileTopic,
    event_topic: ProfileTopic,
    reading_topic: ProfileTopic,
    discrete_control_topic: ProfileTopic,
}

fn topic(profile: Profile, mrid: &Uuid) -> ProfileTopic {
    ProfileTopic::new(Module::RecloserModule, profile, mrid.clone())
}

impl<MB> Recloser<MB>
where
    MB: std::fmt::Debug
        + Clone
        + Publisher<RecloserStatusProfile>
        + Publisher<RecloserEventProfile>
        + Publisher<RecloserReadingProfile>
        + Subscriber<RecloserDiscreteControlProfile>,
{
    /// Create a new recloser client instance
    pub fn new(bus: MB, mrid: Uuid) -> Recloser<MB> {
        Recloser {
            bus,
            mrid,
            status_topic: topic(Profile::RecloserStatusProfile, &mrid),
            event_topic: topic(Profile::RecloserEventProfile, &mrid),
            reading_topic: topic(Profile::RecloserReadingProfile, &mrid),
            discrete_control_topic: topic(Profile::RecloserDiscreteControlProfile, &mrid),
        }
    }

    /// publish recloser status messages
    pub async fn status(&mut self, msg: RecloserStatusProfile) -> PublishResult<()> {
        Ok(self.bus.publish(self.status_topic.iter(), msg).await?)
    }

    /// publish recloser event messages
    pub async fn event(&mut self, msg: RecloserEventProfile) -> PublishResult<()> {
        Ok(self.bus.publish(self.event_topic.iter(), msg).await?)
    }

    /// publish recloser reading messages
    pub async fn reading(&mut self, msg: RecloserReadingProfile) -> PublishResult<()> {
        Ok(self.bus.publish(self.reading_topic.iter(), msg).await?)
    }

    /// subscribe to discrete control messages
    pub async fn discrete_control(&mut self) -> SubscribeResult<RecloserDiscreteControlProfile> {
        self.bus.subscribe(self.discrete_control_topic.iter()).await
    }
}
