// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;
use snafu::{OptionExt, ResultExt};
use uuid::Uuid;

use loadmodule::LoadEventProfile;
use openfmb_messages::{
    commonmodule::*,
    *,
};


use crate::{error::*, OpenFMBExt, OpenFMBExtEvent};

impl OpenFMBExtEvent for LoadEventProfile {
    fn event_message_info(&self) -> OpenFMBResult<&EventMessageInfo> {
        Ok(self
            .event_message_info
            .as_ref()
            .context(NoEventMessageInfo)?)
    }
}

impl OpenFMBExt for LoadEventProfile {
    fn device_state(&self) -> OpenFMBResult<String> {
        match self
            .load_event
            .as_ref()
            .context(NoLoadEvent)?
            .load_event_zgld
            .as_ref()
            .context(NoLoadEventZGld)?
            .load_event_and_status_zgld
            .as_ref()
            .context(NoLoadEventAndStatusZGld)?
            .point_status
            .as_ref()
            .context(NoPointStatus)?
            .state
            .as_ref()
            .context(NoState)            
        {
            Ok(state) => {
                match state.value {
                    0 => Ok("Undefined".into()),
                    1 => Ok("Off".into()),
                    2 => Ok("On".into()),
                    3 => Ok("StandBy".into()),
                    _ => Err(OpenFMBError::InvalidValue)
                }
            }
            Err(_) => Err(OpenFMBError::InvalidOpenFMBMessage)
        }        
    }

    fn message_info(&self) -> OpenFMBResult<&MessageInfo> {
        Ok(self
            .event_message_info
            .as_ref()
            .context(NoEventMessageInfo)?
            .message_info
            .as_ref()
            .context(NoMessageInfo)?)
    }

    fn message_type(&self) -> OpenFMBResult<String> {
        Ok("LoadEventProfile".to_string())
    }

    fn device_mrid(&self) -> OpenFMBResult<Uuid> {
        Ok(Uuid::from_str(
            &self
                .energy_consumer
                .as_ref()
                .context(NoEnergyConsumer)?
                .conducting_equipment
                .as_ref()
                .context(NoConductingEquipment)?
                .m_rid,
        )
        .context(UuidError)?)
    }

    fn device_name(&self) -> OpenFMBResult<String> {
        Ok(self
            .energy_consumer
            .as_ref()
            .context(NoEnergyConsumer)?
            .conducting_equipment
            .as_ref()
            .context(NoConductingEquipment)?
            .named_object
            .as_ref()
            .context(NoNamedObject)?
            .name
            .clone()
            .context(NoName)?)
    }
}