// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

use crate::{error::*, OpenFMBExt, OpenFMBExtReading, ReadingProfileExt};
use openfmb_messages::{
    commonmodule::{MessageInfo, ReadingMessageInfo},
    switchmodule::SwitchReadingProfile,
};
use snafu::{OptionExt, ResultExt};
use std::str::FromStr;
use uuid::Uuid;

impl OpenFMBExt for SwitchReadingProfile {
    fn device_state(&self) -> OpenFMBResult<String> {
        if !self.switch_reading.is_empty() {
            return Ok(self
                .switch_reading
                .first()
                .as_ref()
                .context(NoSwitchReading)?
                .reading_mmxu
                .as_ref()
                .context(NoReadingMmxu)?
                .w
                .as_ref()
                .context(NoW)?
                .net
                .as_ref()
                .context(NoNet)?
                .c_val
                .as_ref()
                .context(NoCVal)?
                .mag
                .to_string());
        }
        Err(OpenFMBError::InvalidOpenFMBMessage)
    }

    fn message_info(&self) -> OpenFMBResult<&MessageInfo> {
        Ok(self
            .reading_message_info
            .as_ref()
            .context(NoStatusMessageInfo)?
            .message_info
            .as_ref()
            .context(NoMessageInfo)?)
    }

    fn message_type(&self) -> OpenFMBResult<String> {
        Ok("SwitchReadingProfile".to_string())
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

impl OpenFMBExtReading for SwitchReadingProfile {
    fn reading_message_info(&self) -> OpenFMBResult<&ReadingMessageInfo> {
        Ok(self
            .reading_message_info
            .as_ref()
            .context(NoStatusMessageInfo)?)
    }
}

pub trait SwitchReadingExt: ReadingProfileExt {
    fn switch_reading(&self) -> Option<f64>;
    fn switch_vars(&self) -> Option<f64>;
}

impl SwitchReadingExt for SwitchReadingProfile {
    fn switch_reading(&self) -> Option<f64> {
        if !self.switch_reading.is_empty() {
            return Some(
                self.switch_reading
                    .first()
                    .as_ref()?
                    .reading_mmxu
                    .as_ref()?
                    .w
                    .as_ref()?
                    .net
                    .as_ref()?
                    .c_val
                    .as_ref()?
                    .mag,
            );
        }
        None
    }
    fn switch_vars(&self) -> Option<f64> {
        if !self.switch_reading.is_empty() {
            return Some(
                self.switch_reading
                    .first()
                    .as_ref()?
                    .reading_mmxu
                    .as_ref()?
                    .v_ar
                    .as_ref()?
                    .net
                    .as_ref()?
                    .c_val
                    .as_ref()?
                    .mag,
            );
        }
        None
    }    
}


impl ReadingProfileExt for SwitchReadingProfile {}
