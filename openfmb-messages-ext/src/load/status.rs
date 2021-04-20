// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use openfmb_messages::{
    commonmodule::{MessageInfo, StatusMessageInfo},
    loadmodule::LoadStatusProfile,
};
use snafu::{OptionExt, ResultExt};
use uuid::Uuid;

use crate::{error::*, OpenFMBExt, OpenFMBExtStatus};

impl OpenFMBExt for LoadStatusProfile {
    fn device_state(&self) -> OpenFMBResult<String> {
        Ok(match self
            .load_status
            .clone()
            .unwrap()
            .load_status_zgld
            .unwrap()
            .load_event_and_status_zgld
            .unwrap()
            .point_status
            .unwrap()
            .state
            .unwrap()
            .value
        {
            0 => "Undefined",
            1 => "Off",
            2 => "On",  
            3 => "StandBy",
            _ => unreachable!()
        }
        .to_string())
    }

    fn message_info(&self) -> OpenFMBResult<&MessageInfo> {
        Ok(self
            .status_message_info
            .as_ref()
            .context(NoStatusMessageInfo)?
            .message_info
            .as_ref()
            .context(NoMessageInfo)?)
    }

    fn message_type(&self) -> OpenFMBResult<String> {
        Ok("LoadStatusProfile".to_string())
    }

    fn device_mrid(&self) -> OpenFMBResult<Uuid> {
        Ok(Uuid::from_str(
            &self
                .energy_consumer
                .clone()
                .context(NoEnergyConsumer)?
                .conducting_equipment
                .context(NoConductingEquipment)?
                .m_rid,
        )
        .context(UuidError)?)
    }

    fn device_name(&self) -> OpenFMBResult<String> {
        Ok(self
            .energy_consumer
            .clone()
            .context(NoEnergyConsumer)?
            .conducting_equipment
            .context(NoConductingEquipment)?
            .named_object
            .context(NoNamedObject)?
            .name
            .context(NoName)?)
    }
}

impl OpenFMBExtStatus for LoadStatusProfile {
    fn status_message_info(&self) -> OpenFMBResult<&StatusMessageInfo> {
        Ok(self
            .status_message_info
            .as_ref()
            .context(NoStatusMessageInfo)?)
    }
}
