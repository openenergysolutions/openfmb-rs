// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

use snafu::{OptionExt, ResultExt};
use std::str::FromStr;
use uuid::Uuid;

use openfmb_messages::{commonmodule::*, solarmodule::*};

use crate::{error::*, ControlProfileExt, OpenFMBExt};

impl OpenFMBExt for SolarDiscreteControlProfile {
    fn device_state(&self) -> OpenFMBResult<String> {
        Err(OpenFMBError::NoState)
    }

    fn message_info(&self) -> OpenFMBResult<&MessageInfo> {
        Ok(self
            .control_message_info
            .as_ref()
            .context(NoControlMessageInfo)?
            .message_info
            .as_ref()
            .context(NoMessageInfo)?)
    }

    fn message_type(&self) -> OpenFMBResult<String> {
        Ok("SolarDiscreteControlProfile".to_string())
    }

    fn device_mrid(&self) -> OpenFMBResult<Uuid> {
        Ok(Uuid::from_str(
            &self
                .solar_inverter
                .as_ref()
                .context(NoValue)?
                .conducting_equipment
                .as_ref()
                .context(NoConductingEquipment)?
                .m_rid,
        )
        .context(UuidError)?)
    }

    fn device_name(&self) -> OpenFMBResult<String> {
        Ok(self
            .solar_inverter
            .as_ref()
            .context(NoValue)?
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

impl ControlProfileExt for SolarDiscreteControlProfile {}
