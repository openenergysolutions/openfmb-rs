use std::str::FromStr;

use reclosermodule::RecloserStatusProfile;
use openfmb_messages::{
    commonmodule::{MessageInfo, StatusMessageInfo},
    *,
};
use snafu::{OptionExt, ResultExt};
use uuid::Uuid;

use crate::{error::*, OpenFMBExt, OpenFMBExtStatus};

impl OpenFMBExtStatus for RecloserStatusProfile {
    fn status_message_info(&self) -> OpenFMBResult<&StatusMessageInfo> {
        Ok(self
            .status_message_info
            .as_ref()
            .context(NoStatusMessageInfo)?)
    }
}

impl OpenFMBExt for RecloserStatusProfile {
    fn device_state(&self) -> OpenFMBResult<String> {
        Ok(match self
            .recloser_status
            .as_ref()
            .context(NoBreakerStatus)?
            .status_and_event_xcbr
            .as_ref()
            .context(NoStatusAndEventXcbr)?
            .pos
           .as_ref()
           .context(NoPos)?
           .phs3
           .as_ref()
           .context(NoPhs3)?
           .st_val
        {
            0 => "Undefined",
            1 => "Transient",
            2 => "Closed",
            3 => "Open",
            4 => "Invalid",
            _ => unreachable!()
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
        Ok("RecloserStatusProfile".to_string())
    }

    fn device_mrid(&self) -> OpenFMBResult<Uuid> {
        Ok(Uuid::from_str(
            &self
                .recloser
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
            .recloser
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
