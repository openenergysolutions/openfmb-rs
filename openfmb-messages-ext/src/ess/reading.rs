// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use openfmb_messages::{
    commonmodule::{MessageInfo, ReadingMessageInfo},
    essmodule::EssReadingProfile,
};
use snafu::{OptionExt, ResultExt};
use uuid::Uuid;

use crate::{error::*, OpenFMBExt, OpenFMBExtReading, OpenFMBReading, ReadingProfileExt};

impl OpenFMBExt for EssReadingProfile {
    fn device_state(&self) -> OpenFMBResult<String> {
        Ok("".to_string())
        //panic!("{:?}", self);
    }

    fn message_info(&self) -> OpenFMBResult<&MessageInfo> {
        Ok(self
            .reading_message_info
            .as_ref()
            .context(NoReadingMessageInfo)?
            .message_info
            .as_ref()
            .context(NoMessageInfo)?)
    }

    fn message_type(&self) -> OpenFMBResult<String> {
        Ok("ESSReadingProfile".to_string())
    }

    fn device_mrid(&self) -> OpenFMBResult<Uuid> {
        Ok(Uuid::from_str(
            &self
                .ess
                .as_ref()
                .context(NoEss)?
                .conducting_equipment
                .as_ref()
                .context(NoConductingEquipment)?
                .m_rid,
        )
        .context(UuidError)?)
    }

    fn device_name(&self) -> OpenFMBResult<String> {
        Ok(self
            .ess
            .as_ref()
            .context(NoEss)?
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

impl OpenFMBExtReading for EssReadingProfile {
    fn reading_message_info(&self) -> OpenFMBResult<&ReadingMessageInfo> {
        Ok(self
            .reading_message_info
            .as_ref()
            .context(NoStatusMessageInfo)?)
    }
}

pub trait EssReadingExt: ReadingProfileExt {}

impl ReadingProfileExt for EssReadingProfile {
    fn w_net(&self) -> OpenFMBResult<f64> {
        Ok(self
            .ess_reading
            .as_ref()
            .context(NoEssReading)?
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
            .mag)
    }

    fn q_net(&self) -> OpenFMBResult<f64> {
        Ok(self
            .ess_reading
            .as_ref()
            .context(NoEssReading)?
            .reading_mmxu
            .as_ref()
            .context(NoReadingMmxu)?
            .v_ar
            .as_ref()
            .context(NoW)?
            .net
            .as_ref()
            .context(NoNet)?
            .c_val
            .as_ref()
            .context(NoCVal)?
            .mag)
    }

    fn v_net(&self) -> OpenFMBResult<f64> {
        Ok(self
            .ess_reading
            .as_ref()
            .context(NoEssReading)?
            .reading_mmxu
            .as_ref()
            .context(NoReadingMmxu)?
            .ph_v
            .as_ref()
            .context(NoW)?
            .net
            .as_ref()
            .context(NoNet)?
            .c_val
            .as_ref()
            .context(NoCVal)?
            .mag)
    }

    fn a_net(&self) -> OpenFMBResult<f64> {
        Ok(self
            .ess_reading
            .as_ref()
            .context(NoEssReading)?
            .reading_mmxu
            .as_ref()
            .context(NoReadingMmxu)?
            .a
            .as_ref()
            .context(NoValue)?
            .net
            .as_ref()
            .context(NoNet)?
            .c_val
            .as_ref()
            .context(NoCVal)?
            .mag)
    }

    fn pf_net(&self) -> OpenFMBResult<f64> {
        Ok(self
            .ess_reading
            .as_ref()
            .context(NoEssReading)?
            .reading_mmxu
            .as_ref()
            .context(NoReadingMmxu)?
            .pf
            .as_ref()
            .context(NoW)?
            .net
            .as_ref()
            .context(NoNet)?
            .c_val
            .as_ref()
            .context(NoCVal)?
            .mag)
    }

    fn va_net(&self) -> OpenFMBResult<f64> {
        Ok(self
            .ess_reading
            .as_ref()
            .context(NoEssReading)?
            .reading_mmxu
            .as_ref()
            .context(NoReadingMmxu)?
            .va
            .as_ref()
            .context(NoW)?
            .net
            .as_ref()
            .context(NoNet)?
            .c_val
            .as_ref()
            .context(NoCVal)?
            .mag)
    }

    fn freq(&self) -> OpenFMBResult<f64> {
        Ok(self
            .ess_reading
            .as_ref()
            .context(NoEssReading)?
            .reading_mmxu
            .as_ref()
            .context(NoReadingMmxu)?
            .hz
            .as_ref()
            .context(NoValue)?
            .mag)
    }
}

impl EssReadingExt for EssReadingProfile {}

impl OpenFMBReading for EssReadingProfile {}
