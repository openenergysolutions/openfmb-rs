use crate::{error::*, ControlProfileExt, OpenFMBExt};
use openfmb_messages::{
    commonmodule::{
        ConductingEquipment, ControlFscc, ControlMessageInfo, ControlScheduleFsch,
        ControlTimestamp, EngScheduleParameter, MessageInfo, OptionalStateKind, ScheduleCsg,
        SchedulePoint, StateKind, ScheduleParameterKind,
    },
    generationmodule::{
        GeneratingUnit, GenerationControl, GenerationControlFscc, GenerationControlProfile,
        GenerationControlScheduleFsch, GenerationCsg, GenerationPoint,
    },
};
use snafu::{OptionExt, ResultExt};
use std::{str::FromStr, time::SystemTime};
use uuid::Uuid;

impl OpenFMBExt for GenerationControlProfile {
    fn device_state(&self) -> OpenFMBResult<String> {
        let ess_control = self
            .generation_control
            .clone()
            .unwrap()
            .generation_control_fscc
            .unwrap()
            .generation_control_schedule_fsch
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
        Ok(format!("Control: {}", ess_control))
    }

    fn message_info(&self) -> OpenFMBResult<&MessageInfo> {
        unimplemented!()
        //        Ok(self.solar_control.clone().context(NoStatusMessageInfo)?..unwrap())
    }

    fn message_type(&self) -> OpenFMBResult<String> {
        Ok("GenerationControlProfile".to_string())
    }

    fn device_mrid(&self) -> OpenFMBResult<Uuid> {
        Ok(Uuid::from_str(
            &self
                .generating_unit
                .clone()
                .context(NoIdentifiedObject)?
                .conducting_equipment
                .context(NoConductingEquipment)?
                .m_rid,
        )
        .context(UuidError)?)
    }

    fn device_name(&self) -> OpenFMBResult<String> {
        Ok("".to_string())
    }
}

pub trait GenerationControlExt: ControlProfileExt {
    fn generator_on_msg(m_rid: &str, val: f64) -> GenerationControlProfile {
        Self::build_control_profile(m_rid, ScheduleParameterKind::WNetMag as i32, val, SystemTime::now(), StateKind::On as i32)
    }

    fn generator_off_msg(m_rid: &str) -> GenerationControlProfile {
        Self::build_control_profile(m_rid, ScheduleParameterKind::WNetMag as i32, 0.0, SystemTime::now(), StateKind::Off as i32)
    }

    fn build_control_profile(
        m_rid: &str,
        sch_param_type: i32,
        sch_param_value: f64,
        start_time: SystemTime,
        state: i32,
    ) -> GenerationControlProfile;
}

impl ControlProfileExt for GenerationControlProfile {}

impl GenerationControlExt for GenerationControlProfile {
    fn build_control_profile(
        m_rid: &str,
        sch_param_type: i32,
        sch_param_value: f64,
        start_time: SystemTime,
        state: i32,
    ) -> GenerationControlProfile {
        let msg_info: ControlMessageInfo = GenerationControlProfile::build_control_message_info();
        let when = SystemTime::now();
        GenerationControlProfile {
            control_message_info: Some(msg_info),
            generating_unit: Some(GeneratingUnit {
                conducting_equipment: Some(ConductingEquipment {
                    named_object: None,
                    m_rid: m_rid.to_string(),
                }),
                max_operating_p: None,
            }),
            generation_control: Some(GenerationControl {
                control_value: None,
                check: None,
                generation_control_fscc: Some(GenerationControlFscc {
                    //TODO remove this when not necessary
                    control_fscc: Some(ControlFscc {
                        logical_node_for_control: None,
                        control_schedule_fsch: Some(ControlScheduleFsch {
                            val_acsg: Some(ScheduleCsg {
                                sch_pts: vec![SchedulePoint {
                                    schedule_parameter: vec![EngScheduleParameter {
                                        schedule_parameter_type: sch_param_type,
                                        value: sch_param_value,
                                    }],
                                    start_time: Some(ControlTimestamp {
                                        nanoseconds: when.duration_since(SystemTime::UNIX_EPOCH).unwrap().subsec_nanos(),
                                        seconds: when
                                            .duration_since(SystemTime::UNIX_EPOCH)
                                            .unwrap()
                                            .as_secs(),
                                    }),
                                }],
                            }),
                        }),
                        island_control_schedule_fsch: None,
                    }),
                    generation_control_schedule_fsch: Some(GenerationControlScheduleFsch {
                        val_dcsg: Some(GenerationCsg {
                            crv_pts: vec![GenerationPoint {
                                black_start_enabled: None,
                                frequency_set_point_enabled: None,
                                pct_hz_droop: None,
                                pct_v_droop: None,
                                ramp_rates: None,
                                reactive_pwr_set_point_enabled: None,
                                real_pwr_set_point_enabled: None,
                                reset: None,
                                state: Some(OptionalStateKind { value: state }),
                                sync_back_to_grid: None,
                                start_time: Some(ControlTimestamp {
                                    nanoseconds: start_time.duration_since(SystemTime::UNIX_EPOCH).unwrap().subsec_nanos(),
                                    seconds: start_time
                                        .duration_since(SystemTime::UNIX_EPOCH)
                                        .unwrap()
                                        .as_secs(),
                                }),
                                voltage_set_point_enabled: None,
                                trans_to_islnd_on_grid_loss_enabled: None,
                            }],
                        }),
                    }),
                }),
            }),
        }
    }
}
