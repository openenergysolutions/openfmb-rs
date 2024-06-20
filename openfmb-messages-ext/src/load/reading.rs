// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use openfmb_messages::{
    commonmodule::{MessageInfo, ReadingMessageInfo},
    loadmodule::LoadReadingProfile,
};
use snafu::{OptionExt, ResultExt};
use uuid::Uuid;

use crate::{error::*, OpenFMBExt, OpenFMBExtReading, ReadingProfileExt};

impl OpenFMBExt for LoadReadingProfile {
    fn device_state(&self) -> OpenFMBResult<String> {
        Ok("".to_string())
        //panic!("{:?}", self);
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
        Ok("LoadReadingProfile".to_string())
    }

    fn device_mrid(&self) -> OpenFMBResult<Uuid> {
        Ok(Uuid::from_str(
            &self
                .energy_consumer
                .as_ref()
                .context(NoEnergyConsumer)?
                .conducting_equipment
                .as_ref()
                .context(NoConductingEquipment)?
                .m_rid,
        )
        .context(UuidError)?)
    }

    fn device_name(&self) -> OpenFMBResult<String> {
        Ok(self
            .energy_consumer
            .as_ref()
            .context(NoEnergyConsumer)?
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

impl OpenFMBExtReading for LoadReadingProfile {
    fn reading_message_info(&self) -> OpenFMBResult<&ReadingMessageInfo> {
        Ok(self
            .reading_message_info
            .as_ref()
            .context(NoStatusMessageInfo)?)
    }
}

pub trait LoadReadingExt: ReadingProfileExt {
    fn v_phs_a(&self) -> OpenFMBResult<f64>;
    fn v_phs_b(&self) -> OpenFMBResult<f64>;
    fn v_phs_c(&self) -> OpenFMBResult<f64>;
}

impl ReadingProfileExt for LoadReadingProfile {
    fn w_net(&self) -> OpenFMBResult<f64> {
        Ok(self
            .load_reading
            .as_ref()
            .context(NoLoadReading)?
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
            .load_reading
            .as_ref()
            .context(NoLoadReading)?
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
        return Ok(self
            .load_reading
            .as_ref()
            .context(NoLoadReading)?
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

    fn a_net(&self) -> OpenFMBResult<f64> {
        return Ok(self
            .load_reading
            .as_ref()
            .context(NoLoadReading)?
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

    fn pf_net(&self) -> OpenFMBResult<f64> {
        return Ok(self
            .load_reading
            .as_ref()
            .context(NoLoadReading)?
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

    fn va_net(&self) -> OpenFMBResult<f64> {
        return Ok(self
            .load_reading
            .as_ref()
            .context(NoLoadReading)?
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

    fn freq(&self) -> OpenFMBResult<f64> {
        return Ok(self
            .load_reading
            .as_ref()
            .context(NoLoadReading)?
            .reading_mmxu
            .as_ref()
            .context(NoReadingMmxu)?
            .hz
            .as_ref()
            .context(NoValue)?
            .mag);
    }
}

impl LoadReadingExt for LoadReadingProfile {
    fn v_phs_a(&self) -> OpenFMBResult<f64> {
        return Ok(self
            .load_reading
            .as_ref()
            .context(NoLoadReading)?
            .reading_mmxu
            .as_ref()
            .context(NoReadingMmxu)?
            .ph_v
            .as_ref()
            .context(NoValue)?
            .phs_a
            .as_ref()
            .context(NoNet)?
            .c_val
            .as_ref()
            .context(NoCVal)?
            .mag);
    }

    fn v_phs_b(&self) -> OpenFMBResult<f64> {
        return Ok(self
            .load_reading
            .as_ref()
            .context(NoLoadReading)?
            .reading_mmxu
            .as_ref()
            .context(NoReadingMmxu)?
            .ph_v
            .as_ref()
            .context(NoValue)?
            .phs_b
            .as_ref()
            .context(NoPhsB)?
            .c_val
            .as_ref()
            .context(NoCVal)?
            .mag);
    }

    fn v_phs_c(&self) -> OpenFMBResult<f64> {
        return Ok(self
            .load_reading
            .as_ref()
            .context(NoLoadReading)?
            .reading_mmxu
            .as_ref()
            .context(NoReadingMmxu)?
            .ph_v
            .as_ref()
            .context(NoValue)?
            .phs_c
            .as_ref()
            .context(NoPhsC)?
            .c_val
            .as_ref()
            .context(NoCVal)?
            .mag);
    }
}
