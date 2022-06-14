// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

use crate::{prelude::*, topic};
use futures::{stream, StreamExt};

use openfmb_messages::{commonmodule::*, generationmodule::*};

use openfmb_messages_ext::GenerationControlExt;
use std::time;
use uuid::Uuid;

/// Control and wait on updates from Generation
///
/// Every function implies a request for the next future state of the generator
/// rather than the last seen state. A variant of this interface could possibly
/// look at the *last* known status and reading instead.
///
/// When writing control algorithms however it is easier determine the next
/// known good value and *then* control it rather than looking at an old status
/// which may be too old to be useful.
pub struct Generation<MB>
where
    MB: Subscriber<GenerationStatusProfile>
        + Subscriber<GenerationEventProfile>
        + Subscriber<GenerationReadingProfile>
        + Publisher<GenerationControlProfile>,
{
    bus: MB,
    mrid: Uuid,
    status_topic: ProfileTopic,
    event_topic: ProfileTopic,
    reading_topic: ProfileTopic,
    control_topic: ProfileTopic,
}

/// Topic string given a message type and mrid
pub fn topic(profile: topic::Profile, mrid: &Uuid) -> ProfileTopic {
    ProfileTopic::new(Module::GenerationModule, profile, mrid.clone())
}

impl<MB> Generation<MB>
where
    MB: Subscriber<GenerationStatusProfile>
        + Subscriber<GenerationEventProfile>
        + Subscriber<GenerationReadingProfile>
        + Publisher<GenerationControlProfile>,
{
    /// Create a new switch client instance
    pub fn new(bus: MB, mrid: Uuid) -> Generation<MB> {
        Generation {
            bus,
            mrid,
            status_topic: topic(Profile::GenerationStatusProfile, &mrid),
            event_topic: topic(Profile::GenerationEventProfile, &mrid),
            reading_topic: topic(Profile::GenerationReadingProfile, &mrid),
            control_topic: topic(Profile::GenerationControlProfile, &mrid),
        }
    }

    /// Get the device MRID as a string
    fn mrid_as_string(&self) -> String {
        format!("{}", self.mrid.hyphenated())
    }

    /// A stream to this devices status messages
    ///
    /// The return may be treated as a stream or as a future returning the
    /// next event
    pub async fn status(&mut self) -> SubscribeResult<GenerationStatusProfile> {
        self.bus.subscribe(self.status_topic.iter()).await
    }

    /// A stream to this devices reading messages
    ///
    /// The return may be treated as a stream or as a future returning the next
    /// reading value.
    pub async fn reading(&mut self) -> SubscribeResult<GenerationReadingProfile> {
        self.bus.subscribe(self.reading_topic.iter()).await
    }

    pub async fn event(&mut self) -> SubscribeResult<GenerationEventProfile> {
        self.bus.subscribe(self.event_topic.iter()).await
    }

    pub async fn control(&mut self, msg: GenerationControlProfile) -> PublishResult<()> {
        Ok(self.bus.publish(self.control_topic.iter(), msg).await?)
    }
}
