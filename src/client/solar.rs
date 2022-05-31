// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc //
// SPDX-License-Identifier: Apache-2.0

use crate::prelude::*;
use openfmb_messages::{commonmodule::*, solarmodule::*, Module, Profile};
use openfmb_messages_ext::solar::SolarControlExt;

use uuid::Uuid;

use std::time;

/// Provide functionality to publish and accept messages a switch device
/// needs.
#[derive(Debug, Clone)]
pub struct Solar<MB>
where
    MB: std::fmt::Debug
        + Clone
        + Subscriber<SolarStatusProfile>
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
    MB: std::fmt::Debug
        + Clone
        + Subscriber<SolarStatusProfile>
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

    fn mrid_as_string(&self) -> String {
        format!("{}", self.mrid.to_hyphenated())
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

    /// Publish a Solar control message with a specified number of WNetMag and VArNetMag schedule values
    pub async fn set_power_schedule(
        &mut self,
        sch_pts: usize,
        wnet_mag: f64,
        varnet_mag: f64,
    ) -> PublishResult<()> {
        let mut msg = SolarControlProfile::build_control_profile(
            &self.mrid_as_string(),
            0_f64,
            time::SystemTime::now(),
            0,
        );
        msg.solar_control_mut()
            .solar_control_fscc_mut()
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
            msg.solar_control_mut()
                .solar_control_fscc_mut()
                .control_fscc_mut()
                .control_schedule_fsch_mut()
                .val_acsg_mut()
                .sch_pts_mut()
                .push(sch_pt);
        }
        self.control(msg).await
    }
}
