use openfmb_messages::{
    commonmodule::*,
    regulatormodule::*,
};
use snafu::{OptionExt, ResultExt};
use std::{str::FromStr};
use uuid::Uuid;

use crate::{error::*, ControlProfileExt, OpenFMBExt};

impl OpenFMBExt for RegulatorControlProfile {
    fn device_state(&self) -> OpenFMBResult<String> {
        unimplemented!() 
    }

    fn message_info(&self) -> OpenFMBResult<&MessageInfo> {
        unimplemented!()       
    }

    fn message_type(&self) -> OpenFMBResult<String> {
        Ok("RegulatorControlProfile".to_string())
    }

    fn device_mrid(&self) -> OpenFMBResult<Uuid> {
        Ok(Uuid::from_str(
            &self
                .regulator_system
                .clone()
                .context(NoProtectedSwitch)?
                .conducting_equipment
                .context(NoConductingEquipment)?
                .m_rid,
        )
        .context(UuidError)?)
    }

    fn device_name(&self) -> OpenFMBResult<String> {
        Ok("No Name Specified".to_string())
    }
}

pub trait RegulatorControlExt: ControlProfileExt {
    
}

impl ControlProfileExt for RegulatorControlProfile {}
