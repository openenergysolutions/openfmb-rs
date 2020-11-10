use std::str::FromStr;

use openfmb_messages::{
    commonmodule::{MessageInfo, StatusMessageInfo},
    solarmodule::SolarStatusProfile,
};
use snafu::{OptionExt, ResultExt};
use uuid::Uuid;
use crate::{error::*, OpenFMBExt, OpenFMBExtStatus, StatusProfileExt};
use openfmb_messages::commonmodule::StateKind;

impl OpenFMBExt for SolarStatusProfile {
    fn device_state(&self) -> OpenFMBResult<String> {
        Ok(
            match self
                .solar_status
                .clone()
                .context(NoSolarStatus)?
                .solar_status_zgen
                .context(NoSolarStatusZGen)?
                .solar_event_and_status_zgen
                .context(NoSolarEventAndStatusZGen)?
                .point_status
                .context(NoPointStatus)?
                .state
                .context(NoState)?
                .value
                .to_string()
                .as_str()
            {
                "0" => "Off".to_string(),
                "1" => "On".to_string(),
                _ => unimplemented!(),
            },
        )
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
                .clone()
                .context(NoSolarInverter)?
                .conducting_equipment
                .context(NoConductingEquipment)?
                .m_rid,
        )
        .context(UuidError)?)
    }

    fn device_name(&self) -> OpenFMBResult<String> {
        Ok(self
            .solar_inverter
            .clone()
            .context(NoSolarInverter)?
            .conducting_equipment
            .context(NoConductingEquipment)?
            .named_object
            .context(NoNamedObject)?
            .name
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
    fn solar_status(&self) -> OpenFMBResult<StateKind>;
}

impl SolarStatusExt for SolarStatusProfile {
    fn solar_status(&self) -> OpenFMBResult<StateKind> {
        {
            Ok(self
                .solar_status //FIXME
                .clone()
                .context(NoSolarInverter)?
                .solar_status_zgen
                .context(NoSolarInverter)?
                .solar_event_and_status_zgen
                .context(NoSolarInverter)?
                .point_status
                .context(NoSolarInverter)?
                .state
                .context(NoSolarInverter)?
                .value())
        }
    }
}

impl StatusProfileExt for SolarStatusProfile {}
