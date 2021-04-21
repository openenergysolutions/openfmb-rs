// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

use crate::{error::*, OpenFMBExt, OpenFMBExtStatus};
use openfmb_messages::{
    commonmodule::{MessageInfo, StatusMessageInfo},
    switchmodule::SwitchStatusProfile,
};
use snafu::{OptionExt, ResultExt};
use std::str::FromStr;
use uuid::Uuid;

impl OpenFMBExt for SwitchStatusProfile {
    fn device_state(&self) -> OpenFMBResult<String> {
        match self
            .switch_status
            .as_ref()
            .context(NoSwitchStatus)?
            .switch_status_xswi
            .as_ref()
            .context(NoSwitchStatusXswi)?
            .pos
            .as_ref()
            .context(NoPos)?
            .phs3
            .as_ref()
            .context(NoPhs3)            
        {
            Ok(phs3) => {
                match phs3.st_val {
                    0 => Ok("Undefined".into()),
                    1 => Ok("Transient".into()),
                    2 => Ok("Closed".into()),
                    3 => Ok("Open".into()),
                    4 => Ok("Invalid".into()),
                    _ => Err(OpenFMBError::InvalidValue)
                }
            }
            Err(_) => Err(OpenFMBError::InvalidOpenFMBMessage)
        }       
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
        Ok("SwitchStatusProfile".to_string())
    }

    fn device_mrid(&self) -> OpenFMBResult<Uuid> {
        Ok(Uuid::from_str(
            &self
                .protected_switch
                .as_ref()
                .context(NoProtectedSwitch)?
                .conducting_equipment
                .as_ref()
                .context(NoConductingEquipment)?
                .m_rid,
        )
        .context(UuidError)?)
    }

    fn device_name(&self) -> OpenFMBResult<String> {
        Ok(self
            .protected_switch
            .as_ref()
            .context(NoProtectedSwitch)?
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

impl OpenFMBExtStatus for SwitchStatusProfile {
    fn status_message_info(&self) -> OpenFMBResult<&StatusMessageInfo> {
        Ok(self
            .status_message_info
            .as_ref()
            .context(NoStatusMessageInfo)?)
    }
}
