use crate::prelude::*;
use openfmb_messages::switchmodule::{
    SwitchDiscreteControlProfile, SwitchEventProfile, SwitchReadingProfile, SwitchStatusProfile,
};
use uuid::Uuid;

/// Provide functionality to publish and accept messages a switch device
/// needs.
#[derive(Debug, Clone)]
pub struct Switch<MB>
where
    MB: std::fmt::Debug
        + Clone
        + Publisher<SwitchStatusProfile>
        + Publisher<SwitchEventProfile>
        + Publisher<SwitchReadingProfile>
        + Subscriber<SwitchDiscreteControlProfile>,
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
    MB: std::fmt::Debug
        + Clone
        + Publisher<SwitchStatusProfile>
        + Publisher<SwitchEventProfile>
        + Publisher<SwitchReadingProfile>
        + Subscriber<SwitchDiscreteControlProfile>,
{
    /// Create a new switch client instance
    pub fn new(bus: MB, mrid: Uuid) -> Switch<MB> {
        Switch { bus, mrid }
    }

    /// publish switch status messages
    pub async fn status(&mut self, msg: SwitchStatusProfile) -> PublishResult<()> {
        Ok(self
            .bus
            .publish(&topic("SwitchStatusProfile", &self.mrid), msg)
            .await?)
    }

    /// publish switch event messages
    pub async fn event(&mut self, msg: SwitchEventProfile) -> PublishResult<()> {
        Ok(self
            .bus
            .publish(&topic("SwitchEventProfile", &self.mrid), msg)
            .await?)
    }

    /// publish switch reading messages
    pub async fn reading(&mut self, msg: SwitchReadingProfile) -> PublishResult<()> {
        Ok(self
            .bus
            .publish(&topic("SwitchReadingProfile", &self.mrid), msg)
            .await?)
    }

    /// Subscribe to control messages
    pub fn control(&mut self) -> SubscribeResult<SwitchDiscreteControlProfile> {
        self.bus
            .subscribe(&topic("SwitchDiscreteControlProfile", &self.mrid))
    }
}
