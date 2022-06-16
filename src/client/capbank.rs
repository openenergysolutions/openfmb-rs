// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

use crate::{prelude::*, topic};
use openfmb_messages::capbankmodule::*;
use uuid::Uuid;

pub struct CapBank<MB>
where
    MB: Subscriber<CapBankStatusProfile>
        + Subscriber<CapBankEventProfile>
        + Subscriber<CapBankReadingProfile>
        + Publisher<CapBankControlProfile>
        + Publisher<CapBankDiscreteControlProfile>,
{
    bus: MB,
    mrid: Uuid,
    status_topic: ProfileTopic,
    event_topic: ProfileTopic,
    reading_topic: ProfileTopic,
    control_topic: ProfileTopic,
    discrete_control_topic: ProfileTopic,
}

fn topic(profile: topic::Profile, mrid: &Uuid) -> ProfileTopic {
    ProfileTopic::new(Module::CapBankModule, profile, mrid.clone())
}

impl<MB> CapBank<MB>
where
    MB: Subscriber<CapBankStatusProfile>
        + Subscriber<CapBankEventProfile>
        + Subscriber<CapBankReadingProfile>
        + Publisher<CapBankControlProfile>
        + Publisher<CapBankDiscreteControlProfile>,
{
    /// Create a new capbank client instance
    pub fn new(bus: MB, mrid: Uuid) -> CapBank<MB> {
        CapBank {
            bus,
            mrid,
            status_topic: topic(Profile::CapBankStatusProfile, &mrid),
            event_topic: topic(Profile::CapBankEventProfile, &mrid),
            reading_topic: topic(Profile::CapBankReadingProfile, &mrid),
            control_topic: topic(Profile::CapBankControlProfile, &mrid),
            discrete_control_topic: topic(Profile::CapBankDiscreteControlProfile, &mrid),
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
    pub async fn status(&mut self) -> SubscribeResult<CapBankStatusProfile> {
        self.bus.subscribe(self.status_topic.iter()).await
    }

    /// A stream to this devices reading messages
    ///
    /// The return may be treated as a stream or as a future returning the
    /// next event
    pub async fn event(&mut self) -> SubscribeResult<CapBankEventProfile> {
        self.bus.subscribe(self.event_topic.iter()).await
    }

    /// A stream to this devices reading messages
    ///
    /// The return may be treated as a stream or as a future returning the next
    /// reading value.
    pub async fn reading(&mut self) -> SubscribeResult<CapBankReadingProfile> {
        self.bus.subscribe(self.reading_topic.iter()).await
    }

    /// Send a control message to the device asynchronously
    ///
    /// Awaits on publishing but no change awaited on.
    pub async fn control(&mut self, msg: CapBankControlProfile) -> PublishResult<()> {
        Ok(self.bus.publish(self.control_topic.iter(), msg).await?)
    }

    /// Send a discrete control message to the device asynchronously
    ///
    /// Awaits on publishing but no change awaited on
    pub async fn discrete_control(
        &mut self,
        msg: CapBankDiscreteControlProfile,
    ) -> PublishResult<()> {
        Ok(self
            .bus
            .publish(self.discrete_control_topic.iter(), msg)
            .await?)
    }
}
