// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

// Custom clippy lints for this crate, used to avoid unwrap() calls on options
// and results in this crate. Nothing in this crate should cause a panic.
#![deny(clippy::all)]

use std::{fmt, fmt::Debug, str::FromStr, time::SystemTime};

use openfmb_messages::commonmodule::{
    ControlMessageInfo, DbPosKind, EventMessageInfo, IdentifiedObject, MessageInfo,
    ReadingMessageInfo, StatusMessageInfo, Timestamp,
};
use snafu::{OptionExt, ResultExt};
use uuid::Uuid;

pub mod breaker;
pub mod capbank;
pub mod circuitsegementservice;
pub mod error;
pub mod ess;
pub mod generation;
pub mod load;
pub mod meter;
pub mod recloser;
pub mod regulator;
pub mod resource;
pub mod solar;
pub mod solarforecast;
pub mod switch;

pub mod utils;

pub use breaker::{BreakerControlExt, BreakerReadingExt};
pub use capbank::CapBankControlExt;
pub use error::{OpenFMBError, OpenFMBResult};
pub use ess::{EssControlExt, EssReadingExt, EssStatusExt};
pub use generation::{GenerationControlExt, GenerationReadingExt};
pub use load::{LoadControlExt, LoadReadingExt, LoadStatusExt};
pub use recloser::{RecloserControlExt, RecloserReadingExt};
pub use regulator::{RegulatorControlExt, RegulatorDiscreteControlExt};
pub use resource::ResourceControlExt;
pub use solar::{SolarControlExt, SolarReadingExt};
pub use switch::{SwitchControlExt, SwitchReadingExt};
pub use utils::*;

pub trait ReadingProfileExt {}
pub trait StatusProfileExt {}

pub trait ControlProfileExt {
    fn build_control_message_info() -> ControlMessageInfo {
        let time_since_epoch = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("Now since epoch should always be valid");
        ControlMessageInfo {
            message_info: Some(MessageInfo {
                identified_object: Some(IdentifiedObject {
                    description: None,
                    m_rid: Some(Uuid::new_v4().to_string()),
                    name: None,
                }),
                message_time_stamp: Some(Timestamp {
                    //fraction: 0,
                    nanoseconds: time_since_epoch.subsec_nanos(),
                    seconds: time_since_epoch.as_secs(), /*+28800*/
                    tq: None,
                }),
            }),
        }
    }
}

pub trait OpenFMBExt {
    fn device_mrid(&self) -> OpenFMBResult<Uuid>;
    fn device_state(&self) -> OpenFMBResult<String>;
    fn device_name(&self) -> OpenFMBResult<String>;
    fn message_type(&self) -> OpenFMBResult<String>;
    fn message_info(&self) -> OpenFMBResult<&MessageInfo>;
    fn message_mrid(&self) -> OpenFMBResult<Uuid> {
        Ok(Uuid::from_str(
            &self
                .message_info()?
                .identified_object
                .as_ref()
                .context(error::NoIdentifiedObject)?
                .m_rid
                .as_ref()
                .context(error::NoMRID)?,
        )
        .context(error::UuidError)?)
    }
}

pub trait OpenFMBExtStatus: Debug {
    fn status_message_info(&self) -> OpenFMBResult<&StatusMessageInfo>;
}

pub trait Position: Debug {
    fn pos(&self) -> OpenFMBResult<DbPosKind>;
    fn pos_per_phase(&self, phase: Phase) -> OpenFMBResult<DbPosKind>;
}

pub trait OpenFMBExtEss {
    fn percent_soc(&self) -> OpenFMBResult<f32>;
}

pub trait OpenFMBExtReading: Debug {
    fn reading_message_info(&self) -> OpenFMBResult<&ReadingMessageInfo>;
}

pub trait OpenFMBExtEvent: Debug {
    fn event_message_info(&self) -> OpenFMBResult<&EventMessageInfo>;
}

impl fmt::Display for dyn OpenFMBExt {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(
            f,
            "{}: {}\nmrid: {}\n{}\n",
            self.message_type().unwrap_or_else(|_| "".into()),
            self.device_name().unwrap_or_else(|_| "".into()),
            self.device_mrid().unwrap_or_else(|_| Default::default()),
            self.device_state().unwrap_or_else(|_| "".into()),
        )
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Phase {
    Phs3,
    PhsA,
    PhsB,
    PhsC,
}
