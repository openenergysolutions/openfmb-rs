use crate::{error::*, OpenFMBExt, OpenFMBExtStatus};
use openfmb_messages::{
    commonmodule::{MessageInfo, StatusMessageInfo},
    switchmodule::SwitchStatusProfile,
};
use snafu::{OptionExt, ResultExt};
use std::str::FromStr;
use uuid::Uuid;

impl OpenFMBExt for SwitchStatusProfile {
    fn device_state(&self) -> OpenFMBResult<String> {
        Ok(match self
            .switch_status
            .clone()
            .unwrap()
            .switch_status_xswi
            .unwrap()
            .pos
            .unwrap()
            .phs3
            .unwrap()
            .st_val
        {
            0 => "Transient",
            1 => "Closed",
            2 => "Open",
            3 => "Invalid",
            _ => unreachable!(),
        }
        .to_string())
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
        Ok("SwitchStatusProfile".to_string())
    }

    fn device_mrid(&self) -> OpenFMBResult<Uuid> {
        Ok(Uuid::from_str(
            &self
                .protected_switch
                .clone()
                .context(NoProtectedSwitch)?
                .conducting_equipment
                .context(NoConductingEquipment)?
                .m_rid,
        )
        .context(UuidError)?)
    }

    fn device_name(&self) -> OpenFMBResult<String> {
        Ok(self
            .protected_switch
            .clone()
            .context(NoProtectedSwitch)?
            .conducting_equipment
            .context(NoConductingEquipment)?
            .named_object
            .context(NoNamedObject)?
            .name
            .context(NoName)?)
    }
}

impl OpenFMBExtStatus for SwitchStatusProfile {
    fn status_message_info(&self) -> OpenFMBResult<&StatusMessageInfo> {
        Ok(self
            .status_message_info
            .as_ref()
            .context(NoStatusMessageInfo)?)
    }
}