// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use crate::{error::*, OpenFMBExt, OpenFMBExtStatus, StatusProfileExt};
use openfmb_messages::commonmodule::StateKind;
use openfmb_messages::{
    commonmodule::{MessageInfo, StatusMessageInfo},
    solarmodule::SolarStatusProfile,
};
use snafu::{OptionExt, ResultExt};
use uuid::Uuid;

impl OpenFMBExt for SolarStatusProfile {
    fn device_state(&self) -> OpenFMBResult<String> {
        match self
            .solar_status
            .as_ref()
            .context(NoSolarStatus)?
            .solar_status_zgen
            .as_ref()
            .context(NoSolarStatusZGen)?
            .solar_event_and_status_zgen
            .as_ref()
            .context(NoSolarEventAndStatusZGen)?
            .point_status
            .as_ref()
            .context(NoPointStatus)?
            .state
            .as_ref()
            .context(NoState)
        {
            Ok(state) => match state.value {
                0 => Ok("Undefined".to_string()),
                1 => Ok("Off".to_string()),
                2 => Ok("On".to_string()),
                3 => Ok("StandBy".to_string()),
                _ => Err(OpenFMBError::InvalidValue),
            },
            Err(_) => Err(OpenFMBError::InvalidOpenFMBMessage),
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
        Ok("SolarStatusProfile".to_string())
    }

    fn device_mrid(&self) -> OpenFMBResult<Uuid> {
        Ok(Uuid::from_str(
            &self
                .solar_inverter
                .as_ref()
                .context(NoSolarInverter)?
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
            .context(NoSolarInverter)?
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

impl OpenFMBExtStatus for SolarStatusProfile {
    fn status_message_info(&self) -> OpenFMBResult<&StatusMessageInfo> {
        Ok(self
            .status_message_info
            .as_ref()
            .context(NoStatusMessageInfo)?)
    }
}

pub trait SolarStatusExt: StatusProfileExt {
    fn solar_state(&self) -> OpenFMBResult<StateKind>;
    fn pct_v_droop(&self) -> OpenFMBResult<Option<f32>>;
}

impl SolarStatusExt for SolarStatusProfile {
    fn solar_state(&self) -> OpenFMBResult<StateKind> {
        Ok(self
            .solar_status
            .as_ref()
            .context(NoSolarStatus)?
            .solar_status_zgen
            .as_ref()
            .context(NoSolarStatusZGen)?
            .solar_event_and_status_zgen
            .as_ref()
            .context(NoSolarEventAndStatusZGen)?
            .point_status
            .as_ref()
            .context(NoPointStatus)?
            .state
            .as_ref()
            .context(NoState)?
            .value())
    }

    fn pct_v_droop(&self) -> OpenFMBResult<Option<f32>> {
        Ok(self
            .solar_status
            .as_ref()
            .context(NoSolarStatus)?
            .solar_status_zgen
            .as_ref()
            .context(NoSolarStatusZGen)?
            .solar_event_and_status_zgen
            .as_ref()
            .context(NoSolarEventAndStatusZGen)?
            .point_status
            .as_ref()
            .context(NoPointStatus)?
            .pct_v_droop            
        )
    }
}

impl StatusProfileExt for SolarStatusProfile {}
