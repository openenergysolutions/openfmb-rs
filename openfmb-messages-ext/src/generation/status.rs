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
        Ok(match self
            .generation_status
            .clone()
            .unwrap()
            .generation_status_zgen
            .unwrap()
            .generation_event_and_status_zgen
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
        Ok("GenerationStatusProfile".to_string())
    }

    fn device_mrid(&self) -> OpenFMBResult<Uuid> {
        Ok(Uuid::from_str(
            &self
                .generating_unit
                .clone()
                .context(NoGeneratingUnit)?
                .conducting_equipment
                .context(NoConductingEquipment)?
                .m_rid,
        )
        .context(UuidError)?)
    }

    fn device_name(&self) -> OpenFMBResult<String> {
        Ok(self
            .generating_unit
            .clone()
            .context(NoMeter)?
            .conducting_equipment
            .context(NoConductingEquipment)?
            .named_object
            .context(NoNamedObject)?
            .name
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
