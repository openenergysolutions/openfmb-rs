// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use openfmb_messages::{
    commonmodule::{MessageInfo, ReadingMessageInfo},
    solarmodule::SolarReadingProfile,
};

use crate::ReadingProfileExt;
use snafu::{OptionExt, ResultExt};
use uuid::Uuid;

use crate::{error::*, OpenFMBExt, OpenFMBExtReading};

impl OpenFMBExt for SolarReadingProfile {
    fn device_state(&self) -> OpenFMBResult<String> {
        Ok(self
            .solar_reading
            .as_ref()
            .context(NoSolarReading)?
            .reading_mmxu
            .as_ref()
            .context(NoReadingMmxu)?
            .w
            .as_ref()
            .context(NoW)?
            .net
            .as_ref()
            .context(NoNet)?
            .c_val
            .as_ref()
            .context(NoCVal)?
            .mag
            .to_string())
    }

    fn message_info(&self) -> Result<&MessageInfo, OpenFMBError> {
        Ok(self
            .reading_message_info
            .as_ref()
            .context(NoStatusMessageInfo)?
            .message_info
            .as_ref()
            .context(NoMessageInfo)?)
    }

    fn message_type(&self) -> OpenFMBResult<String> {
        Ok("SolarReadingProfile".to_string())
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

impl OpenFMBExtReading for SolarReadingProfile {
    fn reading_message_info(&self) -> OpenFMBResult<&ReadingMessageInfo> {
        Ok(self
            .reading_message_info
            .as_ref()
            .context(NoStatusMessageInfo)?)
    }
}

impl ReadingProfileExt for SolarReadingProfile {
    fn w_net(&self) -> OpenFMBResult<f64> {
        Ok(self
            .solar_reading
            .as_ref()
            .context(NoSolarReading)?
            .reading_mmxu
            .as_ref()
            .context(NoReadingMmxu)?
            .w
            .as_ref()
            .context(NoW)?
            .net
            .as_ref()
            .context(NoNet)?
            .c_val
            .as_ref()
            .context(NoCVal)?
            .mag)
    }

    fn q_net(&self) -> OpenFMBResult<f64> {
        Ok(self
            .solar_reading
            .as_ref()
            .context(NoSolarReading)?
            .reading_mmxu
            .as_ref()
            .context(NoReadingMmxu)?
            .v_ar
            .as_ref()
            .context(NoW)?
            .net
            .as_ref()
            .context(NoNet)?
            .c_val
            .as_ref()
            .context(NoCVal)?
            .mag)
    }

    fn v_net(&self) -> OpenFMBResult<f64> {
        Ok(self
            .solar_reading
            .as_ref()
            .context(NoSolarReading)?
            .reading_mmxu
            .as_ref()
            .context(NoReadingMmxu)?
            .ph_v
            .as_ref()
            .context(NoValue)?
            .net
            .as_ref()
            .context(NoNet)?
            .c_val
            .as_ref()
            .context(NoCVal)?
            .mag)
    }

    fn a_net(&self) -> OpenFMBResult<f64> {
        Ok(self
            .solar_reading
            .as_ref()
            .context(NoSolarReading)?
            .reading_mmxu
            .as_ref()
            .context(NoReadingMmxu)?
            .a
            .as_ref()
            .context(NoValue)?
            .net
            .as_ref()
            .context(NoNet)?
            .c_val
            .as_ref()
            .context(NoCVal)?
            .mag)
    }
}
