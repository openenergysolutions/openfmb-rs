// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

use openfmb_messages::commonmodule::EngGridConnectModeKind;
use std::str::FromStr;

use essmodule::EssStatusProfile;
use openfmb_messages::{
    commonmodule::{MessageInfo, StateKind, StatusMessageInfo},
    *,
};

use crate::StatusProfileExt;
use snafu::{OptionExt, ResultExt};
use uuid::Uuid;

use crate::{error::*, OpenFMBExt, OpenFMBExtStatus};

impl OpenFMBExt for EssStatusProfile {
    fn device_state(&self) -> OpenFMBResult<String> {
        match self
            .ess_status
            .as_ref()
            .context(NoEssStatus)?
            .ess_status_zgen
            .as_ref()
            .context(NoEssStatus)?
            .e_ss_event_and_status_zgen
            .as_ref()
            .context(NoEssEventAndStatusZGen)?
            .point_status
            .as_ref()
            .context(NoPointStatus)?
            .state
            .as_ref()
            .context(NoState)
        {
            Ok(state) => match state.value {
                0 => Ok("Undefined".into()),
                1 => Ok("Off".into()),
                2 => Ok("On".into()),
                3 => Ok("StandBy".into()),
                _ => Err(OpenFMBError::InvalidValue),
            },
            Err(_) => Err(OpenFMBError::InvalidOpenFMBMessage),
        }
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

pub trait EssStatusExt: StatusProfileExt {
    fn ess_soc(&self) -> OpenFMBResult<f64>;
    fn ess_mode(&self) -> OpenFMBResult<EngGridConnectModeKind>;
    fn ess_state(&self) -> OpenFMBResult<StateKind>;

    fn ess_soh(&self) -> OpenFMBResult<f64>;
    fn ess_gn_sync_st(&self) -> OpenFMBResult<bool>;
    fn soc_max(&self) -> OpenFMBResult<f32>;
    fn soc_min(&self) -> OpenFMBResult<f32>;
    fn online_status(&self) -> OpenFMBResult<bool>;
    fn supervisory_control(&self) -> OpenFMBResult<bool>;
    fn reactive_power_enabled(&self) -> OpenFMBResult<bool>;
    fn active_power_enabled(&self) -> OpenFMBResult<bool>;
}

impl EssStatusExt for EssStatusProfile {
    fn ess_soc(&self) -> OpenFMBResult<f64> {
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

    fn ess_mode(&self) -> OpenFMBResult<EngGridConnectModeKind> {
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

    fn ess_state(&self) -> OpenFMBResult<StateKind> {
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
            .state
            .as_ref()
            .context(NoState)?
            .value())
    }

    fn ess_gn_sync_st(&self) -> OpenFMBResult<bool> {
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
            .gn_syn_st
            .as_ref()
            .context(NoEssGnSyncSt)?
            .st_val)
    }

    fn ess_soh(&self) -> OpenFMBResult<f64> {
        Ok(self
            .ess_status
            .as_ref()
            .context(NoEssStatus)?
            .ess_status_zbat
            .as_ref()
            .context(NoEssStatusZBat)?
            .so_h
            .as_ref()
            .context(NoSoh)?
            .mag
            / 100.0)
    }

    fn soc_min(&self) -> OpenFMBResult<f32> {
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
            .function
            .as_ref()
            .context(NoFunction)?
            .soc_limit
            .as_ref()
            .context(NoSocLimit)?
            .soc_low_limit
            .as_ref()
            .context(NoSocLow)?
            .clone())
    }

    fn soc_max(&self) -> OpenFMBResult<f32> {
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
            .function
            .as_ref()
            .context(NoFunction)?
            .soc_limit
            .as_ref()
            .context(NoSocLimit)?
            .soc_high_limit
            .as_ref()
            .context(NoSocHigh)?
            .clone())
    }

    fn online_status(&self) -> OpenFMBResult<bool> {
        Ok(self
            .ess_status
            .as_ref()
            .context(NoEssStatus)?
            .ess_status_zbat
            .as_ref()
            .context(NoEssStatusZBat)?
            .bat_st
            .as_ref()
            .context(NoBatSt)?
            .st_val)
    }

    fn supervisory_control(&self) -> OpenFMBResult<bool> {
        Ok(self
            .ess_status
            .as_ref()
            .context(NoEssStatus)?
            .ess_status_zbat
            .as_ref()
            .context(NoEssStatusZBat)?
            .stdby
            .as_ref()
            .context(NoStandby)?
            .st_val)
    }

    fn reactive_power_enabled(&self) -> OpenFMBResult<bool> {
        Ok(self
            .ess_status
            .as_ref()
            .context(NoEssStatus)?
            .ess_status_zgen
            .as_ref()
            .context(NoEssEventZGen)?
            .e_ss_event_and_status_zgen
            .as_ref()
            .context(NoEssEventAndStatusZGen)?
            .point_status
            .as_ref()
            .context(NoPointStatus)?
            .reactive_pwr_set_point_enabled
            .as_ref()
            .context(NoReactivePowerSet)?
            .st_val)
    }

    fn active_power_enabled(&self) -> OpenFMBResult<bool> {
        Ok(self
            .ess_status
            .as_ref()
            .context(NoEssStatus)?
            .ess_status_zgen
            .as_ref()
            .context(NoEssEventZGen)?
            .e_ss_event_and_status_zgen
            .as_ref()
            .context(NoEssEventAndStatusZGen)?
            .point_status
            .as_ref()
            .context(NoPointStatus)?
            .real_pwr_set_point_enabled
            .as_ref()
            .context(NoRealPowerSet)?
            .st_val)
    }
}

impl StatusProfileExt for EssStatusProfile {}
