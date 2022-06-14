// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc //
// SPDX-License-Identifier: Apache-2.0

use crate::prelude::*;
use openfmb_messages::{
    circuitsegmentservicemodule::{
        CircuitSegmentControlProfile, CircuitSegmentEventProfile, CircuitSegmentStatusProfile,
    },
    Module, Profile,
};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct CircuitSegment<MB>
where
    MB: std::fmt::Debug
        + Clone
        + Publisher<CircuitSegmentStatusProfile>
        + Publisher<CircuitSegmentEventProfile>
        + Subscriber<CircuitSegmentControlProfile>,
{
    bus: MB,
    mrid: Uuid,
    status_topic: ProfileTopic,
    event_topic: ProfileTopic,
    control_topic: ProfileTopic,
}

fn topic(profile: Profile, mrid: &Uuid) -> ProfileTopic {
    ProfileTopic::new(Module::CircuitSegmentServiceModule, profile, mrid.clone())
}

impl<MB> CircuitSegment<MB>
where
    MB: std::fmt::Debug
        + Clone
        + Publisher<CircuitSegmentStatusProfile>
        + Publisher<CircuitSegmentEventProfile>
        + Subscriber<CircuitSegmentControlProfile>,
{
    pub fn new(bus: MB, mrid: Uuid) -> CircuitSegment<MB> {
        CircuitSegment {
            bus,
            mrid,
            status_topic: topic(Profile::CircuitSegmentStatusProfile, &mrid),
            event_topic: topic(Profile::CircuitSegmentEventProfile, &mrid),
            control_topic: topic(Profile::CircuitSegmentControlProfile, &mrid),
        }
    }

    pub async fn status(&mut self, msg: CircuitSegmentStatusProfile) -> PublishResult<()> {
        Ok(self.bus.publish(self.status_topic.iter(), msg).await?)
    }

    pub async fn event(&mut self, msg: CircuitSegmentEventProfile) -> PublishResult<()> {
        Ok(self.bus.publish(self.event_topic.iter(), msg).await?)
    }

    pub async fn control(&mut self) -> SubscribeResult<CircuitSegmentControlProfile> {
        self.bus.subscribe(self.control_topic.iter()).await
    }
}
