use openfmb_messages::commonmodule::EngGridConnectModeKind;
use std::str::FromStr;

use essmodule::EssStatusProfile;
use openfmb_messages::{
    commonmodule::{MessageInfo, StatusMessageInfo},
    *,
};

use crate::StatusProfileExt;
use snafu::{OptionExt, ResultExt};
use uuid::Uuid;

use crate::{error::*, ess::ESSStatusExt, OpenFMBExt, OpenFMBExtStatus};

impl OpenFMBExt for EssStatusProfile {
    fn device_state(&self) -> OpenFMBResult<String> {
        Ok(match self
            .ess_status
            .clone()
            .unwrap()
            .ess_status_zgen
            .unwrap()
            .e_ss_event_and_status_zgen
            .unwrap()
            .point_status
            .unwrap()
            .state
            .unwrap()
            .value
        {
            0 => "Undefined",
            1 => "Off",
            2 => "On",  
            3 => "StandBy",
            _ => unreachable!()    
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
        Ok("ESSStatusProfile".to_string())
    }

    fn device_mrid(&self) -> OpenFMBResult<Uuid> {
        Ok(Uuid::from_str(
            &self
                .ess
                .as_ref()
                .context(NoEss)?
                .conducting_equipment
                .as_ref()
                .context(NoConductingEquipment)?
                .m_rid,
        )
        .context(UuidError)?)
    }

    fn device_name(&self) -> OpenFMBResult<String> {
        Ok(self
            .ess
            .as_ref()
            .context(NoEss)?
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

impl OpenFMBExtStatus for EssStatusProfile {
    fn status_message_info(&self) -> OpenFMBResult<&StatusMessageInfo> {
        Ok(self
            .status_message_info
            .as_ref()
            .context(NoStatusMessageInfo)?)
    }
}

impl ESSStatusExt for EssStatusProfile {
    fn soc(&self) -> OpenFMBResult<f64> {
        Ok(self
            .ess_status
            .as_ref()
            .context(NoEssStatus)?
            .ess_status_zbat
            .as_ref()
            .context(NoEssStatusZBat)?
            .soc
            .as_ref()
            .context(NoSoc)?
            .mag)
    }
}

pub trait EssStatusExt: StatusProfileExt {
    fn ess_soc(&self) -> OpenFMBResult<f64>;
    fn ess_mode(&self) -> OpenFMBResult<EngGridConnectModeKind>;
}

impl EssStatusExt for EssStatusProfile {
    fn ess_soc(&self) -> OpenFMBResult<f64> {
        {
            Ok(self
                .ess_status
                .as_ref()
                .context(NoEssStatus)?
                .ess_status_zbat
                .as_ref()
                .context(NoEssStatusZBat)?
                .soc
                .as_ref()
                .context(NoSoc)?
                .mag
                / 100.0)
        }
    }

    fn ess_mode(&self) -> OpenFMBResult<EngGridConnectModeKind> {
        {
            Ok(self
                .ess_status
                .as_ref()
                .context(NoEssStatus)?
                .ess_status_zgen
                .as_ref()
                .context(NoEssStatusZGen)?
                .e_ss_event_and_status_zgen
                .as_ref()
                .context(NoEssEventAndStatusZGen)?
                .point_status
                .as_ref()
                .context(NoPointStatus)?
                .mode
                .clone()
                .context(NoMode)?)
        }
    }
}

impl StatusProfileExt for EssStatusProfile {}
