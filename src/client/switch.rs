use crate::prelude::*;
use futures::{stream, StreamExt};
use openfmb_ops_protobuf::openfmb::{
    commonmodule::{DynamicTestKind, DbPosKind},
    switchmodule::{
        SwitchControlProfile, SwitchEventProfile, SwitchReadingProfile, SwitchStatusProfile,
    },
};
use openfmb_protobuf_ext::switch::SwitchControlExt;
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
        + Subscriber<SwitchReadingProfile>
        + Publisher<SwitchControlProfile>,
{
    bus: MB,
    mrid: Uuid,
}

/// Topic string given a message type and mrid
pub fn topic(typ: &'static str, mrid: &Uuid) -> String {
    format!("openfmb.switchmodule.{}.{}", typ, mrid.to_hyphenated())
}

impl<MB> Switch<MB>
where
    MB: Subscriber<SwitchStatusProfile>
        + Subscriber<SwitchEventProfile>
        + Subscriber<SwitchReadingProfile>
        + Publisher<SwitchControlProfile>,
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

    /// Send a control message to the device asynchronously
    ///
    /// Awaits on publishing but no change awaited on.
    pub async fn control(&mut self, msg: SwitchControlProfile) -> PublishResult<()> {
        Ok(self
            .bus
            .publish(&topic("SwitchControlProfile", &self.mrid), msg)
            .await?)
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
    /// while let Some(Ok(DbPosKind::Open)) = position.next().await {
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

    /// A stream of the switch dynamic test updates
    ///
    /// These may come from either SwitchEventProfile *or* SwitchStatusProfile
    /// messages so we subscribe to both streams and merge them together with
    /// some combinators.
    ///
    /// Can be used directly in logic to wait for a dynamic test change
    ///
    /// ```
    /// let dynamic_test = myswitch.dynamic_test().await?;
    /// while let Some(Ok(DynamicTest::None)) = dynamic_test.next().await {
    ///   myswitch.control(SwitchControlProfile::builder().set_synchro_check(true);
    /// }
    /// ```
    pub async fn dynamic_test(&mut self) -> SubscribeResult<DynamicTestKind> {
        let status = self.status()?.map(|s| match s {
            Ok(s) => Ok(DynamicTestKind::from_i32(
                s.switch_status
                    .unwrap()
                    .switch_status_xswi
                    .unwrap()
                    .dynamic_test
                    .unwrap()
                    .st_val,
            )
            .unwrap()),
            Err(err) => Err(err),
        });
        let event = self.event()?.map(|s| match s {
            Ok(s) => Ok(DynamicTestKind::from_i32(
                s.switch_event
                    .unwrap()
                    .switch_event_xswi
                    .unwrap()
                    .dynamic_test
                    .unwrap()
                    .st_val,
            )
            .unwrap()),
            Err(err) => Err(err),
        });
        Ok(Box::pin(stream::select(status, event)))
    }

    fn mrid_as_string(&self) -> String {
        format!("{}", self.mrid.to_hyphenated())
    }


    /// Close the switch
    ///
    /// TODO add timeout and retry support
    pub async fn close(&mut self) -> ControlResult<()> {
        let mut is_open = self.is_open().await?;
        while let Some(Ok(true)) = is_open.next().await {
            let msg = SwitchControlProfile::switch_close_msg(&self.mrid_as_string());
            self.control(msg).await?
        }
        Ok(())
    }

    /// Open the switch
    ///
    /// TODO add timeout and retry support
    pub async fn open(&mut self) -> ControlResult<()> {
        let mut is_closed = self.is_closed().await?;
        while let Some(Ok(true)) = is_closed.next().await {
            let msg = SwitchControlProfile::switch_open_msg(&self.mrid_as_string());
            self.control(msg).await?;
        }
        Ok(())
    }

    /// Toggle the switch position
    ///
    /// TODO add timeout and retry support
    pub async fn toggle_position(&mut self) -> ControlResult<()> {
        if let Some(Ok(true)) = self.is_closed().await?.next().await {
            Ok(self.open().await?)
        } else {
            Ok(self.close().await?)
        }
    }

    /// Set the switch synchro check dynamic test
    ///
    /// TODO add timeout and retry support
    pub async fn set_synchro_check(&mut self, synchro_check: bool) -> ControlResult<()> {
        if synchro_check {
            self.enable_synchro_check().await
        } else {
            self.disable_synchro_check().await
        }
    }

    /// Enable the switch synchro check dynamic test
    ///
    /// TODO add timeout and retry support
    pub async fn enable_synchro_check(&mut self) -> ControlResult<()> {
        let mut dynamic_test = self.dynamic_test().await?;
        while let Some(Ok(DynamicTestKind::None)) = dynamic_test.next().await {
            let msg = SwitchControlProfile::switch_synchro_msg(&self.mrid_as_string(), true);
            self.control(msg).await?;
        }
        Ok(())
    }

    /// Disable the switch synchro check dynamic test
    ///
    /// TODO add timeout and retry support
    pub async fn disable_synchro_check(&mut self) -> ControlResult<()> {
        let mut dynamic_test = self.dynamic_test().await?;
        while let Some(Ok(testing)) = dynamic_test.next().await {
            if testing == DynamicTestKind::None {
                break;
            }
            let msg = SwitchControlProfile::switch_synchro_msg(&self.mrid_as_string(), false);
            self.control(msg).await?;
        }
        Ok(())
    }
}
