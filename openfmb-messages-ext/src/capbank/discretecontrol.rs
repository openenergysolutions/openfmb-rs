// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

use snafu::{OptionExt, ResultExt};
use std::str::FromStr;
use uuid::Uuid;

use openfmb_messages::{capbankmodule::*, commonmodule::*};

use crate::{error::*, ControlProfileExt, OpenFMBExt};

impl OpenFMBExt for CapBankDiscreteControlProfile {
    fn device_state(&self) -> OpenFMBResult<String> {
        Ok("".into())
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
        Ok("CapBankDiscreteControlProfile".to_string())
    }

    fn device_mrid(&self) -> OpenFMBResult<Uuid> {
        Ok(Uuid::from_str(
            &self
                .cap_bank_system
                .as_ref()
                .context(NoCapBankSystem)?
                .conducting_equipment
                .as_ref()
                .context(NoConductingEquipment)?
                .m_rid,
        )
        .context(UuidError)?)
    }

    fn device_name(&self) -> OpenFMBResult<String> {
        Ok(self
            .cap_bank_system
            .as_ref()
            .context(NoCapBankSystem)?
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

pub trait CapBankDiscreteControlExt: ControlProfileExt {
    fn capbank_toggle_remote_msg(m_rid: &str, ct_val: bool) -> CapBankDiscreteControlProfile;

    fn capbank_toggle_mode_msg(m_rid: &str, ct_val: bool) -> CapBankDiscreteControlProfile;

    fn capbank_toggle_pos_msg(m_rid: &str, ct_val: bool) -> CapBankDiscreteControlProfile;
}

impl CapBankDiscreteControlExt for CapBankDiscreteControlProfile {
    fn capbank_toggle_mode_msg(m_rid: &str, ct_val: bool) -> CapBankDiscreteControlProfile {
        let msg_info: ControlMessageInfo =
            CapBankDiscreteControlProfile::build_control_message_info();
        CapBankDiscreteControlProfile {
            control_message_info: Some(msg_info),
            cap_bank_system: Some(CapBankSystem {
                conducting_equipment: Some(ConductingEquipment {
                    named_object: None,
                    m_rid: m_rid.to_string(),
                }),
            }),
            cap_bank_control: Some(CapBankDiscreteControl {
                cap_bank_discrete_control_ypsh: Some(CapBankDiscreteControlYpsh {
                    logical_node_for_control: None,
                    control: Some(CapBankControlYpsh {
                        ctl_mode_ovr_rd: Some(ControlSpc { ctl_val: ct_val }),
                        ..Default::default()
                    }),
                }),
                ..Default::default()
            }),
        }
    }

    fn capbank_toggle_pos_msg(m_rid: &str, pos: bool) -> CapBankDiscreteControlProfile {
        let msg_info: ControlMessageInfo =
            CapBankDiscreteControlProfile::build_control_message_info();
        CapBankDiscreteControlProfile {
            control_message_info: Some(msg_info),
            cap_bank_system: Some(CapBankSystem {
                conducting_equipment: Some(ConductingEquipment {
                    named_object: None,
                    m_rid: m_rid.to_string(),
                }),
            }),
            cap_bank_control: Some(CapBankDiscreteControl {
                cap_bank_discrete_control_ypsh: Some(CapBankDiscreteControlYpsh {
                    logical_node_for_control: None,
                    control: Some(CapBankControlYpsh {
                        pos: Some(PhaseSpc {
                            phs3: Some(ControlSpc { ctl_val: pos }),
                            ..Default::default()
                        }),
                        ..Default::default()
                    }),
                }),
                ..Default::default()
            }),
        }
    }

    fn capbank_toggle_remote_msg(m_rid: &str, ct_val: bool) -> CapBankDiscreteControlProfile {
        let msg_info: ControlMessageInfo =
            CapBankDiscreteControlProfile::build_control_message_info();
        CapBankDiscreteControlProfile {
            control_message_info: Some(msg_info),
            cap_bank_system: Some(CapBankSystem {
                conducting_equipment: Some(ConductingEquipment {
                    named_object: None,
                    m_rid: m_rid.to_string(),
                }),
            }),
            cap_bank_control: Some(CapBankDiscreteControl {
                cap_bank_discrete_control_ypsh: Some(CapBankDiscreteControlYpsh {
                    logical_node_for_control: None,
                    control: Some(CapBankControlYpsh {
                        ctl_mode_auto: Some(ControlSpc { ctl_val: !ct_val }),
                        ctl_mode_rem: Some(ControlSpc { ctl_val: ct_val }),
                        ..Default::default()
                    }),
                }),
                ..Default::default()
            }),
        }
    }
}

impl ControlProfileExt for CapBankDiscreteControlProfile {}
