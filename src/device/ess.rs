// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc //
// SPDX-License-Identifier: Apache-2.0

use crate::prelude::*;
use openfmb_messages::{
    essmodule::{EssControlProfile, EssEventProfile, EssReadingProfile, EssStatusProfile},
    Module, Profile,
};

use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Ess<MB>
where
    MB: std::fmt::Debug
        + Clone
        + Publisher<EssStatusProfile>
        + Publisher<EssEventProfile>
        + Publisher<EssReadingProfile>
        + Subscriber<EssControlProfile>,
{
    bus: MB,
    _mrid: Uuid,
    status_topic: ProfileTopic,
    event_topic: ProfileTopic,
    reading_topic: ProfileTopic,
    discrete_control_topic: ProfileTopic,
}

fn topic(profile: Profile, mrid: &Uuid) -> ProfileTopic {
    ProfileTopic::new(Module::EssModule, profile, mrid.clone())
}

impl<MB> Ess<MB>
where
    MB: std::fmt::Debug
        + Clone
        + Publisher<EssStatusProfile>
        + Publisher<EssEventProfile>
        + Publisher<EssReadingProfile>
        + Subscriber<EssControlProfile>,
{
    /// Create a new Ess client instance
    pub fn new(bus: MB, mrid: Uuid) -> Ess<MB> {
        Ess {
            bus,
            _mrid: mrid,
            status_topic: topic(Profile::ESSStatusProfile, &mrid),
            event_topic: topic(Profile::ESSEventProfile, &mrid),
            reading_topic: topic(Profile::ESSReadingProfile, &mrid),
            discrete_control_topic: topic(Profile::ESSControlProfile, &mrid),
        }
    }

    /// publish Ess status messages
    pub async fn status(&mut self, msg: EssStatusProfile) -> PublishResult<()> {
        Ok(self.bus.publish(self.status_topic.iter(), msg).await?)
    }

    /// publish Ess event messages
    pub async fn event(&mut self, msg: EssEventProfile) -> PublishResult<()> {
        Ok(self.bus.publish(self.event_topic.iter(), msg).await?)
    }

    /// publish Ess reading messages
    pub async fn reading(&mut self, msg: EssReadingProfile) -> PublishResult<()> {
        Ok(self.bus.publish(self.reading_topic.iter(), msg).await?)
    }

    /// Subscribe to control messages
    pub async fn control(&mut self) -> SubscribeResult<EssControlProfile> {
        self.bus.subscribe(self.discrete_control_topic.iter()).await
    }
}
