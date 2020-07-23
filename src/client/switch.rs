use crate::prelude::*;
use futures::{
    future::{self, BoxFuture},
    stream::{self, Stream},
    FutureExt, StreamExt, TryFutureExt,
};
use openfmb_ops_protobuf::openfmb::{
    commonmodule::DbPosKind,
    switchmodule::{SwitchEventProfile, SwitchReadingProfile, SwitchStatusProfile},
};
use std::pin::Pin;
use uuid::Uuid;

/// Error type erased return result, for simplified returns
pub type SwitchResult<T> = Result<T, Box<dyn std::error::Error>>;

/// Control and wait on updates from a switch
///
/// Every function implies a request for the next future state of the switch
/// rather than the last seen state. A variant of this interface could possibly
/// look at the *last* known status and reading instead.
///
/// When writing control algorithms however it is easier determine the next
/// known good value and *then* control it rather than looking at an old status
/// which may be too old to be useful.
pub struct Switch<MB>
where
    MB: Subscriber<SwitchStatusProfile>
        + Subscriber<SwitchEventProfile>
        + Subscriber<SwitchReadingProfile>,
{
    bus: MB,
    mrid: Uuid,
}

// Topic string given a message type and mrid
fn topic(typ: &'static str, mrid: &Uuid) -> String {
    format!("openfmb.switchmodule.{}.{}", typ, mrid.to_hyphenated())
}

impl<MB> Switch<MB>
where
    MB: MessageBus<SwitchStatusProfile>
        + MessageBus<SwitchEventProfile>
        + MessageBus<SwitchReadingProfile>,
{
    /// Create a new switch client instance
    pub fn new(bus: MB, mrid: Uuid) -> Switch<MB> {
        Switch { bus, mrid }
    }

    /// A stream to this devices status messages
    ///
    /// The return may be treated as a stream or as a future returning the
    /// next event
    pub fn status(&mut self) -> SubscribeResult<SwitchStatusProfile> {
        self.bus
            .subscribe(&topic("SwitchStatusProfile", &self.mrid))
    }

    /// A stream to this devices reading messages
    ///
    /// The return may be treated as a stream or as a future returning the
    /// next event
    pub fn event(&mut self) -> SubscribeResult<SwitchEventProfile> {
        self.bus.subscribe(&topic("SwitchEventProfile", &self.mrid))
    }

    /// A stream to this devices reading messages
    ///
    /// The return may be treated as a stream or as a future returning the next
    /// reading value.
    pub fn reading(&mut self) -> SubscribeResult<SwitchReadingProfile> {
        self.bus
            .subscribe(&topic("SwitchReadingProfile", &self.mrid))
    }

    /// On the next status update checks if the switch is open
    /// returning true if so.
    pub async fn is_open(&mut self) -> SwitchResult<bool> {
        Ok(self.position().await? == DbPosKind::Open)
    }

    /// On the next status update checks if the switch is closed
    /// returning true if so.
    pub async fn is_closed(&mut self) -> SwitchResult<bool> {
        Ok(self.position().await? == DbPosKind::Closed)
    }

    /// On the next status update checks if the switch is transient
    /// returning true if so.
    pub async fn is_transient(&mut self) -> SwitchResult<bool> {
        Ok(self.position().await? == DbPosKind::Transient)
    }

    /// On the next event or status update report back the status of the switch
    ///
    /// TODO simplify this stuff
    pub async fn position(&mut self) -> SwitchResult<DbPosKind> {
        let mut status = self.status()?;
        let status: BoxFuture<Result<DbPosKind, SubscriptionError>> =
            Box::pin(status.next().map(|s| {
                match s {
                    Some(Ok(s)) => Ok(DbPosKind::from_i32(
                        s.switch_status
                            .unwrap()
                            .switch_status_xswi
                            .unwrap()
                            .pos
                            .unwrap()
                            .st_val,
                    )
                    .unwrap()),
                    Some(Err(err)) => Err(err),
                    None => Err(SubscriptionError::Unsubscribed),
                }
            }));
        let mut event = self.event()?;
        let event: BoxFuture<Result<DbPosKind, SubscriptionError>> =
            Box::pin(event.next().map(|s| {
                match s {
                    Some(Ok(s)) => Ok(DbPosKind::from_i32(
                        s.switch_event
                            .unwrap()
                            .switch_event_xswi
                            .unwrap()
                            .pos
                            .unwrap()
                            .st_val,
                    )
                    .unwrap()),
                    Some(Err(err)) => Err(err),
                    None => Err(SubscriptionError::Unsubscribed),
                }
            }));
        let (pos, _) = future::select_ok(vec![status, event]).await?;
        Ok(pos)
    }

    /*
    /// Open the switch, errors if the switch does not open or takes too long
    async fn open(&self) -> SwitchResult<()>;

    /// Close the switch, errors if the switch does not close or takes too long
    async fn close(&self) -> SwitchResult<()>;
    */
}
