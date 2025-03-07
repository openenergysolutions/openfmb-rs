// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc //
// SPDX-License-Identifier: Apache-2.0

use crate::prelude::*;
use openfmb_messages::{
    regulatormodule::{
        RegulatorControlProfile, RegulatorDiscreteControlProfile, RegulatorEventProfile,
        RegulatorReadingProfile, RegulatorStatusProfile,
    },
    Module, Profile,
};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Regulator<MB>
where
    MB: std::fmt::Debug
        + Clone
        + Publisher<RegulatorStatusProfile>
        + Publisher<RegulatorEventProfile>
        + Publisher<RegulatorReadingProfile>
        + Subscriber<RegulatorControlProfile>
        + Subscriber<RegulatorDiscreteControlProfile>,
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
    ProfileTopic::new(Module::RegulatorModule, profile, *mrid)
}

impl<MB> Regulator<MB>
where
    MB: std::fmt::Debug
        + Clone
        + Publisher<RegulatorStatusProfile>
        + Publisher<RegulatorEventProfile>
        + Publisher<RegulatorReadingProfile>
        + Subscriber<RegulatorControlProfile>
        + Subscriber<RegulatorDiscreteControlProfile>,
{
    /// Create a new regulator client instance
    pub fn new(bus: MB, mrid: Uuid) -> Regulator<MB> {
        Regulator {
            bus,
            _mrid: mrid,
            status_topic: topic(Profile::RegulatorStatusProfile, &mrid),
            event_topic: topic(Profile::RegulatorEventProfile, &mrid),
            reading_topic: topic(Profile::RegulatorReadingProfile, &mrid),
            control_topic: topic(Profile::RegulatorControlProfile, &mrid),
            discrete_control_topic: topic(Profile::RegulatorDiscreteControlProfile, &mrid),
        }
    }

    /// publish regulator status messages
    pub async fn status(&mut self, msg: RegulatorStatusProfile) -> PublishResult<()> {
        self.bus.publish(self.status_topic.iter(), msg).await
    }

    /// publish regulator event messages
    pub async fn event(&mut self, msg: RegulatorEventProfile) -> PublishResult<()> {
        self.bus.publish(self.event_topic.iter(), msg).await
    }

    /// publish regulator reading messages
    pub async fn reading(&mut self, msg: RegulatorReadingProfile) -> PublishResult<()> {
        self.bus.publish(self.reading_topic.iter(), msg).await
    }

    /// subscribe to control messages
    pub async fn control(&mut self) -> SubscribeResult<RegulatorControlProfile> {
        self.bus.subscribe(self.control_topic.iter()).await
    }

    /// subscribe to discrete control messages
    pub async fn discrete_control(&mut self) -> SubscribeResult<RegulatorDiscreteControlProfile> {
        self.bus.subscribe(self.discrete_control_topic.iter()).await
    }
}
