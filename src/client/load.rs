// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

use crate::{prelude::*, topic};
use futures::{stream, StreamExt};
use log::trace;
use openfmb_messages::{commonmodule::*, loadmodule::*, *};
use openfmb_messages_ext::load::LoadControlExt;
use uuid::Uuid;

use std::time;

/// Control and wait on updates from a load
///
/// Every function implies a request for the next future state of the switch
/// rather than the last seen state. A variant of this interface could possibly
/// look at the *last* known status and reading instead.
///
/// When writing control algorithms however it is easier determine the next
/// known good value and *then* control it rather than looking at an old status
/// which may be too old to be useful.
pub struct Load<MB>
where
    MB: Subscriber<LoadStatusProfile>
        + Subscriber<LoadEventProfile>
        + Subscriber<LoadReadingProfile>
        + Publisher<LoadControlProfile>,
{
    bus: MB,
    mrid: Uuid,
    status_topic: ProfileTopic,
    event_topic: ProfileTopic,
    reading_topic: ProfileTopic,
    control_topic: ProfileTopic,
}

fn topic(profile: topic::Profile, mrid: &Uuid) -> ProfileTopic {
    ProfileTopic::new(Module::LoadModule, profile, mrid.clone())
}

impl<MB> Load<MB>
where
    MB: Subscriber<LoadStatusProfile>
        + Subscriber<LoadEventProfile>
        + Subscriber<LoadReadingProfile>
        + Publisher<LoadControlProfile>,
{
    /// Create a new load client instance
    pub fn new(bus: MB, mrid: Uuid) -> Load<MB> {
        Load {
            bus,
            mrid,
            status_topic: topic(Profile::LoadStatusProfile, &mrid),
            event_topic: topic(Profile::LoadEventProfile, &mrid),
            reading_topic: topic(Profile::LoadReadingProfile, &mrid),
            control_topic: topic(Profile::LoadControlProfile, &mrid),
        }
    }

    fn mrid_as_string(&self) -> String {
        format!("{}", self.mrid.hyphenated())
    }

    /// A stream to this devices status messages
    ///
    /// The return may be treated as a stream or as a future returning the
    /// next event
    pub async fn status(&mut self) -> SubscribeResult<LoadStatusProfile> {
        self.bus.subscribe(self.status_topic.iter()).await
    }

    /// A stream to this devices reading messages
    ///
    /// The return may be treated as a stream or as a future returning the
    /// next event
    pub async fn event(&mut self) -> SubscribeResult<LoadEventProfile> {
        self.bus.subscribe(self.event_topic.iter()).await
    }

    /// A stream to this devices reading messages
    ///
    /// The return may be treated as a stream or as a future returning the next
    /// reading value.
    pub async fn reading(&mut self) -> SubscribeResult<LoadReadingProfile> {
        self.bus.subscribe(self.reading_topic.iter()).await
    }

    /// Send a control message to the device asynchronously
    ///
    /// Awaits on publishing but no change awaited on.
    pub async fn control(&mut self, msg: LoadControlProfile) -> PublishResult<()> {
        Ok(self.bus.publish(self.control_topic.iter(), msg).await?)
    }
}
