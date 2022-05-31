// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

use crate::{prelude::*, topic};
use futures::{stream, StreamExt};
use log::trace;
use openfmb_messages::{capbankmodule::*, commonmodule::*, *};
use openfmb_messages_ext::capbank::CapBankControlExt;
use uuid::Uuid;

use std::time;

/// Control and wait on updates from a switch
///
/// Every function implies a request for the next future state of the switch
/// rather than the last seen state. A variant of this interface could possibly
/// look at the *last* known status and reading instead.
///
/// When writing control algorithms however it is easier determine the next
/// known good value and *then* control it rather than looking at an old status
/// which may be too old to be useful.
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

    fn mrid_as_string(&self) -> String {
        format!("{}", self.mrid.to_hyphenated())
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

    /// Set a `WNetMag` and `VArNetMag` schedule for this capbank device asynchronously
    ///
    /// Awaits on publishing but no change awaited on
    pub async fn set_power_schedule(
        &mut self,
        sch_pts: usize,
        wnet_mag: f64,
        varnet_mag: f64,
    ) -> PublishResult<()> {
        let mut msg = CapBankControlProfile::capbank_schedule_message(
            &self.mrid_as_string(),
            ScheduleParameterKind::WNetMag,
            wnet_mag,
        );
        msg.cap_bank_control_mut()
            .cap_bank_control_fscc_mut()
            .control_fscc_mut()
            .control_schedule_fsch_mut()
            .val_acsg_mut()
            .sch_pts_mut()
            .clear();

        for _ in 0..sch_pts {
            let (seconds, nanoseconds) =
                match time::SystemTime::now().duration_since(time::SystemTime::UNIX_EPOCH) {
                    Ok(time) => (time.as_secs(), time.subsec_nanos()),
                    Err(_) => panic!("SystemTime before UNIX_EPOCH!"),
                };
            let ctrl_timestamp = ControlTimestamp {
                seconds,
                nanoseconds,
            };
            let sch_pt = SchedulePoint {
                schedule_parameter: vec![
                    EngScheduleParameter {
                        schedule_parameter_type: ScheduleParameterKind::WNetMag.into(),
                        value: wnet_mag,
                    },
                    EngScheduleParameter {
                        schedule_parameter_type: ScheduleParameterKind::VArNetMag.into(),
                        value: varnet_mag,
                    },
                ],
                start_time: Some(ctrl_timestamp),
            };
            msg.cap_bank_control_mut()
                .cap_bank_control_fscc_mut()
                .control_fscc_mut()
                .control_schedule_fsch_mut()
                .val_acsg_mut()
                .sch_pts_mut()
                .push(sch_pt);
        }
        Ok(self.control(msg).await?)
    }
}
