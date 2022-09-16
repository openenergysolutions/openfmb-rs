// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

use crate::{prelude::*, topic};
use openfmb_messages::metermodule::*;
use uuid::Uuid;

pub struct Meter<MB>
where
    MB: Subscriber<MeterReadingProfile>

{
    bus: MB,
    mrid: Uuid,
    reading_topic: ProfileTopic,
}

fn topic(profile: topic::Profile, mrid: &Uuid) -> ProfileTopic {
    ProfileTopic::new(Module::MeterModule, profile, mrid.clone())
}

impl<MB> Meter<MB>
where
    MB: Subscriber<MeterReadingProfile>
{
    /// Create a new load client instance
    pub fn new(bus: MB, mrid: Uuid) -> Meter<MB> {
        Meter {
            bus,
            mrid,
            reading_topic: topic(Profile::MeterReadingProfile, &mrid),
        }
    }

    #[allow(dead_code)]
    fn mrid_as_string(&self) -> String {
        format!("{}", self.mrid.hyphenated())
    }

    /// A stream to this devices reading messages
    ///
    /// The return may be treated as a stream or as a future returning the next
    /// reading value.
    pub async fn reading(&mut self) -> SubscribeResult<MeterReadingProfile> {
        self.bus.subscribe(self.reading_topic.iter()).await
    }

}
