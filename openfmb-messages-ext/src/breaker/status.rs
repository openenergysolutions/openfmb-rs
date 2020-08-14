use std::str::FromStr;

use breakermodule::BreakerStatusProfile;
use openfmb_messages::{
    commonmodule::{MessageInfo, StatusMessageInfo},
    *,
};
use snafu::{OptionExt, ResultExt};
use uuid::Uuid;

use crate::{error::*, OpenFMBExt, OpenFMBExtStatus};

impl OpenFMBExtStatus for BreakerStatusProfile {
    fn status_message_info(&self) -> OpenFMBResult<&StatusMessageInfo> {
        Ok(self
            .status_message_info
            .as_ref()
            .context(NoStatusMessageInfo)?)
    }
}

impl OpenFMBExt for BreakerStatusProfile {
    fn device_state(&self) -> OpenFMBResult<String> {
        Ok(match self
            .breaker_status
            .as_ref()
            .context(NoBreakerStatus)?
            .status_and_event_xcbr
            .as_ref()
            .context(NoStatusAndEventXcbr)?
            .pos
            .as_ref()
            .context(NoPos)?
            .st_val
        {
            0 => "Transient",
            1 => "Closed",
            2 => "Open",
            3 => "Invalid",
            _ => unreachable!(),
        }
        .into())
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
        Ok("BreakerStatusProfile".to_string())
    }

    fn device_mrid(&self) -> OpenFMBResult<Uuid> {
        Ok(Uuid::from_str(
            &self
                .breaker
                .as_ref()
                .context(NoBreaker)?
                .conducting_equipment
                .as_ref()
                .context(NoConductingEquipment)?
                .m_rid,
        )
        .context(UuidError)?)
    }

    fn device_name(&self) -> OpenFMBResult<String> {
        Ok(self
            .breaker
            .as_ref()
            .context(NoBreaker)?
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
