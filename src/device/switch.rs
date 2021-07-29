// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc //
// SPDX-License-Identifier: Apache-2.0

use crate::{prelude::*, topic};
use openfmb_messages::switchmodule::{
    SwitchDiscreteControlProfile, SwitchEventProfile, SwitchReadingProfile, SwitchStatusProfile,
};
use uuid::Uuid;

/// Provide functionality to publish and accept messages a switch device
/// needs.
#[derive(Debug, Clone)]
pub struct Switch<MB>
where
    MB: std::fmt::Debug
        + Clone
        + Publisher<SwitchStatusProfile>
        + Publisher<SwitchEventProfile>
        + Publisher<SwitchReadingProfile>
        + Subscriber<SwitchDiscreteControlProfile>,
{
    bus: MB,
    mrid: Uuid,
    status_topic: ProfileTopic,
    event_topic: ProfileTopic,
    reading_topic: ProfileTopic,
    discrete_control_topic: ProfileTopic,

}

fn topic(profile: topic::Profile, mrid: &Uuid) -> ProfileTopic {
    ProfileTopic::new(topic::Module::Switch, profile, mrid.clone())
}

impl<MB> Switch<MB>
where
    MB: std::fmt::Debug
        + Clone
        + Publisher<SwitchStatusProfile>
        + Publisher<SwitchEventProfile>
        + Publisher<SwitchReadingProfile>
        + Subscriber<SwitchDiscreteControlProfile>,
{
    /// Create a new switch client instance
    pub fn new(bus: MB, mrid: Uuid) -> Switch<MB> {
        Switch {
            bus,
            mrid,
            status_topic: topic(topic::Profile::Status, &mrid),
            event_topic: topic(topic::Profile::Event, &mrid),
            reading_topic: topic(topic::Profile::Reading, &mrid),
            discrete_control_topic: topic(topic::Profile::DiscreteControl, &mrid),
        }
    }

    /// publish switch status messages
    pub async fn status(&mut self, msg: SwitchStatusProfile) -> PublishResult<()> {
        Ok(self
            .bus
            .publish(self.status_topic.iter(), msg)
            .await?)
    }

    /// publish switch event messages
    pub async fn event(&mut self, msg: SwitchEventProfile) -> PublishResult<()> {
        Ok(self
            .bus
            .publish(self.event_topic.iter(), msg)
            .await?)
    }

    /// publish switch reading messages
    pub async fn reading(&mut self, msg: SwitchReadingProfile) -> PublishResult<()> {
        Ok(self
            .bus
            .publish(self.reading_topic.iter(), msg)
            .await?)
    }

    /// Subscribe to control messages
    pub async fn control(&mut self) -> SubscribeResult<SwitchDiscreteControlProfile> {
        self.bus
            .subscribe(self.discrete_control_topic.iter())
            .await
    }
}
