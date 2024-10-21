// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

use snafu::{OptionExt, ResultExt};
use std::str::FromStr;
use uuid::Uuid;

use openfmb_messages::{commonmodule::*, *};
use regulatormodule::RegulatorReadingProfile;

use crate::{error::*, OpenFMBExt, OpenFMBExtReading, ReadingProfileExt};

impl OpenFMBExtReading for RegulatorReadingProfile {
    fn reading_message_info(&self) -> OpenFMBResult<&ReadingMessageInfo> {
        Ok(self
            .reading_message_info
            .as_ref()
            .context(NoReadingMessageInfo)?)
    }
}

impl OpenFMBExt for RegulatorReadingProfile {
    fn device_state(&self) -> OpenFMBResult<String> {
        Ok("".into())
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
        Ok("RegulatorReadingProfile".to_string())
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

impl ReadingProfileExt for RegulatorReadingProfile {
    fn w_net(&self) -> OpenFMBResult<f64> {
        if !self.regulator_reading.is_empty() {
            return Ok(self
                .regulator_reading
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
                .mag);
        }
        Err(OpenFMBError::NoSwitchReading)
    }

    fn q_net(&self) -> OpenFMBResult<f64> {
        if !self.regulator_reading.is_empty() {
            return Ok(self
                .regulator_reading
                .first()
                .as_ref()
                .context(NoSwitchReading)?
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
        Err(OpenFMBError::NoSwitchReading)
    }

    fn s_net(&self) -> OpenFMBResult<f64> {
        if !self.regulator_reading.is_empty() {
            return Ok(self
                .regulator_reading
                .first()
                .as_ref()
                .context(NoSwitchReading)?
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
                .mag);
        }
        Err(OpenFMBError::NoSwitchReading)
    }

    fn w_net_load_side(&self) -> OpenFMBResult<f64> {
        if self.regulator_reading.len() > 1 {
            return Ok(self
                .regulator_reading
                .get(1)
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
                .mag);
        }
        Err(OpenFMBError::NoSwitchReading)
    }

    fn q_net_load_side(&self) -> OpenFMBResult<f64> {
        if self.regulator_reading.len() > 1 {
            return Ok(self
                .regulator_reading
                .get(1)
                .as_ref()
                .context(NoSwitchReading)?
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
        Err(OpenFMBError::NoSwitchReading)
    }

    fn s_net_load_side(&self) -> OpenFMBResult<f64> {
        if self.regulator_reading.len() > 1 {
            return Ok(self
                .regulator_reading
                .get(1)
                .as_ref()
                .context(NoSwitchReading)?
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
                .mag);
        }
        Err(OpenFMBError::NoSwitchReading)
    }

    fn v_net(&self) -> OpenFMBResult<f64> {
        if !self.regulator_reading.is_empty() {
            return Ok(self
                .regulator_reading
                .first()
                .as_ref()
                .context(NoSwitchReading)?
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
        Err(OpenFMBError::NoSwitchReading)
    }

    fn v_net_load_side(&self) -> OpenFMBResult<f64> {
        if self.regulator_reading.len() > 1 {
            return Ok(self
                .regulator_reading
                .get(1)
                .as_ref()
                .context(NoSwitchReading)?
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
        Err(OpenFMBError::NoSwitchReading)
    }

    fn a_net(&self) -> OpenFMBResult<f64> {
        if !self.regulator_reading.is_empty() {
            return Ok(self
                .regulator_reading
                .first()
                .as_ref()
                .context(NoSwitchReading)?
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
        Err(OpenFMBError::NoSwitchReading)
    }

    fn a_net_load_side(&self) -> OpenFMBResult<f64> {
        if self.regulator_reading.len() > 1 {
            return Ok(self
                .regulator_reading
                .get(1)
                .as_ref()
                .context(NoSwitchReading)?
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
        Err(OpenFMBError::NoSwitchReading)
    }

    fn pf_net(&self) -> OpenFMBResult<f64> {
        if !self.regulator_reading.is_empty() {
            return Ok(self
                .regulator_reading
                .first()
                .as_ref()
                .context(NoSwitchReading)?
                .reading_mmxu
                .as_ref()
                .context(NoReadingMmxu)?
                .pf
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
        Err(OpenFMBError::NoSwitchReading)
    }

    fn pf_net_load_side(&self) -> OpenFMBResult<f64> {
        if self.regulator_reading.len() > 1 {
            return Ok(self
                .regulator_reading
                .get(1)
                .as_ref()
                .context(NoSwitchReading)?
                .reading_mmxu
                .as_ref()
                .context(NoReadingMmxu)?
                .pf
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
        Err(OpenFMBError::NoSwitchReading)
    }

    fn va_net(&self) -> OpenFMBResult<f64> {
        if !self.regulator_reading.is_empty() {
            return Ok(self
                .regulator_reading
                .first()
                .as_ref()
                .context(NoSwitchReading)?
                .reading_mmxu
                .as_ref()
                .context(NoReadingMmxu)?
                .va
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
        Err(OpenFMBError::NoSwitchReading)
    }

    fn va_net_load_side(&self) -> OpenFMBResult<f64> {
        if self.regulator_reading.len() > 1 {
            return Ok(self
                .regulator_reading
                .get(1)
                .as_ref()
                .context(NoSwitchReading)?
                .reading_mmxu
                .as_ref()
                .context(NoReadingMmxu)?
                .va
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
        Err(OpenFMBError::NoSwitchReading)
    }

    fn freq(&self) -> OpenFMBResult<f64> {
        if !self.regulator_reading.is_empty() {
            return Ok(self
                .regulator_reading
                .first()
                .as_ref()
                .context(NoSwitchReading)?
                .reading_mmxu
                .as_ref()
                .context(NoReadingMmxu)?
                .hz
                .as_ref()
                .context(NoValue)?
                .mag);
        }
        Err(OpenFMBError::NoSwitchReading)
    }

    fn freq_load_side(&self) -> OpenFMBResult<f64> {
        if !self.regulator_reading.is_empty() || self.regulator_reading.len() < 2 {
            return Ok(self
                .regulator_reading
                .get(1)
                .as_ref()
                .context(NoSwitchReading)?
                .reading_mmxu
                .as_ref()
                .context(NoReadingMmxu)?
                .hz
                .as_ref()
                .context(NoValue)?
                .mag);
        }
        Err(OpenFMBError::NoSwitchReading)
    }
}
