// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

use openfmb_messages::commonmodule::ControlMessageInfo;
use openfmb_messages::commonmodule::{
    CheckConditions, ConductingEquipment, ControlFscc, ControlScheduleFsch, ControlTimestamp,
    ControlValue, EngGridConnectModeKind, EngScheduleParameter, Ess, MessageInfo, NamedObject,
    OptionalStateKind, ScheduleCsg, ScheduleParameterKind, SchedulePoint, SocLimit, SocManagement,
    StateKind,
};
use openfmb_messages::evsemodule::EvseControlProfile;

use snafu::{OptionExt, ResultExt};
use std::{str::FromStr, time::SystemTime};
use uuid::Uuid;

use crate::{error::*, ControlProfileExt, OpenFMBExt};

impl OpenFMBExt for EvseControlProfile {
    fn device_state(&self) -> OpenFMBResult<String> {
        let eng_grid_connected_modekind = self
            .evse_control
            .as_ref()
            .unwrap()
            .control_dese
            .first()
            .as_ref()
            .unwrap()
            .dese_control_schedule_fsch
            .as_ref()
            .unwrap()
            .val_dcsg
            .as_ref()
            .unwrap()
            .crv_pts
            .first()
            .as_ref()
            .unwrap()
            .control
            .as_ref()
            .unwrap()
            .mode
            .as_ref()
            .unwrap();
        Ok(format!("param: {}", &eng_grid_connected_modekind.set_val))
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
        Ok("EVSEControlProfile".to_string())
    }

    fn device_mrid(&self) -> OpenFMBResult<Uuid> {
        Ok(Uuid::from_str(
            &self
                .evse
                .as_ref()
                .context(NoValue)?
                .conducting_equipment
                .as_ref()
                .context(NoConductingEquipment)?
                .m_rid,
        )
        .context(UuidError)?)
    }

    fn device_name(&self) -> OpenFMBResult<String> {
        Ok(self
            .evse
            .as_ref()
            .context(NoValue)?
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
