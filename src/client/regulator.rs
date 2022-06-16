// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

use crate::{prelude::*, topic};
use openfmb_messages::regulatormodule::*;
use uuid::Uuid;

pub struct Regulator<MB>
where
    MB: Subscriber<RegulatorStatusProfile>
        + Subscriber<RegulatorEventProfile>
        + Subscriber<RegulatorReadingProfile>
        + Publisher<RegulatorControlProfile>
        + Publisher<RegulatorDiscreteControlProfile>,
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
    ProfileTopic::new(Module::RegulatorModule, profile, mrid.clone())
}

impl<MB> Regulator<MB>
where
    MB: Subscriber<RegulatorStatusProfile>
        + Subscriber<RegulatorEventProfile>
        + Subscriber<RegulatorReadingProfile>
        + Publisher<RegulatorControlProfile>
        + Publisher<RegulatorDiscreteControlProfile>,
{
    /// Create a new regulator client instance
    pub fn new(bus: MB, mrid: Uuid) -> Regulator<MB> {
        Regulator {
            bus,
            mrid,
            status_topic: topic(Profile::RegulatorStatusProfile, &mrid),
            event_topic: topic(Profile::RegulatorEventProfile, &mrid),
            reading_topic: topic(Profile::RegulatorReadingProfile, &mrid),
            control_topic: topic(Profile::RegulatorControlProfile, &mrid),
            discrete_control_topic: topic(Profile::RegulatorDiscreteControlProfile, &mrid),
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
    pub async fn status(&mut self) -> SubscribeResult<RegulatorStatusProfile> {
        self.bus.subscribe(self.status_topic.iter()).await
    }

    /// A stream to this devices reading messages
    ///
    /// The return may be treated as a stream or as a future returning the
    /// next event
    pub async fn event(&mut self) -> SubscribeResult<RegulatorEventProfile> {
        self.bus.subscribe(self.event_topic.iter()).await
    }

    /// A stream to this devices reading messages
    ///
    /// The return may be treated as a stream or as a future returning the next
    /// reading value.
    pub async fn reading(&mut self) -> SubscribeResult<RegulatorReadingProfile> {
        self.bus.subscribe(self.reading_topic.iter()).await
    }

    /// Send a control message to the device asynchronously
    ///
    /// Awaits on publishing but no change awaited on.
    pub async fn control(&mut self, msg: RegulatorControlProfile) -> PublishResult<()> {
        Ok(self.bus.publish(self.control_topic.iter(), msg).await?)
    }

    /// Send a discrete control message to the device asynchronously
    ///
    /// Awaits on publishing but no change awaited on
    pub async fn discrete_control(
        &mut self,
        msg: RegulatorDiscreteControlProfile,
    ) -> PublishResult<()> {
        Ok(self
            .bus
            .publish(self.discrete_control_topic.iter(), msg)
            .await?)
    }
}
