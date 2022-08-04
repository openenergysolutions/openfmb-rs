// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use breakermodule::BreakerReadingProfile;
use openfmb_messages::{
    commonmodule::{MessageInfo, ReadingMessageInfo},
    *,
};
use snafu::{OptionExt, ResultExt};
use uuid::Uuid;

use crate::{error::*, OpenFMBExt, OpenFMBExtReading, ReadingProfileExt};

impl OpenFMBExt for BreakerReadingProfile {
    fn device_state(&self) -> OpenFMBResult<String> {
        if !self.breaker_reading.is_empty() {
            return Ok(self
                .breaker_reading
                .first()
                .as_ref()
                .context(NoBreakerReading)?
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
            .context(NoReadingMessageInfo)?
            .message_info
            .as_ref()
            .context(NoMessageInfo)?)
    }

    fn message_type(&self) -> OpenFMBResult<String> {
        Ok("BreakerReadingProfile".to_string())
    }

    fn device_mrid(&self) -> OpenFMBResult<Uuid> {
        Ok(Uuid::from_str(
            &self
                .breaker
                .as_ref()
                .context(NoBreaker)?
                .conducting_equipment
                .as_ref()
                .context(NoConductingEquipment)?
                .m_rid,
        )
        .context(UuidError)?)
    }

    fn device_name(&self) -> OpenFMBResult<String> {
        Ok(self
            .breaker
            .as_ref()
            .context(NoBreaker)?
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

impl OpenFMBExtReading for BreakerReadingProfile {
    fn reading_message_info(&self) -> OpenFMBResult<&ReadingMessageInfo> {
        Ok(self
            .reading_message_info
            .as_ref()
            .context(NoStatusMessageInfo)?)
    }
}

pub trait BreakerReadingExt: ReadingProfileExt {
    fn w_net(&self) -> OpenFMBResult<f64>;
    fn w_net_load_side(&self) -> OpenFMBResult<f64>;

    fn q_net(&self) -> OpenFMBResult<f64>;
    fn q_net_load_side(&self) -> OpenFMBResult<f64>;

    fn v_net(&self) -> OpenFMBResult<f64>;
    fn v_net_load_side(&self) -> OpenFMBResult<f64>;

    fn a_net(&self) -> OpenFMBResult<f64>;
    fn a_net_load_side(&self) -> OpenFMBResult<f64>;
}

impl BreakerReadingExt for BreakerReadingProfile {
    fn w_net(&self) -> OpenFMBResult<f64> {
        if !self.breaker_reading.is_empty() {
            return Ok(self
                .breaker_reading
                .first()
                .as_ref()
                .context(NoBreakerReading)?
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
        Err(OpenFMBError::NoBreakerReading)
    }

    fn w_net_load_side(&self) -> OpenFMBResult<f64> {
        if !self.breaker_reading.is_empty() || self.breaker_reading.len() < 2 {
            return Ok(self
                .breaker_reading
                .get(1)
                .as_ref()
                .context(NoBreakerReading)?
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
        Err(OpenFMBError::NoBreakerReading)
    }

    fn q_net(&self) -> OpenFMBResult<f64> {
        if !self.breaker_reading.is_empty() {
            return Ok(self
                .breaker_reading
                .first()
                .as_ref()
                .context(NoBreakerReading)?
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
        Err(OpenFMBError::NoBreakerReading)
    }

    fn q_net_load_side(&self) -> OpenFMBResult<f64> {
        if !self.breaker_reading.is_empty() || self.breaker_reading.len() < 2 {
            return Ok(self
                .breaker_reading
                .get(1)
                .as_ref()
                .context(NoBreakerReading)?
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
        Err(OpenFMBError::NoBreakerReading)
    }

    fn v_net(&self) -> OpenFMBResult<f64> {
        if !self.breaker_reading.is_empty() {
            return Ok(self
                .breaker_reading
                .first()
                .as_ref()
                .context(NoBreakerReading)?
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
        Err(OpenFMBError::NoBreakerReading)
    }

    fn v_net_load_side(&self) -> OpenFMBResult<f64> {
        if !self.breaker_reading.is_empty() || self.breaker_reading.len() < 2 {
            return Ok(self
                .breaker_reading
                .get(1)
                .as_ref()
                .context(NoBreakerReading)?
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
        Err(OpenFMBError::NoBreakerReading)
    }

    fn a_net(&self) -> OpenFMBResult<f64> {
        if !self.breaker_reading.is_empty() {
            return Ok(self
                .breaker_reading
                .first()
                .as_ref()
                .context(NoBreakerReading)?
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
        Err(OpenFMBError::NoBreakerReading)
    }

    fn a_net_load_side(&self) -> OpenFMBResult<f64> {
        if !self.breaker_reading.is_empty() || self.breaker_reading.len() < 2 {
            return Ok(self
                .breaker_reading
                .get(1)
                .as_ref()
                .context(NoBreakerReading)?
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
        Err(OpenFMBError::NoBreakerReading)
    }
}

impl ReadingProfileExt for BreakerReadingProfile {}
