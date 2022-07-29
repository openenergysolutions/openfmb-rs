// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

use snafu::{OptionExt, ResultExt};
use std::str::FromStr;
use uuid::Uuid;

use capbankmodule::CapBankStatusProfile;
use openfmb_messages::{commonmodule::*, *};

use crate::{error::*, OpenFMBExt, OpenFMBExtStatus, Phase, Position, StatusProfileExt};

impl OpenFMBExtStatus for CapBankStatusProfile {
    fn status_message_info(&self) -> OpenFMBResult<&StatusMessageInfo> {
        Ok(self
            .status_message_info
            .as_ref()
            .context(NoStatusMessageInfo)?)
    }
}

impl OpenFMBExt for CapBankStatusProfile {
    fn device_state(&self) -> OpenFMBResult<String> {
        Ok("".into())
    }

    fn message_info(&self) -> OpenFMBResult<&MessageInfo> {
        Ok(self
            .status_message_info
            .as_ref()
            .context(NoStatusMessageInfo)?
            .message_info
            .as_ref()
            .context(NoMessageInfo)?)
    }

    fn message_type(&self) -> OpenFMBResult<String> {
        Ok("CapBankStatusProfile".to_string())
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

impl Position for CapBankStatusProfile {
    fn pos(&self) -> OpenFMBResult<DbPosKind> {
        self.pos_per_phase(Phase::Phs3)
    }

    fn pos_per_phase(&self, phase: Phase) -> OpenFMBResult<DbPosKind> {
        let st_val = match phase {
            Phase::Phs3 => {
                self.cap_bank_status
                    .as_ref()
                    .context(NoCapBankStatus)?
                    .cap_bank_event_and_status_ypsh
                    .as_ref()
                    .context(NoCapBankEventAndStatusYpsh)?
                    .pos
                    .as_ref()
                    .context(NoPos)?
                    .phs3
                    .as_ref()
                    .context(NoPhs3)?
                    .st_val
            }
            Phase::PhsA => {
                self.cap_bank_status
                    .as_ref()
                    .context(NoCapBankStatus)?
                    .cap_bank_event_and_status_ypsh
                    .as_ref()
                    .context(NoCapBankEventAndStatusYpsh)?
                    .pos
                    .as_ref()
                    .context(NoPos)?
                    .phs_a
                    .as_ref()
                    .context(NoPhsA)?
                    .st_val
            }
            Phase::PhsB => {
                self.cap_bank_status
                    .as_ref()
                    .context(NoCapBankStatus)?
                    .cap_bank_event_and_status_ypsh
                    .as_ref()
                    .context(NoCapBankEventAndStatusYpsh)?
                    .pos
                    .as_ref()
                    .context(NoPos)?
                    .phs_b
                    .as_ref()
                    .context(NoPhsB)?
                    .st_val
            }
            Phase::PhsC => {
                self.cap_bank_status
                    .as_ref()
                    .context(NoCapBankStatus)?
                    .cap_bank_event_and_status_ypsh
                    .as_ref()
                    .context(NoCapBankEventAndStatusYpsh)?
                    .pos
                    .as_ref()
                    .context(NoPos)?
                    .phs_c
                    .as_ref()
                    .context(NoPhsC)?
                    .st_val
            }
        };

        match st_val {
            0 => Ok(DbPosKind::Undefined),
            1 => Ok(DbPosKind::Transient),
            2 => Ok(DbPosKind::Closed),
            3 => Ok(DbPosKind::Open),
            4 => Ok(DbPosKind::Invalid),
            _ => Ok(DbPosKind::Undefined),
        }
    }
}

pub trait CapBankStatusExt: StatusProfileExt {
    fn temp_lmt(&self, phase: Phase) -> OpenFMBResult<bool>;
    fn var_lmt(&self, phase: Phase) -> OpenFMBResult<bool>;
    fn vol_lmt(&self, phase: Phase) -> OpenFMBResult<bool>;
    fn amp_lmt(&self, phase: Phase) -> OpenFMBResult<bool>;
    fn dir_rev(&self, phase: Phase) -> OpenFMBResult<bool>;
    fn ctl_mode(&self) -> OpenFMBResult<ControlModeKind>;
}

impl CapBankStatusExt for CapBankStatusProfile {
    fn temp_lmt(&self, phase: Phase) -> OpenFMBResult<bool> {
        Ok(match phase {
            Phase::Phs3 => {
                self.cap_bank_status
                    .as_ref()
                    .context(NoCapBankStatus)?
                    .cap_bank_event_and_status_ypsh
                    .as_ref()
                    .context(NoCapBankEventAndStatusYpsh)?
                    .temp_lmt
                    .as_ref()
                    .context(NoValue)?
                    .phs3
                    .as_ref()
                    .context(NoPhs3)?
                    .st_val
            }
            Phase::PhsA => {
                self.cap_bank_status
                    .as_ref()
                    .context(NoCapBankStatus)?
                    .cap_bank_event_and_status_ypsh
                    .as_ref()
                    .context(NoCapBankEventAndStatusYpsh)?
                    .temp_lmt
                    .as_ref()
                    .context(NoValue)?
                    .phs_a
                    .as_ref()
                    .context(NoPhsA)?
                    .st_val
            }
            Phase::PhsB => {
                self.cap_bank_status
                    .as_ref()
                    .context(NoCapBankStatus)?
                    .cap_bank_event_and_status_ypsh
                    .as_ref()
                    .context(NoCapBankEventAndStatusYpsh)?
                    .temp_lmt
                    .as_ref()
                    .context(NoValue)?
                    .phs_b
                    .as_ref()
                    .context(NoPhsB)?
                    .st_val
            }
            Phase::PhsC => {
                self.cap_bank_status
                    .as_ref()
                    .context(NoCapBankStatus)?
                    .cap_bank_event_and_status_ypsh
                    .as_ref()
                    .context(NoCapBankEventAndStatusYpsh)?
                    .temp_lmt
                    .as_ref()
                    .context(NoValue)?
                    .phs_c
                    .as_ref()
                    .context(NoPhsC)?
                    .st_val
            }
        })
    }

    fn var_lmt(&self, phase: Phase) -> OpenFMBResult<bool> {
        Ok(match phase {
            Phase::Phs3 => {
                self.cap_bank_status
                    .as_ref()
                    .context(NoCapBankStatus)?
                    .cap_bank_event_and_status_ypsh
                    .as_ref()
                    .context(NoCapBankEventAndStatusYpsh)?
                    .v_ar_lmt
                    .as_ref()
                    .context(NoValue)?
                    .phs3
                    .as_ref()
                    .context(NoPhs3)?
                    .st_val
            }
            Phase::PhsA => {
                self.cap_bank_status
                    .as_ref()
                    .context(NoCapBankStatus)?
                    .cap_bank_event_and_status_ypsh
                    .as_ref()
                    .context(NoCapBankEventAndStatusYpsh)?
                    .v_ar_lmt
                    .as_ref()
                    .context(NoValue)?
                    .phs_a
                    .as_ref()
                    .context(NoPhsA)?
                    .st_val
            }
            Phase::PhsB => {
                self.cap_bank_status
                    .as_ref()
                    .context(NoCapBankStatus)?
                    .cap_bank_event_and_status_ypsh
                    .as_ref()
                    .context(NoCapBankEventAndStatusYpsh)?
                    .v_ar_lmt
                    .as_ref()
                    .context(NoValue)?
                    .phs_b
                    .as_ref()
                    .context(NoPhsB)?
                    .st_val
            }
            Phase::PhsC => {
                self.cap_bank_status
                    .as_ref()
                    .context(NoCapBankStatus)?
                    .cap_bank_event_and_status_ypsh
                    .as_ref()
                    .context(NoCapBankEventAndStatusYpsh)?
                    .v_ar_lmt
                    .as_ref()
                    .context(NoValue)?
                    .phs_c
                    .as_ref()
                    .context(NoPhsC)?
                    .st_val
            }
        })
    }

    fn vol_lmt(&self, phase: Phase) -> OpenFMBResult<bool> {
        Ok(match phase {
            Phase::Phs3 => {
                self.cap_bank_status
                    .as_ref()
                    .context(NoCapBankStatus)?
                    .cap_bank_event_and_status_ypsh
                    .as_ref()
                    .context(NoCapBankEventAndStatusYpsh)?
                    .vol_lmt
                    .as_ref()
                    .context(NoValue)?
                    .phs3
                    .as_ref()
                    .context(NoPhs3)?
                    .st_val
            }
            Phase::PhsA => {
                self.cap_bank_status
                    .as_ref()
                    .context(NoCapBankStatus)?
                    .cap_bank_event_and_status_ypsh
                    .as_ref()
                    .context(NoCapBankEventAndStatusYpsh)?
                    .vol_lmt
                    .as_ref()
                    .context(NoValue)?
                    .phs_a
                    .as_ref()
                    .context(NoPhsA)?
                    .st_val
            }
            Phase::PhsB => {
                self.cap_bank_status
                    .as_ref()
                    .context(NoCapBankStatus)?
                    .cap_bank_event_and_status_ypsh
                    .as_ref()
                    .context(NoCapBankEventAndStatusYpsh)?
                    .vol_lmt
                    .as_ref()
                    .context(NoValue)?
                    .phs_b
                    .as_ref()
                    .context(NoPhsB)?
                    .st_val
            }
            Phase::PhsC => {
                self.cap_bank_status
                    .as_ref()
                    .context(NoCapBankStatus)?
                    .cap_bank_event_and_status_ypsh
                    .as_ref()
                    .context(NoCapBankEventAndStatusYpsh)?
                    .vol_lmt
                    .as_ref()
                    .context(NoValue)?
                    .phs_c
                    .as_ref()
                    .context(NoPhsC)?
                    .st_val
            }
        })
    }

    fn amp_lmt(&self, phase: Phase) -> OpenFMBResult<bool> {
        Ok(match phase {
            Phase::Phs3 => {
                self.cap_bank_status
                    .as_ref()
                    .context(NoCapBankStatus)?
                    .cap_bank_event_and_status_ypsh
                    .as_ref()
                    .context(NoCapBankEventAndStatusYpsh)?
                    .amp_lmt
                    .as_ref()
                    .context(NoValue)?
                    .phs3
                    .as_ref()
                    .context(NoPhs3)?
                    .st_val
            }
            Phase::PhsA => {
                self.cap_bank_status
                    .as_ref()
                    .context(NoCapBankStatus)?
                    .cap_bank_event_and_status_ypsh
                    .as_ref()
                    .context(NoCapBankEventAndStatusYpsh)?
                    .amp_lmt
                    .as_ref()
                    .context(NoValue)?
                    .phs_a
                    .as_ref()
                    .context(NoPhsA)?
                    .st_val
            }
            Phase::PhsB => {
                self.cap_bank_status
                    .as_ref()
                    .context(NoCapBankStatus)?
                    .cap_bank_event_and_status_ypsh
                    .as_ref()
                    .context(NoCapBankEventAndStatusYpsh)?
                    .amp_lmt
                    .as_ref()
                    .context(NoValue)?
                    .phs_b
                    .as_ref()
                    .context(NoPhsB)?
                    .st_val
            }
            Phase::PhsC => {
                self.cap_bank_status
                    .as_ref()
                    .context(NoCapBankStatus)?
                    .cap_bank_event_and_status_ypsh
                    .as_ref()
                    .context(NoCapBankEventAndStatusYpsh)?
                    .amp_lmt
                    .as_ref()
                    .context(NoValue)?
                    .phs_c
                    .as_ref()
                    .context(NoPhsC)?
                    .st_val
            }
        })
    }

    fn dir_rev(&self, phase: Phase) -> OpenFMBResult<bool> {
        Ok(match phase {
            Phase::Phs3 => {
                self.cap_bank_status
                    .as_ref()
                    .context(NoCapBankStatus)?
                    .cap_bank_event_and_status_ypsh
                    .as_ref()
                    .context(NoCapBankEventAndStatusYpsh)?
                    .dir_rev
                    .as_ref()
                    .context(NoValue)?
                    .phs3
                    .as_ref()
                    .context(NoPhs3)?
                    .st_val
            }
            Phase::PhsA => {
                self.cap_bank_status
                    .as_ref()
                    .context(NoCapBankStatus)?
                    .cap_bank_event_and_status_ypsh
                    .as_ref()
                    .context(NoCapBankEventAndStatusYpsh)?
                    .dir_rev
                    .as_ref()
                    .context(NoValue)?
                    .phs_a
                    .as_ref()
                    .context(NoPhsA)?
                    .st_val
            }
            Phase::PhsB => {
                self.cap_bank_status
                    .as_ref()
                    .context(NoCapBankStatus)?
                    .cap_bank_event_and_status_ypsh
                    .as_ref()
                    .context(NoCapBankEventAndStatusYpsh)?
                    .dir_rev
                    .as_ref()
                    .context(NoValue)?
                    .phs_b
                    .as_ref()
                    .context(NoPhsB)?
                    .st_val
            }
            Phase::PhsC => {
                self.cap_bank_status
                    .as_ref()
                    .context(NoCapBankStatus)?
                    .cap_bank_event_and_status_ypsh
                    .as_ref()
                    .context(NoCapBankEventAndStatusYpsh)?
                    .dir_rev
                    .as_ref()
                    .context(NoValue)?
                    .phs_c
                    .as_ref()
                    .context(NoPhsC)?
                    .st_val
            }
        })
    }

    fn ctl_mode(&self) -> OpenFMBResult<ControlModeKind> {
        Ok(
            match self
                .cap_bank_status
                .as_ref()
                .context(NoCapBankStatus)?
                .cap_bank_event_and_status_ypsh
                .as_ref()
                .context(NoCapBankEventAndStatusYpsh)?
                .ctl_mode
                .as_ref()
                .context(NoValue)?
                .value
            {
                1 => ControlModeKind::Auto,
                2 => ControlModeKind::Manual,
                3 => ControlModeKind::Override,
                4 => ControlModeKind::Remote,
                _ => ControlModeKind::Undefined,
            },
        )
    }
}

impl StatusProfileExt for CapBankStatusProfile {}
