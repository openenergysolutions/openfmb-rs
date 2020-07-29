use crate::prelude::*;
use crate::prelude::*;
use futures::{stream, StreamExt};
use openfmb_ops_protobuf::openfmb::{
    commonmodule::{DynamicTestKind, DbPosKind},
    essmodule::{
        EssControlProfile, EssEventProfile, EssReadingProfile, EssStatusProfile,
    },
};
use openfmb_protobuf_ext::ess::EssControlExt;
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

    /// A stream to this devices status messages
    ///
    /// The return may be treated as a stream or as a future returning the
    /// next event
    pub fn status(&mut self) -> SubscribeResult<EssStatusProfile> {
        self.bus
            .subscribe(&topic("EssStatusProfile", &self.mrid))
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
        self.bus
            .subscribe(&topic("EssReadingProfile", &self.mrid))
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
}
