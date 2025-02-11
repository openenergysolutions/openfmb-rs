// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use openfmb_messages::{
    commonmodule::{DbPosKind, MessageInfo, StatusMessageInfo},
    *,
};
use reclosermodule::RecloserStatusProfile;
use snafu::{OptionExt, ResultExt};
use uuid::Uuid;

use crate::{error::*, OpenFMBExt, OpenFMBExtStatus, Phase, Position};

impl OpenFMBExtStatus for RecloserStatusProfile {
    fn status_message_info(&self) -> OpenFMBResult<&StatusMessageInfo> {
        Ok(self
            .status_message_info
            .as_ref()
            .context(NoStatusMessageInfo)?)
    }
}

impl OpenFMBExt for RecloserStatusProfile {
    fn device_state(&self) -> OpenFMBResult<String> {
        match self
            .recloser_status
            .as_ref()
            .context(NoRecloserStatus)?
            .status_and_event_xcbr
            .as_ref()
            .context(NoStatusAndEventXcbr)?
            .pos
            .as_ref()
            .context(NoPos)?
            .phs3
            .as_ref()
            .context(NoPhs3)
        {
            Ok(phs3) => match phs3.st_val {
                0 => Ok("Undefined".into()),
                1 => Ok("Transient".into()),
                2 => Ok("Closed".into()),
                3 => Ok("Open".into()),
                4 => Ok("Invalid".into()),
                _ => Err(OpenFMBError::InvalidValue),
            },
            Err(_) => Err(OpenFMBError::InvalidOpenFMBMessage),
        }
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
        Ok("RecloserStatusProfile".to_string())
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

impl Position for RecloserStatusProfile {
    fn pos(&self) -> OpenFMBResult<DbPosKind> {
        self.pos_per_phase(Phase::Phs3)
    }

    fn pos_per_phase(&self, phase: Phase) -> OpenFMBResult<DbPosKind> {
        let st_val = match phase {
            Phase::Phs3 => {
                self.recloser_status
                    .as_ref()
                    .context(NoRecloserStatus)?
                    .status_and_event_xcbr
                    .as_ref()
                    .context(NoStatusAndEventXcbr)?
                    .pos
                    .as_ref()
                    .context(NoPos)?
                    .phs3
                    .as_ref()
                    .context(NoPhs3)?
                    .st_val
            }
            Phase::PhsA => {
                self.recloser_status
                    .as_ref()
                    .context(NoRecloserStatus)?
                    .status_and_event_xcbr
                    .as_ref()
                    .context(NoStatusAndEventXcbr)?
                    .pos
                    .as_ref()
                    .context(NoPos)?
                    .phs_a
                    .as_ref()
                    .context(NoPhsA)?
                    .st_val
            }
            Phase::PhsB => {
                self.recloser_status
                    .as_ref()
                    .context(NoRecloserStatus)?
                    .status_and_event_xcbr
                    .as_ref()
                    .context(NoStatusAndEventXcbr)?
                    .pos
                    .as_ref()
                    .context(NoPos)?
                    .phs_b
                    .as_ref()
                    .context(NoPhsB)?
                    .st_val
            }
            Phase::PhsC => {
                self.recloser_status
                    .as_ref()
                    .context(NoRecloserStatus)?
                    .status_and_event_xcbr
                    .as_ref()
                    .context(NoStatusAndEventXcbr)?
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
