// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use openfmb_messages::{
    commonmodule::{MessageInfo, ReadingMessageInfo},
    evsemodule::EvseReadingProfile,
};
use snafu::{OptionExt, ResultExt};
use uuid::Uuid;

use crate::{error::*, OpenFMBExt, OpenFMBExtReading, OpenFMBReading, ReadingProfileExt};

impl OpenFMBExt for EvseReadingProfile {
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
                .evse
                .as_ref()
                .context(NoValue)?
                .conducting_equipment
                .as_ref()
                .context(NoConductingEquipment)?
                .m_rid,
        )
        .context(UuidError)?)
    }

    fn device_name(&self) -> OpenFMBResult<String> {
        Ok(self
            .evse
            .as_ref()
            .context(NoValue)?
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

impl OpenFMBExtReading for EvseReadingProfile {
    fn reading_message_info(&self) -> OpenFMBResult<&ReadingMessageInfo> {
        Ok(self
            .reading_message_info
            .as_ref()
            .context(NoStatusMessageInfo)?)
    }
}

pub trait EvseReadingExt: ReadingProfileExt {}

impl ReadingProfileExt for EvseReadingProfile {
    fn w_net(&self) -> OpenFMBResult<f64> {
        Ok(self
            .evse_reading
            .as_ref()
            .context(NoValue)?
            .reading_mmxu
            .as_ref()
            .context(NoValue)?
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

    fn w_phs_a(&self) -> OpenFMBResult<f64> {
        Ok(self
            .evse_reading
            .as_ref()
            .context(NoValue)?
            .reading_mmxu
            .as_ref()
            .context(NoValue)?
            .w
            .as_ref()
            .context(NoW)?
            .phs_a
            .as_ref()
            .context(NoPhsA)?
            .c_val
            .as_ref()
            .context(NoCVal)?
            .mag)
    }

    fn w_phs_b(&self) -> OpenFMBResult<f64> {
        Ok(self
            .evse_reading
            .as_ref()
            .context(NoValue)?
            .reading_mmxu
            .as_ref()
            .context(NoValue)?
            .w
            .as_ref()
            .context(NoW)?
            .phs_b
            .as_ref()
            .context(NoPhsB)?
            .c_val
            .as_ref()
            .context(NoCVal)?
            .mag)
    }

    fn w_phs_c(&self) -> OpenFMBResult<f64> {
        Ok(self
            .evse_reading
            .as_ref()
            .context(NoValue)?
            .reading_mmxu
            .as_ref()
            .context(NoValue)?
            .w
            .as_ref()
            .context(NoW)?
            .phs_c
            .as_ref()
            .context(NoPhsC)?
            .c_val
            .as_ref()
            .context(NoCVal)?
            .mag)
    }

    fn q_net(&self) -> OpenFMBResult<f64> {
        Ok(self
            .evse_reading
            .as_ref()
            .context(NoValue)?
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

    fn q_phs_a(&self) -> OpenFMBResult<f64> {
        Ok(self
            .evse_reading
            .as_ref()
            .context(NoValue)?
            .reading_mmxu
            .as_ref()
            .context(NoReadingMmxu)?
            .v_ar
            .as_ref()
            .context(NoW)?
            .phs_a
            .as_ref()
            .context(NoPhsA)?
            .c_val
            .as_ref()
            .context(NoCVal)?
            .mag)
    }

    fn q_phs_b(&self) -> OpenFMBResult<f64> {
        Ok(self
            .evse_reading
            .as_ref()
            .context(NoValue)?
            .reading_mmxu
            .as_ref()
            .context(NoReadingMmxu)?
            .v_ar
            .as_ref()
            .context(NoW)?
            .phs_b
            .as_ref()
            .context(NoPhsB)?
            .c_val
            .as_ref()
            .context(NoCVal)?
            .mag)
    }

    fn q_phs_c(&self) -> OpenFMBResult<f64> {
        Ok(self
            .evse_reading
            .as_ref()
            .context(NoValue)?
            .reading_mmxu
            .as_ref()
            .context(NoReadingMmxu)?
            .v_ar
            .as_ref()
            .context(NoW)?
            .phs_c
            .as_ref()
            .context(NoPhsC)?
            .c_val
            .as_ref()
            .context(NoCVal)?
            .mag)
    }

    fn s_net(&self) -> OpenFMBResult<f64> {
        Ok(self
            .evse_reading
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

    fn s_phs_a(&self) -> OpenFMBResult<f64> {
        Ok(self
            .evse_reading
            .as_ref()
            .context(NoEssReading)?
            .reading_mmxu
            .as_ref()
            .context(NoReadingMmxu)?
            .va
            .as_ref()
            .context(NoW)?
            .phs_a
            .as_ref()
            .context(NoPhsA)?
            .c_val
            .as_ref()
            .context(NoCVal)?
            .mag)
    }

    fn s_phs_b(&self) -> OpenFMBResult<f64> {
        Ok(self
            .evse_reading
            .as_ref()
            .context(NoEssReading)?
            .reading_mmxu
            .as_ref()
            .context(NoReadingMmxu)?
            .va
            .as_ref()
            .context(NoW)?
            .phs_b
            .as_ref()
            .context(NoPhsB)?
            .c_val
            .as_ref()
            .context(NoCVal)?
            .mag)
    }

    fn s_phs_c(&self) -> OpenFMBResult<f64> {
        Ok(self
            .evse_reading
            .as_ref()
            .context(NoEssReading)?
            .reading_mmxu
            .as_ref()
            .context(NoReadingMmxu)?
            .va
            .as_ref()
            .context(NoW)?
            .phs_c
            .as_ref()
            .context(NoPhsC)?
            .c_val
            .as_ref()
            .context(NoCVal)?
            .mag)
    }

    fn v_net(&self) -> OpenFMBResult<f64> {
        Ok(self
            .evse_reading
            .as_ref()
            .context(NoValue)?
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

    fn v_phs_a(&self) -> OpenFMBResult<f64> {
        Ok(self
            .evse_reading
            .as_ref()
            .context(NoValue)?
            .reading_mmxu
            .as_ref()
            .context(NoReadingMmxu)?
            .ph_v
            .as_ref()
            .context(NoW)?
            .phs_a
            .as_ref()
            .context(NoPhsA)?
            .c_val
            .as_ref()
            .context(NoCVal)?
            .mag)
    }

    fn v_phs_b(&self) -> OpenFMBResult<f64> {
        Ok(self
            .evse_reading
            .as_ref()
            .context(NoValue)?
            .reading_mmxu
            .as_ref()
            .context(NoReadingMmxu)?
            .ph_v
            .as_ref()
            .context(NoW)?
            .phs_b
            .as_ref()
            .context(NoPhsB)?
            .c_val
            .as_ref()
            .context(NoCVal)?
            .mag)
    }

    fn v_phs_c(&self) -> OpenFMBResult<f64> {
        Ok(self
            .evse_reading
            .as_ref()
            .context(NoValue)?
            .reading_mmxu
            .as_ref()
            .context(NoReadingMmxu)?
            .ph_v
            .as_ref()
            .context(NoW)?
            .phs_c
            .as_ref()
            .context(NoPhsC)?
            .c_val
            .as_ref()
            .context(NoCVal)?
            .mag)
    }

    fn a_net(&self) -> OpenFMBResult<f64> {
        Ok(self
            .evse_reading
            .as_ref()
            .context(NoValue)?
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

    fn a_phs_a(&self) -> OpenFMBResult<f64> {
        Ok(self
            .evse_reading
            .as_ref()
            .context(NoValue)?
            .reading_mmxu
            .as_ref()
            .context(NoReadingMmxu)?
            .a
            .as_ref()
            .context(NoValue)?
            .phs_a
            .as_ref()
            .context(NoPhsA)?
            .c_val
            .as_ref()
            .context(NoCVal)?
            .mag)
    }

    fn a_phs_b(&self) -> OpenFMBResult<f64> {
        Ok(self
            .evse_reading
            .as_ref()
            .context(NoValue)?
            .reading_mmxu
            .as_ref()
            .context(NoReadingMmxu)?
            .a
            .as_ref()
            .context(NoValue)?
            .phs_b
            .as_ref()
            .context(NoPhsB)?
            .c_val
            .as_ref()
            .context(NoCVal)?
            .mag)
    }

    fn a_phs_c(&self) -> OpenFMBResult<f64> {
        Ok(self
            .evse_reading
            .as_ref()
            .context(NoValue)?
            .reading_mmxu
            .as_ref()
            .context(NoReadingMmxu)?
            .a
            .as_ref()
            .context(NoValue)?
            .phs_c
            .as_ref()
            .context(NoPhsC)?
            .c_val
            .as_ref()
            .context(NoCVal)?
            .mag)
    }

    fn pf_net(&self) -> OpenFMBResult<f64> {
        Ok(self
            .evse_reading
            .as_ref()
            .context(NoValue)?
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

    fn freq(&self) -> OpenFMBResult<f64> {
        Ok(self
            .evse_reading
            .as_ref()
            .context(NoValue)?
            .reading_mmxu
            .as_ref()
            .context(NoReadingMmxu)?
            .hz
            .as_ref()
            .context(NoValue)?
            .mag)
    }
}

impl EvseReadingExt for EvseReadingProfile {}

impl OpenFMBReading for EvseReadingProfile {}
