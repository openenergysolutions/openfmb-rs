use std::str::FromStr;
use snafu::{OptionExt, ResultExt};
use uuid::Uuid;

use resourcemodule::ResourceStatusProfile;
use openfmb_messages::{
    commonmodule::*,
    *,
};


use crate::{error::*, OpenFMBExt, OpenFMBExtStatus};

impl OpenFMBExtStatus for ResourceStatusProfile {
    fn status_message_info(&self) -> OpenFMBResult<&StatusMessageInfo> {
        Ok(self
            .status_message_info
            .as_ref()
            .context(NoStatusMessageInfo)?)
    }
}

impl OpenFMBExt for ResourceStatusProfile {
    fn device_state(&self) -> OpenFMBResult<String> {
        Ok("".into())
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
        Ok("ResourceStatusProfile".to_string())
    }

    fn device_mrid(&self) -> OpenFMBResult<Uuid> {
        Ok(Uuid::from_str(
            &self
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