use std::str::FromStr;

use openfmb_messages::{
    commonmodule::{MessageInfo, ReadingMessageInfo},
    generationmodule::GenerationReadingProfile,
};
use snafu::{OptionExt, ResultExt};
use uuid::Uuid;

use crate::{error::*, OpenFMBExt, OpenFMBExtReading, ReadingProfileExt};

impl OpenFMBExt for GenerationReadingProfile {
    fn device_state(&self) -> OpenFMBResult<String> {
        Ok(self
            .generation_reading
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
            .to_string())
    }

    fn message_info(&self) -> OpenFMBResult<&MessageInfo> {
        Ok(self
            .reading_message_info
            .as_ref()
            .context(NoStatusMessageInfo)?
            .message_info
            .as_ref()
            .context(NoMessageInfo)?)
    }

    fn message_type(&self) -> OpenFMBResult<String> {
        Ok("GenerationReadingProfile".to_string())
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
        self.generating_unit
            .clone()
            .context(NoGeneratingUnit)?
            .conducting_equipment
            .context(NoConductingEquipment)?
            .named_object
            .context(NoNamedObject)?
            .name
            .context(NoName)
    }
}

impl OpenFMBExtReading for GenerationReadingProfile {
    fn reading_message_info(&self) -> OpenFMBResult<&ReadingMessageInfo> {
        Ok(self
            .reading_message_info
            .as_ref()
            .context(NoStatusMessageInfo)?)
    }
}

pub trait GenerationReadingExt: ReadingProfileExt {
    fn generation_reading(&self) -> f32;
}

impl GenerationReadingExt for GenerationReadingProfile {
    fn generation_reading(&self) -> f32 {
        {
            self.generation_reading
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

impl ReadingProfileExt for GenerationReadingProfile {}
