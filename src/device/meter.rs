// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc //
// SPDX-License-Identifier: Apache-2.0

use crate::prelude::*;
use openfmb_messages::{
    metermodule::{MeterReadingProfile},
    Module, Profile,
};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Meter<MB>
where
    MB: std::fmt::Debug
        + Clone
        + Publisher<MeterReadingProfile>,
{
    bus: MB,
    _mrid: Uuid,
    reading_topic: ProfileTopic,
}

fn topic(profile: Profile, mrid: &Uuid) -> ProfileTopic {
    ProfileTopic::new(Module::MeterModule, profile, mrid.clone())
}

impl<MB> Meter<MB>
where
    MB: std::fmt::Debug
        + Clone
        + Publisher<MeterReadingProfile>,
{
    /// Create a new load device instance
    pub fn new(bus: MB, mrid: Uuid) -> Meter<MB> {
        Meter {
            bus,
            _mrid: mrid,
            reading_topic: topic(Profile::MeterReadingProfile, &mrid),
        }
    }

    /// publish meter reading messages
    pub async fn reading(&mut self, msg: MeterReadingProfile) -> PublishResult<()> {
        Ok(self.bus.publish(self.reading_topic.iter(), msg).await?)
    }

}
