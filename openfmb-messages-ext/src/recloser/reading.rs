// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

use snafu::{OptionExt, ResultExt};
use std::str::FromStr;
use uuid::Uuid;

use openfmb_messages::{commonmodule::*, *};
use reclosermodule::RecloserReadingProfile;

use crate::{error::*, OpenFMBExt, OpenFMBExtReading, ReadingProfileExt};

impl OpenFMBExt for RecloserReadingProfile {
    fn device_state(&self) -> OpenFMBResult<String> {
        Ok("".into())
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
        Ok("RecloserReadingProfile".to_string())
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

impl OpenFMBExtReading for RecloserReadingProfile {
    fn reading_message_info(&self) -> OpenFMBResult<&ReadingMessageInfo> {
        Ok(self
            .reading_message_info
            .as_ref()
            .context(NoStatusMessageInfo)?)
    }
}



impl ReadingProfileExt for RecloserReadingProfile {
    fn w_net(&self) -> OpenFMBResult<f64> {
        if !self.recloser_reading.is_empty() {
            return Ok(self
                .recloser_reading
                .first()
                .as_ref()
                .context(NoRecloserReading)?
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
                .mag);
        }
        Err(OpenFMBError::NoRecloserReading)
    }

    fn q_net(&self) -> OpenFMBResult<f64> {
        if !self.recloser_reading.is_empty() {
            return Ok(self
                .recloser_reading
                .first()
                .as_ref()
                .context(NoRecloserReading)?
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
                .mag);
        }
        Err(OpenFMBError::NoRecloserReading)
    }

    fn w_net_load_side(&self) -> OpenFMBResult<f64> {
        if !self.recloser_reading.is_empty() || self.recloser_reading.len() < 2 {
            return Ok(self
                .recloser_reading
                .get(1)
                .as_ref()
                .context(NoRecloserReading)?
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
                .mag);
        }
        Err(OpenFMBError::NoRecloserReading)
    }

    fn q_net_load_side(&self) -> OpenFMBResult<f64> {
        if !self.recloser_reading.is_empty() || self.recloser_reading.len() < 2 {
            return Ok(self
                .recloser_reading
                .get(1)
                .as_ref()
                .context(NoRecloserReading)?
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
                .mag);
        }
        Err(OpenFMBError::NoRecloserReading)
    }

    fn v_net(&self) -> OpenFMBResult<f64> {
        if !self.recloser_reading.is_empty() {
            return Ok(self
                .recloser_reading
                .first()
                .as_ref()
                .context(NoRecloserReading)?
                .reading_mmxu
                .as_ref()
                .context(NoReadingMmxu)?
                .ph_v
                .as_ref()
                .context(NoValue)?
                .net
                .as_ref()
                .context(NoNet)?
                .c_val
                .as_ref()
                .context(NoCVal)?
                .mag);
        }
        Err(OpenFMBError::NoRecloserReading)
    }

    fn v_net_load_side(&self) -> OpenFMBResult<f64> {
        if !self.recloser_reading.is_empty() || self.recloser_reading.len() < 2 {
            return Ok(self
                .recloser_reading
                .get(1)
                .as_ref()
                .context(NoRecloserReading)?
                .reading_mmxu
                .as_ref()
                .context(NoReadingMmxu)?
                .ph_v
                .as_ref()
                .context(NoValue)?
                .net
                .as_ref()
                .context(NoNet)?
                .c_val
                .as_ref()
                .context(NoCVal)?
                .mag);
        }
        Err(OpenFMBError::NoRecloserReading)
    }

    fn a_net(&self) -> OpenFMBResult<f64> {
        if !self.recloser_reading.is_empty() {
            return Ok(self
                .recloser_reading
                .first()
                .as_ref()
                .context(NoRecloserReading)?
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
                .mag);
        }
        Err(OpenFMBError::NoRecloserReading)
    }

    fn a_net_load_side(&self) -> OpenFMBResult<f64> {
        if !self.recloser_reading.is_empty() || self.recloser_reading.len() < 2 {
            return Ok(self
                .recloser_reading
                .get(1)
                .as_ref()
                .context(NoRecloserReading)?
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
                .mag);
        }
        Err(OpenFMBError::NoRecloserReading)
    }
}

