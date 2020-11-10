use crate::{error::*, ControlProfileExt, OpenFMBExt};
use openfmb_messages::{
    commonmodule::{
        ConductingEquipment, ControlFscc, ControlScheduleFsch, ControlTimestamp,
        EngScheduleParameter, MessageInfo, OptionalStateKind, ScheduleCsg, SchedulePoint,
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
                .clone()
                .context(NoSolarInverter)?
                .conducting_equipment
                .context(NoConductingEquipment)?
                .m_rid,
        )
        .context(UuidError)?)
    }

    fn device_name(&self) -> OpenFMBResult<String> {
        Ok(self
            .solar_inverter
            .clone()
            .context(NoSolarInverter)?
            .conducting_equipment
            .context(NoConductingEquipment)?
            .named_object
            .context(NoNamedObject)?
            .name
            .context(NoName)?)
    }
}

pub trait SolarControlExt: ControlProfileExt {
    fn solar_on_msg(m_rid: &str, val: f64) -> SolarControlProfile {
        Self::build_control_profile(m_rid, val, SystemTime::now(), 1)
    }

    fn solar_off_msg(m_rid: &str) -> SolarControlProfile {
        Self::build_control_profile(m_rid, 0.0, SystemTime::now(), 0)
    }

    fn build_control_profile(
        m_rid: &str,
        sch_param_value: f64,
        start_time: SystemTime,
        state: i32,
    ) -> SolarControlProfile;

    fn build_solar_control(
        solar_acsg: f64,
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
                    control_schedule_fsch: Some(ControlScheduleFsch {
                        val_acsg: Some(ScheduleCsg {
                            sch_pts: vec![SchedulePoint {
                                schedule_parameter: vec![
                                    EngScheduleParameter {
                                        schedule_parameter_type: 39,
                                        value: solar_acsg,
                                    },
                                    EngScheduleParameter {
                                        schedule_parameter_type: 8,
                                        value: 1.0,
                                    },
                                ],
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
                                nanoseconds: when.duration_since(SystemTime::UNIX_EPOCH).unwrap().subsec_nanos(),
                                seconds: when
                                    .duration_since(SystemTime::UNIX_EPOCH)
                                    .unwrap()
                                    .as_secs(),
                            }),
                            voltage_set_point_enabled: None,
                        }],
                    }),
                }),
            }),
        }
    }
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
}
