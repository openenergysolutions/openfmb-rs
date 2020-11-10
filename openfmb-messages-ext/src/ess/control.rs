use openfmb_messages::commonmodule::ControlMessageInfo;
use openfmb_messages::{
    commonmodule::{
        CheckConditions, ConductingEquipment, ControlDpc, ControlFscc, ControlScheduleFsch,
        ControlTimestamp, EngGridConnectModeKind, EngScheduleParameter, Ess, MessageInfo,
        NamedObject, OptionalStateKind, ScheduleCsg, SchedulePoint,
    },
    essmodule::{
        EssControl, EssControlFscc, EssControlProfile, EssControlScheduleFsch, EssFunction,
        EssPoint, Esscsg, SocLimit, SocManagement,
    },
};
use snafu::{OptionExt, ResultExt};
use std::{str::FromStr, time::SystemTime};
use uuid::Uuid;

use crate::{error::*, ControlProfileExt, OpenFMBExt};

impl OpenFMBExt for EssControlProfile {
    fn device_state(&self) -> OpenFMBResult<String> {
        let eng_grid_connected_modekind = self
            .ess_control
            .clone()
            .unwrap()
            .ess_control_fscc
            .unwrap()
            .ess_control_schedule_fsch
            .unwrap()
            .val_dcsg
            .unwrap()
            .crv_pts
            .first()
            .unwrap()
            .mode
            .clone()
            .unwrap();
        Ok(format!("param: {}", &eng_grid_connected_modekind.set_val))
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
                .ess
                .clone()
                .context(NoConductingEquipment)?
                .conducting_equipment
                .context(NoConductingEquipment)?
                .m_rid,
        )
        .context(UuidError)?)
    }

    fn device_name(&self) -> OpenFMBResult<String> {
        Ok(self
            .ess
            .clone()
            .context(NoConductingEquipment)?
            .conducting_equipment
            .context(NoConductingEquipment)?
            .named_object
            .context(NoNamedObject)?
            .name
            .context(NoName)?)
    }
}
use openfmb_messages::commonmodule::GridConnectModeKind;
use EssControlProfile as Ctrl;
pub trait EssControlExt: ControlProfileExt {
    fn ess_synchro_msg(m_rid: &str, synchro_check: bool) -> EssControlProfile {
        Self::build_synchro_profile(m_rid, SystemTime::now(), synchro_check)
    }

    fn build_synchro_profile(
        m_rid: &str,
        _start_time: SystemTime,
        synchro_check: bool,
    ) -> EssControlProfile {
        let msg_info: ControlMessageInfo = EssControlProfile::build_control_message_info();
        EssControlProfile {
            ess: Some(Ess {
                conducting_equipment: Some(ConductingEquipment {
                    m_rid: m_rid.to_string(),
                    named_object: Some(NamedObject {
                        description: None,
                        name: None,
                    }),
                }),
            }),
            ess_control: Some(EssControl {
                control_value: None,
                ess_control_fscc: None,
                check: Some(CheckConditions {
                    interlock_check: None,
                    synchro_check: Some(synchro_check),
                }),
            }),

            control_message_info: Some(msg_info),
            //                        protected_switch: Some(ProtectedSwitch {
            //                            conducting_equipment: Some(ConductingEquipment {
            //                                named_object: None,
            //                                m_rid: m_rid.to_string(),
            //                            }),
            //                        }),
            //                        switch_control: Some(SwitchControl {
            //                            check: Some(CheckConditions{
            //                            interlock_check: None,synchro_check: Some(synchro_check)}),
            //                            control_value: None,
            //                            switch_control_fscc:  None
            //                        }),
           // ied: None,
        }
    }

    fn start_now_gridconnected_msg(m_rid: &str, charge_rate_kw: f64) -> EssControlProfile {
        Ctrl::build_charge_control_profile(
            m_rid,
            charge_rate_kw,
            SystemTime::now(),
            GridConnectModeKind::VsiPq,
            1,
        )
    }

    fn discharge_now_msg(m_rid: &str, charge_rate_kw: f64) -> EssControlProfile {
        Ctrl::build_discharge_control_profile(
            m_rid,
            charge_rate_kw,
            SystemTime::now(),
            GridConnectModeKind::VsiPq,
            1,
        )
    }

    fn soc_limits_msg(m_rid: &str) -> EssControlProfile {
        Ctrl::build_soclimits_control_profile(m_rid, 92.0, 10.0, SystemTime::now())
    }

    fn soc_manage_msg(m_rid: &str) -> EssControlProfile {
        Ctrl::build_socmanage_control_profile(m_rid, SystemTime::now(), 92.0, 10.0)
    }

    fn vsi_iso_msg(m_rid: &str) -> EssControlProfile {
        Ctrl::build_vsi_iso_control_profile(m_rid, SystemTime::now(), 92.0, 10.0)
    }

    fn vsi_pq_msg(m_rid: &str) -> EssControlProfile {
        Ctrl::build_vsi_pq_control_profile(m_rid, SystemTime::now())
    }

    fn stop_now_msg(m_rid: &str) -> EssControlProfile {
        Ctrl::build_stop_control_profile(m_rid, SystemTime::now())
    }

    fn build_charge_control_profile(
        m_rid: &str,
        charge_rate_kw: f64,
        start_time: SystemTime,
        mod_val: GridConnectModeKind,
        state: i32,
    ) -> EssControlProfile {
        EssControlProfile {
            control_message_info: Some(Self::build_control_message_info()),
            ess: Some(Ess {
                conducting_equipment: Some(ConductingEquipment {
                    m_rid: m_rid.to_string(),
                    named_object: Some(NamedObject {
                        description: None,
                        name: None,
                    }),
                }),
            }),
            ess_control: Some(Self::build_ess_charge_control(
                charge_rate_kw,
                state,
                mod_val,
                start_time,
            )),
            //ied: None,
        }
    }
    fn build_stop_control_profile(m_rid: &str, start_time: SystemTime) -> EssControlProfile {
        EssControlProfile {
            control_message_info: Some(Self::build_control_message_info()),
            ess: Some(Ess {
                conducting_equipment: Some(ConductingEquipment {
                    m_rid: m_rid.to_string(),
                    named_object: Some(NamedObject {
                        description: None,
                        name: None,
                    }),
                }),
            }),
            ess_control: Some(EssControl {
                check: None,
                control_value: None,
                ess_control_fscc: Some(EssControlFscc {
                    control_fscc: None,
                    ess_control_schedule_fsch: Some(EssControlScheduleFsch {
                        val_dcsg: Some(Esscsg {
                            crv_pts: vec![EssPoint {
                                black_start_enabled: None,
                                frequency_set_point_enabled: None,
                                function: None,
                                mode: None,
                                pct_hz_droop: None,
                                pct_v_droop: None,
                                ramp_rates: None,
                                reactive_pwr_set_point_enabled: None,
                                real_pwr_set_point_enabled: None,
                                reset: None,
                                state: Some(OptionalStateKind { value: 0 }),
                                sync_back_to_grid: None,
                                trans_to_islnd_on_grid_loss_enabled: None,
                                voltage_set_point_enabled: None,
                                start_time: Some(ControlTimestamp {
                                    nanoseconds: start_time.duration_since(SystemTime::UNIX_EPOCH).unwrap().subsec_nanos(),
                                    seconds: start_time
                                        .duration_since(SystemTime::UNIX_EPOCH)
                                        .unwrap()
                                        .as_secs(),
                                }),
                            }],
                        }),
                    }),
                }),
            }),
        }
    }

    fn build_soclimits_control_profile(
        m_rid: &str,
        high: f32,
        low: f32,
        start_time: SystemTime,
    ) -> EssControlProfile {
        EssControlProfile {
            control_message_info: Some(Self::build_control_message_info()),
            ess: Some(Ess {
                conducting_equipment: Some(ConductingEquipment {
                    m_rid: m_rid.to_string(),
                    named_object: Some(NamedObject {
                        description: None,
                        name: None,
                    }),
                }),
            }),
            ess_control: Some(Self::build_ess_soclimit_control(high, low, start_time)),
        }
    }

    fn build_discharge_control_profile(
        m_rid: &str,
        charge_rate_kw: f64,
        start_time: SystemTime,
        mod_val: GridConnectModeKind,
        state: i32,
    ) -> EssControlProfile {
        EssControlProfile {
            control_message_info: Some(Self::build_control_message_info()),
            ess: Some(Ess {
                conducting_equipment: Some(ConductingEquipment {
                    m_rid: m_rid.to_string(),
                    named_object: Some(NamedObject {
                        description: None,
                        name: None,
                    }),
                }),
            }),
            ess_control: Some(Self::build_ess_discharge_control(
                charge_rate_kw,
                state,
                mod_val,
                start_time,
            )),
        }
    }

    fn build_socmanage_control_profile(
        m_rid: &str,
        start_time: SystemTime,
        high_limit: f32,
        low_limit: f32,
    ) -> EssControlProfile {
        EssControlProfile {
            control_message_info: Some(Self::build_control_message_info()),
            ess: Some(Ess {
                conducting_equipment: Some(ConductingEquipment {
                    m_rid: m_rid.to_string(),
                    named_object: Some(NamedObject {
                        description: None,
                        name: None,
                    }),
                }),
            }),
            ess_control: Some(Self::build_ess_socmanage_control(
                high_limit, low_limit, start_time,
            )),
        }
    }

    fn build_vsi_iso_control_profile(
        m_rid: &str,
        start_time: SystemTime,
        _high_limit: f32,
        _low_limit: f32,
    ) -> EssControlProfile {
        EssControlProfile {
            control_message_info: Some(Self::build_control_message_info()),
            ess: Some(Ess {
                conducting_equipment: Some(ConductingEquipment {
                    m_rid: m_rid.to_string(),
                    named_object: Some(NamedObject {
                        description: None,
                        name: None,
                    }),
                }),
            }),
            ess_control: Some(Self::build_ess_vsi_iso_control(start_time)),
        }
    }

    fn build_vsi_pq_control_profile(m_rid: &str, start_time: SystemTime) -> EssControlProfile {
        EssControlProfile {
            control_message_info: Some(Self::build_control_message_info()),
            ess: Some(Ess {
                conducting_equipment: Some(ConductingEquipment {
                    m_rid: m_rid.to_string(),
                    named_object: Some(NamedObject {
                        description: None,
                        name: None,
                    }),
                }),
            }),
            ess_control: Some(Self::build_ess_vsi_pq_control(start_time)),
        }
    }

    fn build_ess_charge_control(
        charge_rate_kw: f64,
        state: i32,
        mod_val: GridConnectModeKind,
        when: SystemTime,
    ) -> EssControl {
        EssControl {
            control_value: None,
            check: Some(CheckConditions {
                interlock_check: None,
                synchro_check: None,
            }),
            ess_control_fscc: Some(EssControlFscc {
                control_fscc: Some(ControlFscc {
                    logical_node_for_control: None,
                    control_schedule_fsch: Some(ControlScheduleFsch {
                        val_acsg: Some(ScheduleCsg {
                            sch_pts: vec![
                                SchedulePoint {
                                    schedule_parameter: vec![EngScheduleParameter {
                                        schedule_parameter_type: 39,
                                        value: charge_rate_kw,
                                    }],
                                    start_time: Some(ControlTimestamp {
                                        nanoseconds: when.duration_since(SystemTime::UNIX_EPOCH).unwrap().subsec_nanos(),
                                        seconds: when
                                            .duration_since(SystemTime::UNIX_EPOCH)
                                            .unwrap()
                                            .as_secs(),
                                    }),
                                },
                                SchedulePoint {
                                    schedule_parameter: vec![EngScheduleParameter {
                                        schedule_parameter_type: 34,
                                        value: 0.0,
                                    }],
                                    start_time: Some(ControlTimestamp {
                                        nanoseconds: when.duration_since(SystemTime::UNIX_EPOCH).unwrap().subsec_nanos(),
                                        seconds: when
                                            .duration_since(SystemTime::UNIX_EPOCH)
                                            .unwrap()
                                            .as_secs(),
                                    }),
                                },
                            ],
                        }),
                    }),
                    island_control_schedule_fsch: None,
                }),
                ess_control_schedule_fsch: Some(EssControlScheduleFsch {
                    val_dcsg: Some(Esscsg {
                        crv_pts: vec![EssPoint {
                            black_start_enabled: None,
                            frequency_set_point_enabled: None,
                            function: None,
                            mode: Some(EngGridConnectModeKind {
                                set_val: mod_val as i32,
                                set_val_extension: None,
                            }),
                            pct_hz_droop: None,
                            pct_v_droop: None,
                            ramp_rates: None,
                            reactive_pwr_set_point_enabled: { Some(ControlDpc { ctl_val: true }) },
                            real_pwr_set_point_enabled: { Some(ControlDpc { ctl_val: true }) },
                            reset: None,
                            state: Some(OptionalStateKind { value: state }),
                            sync_back_to_grid: None,
                            trans_to_islnd_on_grid_loss_enabled: None,
                            voltage_set_point_enabled: None,
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
            }),
        }
    }

    fn build_ess_soclimit_control(high_limit: f32, low_limit: f32, when: SystemTime) -> EssControl {
        EssControl {
            control_value: None,
            check: Some(CheckConditions {
                interlock_check: None,
                synchro_check: Some(false),
            }),
            ess_control_fscc: Some(EssControlFscc {
                control_fscc: Some(ControlFscc {
                    logical_node_for_control: None,
                    control_schedule_fsch: Some(ControlScheduleFsch { val_acsg: None }),
                    island_control_schedule_fsch: None,
                }),
                ess_control_schedule_fsch: Some(EssControlScheduleFsch {
                    val_dcsg: Some(Esscsg {
                        crv_pts: vec![EssPoint {
                            black_start_enabled: None,
                            frequency_set_point_enabled: None,
                            function: Some(EssFunction {
                                capacity_firming: None,
                                frequency_regulation: None,
                                peak_shaving: None,
                                soc_limit: Some(SocLimit {
                                    soc_high_limit: Some(high_limit),
                                    soc_low_limit: Some(low_limit),
                                    soc_high_limit_hysteresis: None,
                                    soc_low_limit_hysteresis: None,
                                    soc_limit_ctl: Some(true),
                                }),
                                soc_management: None,
                                voltage_droop: None,
                                voltage_pi: None,
                            }),
                            mode: None,
                            pct_hz_droop: None,
                            pct_v_droop: None,
                            ramp_rates: None,
                            reactive_pwr_set_point_enabled: { Some(ControlDpc { ctl_val: false }) },
                            real_pwr_set_point_enabled: { Some(ControlDpc { ctl_val: false }) },
                            reset: None,
                            state: None,
                            sync_back_to_grid: None,
                            trans_to_islnd_on_grid_loss_enabled: None,
                            voltage_set_point_enabled: None,
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
            }),
        }
    }

    fn build_ess_socmanage_control(
        high_limit: f32,
        low_limit: f32,
        when: SystemTime,
    ) -> EssControl {
        EssControl {
            control_value: None,
            check: Some(CheckConditions {
                interlock_check: None,
                synchro_check: None,
            }),
            ess_control_fscc: Some(EssControlFscc {
                control_fscc: Some(ControlFscc {
                    logical_node_for_control: None,
                    control_schedule_fsch: Some(ControlScheduleFsch { val_acsg: None }),
                    island_control_schedule_fsch: None,
                }),
                ess_control_schedule_fsch: Some(EssControlScheduleFsch {
                    val_dcsg: Some(Esscsg {
                        crv_pts: vec![EssPoint {
                            black_start_enabled: None,
                            frequency_set_point_enabled: None,
                            function: Some(EssFunction {
                                capacity_firming: None,
                                frequency_regulation: None,
                                peak_shaving: None,
                                soc_limit: None,
                                soc_management: Some(SocManagement {
                                    soc_dead_band_minus: Some(5.0),
                                    soc_dead_band_plus: Some(5.0),
                                    soc_management_ctl: Some(true),
                                    soc_power_set_point: Some(low_limit),
                                    soc_set_point: Some(high_limit),
                                }),
                                voltage_droop: None,
                                voltage_pi: None,
                            }),
                            mode: None,
                            pct_hz_droop: None,
                            pct_v_droop: None,
                            ramp_rates: None,
                            reactive_pwr_set_point_enabled: { Some(ControlDpc { ctl_val: false }) },
                            real_pwr_set_point_enabled: { Some(ControlDpc { ctl_val: false }) },
                            reset: None,
                            state: None,
                            sync_back_to_grid: None,
                            trans_to_islnd_on_grid_loss_enabled: None,
                            voltage_set_point_enabled: None,
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
            }),
        }
    }

    fn build_ess_vsi_iso_control(
        //        high_limit: f32,
        //        low_limit: f32,
        when: SystemTime,
    ) -> EssControl {
        EssControl {
            control_value: None,
            check: None,
            ess_control_fscc: Some(EssControlFscc {
                control_fscc: Some(ControlFscc {
                    logical_node_for_control: None,
                    control_schedule_fsch: Some(ControlScheduleFsch { val_acsg: None }),
                    island_control_schedule_fsch: None,
                }),
                ess_control_schedule_fsch: Some(EssControlScheduleFsch {
                    val_dcsg: Some(Esscsg {
                        crv_pts: vec![EssPoint {
                            black_start_enabled: None,
                            frequency_set_point_enabled: None,
                            function: None,
                            mode: Some(EngGridConnectModeKind {
                                set_val: GridConnectModeKind::VsiIso as i32,
                                set_val_extension: None,
                            }),
                            pct_hz_droop: None,
                            pct_v_droop: None,
                            ramp_rates: None,
                            reactive_pwr_set_point_enabled: { None },
                            real_pwr_set_point_enabled: { None },
                            reset: None,
                            state: None,
                            sync_back_to_grid: None,
                            trans_to_islnd_on_grid_loss_enabled: None,
                            voltage_set_point_enabled: None,
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
            }),
        }
    }

    fn build_ess_vsi_pq_control(
        //        high_limit: f32,
        //        low_limit: f32,
        when: SystemTime,
    ) -> EssControl {
        EssControl {
            control_value: None,
            check: Some(CheckConditions {
                interlock_check: None,
                synchro_check: None,
            }),
            ess_control_fscc: Some(EssControlFscc {
                control_fscc: Some(ControlFscc {
                    logical_node_for_control: None,
                    control_schedule_fsch: Some(ControlScheduleFsch { val_acsg: None }),
                    island_control_schedule_fsch: None,
                }),
                ess_control_schedule_fsch: Some(EssControlScheduleFsch {
                    val_dcsg: Some(Esscsg {
                        crv_pts: vec![EssPoint {
                            black_start_enabled: None,
                            frequency_set_point_enabled: None,
                            function: None,
                            mode: Some(EngGridConnectModeKind {
                                set_val: GridConnectModeKind::VsiPq as i32,
                                set_val_extension: None,
                            }),
                            pct_hz_droop: None,
                            pct_v_droop: None,
                            ramp_rates: None,
                            reactive_pwr_set_point_enabled: { None },
                            real_pwr_set_point_enabled: { None },
                            reset: None,
                            state: None,
                            sync_back_to_grid: None,
                            trans_to_islnd_on_grid_loss_enabled: None,
                            voltage_set_point_enabled: None,
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
            }),
        }
    }

    fn build_ess_discharge_control(
        charge_rate_kw: f64,
        state: i32,
        mod_val: GridConnectModeKind,
        when: SystemTime,
    ) -> EssControl {
        EssControl {
            control_value: None,
            check: Some(CheckConditions {
                interlock_check: None,
                synchro_check: None,
            }),
            ess_control_fscc: Some(EssControlFscc {
                control_fscc: Some(ControlFscc {
                    logical_node_for_control: None,
                    control_schedule_fsch: Some(ControlScheduleFsch {
                        val_acsg: Some(ScheduleCsg {
                            sch_pts: vec![
                                SchedulePoint {
                                    schedule_parameter: vec![EngScheduleParameter {
                                        schedule_parameter_type: 39,
                                        value: charge_rate_kw,
                                    }],
                                    start_time: Some(ControlTimestamp {
                                        nanoseconds: when.duration_since(SystemTime::UNIX_EPOCH).unwrap().subsec_nanos(),
                                        seconds: when
                                            .duration_since(SystemTime::UNIX_EPOCH)
                                            .unwrap()
                                            .as_secs(),
                                    }),
                                },
                                SchedulePoint {
                                    schedule_parameter: vec![EngScheduleParameter {
                                        schedule_parameter_type: 34,
                                        value: 0.0,
                                    }],
                                    start_time: Some(ControlTimestamp {
                                        nanoseconds: when.duration_since(SystemTime::UNIX_EPOCH).unwrap().subsec_nanos(),
                                        seconds: when
                                            .duration_since(SystemTime::UNIX_EPOCH)
                                            .unwrap()
                                            .as_secs(),
                                    }),
                                },
                            ],
                        }),
                    }),
                    island_control_schedule_fsch: None,
                }),
                ess_control_schedule_fsch: Some(EssControlScheduleFsch {
                    val_dcsg: Some(Esscsg {
                        crv_pts: vec![EssPoint {
                            black_start_enabled: None,
                            frequency_set_point_enabled: None,
                            function: None,
                            mode: Some(EngGridConnectModeKind {
                                set_val: mod_val as i32,
                                set_val_extension: None,
                            }),
                            pct_hz_droop: None,
                            pct_v_droop: None,
                            ramp_rates: None,
                            reactive_pwr_set_point_enabled: { Some(ControlDpc { ctl_val: true }) },
                            real_pwr_set_point_enabled: { Some(ControlDpc { ctl_val: true }) },
                            reset: None,
                            state: Some(OptionalStateKind { value: state }),
                            sync_back_to_grid: None,
                            trans_to_islnd_on_grid_loss_enabled: None,
                            voltage_set_point_enabled: None,
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
            }),
        }
    }
}

impl ControlProfileExt for EssControlProfile {}

impl EssControlExt for EssControlProfile {}
