use std::str::FromStr;

use metermodule::MeterReadingProfile;
use openfmb_messages::{
    commonmodule::{Ied, MessageInfo, ReadingMessageInfo},
    *,
};
use snafu::{OptionExt, ResultExt};
use uuid::Uuid;

use crate::{common::*, error::*, OpenFMBExt, OpenFMBExtReading, ReadingProfileExt};

impl OpenFMBExt for MeterReadingProfile {
    fn device_state(&self) -> OpenFMBResult<String> {
        let mut state = String::new();
        state.push_str(
            &self
                .meter_reading
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

    fn message_info(&self) -> OpenFMBResult<&MessageInfo> {
        Ok(self
            .reading_message_info
            .as_ref()
            .context(NoReadingMessageInfo)?
            .message_info
            .as_ref()
            .context(NoMessageInfo)?)
    }

    fn message_type(&self) -> OpenFMBResult<String> {
        Ok("MeterReadingProfile".to_string())
    }

    fn device_mrid(&self) -> OpenFMBResult<Uuid> {
        Ok(Uuid::from_str(
            &self
                .meter
                .clone()
                .context(NoMeter)?
                .conducting_equipment
                .context(NoConductingEquipment)?
                .m_rid,
        )
        .context(UuidError)?)
    }

    fn device_name(&self) -> OpenFMBResult<String> {
        Ok(self
            .meter
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

impl OpenFMBExtReading for MeterReadingProfile {
    fn reading_message_info(&self) -> OpenFMBResult<&ReadingMessageInfo> {
        Ok(self
            .reading_message_info
            .as_ref()
            .context(NoStatusMessageInfo)?)
    }
}

pub trait MeterReadingExt: ReadingProfileExt {
    fn meter_reading(&self) -> f32;
}

impl MeterReadingExt for MeterReadingProfile {
    fn meter_reading(&self) -> f32 {
        {
            self.meter_reading
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

impl ReadingProfileExt for MeterReadingProfile {}

/// MeterReadingProfile has ReadingMessageInfo
impl HasReadingMessageInfo for MeterReadingProfile {
    fn get_reading_message_info(&self) -> OpenFMBResult<&ReadingMessageInfo> {
        Ok(self
            .reading_message_info
            .as_ref()
            .context(NoReadingMessageInfo)?)
    }
}

/// MeterReadingProfile has ReadingMessageInfo
impl HasIED for MeterReadingProfile {
    fn get_ied(&self) -> OpenFMBResult<&Ied> {
        Ok(self.ied.as_ref().context(NoIED)?)
    }
}
