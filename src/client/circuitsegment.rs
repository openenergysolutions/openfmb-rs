// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

use crate::{prelude::*, topic};
use futures::{stream, StreamExt};
use log::trace;
use openfmb_messages::{circuitsegmentservicemodule::*, commonmodule::*, *};
use openfmb_messages_ext::circuitsegmentservice::CircuitSegmentControlExt;
use uuid::Uuid;

use std::time;

pub struct CircuitSegment<MB>
where
    MB: Subscriber<CircuitSegmentStatusProfile>
        + Subscriber<CircuitSegmentEventProfile>
        + Publisher<CircuitSegmentControlProfile>,
{
    bus: MB,
    mrid: Uuid,
    status_topic: ProfileTopic,
    event_topic: ProfileTopic,
    control_topic: ProfileTopic,
}

fn topic(profile: topic::Profile, mrid: &Uuid) -> ProfileTopic {
    ProfileTopic::new(Module::CircuitSegmentServiceModule, profile, mrid.clone())
}

impl<MB> CircuitSegment<MB>
where
    MB: Subscriber<CircuitSegmentStatusProfile>
        + Subscriber<CircuitSegmentEventProfile>
        + Publisher<CircuitSegmentControlProfile>,
{
    /// Create a new circuitsegment client instance
    pub fn new(bus: MB, mrid: Uuid) -> CircuitSegment<MB> {
        CircuitSegment {
            bus,
            mrid,
            status_topic: topic(Profile::CircuitSegmentStatusProfile, &mrid),
            event_topic: topic(Profile::CircuitSegmentEventProfile, &mrid),
            control_topic: topic(Profile::CircuitSegmentControlProfile, &mrid),
        }
    }

    fn mrid_as_string(&self) -> String {
        format!("{}", self.mrid.hyphenated())
    }

    pub async fn status(&mut self) -> SubscribeResult<CircuitSegmentStatusProfile> {
        self.bus.subscribe(self.status_topic.iter()).await
    }

    pub async fn event(&mut self) -> SubscribeResult<CircuitSegmentEventProfile> {
        self.bus.subscribe(self.event_topic.iter()).await
    }

    pub async fn control(&mut self, msg: CircuitSegmentControlProfile) -> PublishResult<()> {
        Ok(self.bus.publish(self.control_topic.iter(), msg).await?)
    }
}
