// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

use crate::ControlProfileExt;

use std::time::SystemTime;

use crate::{error::*, OpenFMBExt};
use openfmb_messages::{
    commonmodule::{
        ConductingEquipment, ControlFscc, ControlScheduleFsch, ControlTimestamp, EnergyConsumer,
        EngScheduleParameter, MessageInfo, NamedObject, OptionalStateKind, ScheduleCsg,
        ScheduleParameterKind, SchedulePoint, StateKind,
    },
    loadmodule::{
        LoadControl, LoadControlFscc, LoadControlProfile, LoadControlScheduleFsch, LoadCsg,
        LoadPoint,
    },
};
use snafu::{OptionExt, ResultExt};
use std::str::FromStr;
use uuid::Uuid;

impl OpenFMBExt for LoadControlProfile {
    fn device_state(&self) -> OpenFMBResult<String> {
        let load_control = self
            .load_control
            .clone()
            .unwrap()
            .load_control_fscc
            .unwrap()
            .load_control_schedule_fsch
            .unwrap()
            .val_dcsg
            .unwrap()
            .crv_pts
            .first()
            .unwrap()
            .state
            .clone()
            .unwrap()
            .value;
        Ok(format!("Control: {}", load_control))
    }

    fn message_info(&self) -> OpenFMBResult<&MessageInfo> {
        unimplemented!()
        //        Ok(self.solar_control.clone().context(NoStatusMessageInfo)?..unwrap())
    }

    fn message_type(&self) -> OpenFMBResult<String> {
        Ok("LoadControlProfile".to_string())
    }

    fn device_mrid(&self) -> OpenFMBResult<Uuid> {
        Ok(Uuid::from_str(
            &self
                .energy_consumer
                .as_ref()
                .context(NoEnergyConsumer)?
                .conducting_equipment
                .as_ref()
                .context(NoConductingEquipment)?
                .m_rid,
        )
        .context(UuidError)?)
    }

    fn device_name(&self) -> OpenFMBResult<String> {
        Ok(self
            .energy_consumer
            .as_ref()
            .context(NoEnergyConsumer)?
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

pub trait LoadControlExt: ControlProfileExt {
    fn loadbank_on_msg(m_rid: &str, load_value: f64) -> LoadControlProfile {
        LoadControlProfile {
            control_message_info: Some(Self::build_control_message_info()),
            energy_consumer: Self::build_energy_consumer(Uuid::from_str(m_rid).unwrap()),
            load_control: Self::build_load_control(load_value, StateKind::On as i32),
        }
    }

    fn loadbank_off_msg(m_rid: &str) -> LoadControlProfile {
        LoadControlProfile {
            control_message_info: Some(Self::build_control_message_info()),
            energy_consumer: Self::build_energy_consumer(Uuid::from_str(m_rid).unwrap()),
            load_control: Self::build_load_control(0.0, StateKind::Off as i32),
        }
    }

    fn build_control_profile() -> LoadControlProfile {
        LoadControlProfile {
            control_message_info: Some(Self::build_control_message_info()),
            energy_consumer: None,
            load_control: None,
        }
    }

    fn build_energy_consumer(mrid: Uuid) -> Option<EnergyConsumer> {
        Some(EnergyConsumer {
            conducting_equipment: Some(ConductingEquipment {
                m_rid: mrid.to_string(),
                named_object: Some(NamedObject {
                    description: None,
                    name: Some("load-bank".to_string()),
                }),
            }),
            operating_limit: None,
        })
    }

    // fn build_control_message_info() -> Option<ControlMessageInfo> {
    //     Some(ControlMessageInfo {
    //         message_info: Some(MessageInfo {
    //             identified_object: Some(IdentifiedObject {
    //                 description: None,
    //                  m_rid: Some(Uuid::new_v4().to_string()),
    //                   name: None,
    //               }),
    //               message_time_stamp: Some(Timestamp {
    //                   fraction: 0,
    //                   seconds: SystemTime::now()
    //                       .duration_since(SystemTime::UNIX_EPOCH)
    //                       .unwrap()
    //                        .as_secs(), /*+28800*/
    //                    tq: None,
    //                }),
    //            }),
    //        })
    //    }

    fn build_load_control(load_value: f64, state: i32) -> Option<LoadControl> {
        let start_time = SystemTime::now();
        Some(LoadControl {
            control_value: None,
            check: None,
            load_control_fscc: Some(LoadControlFscc {
                control_fscc: Some(ControlFscc {
                    logical_node_for_control: None,
                    control_schedule_fsch: Some(ControlScheduleFsch {
                        val_acsg: Some(ScheduleCsg {
                            sch_pts: vec![SchedulePoint {
                                schedule_parameter: vec![EngScheduleParameter {
                                    schedule_parameter_type: ScheduleParameterKind::WNetMag as i32,
                                    value: load_value,
                                }],
                                start_time: Some(ControlTimestamp {
                                    nanoseconds: start_time
                                        .duration_since(SystemTime::UNIX_EPOCH)
                                        .unwrap()
                                        .subsec_nanos(),
                                    seconds: start_time
                                        .duration_since(SystemTime::UNIX_EPOCH)
                                        .unwrap()
                                        .as_secs(), /*+28800*/
                                }),
                            }],
                        }),
                    }),
                    island_control_schedule_fsch: None,
                }),
                load_control_schedule_fsch: Some(LoadControlScheduleFsch {
                    val_dcsg: Some(LoadCsg {
                        crv_pts: vec![LoadPoint {
                            ramp_rates: None,
                            reactive_pwr_set_point_enabled: None,
                            real_pwr_set_point_enabled: None,
                            reset: None,
                            start_time: Some(ControlTimestamp {
                                nanoseconds: start_time
                                    .duration_since(SystemTime::UNIX_EPOCH)
                                    .unwrap()
                                    .subsec_nanos(),
                                seconds: start_time
                                    .duration_since(SystemTime::UNIX_EPOCH)
                                    .unwrap()
                                    .as_secs(), /*+28800*/
                            }),
                            state: Some(OptionalStateKind { value: state }),
                        }],
                    }),
                }),
            }),
        })
    }
}

impl ControlProfileExt for LoadControlProfile {}

impl LoadControlExt for LoadControlProfile {}
