// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

use openfmb_messages::{
    commonmodule::*,
    regulatormodule::*,
};
use snafu::{OptionExt, ResultExt};
use std::{str::FromStr};
use uuid::Uuid;

use crate::{error::*, ControlProfileExt, OpenFMBExt};

impl OpenFMBExt for RegulatorControlProfile {
    fn device_state(&self) -> OpenFMBResult<String> {
        unimplemented!() 
    }

    fn message_info(&self) -> OpenFMBResult<&MessageInfo> {
        unimplemented!()       
    }

    fn message_type(&self) -> OpenFMBResult<String> {
        Ok("RegulatorControlProfile".to_string())
    }

    fn device_mrid(&self) -> OpenFMBResult<Uuid> {
        Ok(Uuid::from_str(
            &self
                .regulator_system
                .as_ref()
                .context(NoRegulatorSystem)?
                .conducting_equipment
                .as_ref()
                .context(NoConductingEquipment)?
                .m_rid,
        )
        .context(UuidError)?)
    }

    fn device_name(&self) -> OpenFMBResult<String> {
        Ok(self
            .regulator_system
            .as_ref()
            .context(NoRegulatorSystem)?
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

pub trait RegulatorControlExt: ControlProfileExt {
    
}

impl ControlProfileExt for RegulatorControlProfile {}
