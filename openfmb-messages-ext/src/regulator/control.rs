// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

use openfmb_messages::{commonmodule::*, regulatormodule::*};
use snafu::{OptionExt, ResultExt};
use std::{str::FromStr, time::SystemTime};
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

pub trait RegulatorControlExt: ControlProfileExt {
    fn schedule_regulator_control(
        m_rid: &str,
        schedule_parameter_type: ScheduleParameterKind,
        value: f64,
        schedule_time: SystemTime,
    ) -> RegulatorControlProfile;
}

impl RegulatorControlExt for RegulatorControlProfile {
    fn schedule_regulator_control(
        m_rid: &str,
        schedule_parameter_type: ScheduleParameterKind,
        value: f64,
        schedule_time: SystemTime,
    ) -> RegulatorControlProfile {
        let msg_info: ControlMessageInfo = RegulatorControlProfile::build_control_message_info();
        RegulatorControlProfile {
            control_message_info: Some(msg_info),
            regulator_system: Some(RegulatorSystem {
                conducting_equipment: Some(ConductingEquipment {
                    m_rid: m_rid.to_string(),
                    named_object: None,
                }),
            }),
            regulator_control: Some(RegulatorControl {
                check: None,
                control_value: None,
                regulator_control_fscc: Some(RegulatorControlFscc {
                    regulator_control_schedule_fsch: None,
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

impl ControlProfileExt for RegulatorControlProfile {}
