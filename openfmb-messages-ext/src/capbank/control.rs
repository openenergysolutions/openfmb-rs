use std::str::FromStr;
use snafu::{OptionExt, ResultExt};
use uuid::Uuid;

use capbankmodule::CapBankControlProfile;
use openfmb_messages::{
    commonmodule::*,
    *,
};


use crate::{error::*, OpenFMBExt};

impl OpenFMBExt for CapBankControlProfile {
    fn device_state(&self) -> OpenFMBResult<String> {
        Ok("".into())
    }

    fn message_info(&self) -> OpenFMBResult<&MessageInfo> {
        Ok(self
            .control_message_info
            .as_ref()
            .context(NoControlMessageInfo)?
            .message_info
            .as_ref()
            .context(NoMessageInfo)?)
    }

    fn message_type(&self) -> OpenFMBResult<String> {
        Ok("CapBankControlProfile".to_string())
    }

    fn device_mrid(&self) -> OpenFMBResult<Uuid> {
        Ok(Uuid::from_str(
            &self
                .cap_bank_system
                .as_ref()
                .context(NoCapBankSystem)?
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