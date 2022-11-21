// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

use snafu::{OptionExt, ResultExt};
use std::str::FromStr;
use uuid::Uuid;

use openfmb_messages::{commonmodule::*, *};
use regulatormodule::RegulatorStatusProfile;

use crate::{error::*, OpenFMBExt, OpenFMBExtStatus, StatusProfileExt};

impl OpenFMBExtStatus for RegulatorStatusProfile {
    fn status_message_info(&self) -> OpenFMBResult<&StatusMessageInfo> {
        Ok(self
            .status_message_info
            .as_ref()
            .context(NoStatusMessageInfo)?)
    }
}

impl OpenFMBExt for RegulatorStatusProfile {
    fn device_state(&self) -> OpenFMBResult<String> {
        match self
            .regulator_status
            .as_ref()
            .context(NoRegulatorStatus)?
            .regulator_event_and_status_ancr
            .as_ref()
            .context(NoRegulatorEventAndStatusAncr)?
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
        Ok("RegulatorStatusProfile".to_string())
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
        Ok(self
            .regulator_system
            .as_ref()
            .context(NoRegulatorSystem)?
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

pub trait RegulatorStatusExt: StatusProfileExt {
    fn tap_pos_phs3(&self) -> OpenFMBResult<i32>;
    fn tap_pos_phs_a(&self) -> OpenFMBResult<i32>;
    fn tap_pos_phs_b(&self) -> OpenFMBResult<i32>;
    fn tap_pos_phs_c(&self) -> OpenFMBResult<i32>;

    fn vol_lmt_hi_phs3(&self) -> OpenFMBResult<bool>;
    fn vol_lmt_lo_phs3(&self) -> OpenFMBResult<bool>;

    fn voltage_setpoint_enabled(&self) -> OpenFMBResult<bool>;

    fn bnd_wid_hi_phs3(&self) -> OpenFMBResult<bool>;
    fn bnd_wid_lo_phs3(&self) -> OpenFMBResult<bool>;

    fn hot_line_tag(&self) -> OpenFMBResult<bool>;
    fn tap_op_err(&self) -> OpenFMBResult<bool>;

    fn state(&self) -> OpenFMBResult<StateKind>;
}

impl StatusProfileExt for RegulatorStatusProfile {}

impl RegulatorStatusExt for RegulatorStatusProfile {
    fn tap_pos_phs3(&self) -> OpenFMBResult<i32> {
        Ok(self
            .regulator_status
            .as_ref()
            .context(NoRegulatorStatus)?
            .regulator_event_and_status_ancr
            .as_ref()
            .context(NoRegulatorEventAndStatusAncr)?
            .point_status
            .as_ref()
            .context(NoPointStatus)?
            .tap_pos
            .as_ref()
            .context(NoTapPos)?
            .phs3
            .as_ref()
            .context(NoPhs3)?
            .st_val)
    }

    fn tap_pos_phs_a(&self) -> OpenFMBResult<i32> {
        Ok(self
            .regulator_status
            .as_ref()
            .context(NoRegulatorStatus)?
            .regulator_event_and_status_ancr
            .as_ref()
            .context(NoRegulatorEventAndStatusAncr)?
            .point_status
            .as_ref()
            .context(NoPointStatus)?
            .tap_pos
            .as_ref()
            .context(NoTapPos)?
            .phs_a
            .as_ref()
            .context(NoPhsA)?
            .st_val)
    }

    fn tap_pos_phs_b(&self) -> OpenFMBResult<i32> {
        Ok(self
            .regulator_status
            .as_ref()
            .context(NoRegulatorStatus)?
            .regulator_event_and_status_ancr
            .as_ref()
            .context(NoRegulatorEventAndStatusAncr)?
            .point_status
            .as_ref()
            .context(NoPointStatus)?
            .tap_pos
            .as_ref()
            .context(NoTapPos)?
            .phs_b
            .as_ref()
            .context(NoPhsB)?
            .st_val)
    }

    fn tap_pos_phs_c(&self) -> OpenFMBResult<i32> {
        Ok(self
            .regulator_status
            .as_ref()
            .context(NoRegulatorStatus)?
            .regulator_event_and_status_ancr
            .as_ref()
            .context(NoRegulatorEventAndStatusAncr)?
            .point_status
            .as_ref()
            .context(NoPointStatus)?
            .tap_pos
            .as_ref()
            .context(NoTapPos)?
            .phs_c
            .as_ref()
            .context(NoPhsC)?
            .st_val)
    }

    fn vol_lmt_hi_phs3(&self) -> OpenFMBResult<bool> {
        Ok(self
            .regulator_status
            .as_ref()
            .context(NoRegulatorStatus)?
            .regulator_event_and_status_ancr
            .as_ref()
            .context(NoRegulatorEventAndStatusAncr)?
            .point_status
            .as_ref()
            .context(NoPointStatus)?
            .vol_lmt_hi
            .as_ref()
            .context(NoVolLmtHi)?
            .phs3
            .as_ref()
            .context(NoPhs3)?
            .st_val)
    }

    fn vol_lmt_lo_phs3(&self) -> OpenFMBResult<bool> {
        Ok(self
            .regulator_status
            .as_ref()
            .context(NoRegulatorStatus)?
            .regulator_event_and_status_ancr
            .as_ref()
            .context(NoRegulatorEventAndStatusAncr)?
            .point_status
            .as_ref()
            .context(NoPointStatus)?
            .vol_lmt_lo
            .as_ref()
            .context(NoVolLmtLo)?
            .phs3
            .as_ref()
            .context(NoPhs3)?
            .st_val)
    }

    fn bnd_wid_hi_phs3(&self) -> OpenFMBResult<bool> {
        Ok(self
            .regulator_status
            .as_ref()
            .context(NoRegulatorStatus)?
            .regulator_event_and_status_ancr
            .as_ref()
            .context(NoRegulatorEventAndStatusAncr)?
            .point_status
            .as_ref()
            .context(NoPointStatus)?
            .bnd_wid_hi
            .as_ref()
            .context(NoBndWidHi)?
            .phs3
            .as_ref()
            .context(NoPhs3)?
            .st_val)
    }

    fn bnd_wid_lo_phs3(&self) -> OpenFMBResult<bool> {
        Ok(self
            .regulator_status
            .as_ref()
            .context(NoRegulatorStatus)?
            .regulator_event_and_status_ancr
            .as_ref()
            .context(NoRegulatorEventAndStatusAncr)?
            .point_status
            .as_ref()
            .context(NoPointStatus)?
            .bnd_wid_lo
            .as_ref()
            .context(NoBndWidLo)?
            .phs3
            .as_ref()
            .context(NoPhs3)?
            .st_val)
    }

    fn tap_op_err(&self) -> OpenFMBResult<bool> {
        Ok(self
            .regulator_status
            .as_ref()
            .context(NoRegulatorStatus)?
            .regulator_event_and_status_ancr
            .as_ref()
            .context(NoRegulatorEventAndStatusAncr)?
            .point_status
            .as_ref()
            .context(NoPointStatus)?
            .tap_op_err
            .as_ref()
            .context(NoTapOpErr)?
            .st_val)
    }

    fn hot_line_tag(&self) -> OpenFMBResult<bool> {
        Ok(self
            .regulator_status
            .as_ref()
            .context(NoRegulatorStatus)?
            .regulator_event_and_status_ancr
            .as_ref()
            .context(NoRegulatorEventAndStatusAncr)?
            .logical_node_for_event_and_status
            .as_ref()
            .context(NoLogicalNodeForEventAndStatus)?
            .hot_line_tag
            .as_ref()
            .context(NoHotLineTag)?
            .st_val)
    }

    fn voltage_setpoint_enabled(&self) -> OpenFMBResult<bool> {
        Ok(self
            .regulator_status
            .as_ref()
            .context(NoRegulatorStatus)?
            .regulator_event_and_status_ancr
            .as_ref()
            .context(NoRegulatorEventAndStatusAncr)?
            .point_status
            .as_ref()
            .context(NoPointStatus)?
            .voltage_set_point_enabled
            .as_ref()
            .context(NoVoltageSetPointEnabled)?
            .st_val)
    }

    fn state(&self) -> OpenFMBResult<StateKind> {
        Ok(
            match self
                .regulator_status
                .as_ref()
                .context(NoRegulatorStatus)?
                .regulator_event_and_status_ancr
                .as_ref()
                .context(NoRegulatorEventAndStatusAncr)?
                .point_status
                .as_ref()
                .context(NoPointStatus)?
                .state
                .as_ref()
                .context(NoState)?
                .value
            {
                1 => StateKind::Off,
                2 => StateKind::On,
                3 => StateKind::Standby,
                _ => StateKind::Undefined,
            },
        )
    }
}
