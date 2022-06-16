// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

use crate::{prelude::*, topic};
use openfmb_messages::reclosermodule::*;
use uuid::Uuid;

pub struct Recloser<MB>
where
    MB: Subscriber<RecloserStatusProfile>
        + Subscriber<RecloserEventProfile>
        + Subscriber<RecloserReadingProfile>
        + Publisher<RecloserDiscreteControlProfile>,
{
    bus: MB,
    mrid: Uuid,
    status_topic: ProfileTopic,
    event_topic: ProfileTopic,
    reading_topic: ProfileTopic,
    discrete_control_topic: ProfileTopic,
}

fn topic(profile: topic::Profile, mrid: &Uuid) -> ProfileTopic {
    ProfileTopic::new(Module::RecloserModule, profile, mrid.clone())
}

impl<MB> Recloser<MB>
where
    MB: Subscriber<RecloserStatusProfile>
        + Subscriber<RecloserEventProfile>
        + Subscriber<RecloserReadingProfile>
        + Publisher<RecloserDiscreteControlProfile>,
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

    #[allow(dead_code)]
    fn mrid_as_string(&self) -> String {
        format!("{}", self.mrid.hyphenated())
    }

    /// A stream to this devices status messages
    ///
    /// The return may be treated as a stream or as a future returning the
    /// next event
    pub async fn status(&mut self) -> SubscribeResult<RecloserStatusProfile> {
        self.bus.subscribe(self.status_topic.iter()).await
    }

    /// A stream to this devices reading messages
    ///
    /// The return may be treated as a stream or as a future returning the
    /// next event
    pub async fn event(&mut self) -> SubscribeResult<RecloserEventProfile> {
        self.bus.subscribe(self.event_topic.iter()).await
    }

    /// A stream to this devices reading messages
    ///
    /// The return may be treated as a stream or as a future returning the next
    /// reading value.
    pub async fn reading(&mut self) -> SubscribeResult<RecloserReadingProfile> {
        self.bus.subscribe(self.reading_topic.iter()).await
    }

    /// Send a discrete control message to the device asynchronously
    ///
    /// Awaits on publishing but no change awaited on
    pub async fn discrete_control(
        &mut self,
        msg: RecloserDiscreteControlProfile,
    ) -> PublishResult<()> {
        Ok(self
            .bus
            .publish(self.discrete_control_topic.iter(), msg)
            .await?)
    }
}
