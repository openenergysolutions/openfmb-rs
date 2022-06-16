// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

use crate::{prelude::*, topic};
use futures::{stream, StreamExt};

use openfmb_messages::{
    commonmodule::GridConnectModeKind,
    essmodule::{EssControlProfile, EssEventProfile, EssReadingProfile, EssStatusProfile},
};

use openfmb_messages_ext::EssControlExt;
use std::time::SystemTime;
use uuid::Uuid;

pub struct Ess<MB>
where
    MB: Subscriber<EssStatusProfile>
        + Subscriber<EssEventProfile>
        + Subscriber<EssReadingProfile>
        + Publisher<EssControlProfile>,
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
    ProfileTopic::new(Module::EssModule, profile, mrid.clone())
}

impl<MB> Ess<MB>
where
    MB: Subscriber<EssStatusProfile>
        + Subscriber<EssEventProfile>
        + Subscriber<EssReadingProfile>
        + Publisher<EssControlProfile>,
{
    /// Create a new switch client instance
    pub fn new(bus: MB, mrid: Uuid) -> Ess<MB> {
        Ess {
            bus,
            mrid,
            status_topic: topic(Profile::ESSStatusProfile, &mrid),
            event_topic: topic(Profile::ESSEventProfile, &mrid),
            reading_topic: topic(Profile::ESSReadingProfile, &mrid),
            control_topic: topic(Profile::ESSControlProfile, &mrid),
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
    pub async fn status(&mut self) -> SubscribeResult<EssStatusProfile> {
        self.bus.subscribe(self.status_topic.iter()).await
    }

    /// A stream to this devices reading messages
    ///
    /// The return may be treated as a stream or as a future returning the
    /// next event
    pub async fn event(&mut self) -> SubscribeResult<EssEventProfile> {
        self.bus.subscribe(self.event_topic.iter()).await
    }

    /// A stream to this devices reading messages
    ///
    /// The return may be treated as a stream or as a future returning the next
    /// reading value.
    pub async fn reading(&mut self) -> SubscribeResult<EssReadingProfile> {
        self.bus.subscribe(self.reading_topic.iter()).await
    }

    /// Send a control message to the device asynchronously
    ///
    /// Awaits on publishing but no change awaited on.
    pub async fn control(&mut self, msg: EssControlProfile) -> PublishResult<()> {
        Ok(self.bus.publish(self.control_topic.iter(), msg).await?)
    }

    pub async fn is_synchro_enabled(&mut self) -> SubscribeResult<bool> {
        let status = self.status().await?.map(|s| match s {
            Ok(s) => Ok(s
                .ess_status
                .unwrap()
                .ess_status_zgen
                .unwrap()
                .e_ss_event_and_status_zgen
                .unwrap()
                .gn_syn_st
                .unwrap()
                .st_val),
            Err(err) => Err(err),
        });
        let event = self.event().await?.map(|s| match s {
            Ok(s) => Ok(s
                .ess_event
                .unwrap()
                .ess_event_zgen
                .unwrap()
                .e_ss_event_and_status_zgen
                .unwrap()
                .gn_syn_st
                .unwrap()
                .st_val),
            Err(err) => Err(err),
        });
        Ok(Box::pin(stream::select(status, event)))
    }

    /// Check if the inverter is in Grid Following mode
    pub async fn is_grid_following(&mut self) -> SubscribeResult<bool> {
        Ok(Box::pin(self.status().await?.map(|s| {
            match s {
                Ok(s) => Ok(GridConnectModeKind::from_i32(
                    s.ess_status
                        .unwrap()
                        .ess_status_zbat
                        .unwrap()
                        .gri_mod
                        .unwrap()
                        .set_val,
                )
                .unwrap()
                    == GridConnectModeKind::VsiPq),
                Err(err) => Err(err),
            }
        })))
    }

    /// Check if the inverter is in Grid Forming mode
    pub async fn is_grid_forming(&mut self) -> SubscribeResult<bool> {
        Ok(Box::pin(self.status().await?.map(|s| {
            match s {
                Ok(s) => Ok(GridConnectModeKind::from_i32(
                    s.ess_status
                        .unwrap()
                        .ess_status_zbat
                        .unwrap()
                        .gri_mod
                        .unwrap()
                        .set_val,
                )
                .unwrap()
                    == GridConnectModeKind::VsiIso),
                Err(err) => Err(err),
            }
        })))
    }

    /// Switch to grid following mode letting the inverter match the existing grid
    /// characteristics with a given charge rate. A charge rate of 0 implies auto.
    ///
    /// Charge rates less than 0 imply discharge
    pub async fn grid_follow(&mut self, charge_rate: f64) -> ControlResult<()> {
        let mut grid_following = self.is_grid_following().await?;
        while let Some(Ok(false)) = grid_following.next().await {
            let msg = EssControlProfile::build_charge_control_profile(
                &self.mrid_as_string(),
                charge_rate,
                SystemTime::now(),
                GridConnectModeKind::VsiPq,
                1,
            );
            self.control(msg).await?;
        }
        Ok(())
    }

    /// Switch to grid forming mode letting the inverter to generate the grid
    /// characteristics.
    pub async fn grid_form(&mut self, charge_rate: f64) -> ControlResult<()> {
        let mut grid_forming = self.is_grid_forming().await?;
        while let Some(Ok(false)) = grid_forming.next().await {
            let msg = EssControlProfile::build_charge_control_profile(
                &self.mrid_as_string(),
                charge_rate,
                SystemTime::now(),
                GridConnectModeKind::VsiIso,
                1,
            );
            self.control(msg).await?;
        }
        Ok(())
    }
}
