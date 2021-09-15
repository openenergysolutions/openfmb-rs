// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

use crate::{error::*, ControlProfileExt, OpenFMBExt, Phase};

use openfmb_messages::{
    commonmodule::{
        CheckConditions, ConductingEquipment, ControlDpc, ControlMessageInfo, ControlValue,
        MessageInfo, PhaseDpc,
    },
    switchmodule::{
        ProtectedSwitch, SwitchDiscreteControl, SwitchDiscreteControlProfile,
        SwitchDiscreteControlXswi,
    },
};
use snafu::{OptionExt, ResultExt};
use std::{str::FromStr, time::SystemTime};
use uuid::Uuid;

impl OpenFMBExt for SwitchDiscreteControlProfile {
    fn device_state(&self) -> OpenFMBResult<String> {
        let switch_control = if self
            .switch_discrete_control
            .clone()
            .unwrap()
            .switch_discrete_control_xswi
            .unwrap()
            .pos
            .unwrap()
            .phs3
            .unwrap()
            .ctl_val
        {
            "Request Closed"
        } else {
            "Request Open"
        };
        Ok(switch_control.into())
    }

    fn message_info(&self) -> OpenFMBResult<&MessageInfo> {
        unimplemented!()
    }

    fn message_type(&self) -> OpenFMBResult<String> {
        Ok("SolarStatusProfile".to_string())
    }

    fn device_mrid(&self) -> OpenFMBResult<Uuid> {
        Ok(Uuid::from_str(
            &self
                .protected_switch
                .as_ref()
                .context(NoProtectedSwitch)?
                .conducting_equipment
                .as_ref()
                .context(NoConductingEquipment)?
                .m_rid,
        )
        .context(UuidError)?)
    }

    fn device_name(&self) -> OpenFMBResult<String> {
        Ok(self
            .protected_switch
            .as_ref()
            .context(NoProtectedSwitch)?
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

pub trait SwitchControlExt: ControlProfileExt {
    fn switch_open_msg(m_rid: &str) -> SwitchDiscreteControlProfile {
        Self::build_control_profile(m_rid, SystemTime::now(), false)
    }

    fn switch_close_msg(m_rid: &str) -> SwitchDiscreteControlProfile {
        Self::build_control_profile(m_rid, SystemTime::now(), true)
    }

    fn pos_control(m_rid: &str, closed: bool, phs: Phase) -> SwitchDiscreteControlProfile {
        Self::build_control_profile_single_phase(m_rid, SystemTime::now(), closed, phs)
    }

    fn switch_synchro_msg(m_rid: &str, synchro_check: bool) -> SwitchDiscreteControlProfile {
        Self::build_synchro_profile(m_rid, SystemTime::now(), synchro_check)
    }

    fn switch_modblk_msg(m_rid: &str, modblk: bool) -> SwitchDiscreteControlProfile {
        Self::build_modblk_profile(m_rid, SystemTime::now(), modblk)
    }

    fn build_control_profile(
        m_rid: &str,
        start_time: SystemTime,
        pos: bool,
    ) -> SwitchDiscreteControlProfile;
    fn build_control_profile_single_phase(
        m_rid: &str,
        start_time: SystemTime,        
        pos: bool,
        phase: Phase
    ) -> SwitchDiscreteControlProfile;
    fn build_synchro_profile(
        m_rid: &str,
        start_time: SystemTime,
        synchro_check: bool,
    ) -> SwitchDiscreteControlProfile;
    fn build_modblk_profile(
        m_rid: &str,
        start_time: SystemTime,
        modblk: bool,
    ) -> SwitchDiscreteControlProfile;
}

impl SwitchControlExt for SwitchDiscreteControlProfile {
    fn build_synchro_profile(
        m_rid: &str,
        _start_time: SystemTime,
        synchro_check: bool,
    ) -> SwitchDiscreteControlProfile {
        let msg_info: ControlMessageInfo =
            SwitchDiscreteControlProfile::build_control_message_info();
        SwitchDiscreteControlProfile {
            control_message_info: Some(msg_info),
            protected_switch: Some(ProtectedSwitch {
                conducting_equipment: Some(ConductingEquipment {
                    named_object: None,
                    m_rid: m_rid.to_string(),
                }),
            }),
            switch_discrete_control: Some(SwitchDiscreteControl {
                control_value: None,
                check: Some(CheckConditions {
                    interlock_check: None,
                    synchro_check: Some(synchro_check),
                }),
                switch_discrete_control_xswi: None,
            }),
        }
    }

    fn build_modblk_profile(
        m_rid: &str,
        _start_time: SystemTime,
        modblk: bool,
    ) -> SwitchDiscreteControlProfile {
        let msg_info: ControlMessageInfo =
            SwitchDiscreteControlProfile::build_control_message_info();
        SwitchDiscreteControlProfile {
            control_message_info: Some(msg_info),
            protected_switch: Some(ProtectedSwitch {
                conducting_equipment: Some(ConductingEquipment {
                    named_object: None,
                    m_rid: m_rid.to_string(),
                }),
            }),
            switch_discrete_control: Some(SwitchDiscreteControl {
                control_value: Some(ControlValue {
                    identified_object: None,
                    mod_blk: Some(modblk),
                    reset: None,
                }),
                check: None,
                switch_discrete_control_xswi: None,
            }),
        }
    }

    fn build_control_profile(
        m_rid: &str,
        _start_time: SystemTime,
        pos: bool,
    ) -> SwitchDiscreteControlProfile {
        let msg_info: ControlMessageInfo =
            SwitchDiscreteControlProfile::build_control_message_info();
        SwitchDiscreteControlProfile {
            control_message_info: Some(msg_info),
            protected_switch: Some(ProtectedSwitch {
                conducting_equipment: Some(ConductingEquipment {
                    named_object: None,
                    m_rid: m_rid.to_string(),
                }),
            }),
            switch_discrete_control: Some(SwitchDiscreteControl {
                check: None,
                control_value: None,
                switch_discrete_control_xswi: Some(SwitchDiscreteControlXswi {
                    reset_protection_pickup: None,
                    logical_node_for_control: None,
                    pos: Some(PhaseDpc {
                        phs3: Some(ControlDpc { ctl_val: pos }),
                        phs_a: None,
                        phs_b: None,
                        phs_c: None,
                    }),
                }),
            }),
        }
    }

    fn build_control_profile_single_phase(
        m_rid: &str,
        _start_time: SystemTime,
        pos: bool,
        phase: Phase
    ) -> SwitchDiscreteControlProfile {
        let msg_info: ControlMessageInfo =
            SwitchDiscreteControlProfile::build_control_message_info();
        let control_dpc = match phase {
            Phase::Phs3 => {
                PhaseDpc {
                    phs3: Some(ControlDpc { ctl_val: pos }),
                    ..Default::default()
                }
            }
            Phase::PhsA => {
                PhaseDpc {
                    phs_a: Some(ControlDpc { ctl_val: pos }),
                    ..Default::default()
                }
            }
            Phase::PhsB => {
                PhaseDpc {
                    phs_b: Some(ControlDpc { ctl_val: pos }),
                    ..Default::default()
                }
            }
            Phase::PhsC => {
                PhaseDpc {
                    phs_c: Some(ControlDpc { ctl_val: pos }),
                    ..Default::default()
                }
            }
        };

        SwitchDiscreteControlProfile {
            control_message_info: Some(msg_info),
            protected_switch: Some(ProtectedSwitch {
                conducting_equipment: Some(ConductingEquipment {
                    named_object: None,
                    m_rid: m_rid.to_string(),
                }),
            }),
            switch_discrete_control: Some(SwitchDiscreteControl {
                check: None,
                control_value: None,
                switch_discrete_control_xswi: Some(SwitchDiscreteControlXswi {
                    reset_protection_pickup: None,
                    logical_node_for_control: None,
                    pos: Some(control_dpc),
                }),
            }),
        }
    }
}

impl ControlProfileExt for SwitchDiscreteControlProfile {}
