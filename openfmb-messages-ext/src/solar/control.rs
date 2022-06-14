// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

use crate::{error::*, ControlProfileExt, OpenFMBExt};
use openfmb_messages::{
    commonmodule::{
        ConductingEquipment, ControlFscc, ControlScheduleFsch, ControlTimestamp, ControlValue,
        EngScheduleParameter, MessageInfo, NamedObject, OptionalStateKind, ScheduleCsg,
        ScheduleParameterKind, SchedulePoint, StateKind,
    },
    solarmodule::{
        SolarControl, SolarControlFscc, SolarControlProfile, SolarControlScheduleFsch, SolarCsg,
        SolarInverter, SolarPoint,
    },
};
use snafu::{OptionExt, ResultExt};
use std::{str::FromStr, time::SystemTime};
use uuid::Uuid;

impl OpenFMBExt for SolarControlProfile {
    fn device_state(&self) -> OpenFMBResult<String> {
        let solar_control = self
            .solar_control
            .clone()
            .unwrap()
            .solar_control_fscc
            .unwrap()
            .control_fscc
            .unwrap()
            .control_schedule_fsch
            .unwrap()
            .val_acsg
            .unwrap()
            .sch_pts
            .first()
            .unwrap()
            .schedule_parameter
            .first()
            .unwrap()
            .clone();
        Ok(format!(
            "param: {}, value: {}",
            &solar_control.schedule_parameter_type, &solar_control.value
        ))
    }

    fn message_info(&self) -> OpenFMBResult<&MessageInfo> {
        unimplemented!()
        //        Ok(self.solar_control.clone().context(NoStatusMessageInfo)?..unwrap())
    }

    fn message_type(&self) -> OpenFMBResult<String> {
        Ok("SolarStatusProfile".to_string())
    }

    fn device_mrid(&self) -> OpenFMBResult<Uuid> {
        Ok(Uuid::from_str(
            &self
                .solar_inverter
                .as_ref()
                .context(NoSolarInverter)?
                .conducting_equipment
                .as_ref()
                .context(NoConductingEquipment)?
                .m_rid,
        )
        .context(UuidError)?)
    }

    fn device_name(&self) -> OpenFMBResult<String> {
        Ok(self
            .solar_inverter
            .as_ref()
            .context(NoSolarInverter)?
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

pub trait SolarControlExt: ControlProfileExt {
    fn solar_on_msg(m_rid: &str, val: f64) -> SolarControlProfile {
        Self::build_control_profile(m_rid, val, SystemTime::now(), StateKind::On as i32)
    }

    fn solar_off_msg(m_rid: &str) -> SolarControlProfile {
        Self::build_control_profile(m_rid, 0.0, SystemTime::now(), StateKind::Off as i32)
    }

    fn solar_modblk_msg(m_rid: &str, modblk: bool) -> SolarControlProfile {
        Self::build_modblk_profile(m_rid, SystemTime::now(), modblk)
    }

    fn build_control_profile(
        m_rid: &str,
        sch_param_value: f64,
        start_time: SystemTime,
        state: i32,
    ) -> SolarControlProfile;

    fn build_solar_control(
        _solar_acsg: f64,
        state: OptionalStateKind,
        when: SystemTime,
    ) -> SolarControl {
        SolarControl {
            control_value: None,
            check: None,
            //FIXME when should this be parameterized?
            solar_control_fscc: Some(SolarControlFscc {
                control_fscc: Some(ControlFscc {
                    logical_node_for_control: None,
                    control_schedule_fsch: Some(ControlScheduleFsch { val_acsg: None }),
                    island_control_schedule_fsch: None,
                }),
                solar_control_schedule_fsch: Some(SolarControlScheduleFsch {
                    val_dcsg: Some(SolarCsg {
                        crv_pts: vec![SolarPoint {
                            frequency_set_point_enabled: None,
                            mode: None,
                            pct_hz_droop: None,
                            pct_v_droop: None,
                            ramp_rates: None,
                            reactive_pwr_set_point_enabled: None,
                            real_pwr_set_point_enabled: None,
                            reset: None,
                            state: Some(state),
                            start_time: Some(ControlTimestamp {
                                nanoseconds: when
                                    .duration_since(SystemTime::UNIX_EPOCH)
                                    .unwrap()
                                    .subsec_nanos(),
                                seconds: when
                                    .duration_since(SystemTime::UNIX_EPOCH)
                                    .unwrap()
                                    .as_secs(),
                            }),
                            voltage_set_point_enabled: None,
                            ..Default::default()
                        }],
                    }),
                }),
            }),
        }
    }

    fn build_modblk_profile(
        m_rid: &str,
        _start_time: SystemTime,
        modblk: bool,
    ) -> SolarControlProfile;

    fn solar_reset_msg(m_rid: &str) -> SolarControlProfile;

    fn schedule_solar_control(
        m_rid: &str,
        schedule_parameter_type: ScheduleParameterKind,
        value: f64,
        schedule_time: SystemTime,
    ) -> SolarControlProfile;
}

impl ControlProfileExt for SolarControlProfile {}

impl SolarControlExt for SolarControlProfile {
    fn build_control_profile(
        m_rid: &str,
        //        sch_param_type: i32,
        sch_param_value: f64,
        start_time: SystemTime,
        state: i32,
    ) -> SolarControlProfile {
        SolarControlProfile {
            control_message_info: Some(SolarControlProfile::build_control_message_info()),
            solar_control: Some(Self::build_solar_control(
                sch_param_value,
                OptionalStateKind { value: state },
                start_time,
            )),
            solar_inverter: Some(SolarInverter {
                conducting_equipment: Some(ConductingEquipment {
                    named_object: None,
                    m_rid: m_rid.to_string(),
                }),
            }),
        }
    }

    fn build_modblk_profile(
        m_rid: &str,
        _start_time: SystemTime,
        modblk: bool,
    ) -> SolarControlProfile {
        SolarControlProfile {
            control_message_info: Some(Self::build_control_message_info()),
            solar_inverter: Some(SolarInverter {
                conducting_equipment: Some(ConductingEquipment {
                    m_rid: m_rid.to_string(),
                    named_object: Some(NamedObject {
                        description: None,
                        name: None,
                    }),
                }),
            }),
            solar_control: Some(SolarControl {
                control_value: Some(ControlValue {
                    identified_object: None,
                    mod_blk: Some(modblk),
                    reset: None,
                }),
                check: None,
                solar_control_fscc: None,
            }),
        }
    }

    fn solar_reset_msg(m_rid: &str) -> SolarControlProfile {
        let msg_info = SolarControlProfile::build_control_message_info();

        SolarControlProfile {
            control_message_info: Some(msg_info),
            solar_inverter: Some(SolarInverter {
                conducting_equipment: Some(ConductingEquipment {
                    named_object: None,
                    m_rid: m_rid.to_string(),
                }),
            }),
            solar_control: Some(SolarControl {
                check: None,
                solar_control_fscc: None,
                control_value: Some(ControlValue {
                    identified_object: None,
                    mod_blk: None,
                    reset: Some(true),
                }),
            }),
        }
    }

    fn schedule_solar_control(
        m_rid: &str,
        schedule_parameter_type: ScheduleParameterKind,
        value: f64,
        schedule_time: SystemTime,
    ) -> SolarControlProfile {
        let msg_info = SolarControlProfile::build_control_message_info();
        SolarControlProfile {
            control_message_info: Some(msg_info),
            solar_inverter: Some(SolarInverter {
                conducting_equipment: Some(ConductingEquipment {
                    m_rid: m_rid.to_string(),
                    named_object: None,
                }),
            }),
            solar_control: Some(SolarControl {
                check: None,
                control_value: None,
                solar_control_fscc: Some(SolarControlFscc {
                    solar_control_schedule_fsch: None,
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
