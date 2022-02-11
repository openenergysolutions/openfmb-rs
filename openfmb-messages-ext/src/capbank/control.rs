// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

use snafu::{OptionExt, ResultExt};
use std::{str::FromStr, time::SystemTime};
use uuid::Uuid;

use capbankmodule::{CapBankControl, CapBankControlFscc, CapBankControlProfile, CapBankSystem};
use openfmb_messages::{commonmodule::*, *};

use crate::{error::*, ControlProfileExt, OpenFMBExt};

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
        Ok(self
            .cap_bank_system
            .as_ref()
            .context(NoCapBankSystem)?
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

pub trait CapBankControlExt: ControlProfileExt {
    fn schedule_capbank_control(
        m_rid: &str,
        schedule_parameter_type: ScheduleParameterKind,
        value: f64,
        schedule_time: SystemTime,
    ) -> CapBankControlProfile;
}

impl CapBankControlExt for CapBankControlProfile {
    fn schedule_capbank_control(
        m_rid: &str,
        schedule_parameter_type: ScheduleParameterKind,
        value: f64,
        schedule_time: SystemTime,
    ) -> CapBankControlProfile {
        let msg_info: ControlMessageInfo = CapBankControlProfile::build_control_message_info();
        CapBankControlProfile {
            control_message_info: Some(msg_info),
            cap_bank_system: Some(CapBankSystem {
                conducting_equipment: Some(ConductingEquipment {
                    m_rid: m_rid.to_string(),
                    named_object: None,
                }),
            }),
            cap_bank_control: Some(CapBankControl {
                check: None,
                control_value: None,
                cap_bank_control_fscc: Some(CapBankControlFscc {
                    cap_bank_control_schedule_fsch: None,
                    control_fscc: Some(ControlFscc {
                        logical_node_for_control: None,
                        island_control_schedule_fsch: None,
                        control_schedule_fsch: Some(ControlScheduleFsch {
                            val_acsg: Some(ScheduleCsg {
                                sch_pts: vec![SchedulePoint {
                                    schedule_parameter: vec![EngScheduleParameter {
                                        schedule_parameter_type: schedule_parameter_type as i32,
                                        value: value,
                                    }],
                                    start_time: Some(ControlTimestamp {
                                        nanoseconds: schedule_time
                                            .duration_since(SystemTime::UNIX_EPOCH)
                                            .unwrap()
                                            .subsec_nanos(),
                                        seconds: schedule_time
                                            .duration_since(SystemTime::UNIX_EPOCH)
                                            .unwrap()
                                            .as_secs(),
                                    }),
                                }],
                            }),
                        }),
                    }),
                }),
            }),
        }
    }
}

impl ControlProfileExt for CapBankControlProfile {}
