// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

use openfmb_messages::{commonmodule::*, reclosermodule::*};
use snafu::{OptionExt, ResultExt};
use std::{str::FromStr, time::SystemTime};
use uuid::Uuid;

use crate::{error::*, ControlProfileExt, OpenFMBExt, Phase};

impl OpenFMBExt for RecloserDiscreteControlProfile {
    fn device_state(&self) -> OpenFMBResult<String> {
        let state = if self
            .recloser_discrete_control
            .as_ref()
            .context(NoBreakerDiscreteControl)?
            .recloser_discrete_control_xcbr
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
    }

    fn message_type(&self) -> OpenFMBResult<String> {
        Ok("RecloserDiscreteControlProfile".to_string())
    }

    fn device_mrid(&self) -> OpenFMBResult<Uuid> {
        Ok(Uuid::from_str(
            &self
                .recloser
                .as_ref()
                .context(NoRecloser)?
                .conducting_equipment
                .as_ref()
                .context(NoConductingEquipment)?
                .m_rid,
        )
        .context(UuidError)?)
    }

    fn device_name(&self) -> OpenFMBResult<String> {
        Ok(self
            .recloser
            .as_ref()
            .context(NoRecloser)?
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

pub trait RecloserControlExt: ControlProfileExt {
    fn recloser_open_msg(m_rid: &str) -> RecloserDiscreteControlProfile {
        Self::build_control_profile(m_rid, SystemTime::now(), false)
    }

    fn recloser_close_msg(m_rid: &str) -> RecloserDiscreteControlProfile {
        Self::build_control_profile(m_rid, SystemTime::now(), true)
    }

    fn pos_control(m_rid: &str, closed: bool, phs: Phase) -> RecloserDiscreteControlProfile {
        Self::build_control_profile_single_phase(m_rid, SystemTime::now(), closed, phs)
    }

    fn recloser_synchro_msg(m_rid: &str, synchro_check: bool) -> RecloserDiscreteControlProfile {
        Self::build_synchro_profile(m_rid, SystemTime::now(), synchro_check)
    }

    fn build_control_profile(
        m_rid: &str,
        start_time: SystemTime,
        pos: bool,
    ) -> RecloserDiscreteControlProfile;

    fn build_control_profile_single_phase(
        m_rid: &str,
        start_time: SystemTime,
        pos: bool,
        phase: Phase,
    ) -> RecloserDiscreteControlProfile;

    fn build_synchro_profile(
        m_rid: &str,
        start_time: SystemTime,
        synchro_check: bool,
    ) -> RecloserDiscreteControlProfile;

    fn recloser_reset_msg(m_rid: &str) -> RecloserDiscreteControlProfile;
}

impl RecloserControlExt for RecloserDiscreteControlProfile {
    fn build_control_profile(
        m_rid: &str,
        _start_time: SystemTime,
        pos: bool,
    ) -> RecloserDiscreteControlProfile {
        let msg_info: ControlMessageInfo =
            RecloserDiscreteControlProfile::build_control_message_info();
        RecloserDiscreteControlProfile {
            control_message_info: Some(msg_info),
            recloser: Some(Recloser {
                conducting_equipment: Some(ConductingEquipment {
                    named_object: None,
                    m_rid: m_rid.to_string(),
                }),
                normal_open: None,
            }),
            recloser_discrete_control: Some(RecloserDiscreteControl {
                control_value: None,
                check: None,
                recloser_discrete_control_xcbr: Some(RecloserDiscreteControlXcbr {
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
        }
    }

    fn build_control_profile_single_phase(
        m_rid: &str,
        _start_time: SystemTime,
        pos: bool,
        phase: Phase,
    ) -> RecloserDiscreteControlProfile {
        let msg_info: ControlMessageInfo =
            RecloserDiscreteControlProfile::build_control_message_info();

        let control_dpc = match phase {
            Phase::Phs3 => PhaseDpc {
                phs3: Some(ControlDpc { ctl_val: pos }),
                ..Default::default()
            },
            Phase::PhsA => PhaseDpc {
                phs_a: Some(ControlDpc { ctl_val: pos }),
                ..Default::default()
            },
            Phase::PhsB => PhaseDpc {
                phs_b: Some(ControlDpc { ctl_val: pos }),
                ..Default::default()
            },
            Phase::PhsC => PhaseDpc {
                phs_c: Some(ControlDpc { ctl_val: pos }),
                ..Default::default()
            },
        };

        RecloserDiscreteControlProfile {
            control_message_info: Some(msg_info),
            recloser: Some(Recloser {
                conducting_equipment: Some(ConductingEquipment {
                    named_object: None,
                    m_rid: m_rid.to_string(),
                }),
                normal_open: None,
            }),
            recloser_discrete_control: Some(RecloserDiscreteControl {
                control_value: None,
                check: None,
                recloser_discrete_control_xcbr: Some(RecloserDiscreteControlXcbr {
                    discrete_control_xcbr: Some(DiscreteControlXcbr {
                        pos: Some(control_dpc),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
            }),
        }
    }

    fn build_synchro_profile(
        m_rid: &str,
        _start_time: SystemTime,
        synchro_check: bool,
    ) -> RecloserDiscreteControlProfile {
        let msg_info: ControlMessageInfo =
            RecloserDiscreteControlProfile::build_control_message_info();
        RecloserDiscreteControlProfile {
            control_message_info: Some(msg_info),
            recloser: Some(Recloser {
                conducting_equipment: Some(ConductingEquipment {
                    named_object: None,
                    m_rid: m_rid.to_string(),
                }),
                normal_open: None,
            }),
            recloser_discrete_control: Some(RecloserDiscreteControl {
                control_value: None,
                check: Some(CheckConditions {
                    interlock_check: None,
                    synchro_check: Some(synchro_check),
                }),
                recloser_discrete_control_xcbr: None,
            }),
        }
    }

    fn recloser_reset_msg(m_rid: &str) -> RecloserDiscreteControlProfile {
        let msg_info: ControlMessageInfo =
            RecloserDiscreteControlProfile::build_control_message_info();

        RecloserDiscreteControlProfile {
            control_message_info: Some(msg_info),
            recloser: Some(Recloser {
                conducting_equipment: Some(ConductingEquipment {
                    named_object: None,
                    m_rid: m_rid.to_string(),
                }),
                normal_open: None,
            }),
            recloser_discrete_control: Some(RecloserDiscreteControl {
                check: None,
                recloser_discrete_control_xcbr: None,
                control_value: Some(ControlValue {
                    identified_object: None,
                    mod_blk: None,
                    reset: Some(true),
                }),
            }),
        }
    }
}

impl ControlProfileExt for RecloserDiscreteControlProfile {}
