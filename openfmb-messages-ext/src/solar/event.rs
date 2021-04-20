// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;
use snafu::{OptionExt, ResultExt};
use uuid::Uuid;

use solarmodule::SolarEventProfile;
use openfmb_messages::{
    commonmodule::*,
    *,
};


use crate::{error::*, OpenFMBExt, OpenFMBExtEvent};

impl OpenFMBExtEvent for SolarEventProfile {
    fn event_message_info(&self) -> OpenFMBResult<&EventMessageInfo> {
        Ok(self
            .event_message_info
            .as_ref()
            .context(NoEventMessageInfo)?)
    }
}

impl OpenFMBExt for SolarEventProfile {
    fn device_state(&self) -> OpenFMBResult<String> {
        Ok("".into())
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
        Ok("SolarEventProfile".to_string())
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
        Ok("".to_string())
    }
}