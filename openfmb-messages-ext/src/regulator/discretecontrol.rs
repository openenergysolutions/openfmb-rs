// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

use openfmb_messages::{commonmodule::*, regulatormodule::*};
use snafu::{OptionExt, ResultExt};
use std::str::FromStr;
use uuid::Uuid;

use crate::{error::*, ControlProfileExt, OpenFMBExt, Phase};

impl OpenFMBExt for RegulatorDiscreteControlProfile {
    fn device_state(&self) -> OpenFMBResult<String> {
        unimplemented!()
    }

    fn message_info(&self) -> OpenFMBResult<&MessageInfo> {
        unimplemented!()
    }

    fn message_type(&self) -> OpenFMBResult<String> {
        Ok("RegulatorDiscreteControlProfile".to_string())
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

pub trait RegulatorDiscreteControlExt: ControlProfileExt {
    fn regulator_tap_lower_phs3_msg(m_rid: &str, ct_val: bool) -> RegulatorDiscreteControlProfile {
        Self::build_tap_lower_phs3_profile(m_rid, ct_val)
    }
    fn regulator_tap_raise_phs3_msg(m_rid: &str, ct_val: bool) -> RegulatorDiscreteControlProfile {
        Self::build_tap_raise_phs3_profile(m_rid, ct_val)
    }

    fn regulator_tap_lower_phs_a_msg(m_rid: &str, ct_val: bool) -> RegulatorDiscreteControlProfile {
        Self::build_tap_lower_phs_a_profile(m_rid, ct_val)
    }
    fn regulator_tap_raise_phs_a_msg(m_rid: &str, ct_val: bool) -> RegulatorDiscreteControlProfile {
        Self::build_tap_raise_phs_a_profile(m_rid, ct_val)
    }

    fn regulator_tap_lower_phs_b_msg(m_rid: &str, ct_val: bool) -> RegulatorDiscreteControlProfile {
        Self::build_tap_lower_phs_b_profile(m_rid, ct_val)
    }
    fn regulator_tap_raise_phs_b_msg(m_rid: &str, ct_val: bool) -> RegulatorDiscreteControlProfile {
        Self::build_tap_raise_phs_b_profile(m_rid, ct_val)
    }

    fn regulator_tap_lower_phs_c_msg(m_rid: &str, ct_val: bool) -> RegulatorDiscreteControlProfile {
        Self::build_tap_lower_phs_c_profile(m_rid, ct_val)
    }
    fn regulator_tap_raise_phs_c_msg(m_rid: &str, ct_val: bool) -> RegulatorDiscreteControlProfile {
        Self::build_tap_raise_phs_c_profile(m_rid, ct_val)
    }

    fn build_tap_lower_phs3_profile(m_rid: &str, ct_val: bool) -> RegulatorDiscreteControlProfile;
    fn build_tap_raise_phs3_profile(m_rid: &str, ct_val: bool) -> RegulatorDiscreteControlProfile;

    fn build_tap_lower_phs_a_profile(m_rid: &str, ct_val: bool) -> RegulatorDiscreteControlProfile;
    fn build_tap_raise_phs_a_profile(m_rid: &str, ct_val: bool) -> RegulatorDiscreteControlProfile;

    fn build_tap_lower_phs_b_profile(m_rid: &str, ct_val: bool) -> RegulatorDiscreteControlProfile;
    fn build_tap_raise_phs_b_profile(m_rid: &str, ct_val: bool) -> RegulatorDiscreteControlProfile;

    fn build_tap_lower_phs_c_profile(m_rid: &str, ct_val: bool) -> RegulatorDiscreteControlProfile;
    fn build_tap_raise_phs_c_profile(m_rid: &str, ct_val: bool) -> RegulatorDiscreteControlProfile;

    // Dir Fwd

    fn dir_fwd_bnd_wid_msg(
        m_rid: &str,
        phase: Phase,
        ctl_val: f64,
    ) -> RegulatorDiscreteControlProfile;
    fn dir_fwd_ldcr_msg(m_rid: &str, phase: Phase, ctl_val: f64)
        -> RegulatorDiscreteControlProfile;
    fn dir_fwd_ldcx_msg(m_rid: &str, phase: Phase, ctl_val: f64)
        -> RegulatorDiscreteControlProfile;
    fn dir_fwd_vol_spt_msg(
        m_rid: &str,
        phase: Phase,
        ctl_val: f64,
    ) -> RegulatorDiscreteControlProfile;

    // Dir Rev

    fn dir_rev_bnd_wid_msg(
        m_rid: &str,
        phase: Phase,
        ctl_val: f64,
    ) -> RegulatorDiscreteControlProfile;
    fn dir_rev_ldcr_msg(m_rid: &str, phase: Phase, ctl_val: f64)
        -> RegulatorDiscreteControlProfile;
    fn dir_rev_ldcx_msg(m_rid: &str, phase: Phase, ctl_val: f64)
        -> RegulatorDiscreteControlProfile;
    fn dir_rev_vol_spt_msg(
        m_rid: &str,
        phase: Phase,
        ctl_val: f64,
    ) -> RegulatorDiscreteControlProfile;

    // Dir Mode
    fn dir_mode(m_rid: &str, mode: i32) -> RegulatorDiscreteControlProfile;
}

impl RegulatorDiscreteControlExt for RegulatorDiscreteControlProfile {
    fn build_tap_lower_phs3_profile(m_rid: &str, ct_val: bool) -> RegulatorDiscreteControlProfile {
        let msg_info: ControlMessageInfo =
            RegulatorDiscreteControlProfile::build_control_message_info();
        RegulatorDiscreteControlProfile {
            control_message_info: Some(msg_info),
            regulator_system: Some(RegulatorSystem {
                conducting_equipment: Some(ConductingEquipment {
                    named_object: None,
                    m_rid: m_rid.to_string(),
                }),
            }),
            regulator_discrete_control: Some(RegulatorDiscreteControl {
                control_value: None,
                check: None,
                regulator_control_atcc: Some(RegulatorControlAtcc {
                    tap_op_l: Some(PhaseSpc {
                        phs3: Some(ControlSpc { ctl_val: ct_val }),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
            }),
        }
    }

    fn build_tap_raise_phs3_profile(m_rid: &str, ct_val: bool) -> RegulatorDiscreteControlProfile {
        let msg_info: ControlMessageInfo =
            RegulatorDiscreteControlProfile::build_control_message_info();
        RegulatorDiscreteControlProfile {
            control_message_info: Some(msg_info),
            regulator_system: Some(RegulatorSystem {
                conducting_equipment: Some(ConductingEquipment {
                    named_object: None,
                    m_rid: m_rid.to_string(),
                }),
            }),
            regulator_discrete_control: Some(RegulatorDiscreteControl {
                control_value: None,
                check: None,
                regulator_control_atcc: Some(RegulatorControlAtcc {
                    tap_op_r: Some(PhaseSpc {
                        phs3: Some(ControlSpc { ctl_val: ct_val }),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
            }),
        }
    }

    fn build_tap_lower_phs_a_profile(m_rid: &str, ct_val: bool) -> RegulatorDiscreteControlProfile {
        let msg_info: ControlMessageInfo =
            RegulatorDiscreteControlProfile::build_control_message_info();
        RegulatorDiscreteControlProfile {
            control_message_info: Some(msg_info),
            regulator_system: Some(RegulatorSystem {
                conducting_equipment: Some(ConductingEquipment {
                    named_object: None,
                    m_rid: m_rid.to_string(),
                }),
            }),
            regulator_discrete_control: Some(RegulatorDiscreteControl {
                control_value: None,
                check: None,
                regulator_control_atcc: Some(RegulatorControlAtcc {
                    tap_op_l: Some(PhaseSpc {
                        phs_a: Some(ControlSpc { ctl_val: ct_val }),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
            }),
        }
    }

    fn build_tap_raise_phs_a_profile(m_rid: &str, ct_val: bool) -> RegulatorDiscreteControlProfile {
        let msg_info: ControlMessageInfo =
            RegulatorDiscreteControlProfile::build_control_message_info();
        RegulatorDiscreteControlProfile {
            control_message_info: Some(msg_info),
            regulator_system: Some(RegulatorSystem {
                conducting_equipment: Some(ConductingEquipment {
                    named_object: None,
                    m_rid: m_rid.to_string(),
                }),
            }),
            regulator_discrete_control: Some(RegulatorDiscreteControl {
                control_value: None,
                check: None,
                regulator_control_atcc: Some(RegulatorControlAtcc {
                    tap_op_r: Some(PhaseSpc {
                        phs_a: Some(ControlSpc { ctl_val: ct_val }),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
            }),
        }
    }

    fn build_tap_lower_phs_b_profile(m_rid: &str, ct_val: bool) -> RegulatorDiscreteControlProfile {
        let msg_info: ControlMessageInfo =
            RegulatorDiscreteControlProfile::build_control_message_info();
        RegulatorDiscreteControlProfile {
            control_message_info: Some(msg_info),
            regulator_system: Some(RegulatorSystem {
                conducting_equipment: Some(ConductingEquipment {
                    named_object: None,
                    m_rid: m_rid.to_string(),
                }),
            }),
            regulator_discrete_control: Some(RegulatorDiscreteControl {
                control_value: None,
                check: None,
                regulator_control_atcc: Some(RegulatorControlAtcc {
                    tap_op_l: Some(PhaseSpc {
                        phs_b: Some(ControlSpc { ctl_val: ct_val }),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
            }),
        }
    }

    fn build_tap_raise_phs_b_profile(m_rid: &str, ct_val: bool) -> RegulatorDiscreteControlProfile {
        let msg_info: ControlMessageInfo =
            RegulatorDiscreteControlProfile::build_control_message_info();
        RegulatorDiscreteControlProfile {
            control_message_info: Some(msg_info),
            regulator_system: Some(RegulatorSystem {
                conducting_equipment: Some(ConductingEquipment {
                    named_object: None,
                    m_rid: m_rid.to_string(),
                }),
            }),
            regulator_discrete_control: Some(RegulatorDiscreteControl {
                control_value: None,
                check: None,
                regulator_control_atcc: Some(RegulatorControlAtcc {
                    tap_op_r: Some(PhaseSpc {
                        phs_b: Some(ControlSpc { ctl_val: ct_val }),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
            }),
        }
    }

    fn build_tap_lower_phs_c_profile(m_rid: &str, ct_val: bool) -> RegulatorDiscreteControlProfile {
        let msg_info: ControlMessageInfo =
            RegulatorDiscreteControlProfile::build_control_message_info();
        RegulatorDiscreteControlProfile {
            control_message_info: Some(msg_info),
            regulator_system: Some(RegulatorSystem {
                conducting_equipment: Some(ConductingEquipment {
                    named_object: None,
                    m_rid: m_rid.to_string(),
                }),
            }),
            regulator_discrete_control: Some(RegulatorDiscreteControl {
                control_value: None,
                check: None,
                regulator_control_atcc: Some(RegulatorControlAtcc {
                    tap_op_l: Some(PhaseSpc {
                        phs_c: Some(ControlSpc { ctl_val: ct_val }),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
            }),
        }
    }

    fn build_tap_raise_phs_c_profile(m_rid: &str, ct_val: bool) -> RegulatorDiscreteControlProfile {
        let msg_info: ControlMessageInfo =
            RegulatorDiscreteControlProfile::build_control_message_info();
        RegulatorDiscreteControlProfile {
            control_message_info: Some(msg_info),
            regulator_system: Some(RegulatorSystem {
                conducting_equipment: Some(ConductingEquipment {
                    named_object: None,
                    m_rid: m_rid.to_string(),
                }),
            }),
            regulator_discrete_control: Some(RegulatorDiscreteControl {
                control_value: None,
                check: None,
                regulator_control_atcc: Some(RegulatorControlAtcc {
                    tap_op_r: Some(PhaseSpc {
                        phs_c: Some(ControlSpc { ctl_val: ct_val }),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
            }),
        }
    }

    fn dir_fwd_bnd_wid_msg(
        m_rid: &str,
        phase: Phase,
        ctl_val: f64,
    ) -> RegulatorDiscreteControlProfile {
        let msg_info: ControlMessageInfo =
            RegulatorDiscreteControlProfile::build_control_message_info();
        match phase {
            Phase::Phs3 => RegulatorDiscreteControlProfile {
                control_message_info: Some(msg_info),
                regulator_system: Some(RegulatorSystem {
                    conducting_equipment: Some(ConductingEquipment {
                        named_object: None,
                        m_rid: m_rid.to_string(),
                    }),
                }),
                regulator_discrete_control: Some(RegulatorDiscreteControl {
                    control_value: None,
                    check: None,
                    regulator_control_atcc: Some(RegulatorControlAtcc {
                        dir_fwd: Some(DirectionalAtcc {
                            bnd_wid: Some(PhaseApc {
                                phs3: Some(ControlApc { ctl_val: ctl_val }),
                                ..Default::default()
                            }),
                            ..Default::default()
                        }),
                        ..Default::default()
                    }),
                }),
            },
            Phase::PhsA => RegulatorDiscreteControlProfile {
                control_message_info: Some(msg_info),
                regulator_system: Some(RegulatorSystem {
                    conducting_equipment: Some(ConductingEquipment {
                        named_object: None,
                        m_rid: m_rid.to_string(),
                    }),
                }),
                regulator_discrete_control: Some(RegulatorDiscreteControl {
                    control_value: None,
                    check: None,
                    regulator_control_atcc: Some(RegulatorControlAtcc {
                        dir_fwd: Some(DirectionalAtcc {
                            bnd_wid: Some(PhaseApc {
                                phs_a: Some(ControlApc { ctl_val: ctl_val }),
                                ..Default::default()
                            }),
                            ..Default::default()
                        }),
                        ..Default::default()
                    }),
                }),
            },
            Phase::PhsB => RegulatorDiscreteControlProfile {
                control_message_info: Some(msg_info),
                regulator_system: Some(RegulatorSystem {
                    conducting_equipment: Some(ConductingEquipment {
                        named_object: None,
                        m_rid: m_rid.to_string(),
                    }),
                }),
                regulator_discrete_control: Some(RegulatorDiscreteControl {
                    control_value: None,
                    check: None,
                    regulator_control_atcc: Some(RegulatorControlAtcc {
                        dir_fwd: Some(DirectionalAtcc {
                            bnd_wid: Some(PhaseApc {
                                phs_b: Some(ControlApc { ctl_val: ctl_val }),
                                ..Default::default()
                            }),
                            ..Default::default()
                        }),
                        ..Default::default()
                    }),
                }),
            },
            Phase::PhsC => RegulatorDiscreteControlProfile {
                control_message_info: Some(msg_info),
                regulator_system: Some(RegulatorSystem {
                    conducting_equipment: Some(ConductingEquipment {
                        named_object: None,
                        m_rid: m_rid.to_string(),
                    }),
                }),
                regulator_discrete_control: Some(RegulatorDiscreteControl {
                    control_value: None,
                    check: None,
                    regulator_control_atcc: Some(RegulatorControlAtcc {
                        dir_fwd: Some(DirectionalAtcc {
                            bnd_wid: Some(PhaseApc {
                                phs_c: Some(ControlApc { ctl_val: ctl_val }),
                                ..Default::default()
                            }),
                            ..Default::default()
                        }),
                        ..Default::default()
                    }),
                }),
            },
        }
    }

    fn dir_fwd_ldcr_msg(
        m_rid: &str,
        phase: Phase,
        ctl_val: f64,
    ) -> RegulatorDiscreteControlProfile {
        let msg_info: ControlMessageInfo =
            RegulatorDiscreteControlProfile::build_control_message_info();
        match phase {
            Phase::Phs3 => RegulatorDiscreteControlProfile {
                control_message_info: Some(msg_info),
                regulator_system: Some(RegulatorSystem {
                    conducting_equipment: Some(ConductingEquipment {
                        named_object: None,
                        m_rid: m_rid.to_string(),
                    }),
                }),
                regulator_discrete_control: Some(RegulatorDiscreteControl {
                    control_value: None,
                    check: None,
                    regulator_control_atcc: Some(RegulatorControlAtcc {
                        dir_fwd: Some(DirectionalAtcc {
                            ldcr: Some(PhaseApc {
                                phs3: Some(ControlApc { ctl_val: ctl_val }),
                                ..Default::default()
                            }),
                            ..Default::default()
                        }),
                        ..Default::default()
                    }),
                }),
            },
            Phase::PhsA => RegulatorDiscreteControlProfile {
                control_message_info: Some(msg_info),
                regulator_system: Some(RegulatorSystem {
                    conducting_equipment: Some(ConductingEquipment {
                        named_object: None,
                        m_rid: m_rid.to_string(),
                    }),
                }),
                regulator_discrete_control: Some(RegulatorDiscreteControl {
                    control_value: None,
                    check: None,
                    regulator_control_atcc: Some(RegulatorControlAtcc {
                        dir_fwd: Some(DirectionalAtcc {
                            ldcr: Some(PhaseApc {
                                phs_a: Some(ControlApc { ctl_val: ctl_val }),
                                ..Default::default()
                            }),
                            ..Default::default()
                        }),
                        ..Default::default()
                    }),
                }),
            },
            Phase::PhsB => RegulatorDiscreteControlProfile {
                control_message_info: Some(msg_info),
                regulator_system: Some(RegulatorSystem {
                    conducting_equipment: Some(ConductingEquipment {
                        named_object: None,
                        m_rid: m_rid.to_string(),
                    }),
                }),
                regulator_discrete_control: Some(RegulatorDiscreteControl {
                    control_value: None,
                    check: None,
                    regulator_control_atcc: Some(RegulatorControlAtcc {
                        dir_fwd: Some(DirectionalAtcc {
                            ldcr: Some(PhaseApc {
                                phs_b: Some(ControlApc { ctl_val: ctl_val }),
                                ..Default::default()
                            }),
                            ..Default::default()
                        }),
                        ..Default::default()
                    }),
                }),
            },
            Phase::PhsC => RegulatorDiscreteControlProfile {
                control_message_info: Some(msg_info),
                regulator_system: Some(RegulatorSystem {
                    conducting_equipment: Some(ConductingEquipment {
                        named_object: None,
                        m_rid: m_rid.to_string(),
                    }),
                }),
                regulator_discrete_control: Some(RegulatorDiscreteControl {
                    control_value: None,
                    check: None,
                    regulator_control_atcc: Some(RegulatorControlAtcc {
                        dir_fwd: Some(DirectionalAtcc {
                            ldcr: Some(PhaseApc {
                                phs_c: Some(ControlApc { ctl_val: ctl_val }),
                                ..Default::default()
                            }),
                            ..Default::default()
                        }),
                        ..Default::default()
                    }),
                }),
            },
        }
    }

    fn dir_fwd_ldcx_msg(
        m_rid: &str,
        phase: Phase,
        ctl_val: f64,
    ) -> RegulatorDiscreteControlProfile {
        let msg_info: ControlMessageInfo =
            RegulatorDiscreteControlProfile::build_control_message_info();
        match phase {
            Phase::Phs3 => RegulatorDiscreteControlProfile {
                control_message_info: Some(msg_info),
                regulator_system: Some(RegulatorSystem {
                    conducting_equipment: Some(ConductingEquipment {
                        named_object: None,
                        m_rid: m_rid.to_string(),
                    }),
                }),
                regulator_discrete_control: Some(RegulatorDiscreteControl {
                    control_value: None,
                    check: None,
                    regulator_control_atcc: Some(RegulatorControlAtcc {
                        dir_fwd: Some(DirectionalAtcc {
                            ldcx: Some(PhaseApc {
                                phs3: Some(ControlApc { ctl_val: ctl_val }),
                                ..Default::default()
                            }),
                            ..Default::default()
                        }),
                        ..Default::default()
                    }),
                }),
            },
            Phase::PhsA => RegulatorDiscreteControlProfile {
                control_message_info: Some(msg_info),
                regulator_system: Some(RegulatorSystem {
                    conducting_equipment: Some(ConductingEquipment {
                        named_object: None,
                        m_rid: m_rid.to_string(),
                    }),
                }),
                regulator_discrete_control: Some(RegulatorDiscreteControl {
                    control_value: None,
                    check: None,
                    regulator_control_atcc: Some(RegulatorControlAtcc {
                        dir_fwd: Some(DirectionalAtcc {
                            ldcx: Some(PhaseApc {
                                phs_a: Some(ControlApc { ctl_val: ctl_val }),
                                ..Default::default()
                            }),
                            ..Default::default()
                        }),
                        ..Default::default()
                    }),
                }),
            },
            Phase::PhsB => RegulatorDiscreteControlProfile {
                control_message_info: Some(msg_info),
                regulator_system: Some(RegulatorSystem {
                    conducting_equipment: Some(ConductingEquipment {
                        named_object: None,
                        m_rid: m_rid.to_string(),
                    }),
                }),
                regulator_discrete_control: Some(RegulatorDiscreteControl {
                    control_value: None,
                    check: None,
                    regulator_control_atcc: Some(RegulatorControlAtcc {
                        dir_fwd: Some(DirectionalAtcc {
                            ldcx: Some(PhaseApc {
                                phs_b: Some(ControlApc { ctl_val: ctl_val }),
                                ..Default::default()
                            }),
                            ..Default::default()
                        }),
                        ..Default::default()
                    }),
                }),
            },
            Phase::PhsC => RegulatorDiscreteControlProfile {
                control_message_info: Some(msg_info),
                regulator_system: Some(RegulatorSystem {
                    conducting_equipment: Some(ConductingEquipment {
                        named_object: None,
                        m_rid: m_rid.to_string(),
                    }),
                }),
                regulator_discrete_control: Some(RegulatorDiscreteControl {
                    control_value: None,
                    check: None,
                    regulator_control_atcc: Some(RegulatorControlAtcc {
                        dir_fwd: Some(DirectionalAtcc {
                            ldcx: Some(PhaseApc {
                                phs_c: Some(ControlApc { ctl_val: ctl_val }),
                                ..Default::default()
                            }),
                            ..Default::default()
                        }),
                        ..Default::default()
                    }),
                }),
            },
        }
    }

    fn dir_fwd_vol_spt_msg(
        m_rid: &str,
        phase: Phase,
        ctl_val: f64,
    ) -> RegulatorDiscreteControlProfile {
        let msg_info: ControlMessageInfo =
            RegulatorDiscreteControlProfile::build_control_message_info();
        match phase {
            Phase::Phs3 => RegulatorDiscreteControlProfile {
                control_message_info: Some(msg_info),
                regulator_system: Some(RegulatorSystem {
                    conducting_equipment: Some(ConductingEquipment {
                        named_object: None,
                        m_rid: m_rid.to_string(),
                    }),
                }),
                regulator_discrete_control: Some(RegulatorDiscreteControl {
                    control_value: None,
                    check: None,
                    regulator_control_atcc: Some(RegulatorControlAtcc {
                        dir_fwd: Some(DirectionalAtcc {
                            vol_spt: Some(PhaseApc {
                                phs3: Some(ControlApc { ctl_val: ctl_val }),
                                ..Default::default()
                            }),
                            ..Default::default()
                        }),
                        ..Default::default()
                    }),
                }),
            },
            Phase::PhsA => RegulatorDiscreteControlProfile {
                control_message_info: Some(msg_info),
                regulator_system: Some(RegulatorSystem {
                    conducting_equipment: Some(ConductingEquipment {
                        named_object: None,
                        m_rid: m_rid.to_string(),
                    }),
                }),
                regulator_discrete_control: Some(RegulatorDiscreteControl {
                    control_value: None,
                    check: None,
                    regulator_control_atcc: Some(RegulatorControlAtcc {
                        dir_fwd: Some(DirectionalAtcc {
                            vol_spt: Some(PhaseApc {
                                phs_a: Some(ControlApc { ctl_val: ctl_val }),
                                ..Default::default()
                            }),
                            ..Default::default()
                        }),
                        ..Default::default()
                    }),
                }),
            },
            Phase::PhsB => RegulatorDiscreteControlProfile {
                control_message_info: Some(msg_info),
                regulator_system: Some(RegulatorSystem {
                    conducting_equipment: Some(ConductingEquipment {
                        named_object: None,
                        m_rid: m_rid.to_string(),
                    }),
                }),
                regulator_discrete_control: Some(RegulatorDiscreteControl {
                    control_value: None,
                    check: None,
                    regulator_control_atcc: Some(RegulatorControlAtcc {
                        dir_fwd: Some(DirectionalAtcc {
                            vol_spt: Some(PhaseApc {
                                phs_b: Some(ControlApc { ctl_val: ctl_val }),
                                ..Default::default()
                            }),
                            ..Default::default()
                        }),
                        ..Default::default()
                    }),
                }),
            },
            Phase::PhsC => RegulatorDiscreteControlProfile {
                control_message_info: Some(msg_info),
                regulator_system: Some(RegulatorSystem {
                    conducting_equipment: Some(ConductingEquipment {
                        named_object: None,
                        m_rid: m_rid.to_string(),
                    }),
                }),
                regulator_discrete_control: Some(RegulatorDiscreteControl {
                    control_value: None,
                    check: None,
                    regulator_control_atcc: Some(RegulatorControlAtcc {
                        dir_fwd: Some(DirectionalAtcc {
                            vol_spt: Some(PhaseApc {
                                phs_c: Some(ControlApc { ctl_val: ctl_val }),
                                ..Default::default()
                            }),
                            ..Default::default()
                        }),
                        ..Default::default()
                    }),
                }),
            },
        }
    }

    fn dir_rev_bnd_wid_msg(
        m_rid: &str,
        phase: Phase,
        ctl_val: f64,
    ) -> RegulatorDiscreteControlProfile {
        let msg_info: ControlMessageInfo =
            RegulatorDiscreteControlProfile::build_control_message_info();
        match phase {
            Phase::Phs3 => RegulatorDiscreteControlProfile {
                control_message_info: Some(msg_info),
                regulator_system: Some(RegulatorSystem {
                    conducting_equipment: Some(ConductingEquipment {
                        named_object: None,
                        m_rid: m_rid.to_string(),
                    }),
                }),
                regulator_discrete_control: Some(RegulatorDiscreteControl {
                    control_value: None,
                    check: None,
                    regulator_control_atcc: Some(RegulatorControlAtcc {
                        dir_rev: Some(DirectionalAtcc {
                            bnd_wid: Some(PhaseApc {
                                phs3: Some(ControlApc { ctl_val: ctl_val }),
                                ..Default::default()
                            }),
                            ..Default::default()
                        }),
                        ..Default::default()
                    }),
                }),
            },
            Phase::PhsA => RegulatorDiscreteControlProfile {
                control_message_info: Some(msg_info),
                regulator_system: Some(RegulatorSystem {
                    conducting_equipment: Some(ConductingEquipment {
                        named_object: None,
                        m_rid: m_rid.to_string(),
                    }),
                }),
                regulator_discrete_control: Some(RegulatorDiscreteControl {
                    control_value: None,
                    check: None,
                    regulator_control_atcc: Some(RegulatorControlAtcc {
                        dir_rev: Some(DirectionalAtcc {
                            bnd_wid: Some(PhaseApc {
                                phs_a: Some(ControlApc { ctl_val: ctl_val }),
                                ..Default::default()
                            }),
                            ..Default::default()
                        }),
                        ..Default::default()
                    }),
                }),
            },
            Phase::PhsB => RegulatorDiscreteControlProfile {
                control_message_info: Some(msg_info),
                regulator_system: Some(RegulatorSystem {
                    conducting_equipment: Some(ConductingEquipment {
                        named_object: None,
                        m_rid: m_rid.to_string(),
                    }),
                }),
                regulator_discrete_control: Some(RegulatorDiscreteControl {
                    control_value: None,
                    check: None,
                    regulator_control_atcc: Some(RegulatorControlAtcc {
                        dir_rev: Some(DirectionalAtcc {
                            bnd_wid: Some(PhaseApc {
                                phs_b: Some(ControlApc { ctl_val: ctl_val }),
                                ..Default::default()
                            }),
                            ..Default::default()
                        }),
                        ..Default::default()
                    }),
                }),
            },
            Phase::PhsC => RegulatorDiscreteControlProfile {
                control_message_info: Some(msg_info),
                regulator_system: Some(RegulatorSystem {
                    conducting_equipment: Some(ConductingEquipment {
                        named_object: None,
                        m_rid: m_rid.to_string(),
                    }),
                }),
                regulator_discrete_control: Some(RegulatorDiscreteControl {
                    control_value: None,
                    check: None,
                    regulator_control_atcc: Some(RegulatorControlAtcc {
                        dir_rev: Some(DirectionalAtcc {
                            bnd_wid: Some(PhaseApc {
                                phs_c: Some(ControlApc { ctl_val: ctl_val }),
                                ..Default::default()
                            }),
                            ..Default::default()
                        }),
                        ..Default::default()
                    }),
                }),
            },
        }
    }

    fn dir_rev_ldcr_msg(
        m_rid: &str,
        phase: Phase,
        ctl_val: f64,
    ) -> RegulatorDiscreteControlProfile {
        let msg_info: ControlMessageInfo =
            RegulatorDiscreteControlProfile::build_control_message_info();
        match phase {
            Phase::Phs3 => RegulatorDiscreteControlProfile {
                control_message_info: Some(msg_info),
                regulator_system: Some(RegulatorSystem {
                    conducting_equipment: Some(ConductingEquipment {
                        named_object: None,
                        m_rid: m_rid.to_string(),
                    }),
                }),
                regulator_discrete_control: Some(RegulatorDiscreteControl {
                    control_value: None,
                    check: None,
                    regulator_control_atcc: Some(RegulatorControlAtcc {
                        dir_rev: Some(DirectionalAtcc {
                            ldcr: Some(PhaseApc {
                                phs3: Some(ControlApc { ctl_val: ctl_val }),
                                ..Default::default()
                            }),
                            ..Default::default()
                        }),
                        ..Default::default()
                    }),
                }),
            },
            Phase::PhsA => RegulatorDiscreteControlProfile {
                control_message_info: Some(msg_info),
                regulator_system: Some(RegulatorSystem {
                    conducting_equipment: Some(ConductingEquipment {
                        named_object: None,
                        m_rid: m_rid.to_string(),
                    }),
                }),
                regulator_discrete_control: Some(RegulatorDiscreteControl {
                    control_value: None,
                    check: None,
                    regulator_control_atcc: Some(RegulatorControlAtcc {
                        dir_rev: Some(DirectionalAtcc {
                            ldcr: Some(PhaseApc {
                                phs_a: Some(ControlApc { ctl_val: ctl_val }),
                                ..Default::default()
                            }),
                            ..Default::default()
                        }),
                        ..Default::default()
                    }),
                }),
            },
            Phase::PhsB => RegulatorDiscreteControlProfile {
                control_message_info: Some(msg_info),
                regulator_system: Some(RegulatorSystem {
                    conducting_equipment: Some(ConductingEquipment {
                        named_object: None,
                        m_rid: m_rid.to_string(),
                    }),
                }),
                regulator_discrete_control: Some(RegulatorDiscreteControl {
                    control_value: None,
                    check: None,
                    regulator_control_atcc: Some(RegulatorControlAtcc {
                        dir_rev: Some(DirectionalAtcc {
                            ldcr: Some(PhaseApc {
                                phs_b: Some(ControlApc { ctl_val: ctl_val }),
                                ..Default::default()
                            }),
                            ..Default::default()
                        }),
                        ..Default::default()
                    }),
                }),
            },
            Phase::PhsC => RegulatorDiscreteControlProfile {
                control_message_info: Some(msg_info),
                regulator_system: Some(RegulatorSystem {
                    conducting_equipment: Some(ConductingEquipment {
                        named_object: None,
                        m_rid: m_rid.to_string(),
                    }),
                }),
                regulator_discrete_control: Some(RegulatorDiscreteControl {
                    control_value: None,
                    check: None,
                    regulator_control_atcc: Some(RegulatorControlAtcc {
                        dir_rev: Some(DirectionalAtcc {
                            ldcr: Some(PhaseApc {
                                phs_c: Some(ControlApc { ctl_val: ctl_val }),
                                ..Default::default()
                            }),
                            ..Default::default()
                        }),
                        ..Default::default()
                    }),
                }),
            },
        }
    }

    fn dir_rev_ldcx_msg(
        m_rid: &str,
        phase: Phase,
        ctl_val: f64,
    ) -> RegulatorDiscreteControlProfile {
        let msg_info: ControlMessageInfo =
            RegulatorDiscreteControlProfile::build_control_message_info();
        match phase {
            Phase::Phs3 => RegulatorDiscreteControlProfile {
                control_message_info: Some(msg_info),
                regulator_system: Some(RegulatorSystem {
                    conducting_equipment: Some(ConductingEquipment {
                        named_object: None,
                        m_rid: m_rid.to_string(),
                    }),
                }),
                regulator_discrete_control: Some(RegulatorDiscreteControl {
                    control_value: None,
                    check: None,
                    regulator_control_atcc: Some(RegulatorControlAtcc {
                        dir_rev: Some(DirectionalAtcc {
                            ldcx: Some(PhaseApc {
                                phs3: Some(ControlApc { ctl_val: ctl_val }),
                                ..Default::default()
                            }),
                            ..Default::default()
                        }),
                        ..Default::default()
                    }),
                }),
            },
            Phase::PhsA => RegulatorDiscreteControlProfile {
                control_message_info: Some(msg_info),
                regulator_system: Some(RegulatorSystem {
                    conducting_equipment: Some(ConductingEquipment {
                        named_object: None,
                        m_rid: m_rid.to_string(),
                    }),
                }),
                regulator_discrete_control: Some(RegulatorDiscreteControl {
                    control_value: None,
                    check: None,
                    regulator_control_atcc: Some(RegulatorControlAtcc {
                        dir_rev: Some(DirectionalAtcc {
                            ldcx: Some(PhaseApc {
                                phs_a: Some(ControlApc { ctl_val: ctl_val }),
                                ..Default::default()
                            }),
                            ..Default::default()
                        }),
                        ..Default::default()
                    }),
                }),
            },
            Phase::PhsB => RegulatorDiscreteControlProfile {
                control_message_info: Some(msg_info),
                regulator_system: Some(RegulatorSystem {
                    conducting_equipment: Some(ConductingEquipment {
                        named_object: None,
                        m_rid: m_rid.to_string(),
                    }),
                }),
                regulator_discrete_control: Some(RegulatorDiscreteControl {
                    control_value: None,
                    check: None,
                    regulator_control_atcc: Some(RegulatorControlAtcc {
                        dir_rev: Some(DirectionalAtcc {
                            ldcx: Some(PhaseApc {
                                phs_b: Some(ControlApc { ctl_val: ctl_val }),
                                ..Default::default()
                            }),
                            ..Default::default()
                        }),
                        ..Default::default()
                    }),
                }),
            },
            Phase::PhsC => RegulatorDiscreteControlProfile {
                control_message_info: Some(msg_info),
                regulator_system: Some(RegulatorSystem {
                    conducting_equipment: Some(ConductingEquipment {
                        named_object: None,
                        m_rid: m_rid.to_string(),
                    }),
                }),
                regulator_discrete_control: Some(RegulatorDiscreteControl {
                    control_value: None,
                    check: None,
                    regulator_control_atcc: Some(RegulatorControlAtcc {
                        dir_rev: Some(DirectionalAtcc {
                            ldcx: Some(PhaseApc {
                                phs_c: Some(ControlApc { ctl_val: ctl_val }),
                                ..Default::default()
                            }),
                            ..Default::default()
                        }),
                        ..Default::default()
                    }),
                }),
            },
        }
    }

    fn dir_rev_vol_spt_msg(
        m_rid: &str,
        phase: Phase,
        ctl_val: f64,
    ) -> RegulatorDiscreteControlProfile {
        let msg_info: ControlMessageInfo =
            RegulatorDiscreteControlProfile::build_control_message_info();
        match phase {
            Phase::Phs3 => RegulatorDiscreteControlProfile {
                control_message_info: Some(msg_info),
                regulator_system: Some(RegulatorSystem {
                    conducting_equipment: Some(ConductingEquipment {
                        named_object: None,
                        m_rid: m_rid.to_string(),
                    }),
                }),
                regulator_discrete_control: Some(RegulatorDiscreteControl {
                    control_value: None,
                    check: None,
                    regulator_control_atcc: Some(RegulatorControlAtcc {
                        dir_rev: Some(DirectionalAtcc {
                            vol_spt: Some(PhaseApc {
                                phs3: Some(ControlApc { ctl_val: ctl_val }),
                                ..Default::default()
                            }),
                            ..Default::default()
                        }),
                        ..Default::default()
                    }),
                }),
            },
            Phase::PhsA => RegulatorDiscreteControlProfile {
                control_message_info: Some(msg_info),
                regulator_system: Some(RegulatorSystem {
                    conducting_equipment: Some(ConductingEquipment {
                        named_object: None,
                        m_rid: m_rid.to_string(),
                    }),
                }),
                regulator_discrete_control: Some(RegulatorDiscreteControl {
                    control_value: None,
                    check: None,
                    regulator_control_atcc: Some(RegulatorControlAtcc {
                        dir_rev: Some(DirectionalAtcc {
                            vol_spt: Some(PhaseApc {
                                phs_a: Some(ControlApc { ctl_val: ctl_val }),
                                ..Default::default()
                            }),
                            ..Default::default()
                        }),
                        ..Default::default()
                    }),
                }),
            },
            Phase::PhsB => RegulatorDiscreteControlProfile {
                control_message_info: Some(msg_info),
                regulator_system: Some(RegulatorSystem {
                    conducting_equipment: Some(ConductingEquipment {
                        named_object: None,
                        m_rid: m_rid.to_string(),
                    }),
                }),
                regulator_discrete_control: Some(RegulatorDiscreteControl {
                    control_value: None,
                    check: None,
                    regulator_control_atcc: Some(RegulatorControlAtcc {
                        dir_rev: Some(DirectionalAtcc {
                            vol_spt: Some(PhaseApc {
                                phs_b: Some(ControlApc { ctl_val: ctl_val }),
                                ..Default::default()
                            }),
                            ..Default::default()
                        }),
                        ..Default::default()
                    }),
                }),
            },
            Phase::PhsC => RegulatorDiscreteControlProfile {
                control_message_info: Some(msg_info),
                regulator_system: Some(RegulatorSystem {
                    conducting_equipment: Some(ConductingEquipment {
                        named_object: None,
                        m_rid: m_rid.to_string(),
                    }),
                }),
                regulator_discrete_control: Some(RegulatorDiscreteControl {
                    control_value: None,
                    check: None,
                    regulator_control_atcc: Some(RegulatorControlAtcc {
                        dir_rev: Some(DirectionalAtcc {
                            vol_spt: Some(PhaseApc {
                                phs_c: Some(ControlApc { ctl_val: ctl_val }),
                                ..Default::default()
                            }),
                            ..Default::default()
                        }),
                        ..Default::default()
                    }),
                }),
            },
        }
    }

    fn dir_mode(m_rid: &str, mode: i32) -> RegulatorDiscreteControlProfile {
        let msg_info: ControlMessageInfo =
            RegulatorDiscreteControlProfile::build_control_message_info();

        RegulatorDiscreteControlProfile {
            control_message_info: Some(msg_info),
            regulator_system: Some(RegulatorSystem {
                conducting_equipment: Some(ConductingEquipment {
                    named_object: None,
                    m_rid: m_rid.to_string(),
                }),
            }),
            regulator_discrete_control: Some(RegulatorDiscreteControl {
                control_value: None,
                check: None,
                regulator_control_atcc: Some(RegulatorControlAtcc {
                    dir_mode: Some(OptionalDirectionModeKind { value: mode }),
                    ..Default::default()
                }),
            }),
        }
    }
}

impl ControlProfileExt for RegulatorDiscreteControlProfile {}
