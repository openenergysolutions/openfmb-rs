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
use openfmb_messages::evsemodule::{
    ControlDeao, ControlDeev, ControlDese, DeevControlScheduleFsch, Evse, EvseControl,
    EvseControlProfile,
};

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

pub trait EvseControlExt: ControlProfileExt {
    fn schedule_evse_control_deao(
        m_rid: &str,
        schedule_parameter_type: ScheduleParameterKind,
        value: f64,
        schedule_time: SystemTime,
    ) -> EvseControlProfile {
        let msg_info: ControlMessageInfo = EvseControlProfile::build_control_message_info();
        EvseControlProfile {
            control_message_info: Some(msg_info),
            evse: Some(Evse {
                conducting_equipment: Some(ConductingEquipment {
                    m_rid: m_rid.to_string(),
                    named_object: None,
                }),
            }),
            evse_control: Some(EvseControl {
                check: None,
                control_value: None,
                control_dese: vec![ControlDese {
                    control_deao: Some(ControlDeao {
                        control_deev: Some(ControlDeev {
                            deev_control_schedule_fsch: Some(DeevControlScheduleFsch {
                                val_acsg: Some(ScheduleCsg {
                                    sch_pts: vec![SchedulePoint {
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
                                        schedule_parameter: vec![EngScheduleParameter {
                                            schedule_parameter_type: schedule_parameter_type as i32,
                                            value: value,
                                        }],
                                    }],
                                }),
                            }),
                            ..Default::default()
                        }),
                        ..Default::default()
                    }),
                    ..Default::default()
                }],
            }),
        }
    }

    fn schedule_evse_control_fscc(
        m_rid: &str,
        schedule_parameter_type: ScheduleParameterKind,
        value: f64,
        schedule_time: SystemTime,
    ) -> EvseControlProfile {
        let msg_info: ControlMessageInfo = EvseControlProfile::build_control_message_info();
        EvseControlProfile {
            control_message_info: Some(msg_info),
            evse: Some(Evse {
                conducting_equipment: Some(ConductingEquipment {
                    m_rid: m_rid.to_string(),
                    named_object: None,
                }),
            }),
            evse_control: Some(EvseControl {
                control_dese: vec![ControlDese {
                    control_fscc: Some(ControlFscc {
                        control_schedule_fsch: Some(ControlScheduleFsch {
                            val_acsg: Some(ScheduleCsg {
                                sch_pts: vec![SchedulePoint {
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
                                    schedule_parameter: vec![EngScheduleParameter {
                                        schedule_parameter_type: schedule_parameter_type as i32,
                                        value: value,
                                    }],
                                }],
                            }),
                        }),
                        ..Default::default()
                    }),
                    ..Default::default()
                }],
                ..Default::default()
            }),
        }
    }
}

impl ControlProfileExt for EvseControlProfile {}
impl EvseControlExt for EvseControlProfile {}
