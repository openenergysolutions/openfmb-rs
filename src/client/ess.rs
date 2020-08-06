use crate::prelude::*;
use futures::{stream, StreamExt};
use openfmb_ops_protobuf::openfmb::{
    commonmodule::GridConnectModeKind,
    essmodule::{EssControlProfile, EssEventProfile, EssReadingProfile, EssStatusProfile},
};
use openfmb_protobuf_ext::ess::EssControlExt;
use std::time::SystemTime;
use uuid::Uuid;

/// Control and wait on updates from ESS (battery/storage)
///
/// Every function implies a request for the next future state of the ess
/// rather than the last seen state. A variant of this interface could possibly
/// look at the *last* known status and reading instead.
///
/// When writing control algorithms however it is easier determine the next
/// known good value and *then* control it rather than looking at an old status
/// which may be too old to be useful.
pub struct Ess<MB>
where
    MB: Subscriber<EssStatusProfile>
        + Subscriber<EssEventProfile>
        + Subscriber<EssReadingProfile>
        + Publisher<EssControlProfile>,
{
    bus: MB,
    mrid: Uuid,
}

/// Topic string given a message type and mrid
pub fn topic(typ: &'static str, mrid: &Uuid) -> String {
    format!("openfmb.essmodule.{}.{}", typ, mrid.to_hyphenated())
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
        Ess { bus, mrid }
    }

    /// Get the device MRID as a string
    fn mrid_as_string(&self) -> String {
        format!("{}", self.mrid.to_hyphenated())
    }

    /// A stream to this devices status messages
    ///
    /// The return may be treated as a stream or as a future returning the
    /// next event
    pub fn status(&mut self) -> SubscribeResult<EssStatusProfile> {
        self.bus.subscribe(&topic("EssStatusProfile", &self.mrid))
    }

    /// A stream to this devices reading messages
    ///
    /// The return may be treated as a stream or as a future returning the
    /// next event
    pub fn event(&mut self) -> SubscribeResult<EssEventProfile> {
        self.bus.subscribe(&topic("EssEventProfile", &self.mrid))
    }

    /// A stream to this devices reading messages
    ///
    /// The return may be treated as a stream or as a future returning the next
    /// reading value.
    pub fn reading(&mut self) -> SubscribeResult<EssReadingProfile> {
        self.bus.subscribe(&topic("EssReadingProfile", &self.mrid))
    }

    /// Send a control message to the device asynchronously
    ///
    /// Awaits on publishing but no change awaited on.
    pub async fn control(&mut self, msg: EssControlProfile) -> PublishResult<()> {
        Ok(self
            .bus
            .publish(&topic("EssControlProfile", &self.mrid), msg)
            .await?)
    }

    pub async fn is_synchro_enabled(&mut self) -> SubscribeResult<bool> {
        let status = self.status()?.map(|s| match s {
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
        let event = self.event()?.map(|s| match s {
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
        Ok(Box::pin(self.status()?.map(|s| {
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
        Ok(Box::pin(self.status()?.map(|s| {
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
    pub async fn grid_follow(&mut self, charge_rate: f32) -> ControlResult<()> {
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
    pub async fn grid_form(&mut self, charge_rate: f32) -> ControlResult<()> {
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
