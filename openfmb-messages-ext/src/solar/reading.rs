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
        let mut state = String::new();
        state.push_str(
            &self
                .solar_reading
                .clone()
                .unwrap()
                .reading_mmxu
                .unwrap()
                .w
                .unwrap()
                .net
                .unwrap()
                .c_val
                .unwrap()
                .mag
                .unwrap()
                .f
                .unwrap()
                .to_string(),
        );
        state.push_str(" kWh");
        Ok(state)
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

impl OpenFMBExtReading for SolarReadingProfile {
    fn reading_message_info(&self) -> OpenFMBResult<&ReadingMessageInfo> {
        Ok(self
            .reading_message_info
            .as_ref()
            .context(NoStatusMessageInfo)?)
    }
}

pub trait SolarReadingExt: ReadingProfileExt {
    fn solar_reading(&self) -> f32;
}

impl SolarReadingExt for SolarReadingProfile {
    fn solar_reading(&self) -> f32 {
        {
            self.solar_reading
                .clone()
                .unwrap()
                .reading_mmxu
                .unwrap()
                .w
                .unwrap()
                .net
                .unwrap()
                .c_val
                .unwrap()
                .mag
                .unwrap()
                .f
                .unwrap()
        }
    }
}

impl ReadingProfileExt for SolarReadingProfile {}
