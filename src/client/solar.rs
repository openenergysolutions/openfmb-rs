// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc //
// SPDX-License-Identifier: Apache-2.0

use crate::prelude::*;
use openfmb_messages::{solarmodule::*, Module, Profile};
use uuid::Uuid;

pub struct Solar<MB>
where
    MB: Subscriber<SolarStatusProfile>
        + Subscriber<SolarEventProfile>
        + Subscriber<SolarReadingProfile>
        + Publisher<SolarControlProfile>,
{
    bus: MB,
    mrid: Uuid,
    status_topic: ProfileTopic,
    event_topic: ProfileTopic,
    reading_topic: ProfileTopic,
    control_topic: ProfileTopic,
}

fn topic(profile: Profile, mrid: &Uuid) -> ProfileTopic {
    ProfileTopic::new(Module::SolarModule, profile, mrid.clone())
}

impl<MB> Solar<MB>
where
    MB: Subscriber<SolarStatusProfile>
        + Subscriber<SolarEventProfile>
        + Subscriber<SolarReadingProfile>
        + Publisher<SolarControlProfile>,
{
    /// Create a new Solar client instance
    pub fn new(bus: MB, mrid: Uuid) -> Solar<MB> {
        Solar {
            bus,
            mrid,
            status_topic: topic(Profile::SolarStatusProfile, &mrid),
            event_topic: topic(Profile::SolarEventProfile, &mrid),
            reading_topic: topic(Profile::SolarReadingProfile, &mrid),
            control_topic: topic(Profile::SolarControlProfile, &mrid),
        }
    }

    #[allow(dead_code)]
    fn mrid_as_string(&self) -> String {
        format!("{}", self.mrid.hyphenated())
    }

    /// Subscribe to Solar status messages
    pub async fn status(&mut self) -> SubscribeResult<SolarStatusProfile> {
        Ok(self.bus.subscribe(self.status_topic.iter()).await?)
    }

    /// Subscribe to Solar event messages
    pub async fn event(&mut self) -> SubscribeResult<SolarEventProfile> {
        Ok(self.bus.subscribe(self.event_topic.iter()).await?)
    }

    /// Subscribe to Solar reading messages
    pub async fn reading(&mut self) -> SubscribeResult<SolarReadingProfile> {
        Ok(self.bus.subscribe(self.reading_topic.iter()).await?)
    }

    /// Publish Solar control messages
    pub async fn control(&mut self, msg: SolarControlProfile) -> PublishResult<()> {
        self.bus.publish(self.control_topic.iter(), msg).await
    }
}
