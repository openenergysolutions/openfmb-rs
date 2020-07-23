use crate::prelude::*;
use futures::{
    future::{self, BoxFuture},
    stream, FutureExt, StreamExt,
};
use openfmb_ops_protobuf::openfmb::{
    commonmodule::DbPosKind,
    switchmodule::{SwitchEventProfile, SwitchReadingProfile, SwitchStatusProfile},
};
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

    /// A returned subscription transform that checks if the switch was closed
    /// on any event or status messages.
    pub async fn is_open(&mut self) -> SubscribeResult<bool> {
        Ok(Box::pin(self.position().await?.map(|p| match p {
            Ok(DbPosKind::Open) => Ok(true),
            Ok(_) => Ok(false),
            Err(err) => Err(err),
        })))
    }

    /// A returned subscription transform that checks if the switch was open
    /// on any event or status messages.
    pub async fn is_closed(&mut self) -> SubscribeResult<bool> {
        Ok(Box::pin(self.position().await?.map(|p| match p {
            Ok(DbPosKind::Closed) => Ok(true),
            Ok(_) => Ok(false),
            Err(err) => Err(err),
        })))
    }

    /// A returned subscription transform that checks if the switch was transient
    /// on any event or status messages.
    pub async fn is_transient(&mut self) -> SubscribeResult<bool> {
        Ok(Box::pin(self.position().await?.map(|p| match p {
            Ok(DbPosKind::Transient) => Ok(true),
            Ok(_) => Ok(false),
            Err(err) => Err(err),
        })))
    }

    /// A stream of the switch position updates
    ///
    /// These may come from either SwitchEventProfile *or* SwitchStatusProfile
    /// messages so we subscribe to both streams and merge them together with
    /// some combinators.
    ///
    /// Can be used directly in logic to wait for a position to change for example
    ///
    /// ```
    /// let position = myswitch.position().await?;
    /// while let Some(Ok(DbPosKind::Open)) = position.next() {
    ///   myswitch.control(SwitchControlProfile::builder().set_position(DbPosKind::Closed));
    /// }
    /// ```
    pub async fn position(&mut self) -> SubscribeResult<DbPosKind> {
        let status = self.status()?.map(|s| match s {
            Ok(s) => Ok(DbPosKind::from_i32(
                s.switch_status
                    .unwrap()
                    .switch_status_xswi
                    .unwrap()
                    .pos
                    .unwrap()
                    .st_val,
            )
            .unwrap()),
            Err(err) => Err(err),
        });
        let event = self.event()?.map(|s| match s {
            Ok(s) => Ok(DbPosKind::from_i32(
                s.switch_event
                    .unwrap()
                    .switch_event_xswi
                    .unwrap()
                    .pos
                    .unwrap()
                    .st_val,
            )
            .unwrap()),
            Err(err) => Err(err),
        });
        Ok(Box::pin(stream::select(status, event)))
    }
}
