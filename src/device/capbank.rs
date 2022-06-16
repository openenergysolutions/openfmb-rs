// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc //
// SPDX-License-Identifier: Apache-2.0

use crate::prelude::*;
use openfmb_messages::{
    capbankmodule::{
        CapBankControlProfile, CapBankDiscreteControlProfile, CapBankEventProfile,
        CapBankReadingProfile, CapBankStatusProfile,
    },
    Module, Profile,
};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct CapBank<MB>
where
    MB: std::fmt::Debug
        + Clone
        + Publisher<CapBankStatusProfile>
        + Publisher<CapBankEventProfile>
        + Publisher<CapBankReadingProfile>
        + Subscriber<CapBankControlProfile>
        + Subscriber<CapBankDiscreteControlProfile>,
{
    bus: MB,
    _mrid: Uuid,
    status_topic: ProfileTopic,
    event_topic: ProfileTopic,
    reading_topic: ProfileTopic,
    control_topic: ProfileTopic,
    discrete_control_topic: ProfileTopic,
}

fn topic(profile: Profile, mrid: &Uuid) -> ProfileTopic {
    ProfileTopic::new(Module::CapBankModule, profile, mrid.clone())
}

impl<MB> CapBank<MB>
where
    MB: std::fmt::Debug
        + Clone
        + Publisher<CapBankStatusProfile>
        + Publisher<CapBankEventProfile>
        + Publisher<CapBankReadingProfile>
        + Subscriber<CapBankControlProfile>
        + Subscriber<CapBankDiscreteControlProfile>,
{
    /// Create a new capbank client instance
    pub fn new(bus: MB, mrid: Uuid) -> CapBank<MB> {
        CapBank {
            bus,
            _mrid: mrid,
            status_topic: topic(Profile::CapBankStatusProfile, &mrid),
            event_topic: topic(Profile::CapBankEventProfile, &mrid),
            reading_topic: topic(Profile::CapBankReadingProfile, &mrid),
            control_topic: topic(Profile::CapBankControlProfile, &mrid),
            discrete_control_topic: topic(Profile::CapBankDiscreteControlProfile, &mrid),
        }
    }

    /// publish capbank status messages
    pub async fn status(&mut self, msg: CapBankStatusProfile) -> PublishResult<()> {
        Ok(self.bus.publish(self.status_topic.iter(), msg).await?)
    }

    /// publish capbank event messages
    pub async fn event(&mut self, msg: CapBankEventProfile) -> PublishResult<()> {
        Ok(self.bus.publish(self.event_topic.iter(), msg).await?)
    }

    /// publish capbank reading messages
    pub async fn reading(&mut self, msg: CapBankReadingProfile) -> PublishResult<()> {
        Ok(self.bus.publish(self.reading_topic.iter(), msg).await?)
    }

    /// subscribe to control messages
    pub async fn control(&mut self) -> SubscribeResult<CapBankControlProfile> {
        self.bus.subscribe(self.control_topic.iter()).await
    }

    /// subscribe to discrete control messages
    pub async fn discrete_control(&mut self) -> SubscribeResult<CapBankDiscreteControlProfile> {
        self.bus.subscribe(self.discrete_control_topic.iter()).await
    }
}
