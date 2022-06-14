// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

use crate::{prelude::*, topic};
use futures::{stream, StreamExt};
use log::trace;
use openfmb_messages::{commonmodule::*, resourcemodule::*, *};
use openfmb_messages_ext::resource::ResourceControlExt;
use uuid::Uuid;

use std::time;

pub struct Resource<MB>
where
    MB: Subscriber<ResourceStatusProfile>
        + Subscriber<ResourceEventProfile>
        + Subscriber<ResourceReadingProfile>
        + Publisher<ResourceDiscreteControlProfile>,
{
    bus: MB,
    mrid: Uuid,
    status_topic: ProfileTopic,
    event_topic: ProfileTopic,
    reading_topic: ProfileTopic,
    discrete_control_topic: ProfileTopic,
}

fn topic(profile: topic::Profile, mrid: &Uuid) -> ProfileTopic {
    ProfileTopic::new(Module::ResourceModule, profile, mrid.clone())
}

impl<MB> Resource<MB>
where
    MB: Subscriber<ResourceStatusProfile>
        + Subscriber<ResourceEventProfile>
        + Subscriber<ResourceReadingProfile>
        + Publisher<ResourceDiscreteControlProfile>,
{
    /// Create a new resource client instance
    pub fn new(bus: MB, mrid: Uuid) -> Resource<MB> {
        Resource {
            bus,
            mrid,
            status_topic: topic(Profile::ResourceStatusProfile, &mrid),
            event_topic: topic(Profile::ResourceEventProfile, &mrid),
            reading_topic: topic(Profile::ResourceReadingProfile, &mrid),
            discrete_control_topic: topic(Profile::ResourceDiscreteControlProfile, &mrid),
        }
    }

    fn mrid_as_string(&self) -> String {
        format!("{}", self.mrid.hyphenated())
    }

    /// A stream to this devices status messages
    ///
    /// The return may be treated as a stream or as a future returning the
    /// next event
    pub async fn status(&mut self) -> SubscribeResult<ResourceStatusProfile> {
        self.bus.subscribe(self.status_topic.iter()).await
    }

    /// A stream to this devices reading messages
    ///
    /// The return may be treated as a stream or as a future returning the
    /// next event
    pub async fn event(&mut self) -> SubscribeResult<ResourceEventProfile> {
        self.bus.subscribe(self.event_topic.iter()).await
    }

    /// A stream to this devices reading messages
    ///
    /// The return may be treated as a stream or as a future returning the next
    /// reading value.
    pub async fn reading(&mut self) -> SubscribeResult<ResourceReadingProfile> {
        self.bus.subscribe(self.reading_topic.iter()).await
    }

    /// Send a discrete control message to the device asynchronously
    ///
    /// Awaits on publishing but no change awaited on
    pub async fn discrete_control(
        &mut self,
        msg: ResourceDiscreteControlProfile,
    ) -> PublishResult<()> {
        Ok(self
            .bus
            .publish(self.discrete_control_topic.iter(), msg)
            .await?)
    }
}
