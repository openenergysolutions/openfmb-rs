// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

use crate::StatusProfileExt;
use std::str::FromStr;

use generationmodule::GenerationStatusProfile;
use openfmb_messages::{
    commonmodule::{MessageInfo, StatusMessageInfo},
    *,
};
use snafu::{OptionExt, ResultExt};
use uuid::Uuid;

use crate::{error::*, OpenFMBExt, OpenFMBExtStatus};
use openfmb_messages::commonmodule::StateKind;

impl OpenFMBExt for GenerationStatusProfile {
    fn device_state(&self) -> OpenFMBResult<String> {
        match self
            .generation_status
            .as_ref()
            .context(NoGenerationStatus)?
            .generation_status_zgen
            .as_ref()
            .context(NoGenerationStatusZGen)?
            .generation_event_and_status_zgen
            .as_ref()
            .context(NoGenerationEventAndStatusZGen)?
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
            .status_message_info
            .as_ref()
            .context(NoStatusMessageInfo)?
            .message_info
            .as_ref()
            .context(NoMessageInfo)?)
    }

    fn message_type(&self) -> OpenFMBResult<String> {
        Ok("GenerationStatusProfile".to_string())
    }

    fn device_mrid(&self) -> OpenFMBResult<Uuid> {
        Ok(Uuid::from_str(
            &self
                .generating_unit
                .as_ref()
                .context(NoGeneratingUnit)?
                .conducting_equipment
                .as_ref()
                .context(NoConductingEquipment)?
                .m_rid,
        )
        .context(UuidError)?)
    }

    fn device_name(&self) -> OpenFMBResult<String> {
        Ok(self
            .generating_unit
            .as_ref()
            .context(NoGeneratingUnit)?
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

impl OpenFMBExtStatus for GenerationStatusProfile {
    fn status_message_info(&self) -> OpenFMBResult<&StatusMessageInfo> {
        Ok(self
            .status_message_info
            .as_ref()
            .context(NoStatusMessageInfo)?)
    }
}

pub trait GenerationStatusExt: StatusProfileExt {
    fn generation_status(&self) -> OpenFMBResult<StateKind>;
}

impl GenerationStatusExt for GenerationStatusProfile {
    fn generation_status(&self) -> OpenFMBResult<StateKind> {
        {
            Ok(self
                .generation_status
                .clone()
                .context(NoGenerationStatus)?
                .generation_status_zgen
                .context(NoGenerationStatusZGen)?
                .generation_event_and_status_zgen
                .unwrap()
                .point_status
                .unwrap()
                .state
                .unwrap()
                .value())
        }
    }
}

impl StatusProfileExt for GenerationStatusProfile {}
