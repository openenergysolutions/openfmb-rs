// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

use crate::{prelude::*, topic};
use futures::{stream, StreamExt};
use log::trace;
use openfmb_messages::{
    commonmodule::{DbPosKind, DynamicTestKind},
    switchmodule::{
        SwitchDiscreteControlProfile, SwitchEventProfile, SwitchReadingProfile, SwitchStatusProfile,
    },
};
use openfmb_messages_ext::switch::SwitchControlExt;
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
        + Publisher<SwitchDiscreteControlProfile>,
{
    bus: MB,
    mrid: Uuid,
    status_topic: ProfileTopic,
    event_topic: ProfileTopic,
    reading_topic: ProfileTopic,
    discrete_control_topic: ProfileTopic,
}

fn topic(profile: topic::Profile, mrid: &Uuid) -> ProfileTopic {
    ProfileTopic::new(Module::SwitchModule, profile, mrid.clone())
}

impl<MB> Switch<MB>
where
    MB: Subscriber<SwitchStatusProfile>
        + Subscriber<SwitchEventProfile>
        + Subscriber<SwitchReadingProfile>
        + Publisher<SwitchDiscreteControlProfile>,
{
    /// Create a new switch client instance
    pub fn new(bus: MB, mrid: Uuid) -> Switch<MB> {
        Switch {
            bus,
            mrid,
            status_topic: topic(Profile::SwitchStatusProfile, &mrid),
            event_topic: topic(Profile::SwitchEventProfile, &mrid),
            reading_topic: topic(Profile::SwitchReadingProfile, &mrid),
            discrete_control_topic: topic(Profile::SwitchDiscreteControlProfile, &mrid),
        }
    }

    /// A stream to this devices status messages
    ///
    /// The return may be treated as a stream or as a future returning the
    /// next event
    pub async fn status(&mut self) -> SubscribeResult<SwitchStatusProfile> {
        self.bus.subscribe(self.status_topic.iter()).await
    }

    /// A stream to this devices reading messages
    ///
    /// The return may be treated as a stream or as a future returning the
    /// next event
    pub async fn event(&mut self) -> SubscribeResult<SwitchEventProfile> {
        self.bus.subscribe(self.event_topic.iter()).await
    }

    /// A stream to this devices reading messages
    ///
    /// The return may be treated as a stream or as a future returning the next
    /// reading value.
    pub async fn reading(&mut self) -> SubscribeResult<SwitchReadingProfile> {
        self.bus.subscribe(self.reading_topic.iter()).await
    }

    /// Send a control message to the device asynchronously
    ///
    /// Awaits on publishing but no change awaited on.
    pub async fn control(&mut self, msg: SwitchDiscreteControlProfile) -> PublishResult<()> {
        Ok(self
            .bus
            .publish(self.discrete_control_topic.iter(), msg)
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
    /// ```ignore
    /// let position = myswitch.position().await?;
    /// while let Some(Ok(DbPosKind::Open)) = position.next().await {
    ///   myswitch.control(SwitchControlProfile::builder().set_position(DbPosKind::Closed));
    /// }
    /// ```
    pub async fn position(&mut self) -> SubscribeResult<DbPosKind> {
        let status = self.status().await?.map(|s| match s {
            Ok(s) => Ok(DbPosKind::from_i32(
                s.switch_status
                    .unwrap_or_default()
                    .switch_status_xswi
                    .unwrap_or_default()
                    .pos
                    .unwrap_or_default()
                    .phs3
                    .unwrap_or_default()
                    .st_val,
            )
            .unwrap()),
            Err(err) => Err(err),
        });
        let event = self.event().await?.map(|s| match s {
            Ok(s) => Ok(DbPosKind::from_i32(
                s.switch_event
                    .unwrap_or_default()
                    .switch_event_xswi
                    .unwrap_or_default()
                    .pos
                    .unwrap_or_default()
                    .phs3
                    .unwrap_or_default()
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
    /// ```ignore
    /// let dynamic_test = myswitch.dynamic_test().await?;
    /// while let Some(Ok(DynamicTest::None)) = dynamic_test.next().await {
    ///   myswitch.control(SwitchControlProfile::builder().set_synchro_check(true);
    /// }
    /// ```
    pub async fn dynamic_test(&mut self) -> SubscribeResult<DynamicTestKind> {
        let status = self.status().await?.map(|s| match s {
            Ok(s) => Ok(DynamicTestKind::from_i32(
                s.switch_status
                    .unwrap_or_default()
                    .switch_status_xswi
                    .unwrap_or_default()
                    .dynamic_test
                    .unwrap_or_default()
                    .st_val,
            )
            .unwrap()),
            Err(err) => Err(err),
        });
        let event = self.event().await?.map(|s| match s {
            Ok(s) => Ok(DynamicTestKind::from_i32(
                s.switch_event
                    .unwrap_or_default()
                    .switch_event_xswi
                    .unwrap_or_default()
                    .dynamic_test
                    .unwrap_or_default()
                    .st_val,
            )
            .unwrap()),
            Err(err) => Err(err),
        });
        Ok(Box::pin(stream::select(status, event)))
    }

    fn mrid_as_string(&self) -> String {
        format!("{}", self.mrid.as_hyphenated())
    }

    /// Close the switch
    ///
    /// TODO add timeout and retry support
    pub async fn close(&mut self) -> ControlResult<()> {
        let mut is_open = self.is_open().await?;
        while let Some(Ok(true)) = is_open.next().await {
            let msg = SwitchDiscreteControlProfile::switch_close_msg(&self.mrid_as_string());
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
            let msg = SwitchDiscreteControlProfile::switch_open_msg(&self.mrid_as_string());
            self.control(msg).await?;
        }
        Ok(())
    }

    /// Toggle the switch position
    ///
    /// TODO add timeout and retry support
    pub async fn toggle_position(&mut self) -> ControlResult<()> {
        if let Some(Ok(true)) = self.is_closed().await?.next().await {
            trace!("Switch {:?} is closed, opening", self.mrid);
            Ok(self.open().await?)
        } else {
            trace!("Switch {:?} is open, closing", self.mrid);
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
            let msg =
                SwitchDiscreteControlProfile::switch_synchro_msg(&self.mrid_as_string(), true);
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
            let msg =
                SwitchDiscreteControlProfile::switch_synchro_msg(&self.mrid_as_string(), false);
            self.control(msg).await?;
        }
        Ok(())
    }

    pub async fn w(&mut self) -> SubscribeResult<f64> {
        let w = self.reading().await?.map( |r| match r {
          Ok(r) => Ok(r
            .switch_reading.first().unwrap()
            .reading_mmxu.as_ref().unwrap()
            .w.as_ref().unwrap()
            .net.as_ref().unwrap()
            .c_val.as_ref().unwrap()
            .mag),
          Err(err) => Err(err)
        });
        Ok(Box::pin(w))
    }

    pub async fn var(&mut self) -> SubscribeResult<f64> {
        let w = self.reading().await?.map( |r| match r {
          Ok(r) => Ok(r
            .switch_reading.first().unwrap()
            .reading_mmxu.as_ref().unwrap()
            .v_ar.as_ref().unwrap()
            .net.as_ref().unwrap()
            .c_val.as_ref().unwrap()
            .mag),
          Err(err) => Err(err)
        });
        Ok(Box::pin(w))
    }

}
