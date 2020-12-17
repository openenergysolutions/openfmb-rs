use openfmb_messages::{
    commonmodule::*,
    breakermodule::*,
};
use snafu::{OptionExt, ResultExt};
use std::{str::FromStr, time::SystemTime};
use uuid::Uuid;

use crate::{error::*, ControlProfileExt, OpenFMBExt};

impl OpenFMBExt for BreakerDiscreteControlProfile {
    fn device_state(&self) -> OpenFMBResult<String> {
        let state = if self
            .breaker_discrete_control
            .as_ref()
            .context(NoBreakerDiscreteControl)?
            .breaker_discrete_control_xcbr
            .as_ref()
            .context(NoBreakerDiscreteControlXcbr)?
            .discrete_control_xcbr
            .as_ref()
            .context(NoDiscreteControlXcbr)?
            .pos
            .as_ref()
            .context(NoPos)?
            .phs3
            .as_ref()
            .context(NoPhs3)?
            .ctl_val
        {
            "Request Closed"
        } else {
            "Request Open"
        };
        Ok(state.into())
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
                .breaker
                .clone()
                .context(NoProtectedSwitch)?
                .conducting_equipment
                .context(NoConductingEquipment)?
                .m_rid,
        )
        .context(UuidError)?)
    }

    fn device_name(&self) -> OpenFMBResult<String> {
        Ok("No Name Specified".to_string())
    }
}

pub trait BreakerControlExt: ControlProfileExt {
    fn breaker_open_msg(m_rid: &str) -> BreakerDiscreteControlProfile {
        Self::build_control_profile(m_rid, SystemTime::now(), false)
    }

    fn breaker_close_msg(m_rid: &str) -> BreakerDiscreteControlProfile {
        Self::build_control_profile(m_rid, SystemTime::now(), true)
    }

    fn breaker_synchro_msg(m_rid: &str, synchro_check: bool) -> BreakerDiscreteControlProfile {
        Self::build_synchro_profile(m_rid, SystemTime::now(), synchro_check)
    }

    fn build_control_profile(
        m_rid: &str,
        start_time: SystemTime,
        pos: bool,
    ) -> BreakerDiscreteControlProfile;

    fn build_synchro_profile(
        m_rid: &str,
        start_time: SystemTime,
        synchro_check: bool,
    ) -> BreakerDiscreteControlProfile;
}

impl BreakerControlExt for BreakerDiscreteControlProfile {
    fn build_control_profile(
        m_rid: &str,
        _start_time: SystemTime,
        pos: bool,
    ) -> BreakerDiscreteControlProfile {
        let msg_info: ControlMessageInfo =
            BreakerDiscreteControlProfile::build_control_message_info();
        BreakerDiscreteControlProfile {
            control_message_info: Some(msg_info),
            breaker: Some(Breaker {
                conducting_equipment: Some(ConductingEquipment {
                    named_object: None,
                    m_rid: m_rid.to_string(),
                }),
            }),
            breaker_discrete_control: Some(BreakerDiscreteControl {
                control_value: None,
                check: None,
                breaker_discrete_control_xcbr: Some(BreakerDiscreteControlXcbr {
                    discrete_control_xcbr: Some(DiscreteControlXcbr {
                        pos: Some(PhaseDpc {
                            phs3: Some(ControlDpc { ctl_val: pos }),
                            ..Default::default()
                        }),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
            }),

            //                        breaker_discrete_control: Some(BreakerDiscreteControl {
            //                            check: None,
            //                            control_value: None,
            //                            Breaker_control_fscc: Some(ControlFscc {
            //                                logical_node_for_control: None,
            //                                Breaker_control_schedule_fsch: Some(ControlScheduleFsch {
            //                                    val_dcsg: Some(BreakerCsg {
            //                                        crv_pts: vec![BreakerPoint {
            //                                            pos: Some(ControlDpc { ctl_val: pos }),
            //                                            start_time: Some(ControlTimestamp {
            //                                                fraction: 0,
            //                                                seconds: start_time
            //                                                    .duration_since(SystemTime::UNIX_EPOCH)
            //                                                    .unwrap()
            //                                                    .as_secs(),
            //                                            }),
            //                                        }],
            //                                    }),
            //                               }),
            //                           }),
            //                       }),

        }
    }

    fn build_synchro_profile(
        m_rid: &str,
        _start_time: SystemTime,
        synchro_check: bool,
    ) -> BreakerDiscreteControlProfile {
        let msg_info: ControlMessageInfo = BreakerDiscreteControlProfile::build_control_message_info();
        BreakerDiscreteControlProfile {
            control_message_info: Some(msg_info),
            breaker: Some(Breaker {
                conducting_equipment: Some(ConductingEquipment {
                    named_object: None,
                    m_rid: m_rid.to_string(),
                }),
            }),
            breaker_discrete_control: Some(BreakerDiscreteControl {
                control_value: None,
                check: Some(CheckConditions {
                    interlock_check: None,
                    synchro_check: Some(synchro_check),
                }),
                breaker_discrete_control_xcbr: None,
            })
        }
    }
}

impl ControlProfileExt for BreakerDiscreteControlProfile {}
