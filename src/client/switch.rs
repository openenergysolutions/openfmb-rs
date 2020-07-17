use crate::prelude::*;
use openfmb_ops_protobuf::openfmb::switchmodule::{
    SwitchEventProfile, SwitchReadingProfile, SwitchStatusProfile,
};
use uuid::Uuid;


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
    pub fn status(
        &mut self,
    ) -> Result<
        Subscription<
            SwitchStatusProfile,
            <MB as Subscriber<SwitchStatusProfile>>::SubscriptionError,
        >,
        <MB as Subscriber<SwitchStatusProfile>>::SubscribeError,
    > {
        self.bus
            .subscribe(&topic("SwitchStatusProfile", &self.mrid))
    }

    /// A stream to this devices reading messages
    ///
    /// The return may be treated as a stream or as a future returning the
    /// next event
    pub fn event(
        &mut self,
    ) -> Result<
        Subscription<SwitchEventProfile, <MB as Subscriber<SwitchEventProfile>>::SubscriptionError>,
        <MB as Subscriber<SwitchEventProfile>>::SubscribeError,
    > {
        self.bus.subscribe(&topic("SwitchEventProfile", &self.mrid))
    }

    /// A stream to this devices reading messages
    ///
    /// The return may be treated as a stream or as a future returning the next
    /// reading value.
    pub fn reading(
        &mut self,
    ) -> Result<
        Subscription<
            SwitchReadingProfile,
            <MB as Subscriber<SwitchReadingProfile>>::SubscriptionError,
        >,
        <MB as Subscriber<SwitchReadingProfile>>::SubscribeError,
    > {
        self.bus
            .subscribe(&topic("SwitchReadingProfile", &self.mrid))
    }

    /*
    /// On the next status update checks if the switch is open
    /// returning true if so.
    async fn is_open(&self) -> SwitchResult<bool> {
        Ok(self.position().await? == SwitchPosition::Open)
    }

    /// On the next status update checks if the switch is closed
    /// returning true if so.
    async fn is_closed(&self) -> SwitchResult<bool> {
        Ok(self.position().await? == SwitchPosition::Closed)
    }

    /// On the next status update checks if the switch is transient
    /// returning true if so.
    async fn is_transient(&self) -> SwitchResult<bool> {
        Ok(self.position().await? == SwitchPosition::Transient)
    }

    /// On the next event or status update report back the status of the switch
    async fn position(&self) -> SwitchResult<SwitchPosition> {
        let status = self.status()?;
        let event = self.event()?;
        select! {
            Ok(status) = status.next() => Ok(status.switch_status.unwrap().switch_status_xswi.unwrap().pos.unwrap().st_val.unwrap().value.unwrap()),
            Ok(event) = event.next() => Ok(event.switch_event.unwrap().switch_status_xswi.unwrap().pos.unwrap().st_val.unwrap().value.unwrap()),
        }
    }
    */

    /*
    /// Open the switch, errors if the switch does not open or takes too long
    async fn open(&self) -> SwitchResult<()>;

    /// Close the switch, errors if the switch does not close or takes too long
    async fn close(&self) -> SwitchResult<()>;
    */
}
