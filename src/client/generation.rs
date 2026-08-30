// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

use crate::{prelude::*, topic};
use openfmb_messages::generationmodule::*;
use uuid::Uuid;

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
    ProfileTopic::new(Module::GenerationModule, profile, *mrid)
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

    #[allow(dead_code)]
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
        self.bus.publish(self.control_topic.iter(), msg).await
    }
}
