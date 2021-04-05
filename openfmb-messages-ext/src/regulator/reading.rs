use std::str::FromStr;
use snafu::{OptionExt, ResultExt};
use uuid::Uuid;

use regulatormodule::RegulatorReadingProfile;
use openfmb_messages::{
    commonmodule::*,
    *,
};


use crate::{error::*, OpenFMBExt, OpenFMBExtReading};

impl OpenFMBExtReading for RegulatorReadingProfile {
    fn reading_message_info(&self) -> OpenFMBResult<&ReadingMessageInfo> {
        Ok(self
            .reading_message_info
            .as_ref()
            .context(NoReadingMessageInfo)?)
    }
}

impl OpenFMBExt for RegulatorReadingProfile {
    fn device_state(&self) -> OpenFMBResult<String> {
        Ok("".into())
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
        Ok("RegulatorReadingProfile".to_string())
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
        Ok("".to_string())
    }
}