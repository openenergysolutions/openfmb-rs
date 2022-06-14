// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

use crate::{prelude::*, topic};
use futures::{stream, StreamExt};
use log::trace;
use openfmb_messages::{breakermodule::*, commonmodule::*, *};
use openfmb_messages_ext::breaker::BreakerControlExt;
use uuid::Uuid;

use std::time;

pub struct Breaker<MB>
where
    MB: Subscriber<BreakerStatusProfile>
        + Subscriber<BreakerEventProfile>
        + Subscriber<BreakerReadingProfile>
        + Publisher<BreakerDiscreteControlProfile>,
{
    bus: MB,
    mrid: Uuid,
    status_topic: ProfileTopic,
    event_topic: ProfileTopic,
    reading_topic: ProfileTopic,
    discrete_control_topic: ProfileTopic,
}

fn topic(profile: topic::Profile, mrid: &Uuid) -> ProfileTopic {
    ProfileTopic::new(Module::BreakerModule, profile, mrid.clone())
}

impl<MB> Breaker<MB>
where
    MB: Subscriber<BreakerStatusProfile>
        + Subscriber<BreakerEventProfile>
        + Subscriber<BreakerReadingProfile>
        + Publisher<BreakerDiscreteControlProfile>,
{
    /// Create a new breaker client instance
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

    fn mrid_as_string(&self) -> String {
        format!("{}", self.mrid.hyphenated())
    }

    /// A stream to this devices status messages
    ///
    /// The return may be treated as a stream or as a future returning the
    /// next event
    pub async fn status(&mut self) -> SubscribeResult<BreakerStatusProfile> {
        self.bus.subscribe(self.status_topic.iter()).await
    }

    /// A stream to this devices reading messages
    ///
    /// The return may be treated as a stream or as a future returning the
    /// next event
    pub async fn event(&mut self) -> SubscribeResult<BreakerEventProfile> {
        self.bus.subscribe(self.event_topic.iter()).await
    }

    /// A stream to this devices reading messages
    ///
    /// The return may be treated as a stream or as a future returning the next
    /// reading value.
    pub async fn reading(&mut self) -> SubscribeResult<BreakerReadingProfile> {
        self.bus.subscribe(self.reading_topic.iter()).await
    }

    /// Send a discrete control message to the device asynchronously
    ///
    /// Awaits on publishing but no change awaited on
    pub async fn discrete_control(
        &mut self,
        msg: BreakerDiscreteControlProfile,
    ) -> PublishResult<()> {
        Ok(self
            .bus
            .publish(self.discrete_control_topic.iter(), msg)
            .await?)
    }
}
