// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

use snafu::{OptionExt, ResultExt};
use std::str::FromStr;
use uuid::Uuid;

use openfmb_messages::{commonmodule::*, *};
use resourcemodule::ResourceReadingProfile;

use crate::{error::*, OpenFMBExt, OpenFMBExtReading, ReadingProfileExt};

impl OpenFMBExtReading for ResourceReadingProfile {
    fn reading_message_info(&self) -> OpenFMBResult<&ReadingMessageInfo> {
        Ok(self
            .reading_message_info
            .as_ref()
            .context(NoReadingMessageInfo)?)
    }
}

impl OpenFMBExt for ResourceReadingProfile {
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
        Ok("ResourceReadingProfile".to_string())
    }

    fn device_mrid(&self) -> OpenFMBResult<Uuid> {
        Ok(Uuid::from_str(
            &self
                .conducting_equipment
                .as_ref()
                .context(NoConductingEquipment)?
                .m_rid,
        )
        .context(UuidError)?)
    }

    fn device_name(&self) -> OpenFMBResult<String> {
        Ok(self
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

impl ReadingProfileExt for ResourceReadingProfile {
    fn w_net(&self) -> OpenFMBResult<f64> {
        Ok(self
            .resource_reading
            .as_ref()
            .context(NoMeterReading)?
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
            .resource_reading
            .as_ref()
            .context(NoMeterReading)?
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

    fn s_net(&self) -> OpenFMBResult<f64> {
        Ok(self
            .resource_reading
            .as_ref()
            .context(NoMeterReading)?
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

    fn v_net(&self) -> OpenFMBResult<f64> {
        Ok(self
            .resource_reading
            .as_ref()
            .context(NoMeterReading)?
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
            .mag)
    }

    fn a_net(&self) -> OpenFMBResult<f64> {
        Ok(self
            .resource_reading
            .as_ref()
            .context(NoMeterReading)?
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
            .resource_reading
            .as_ref()
            .context(NoMeterReading)?
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
            .mag)
    }

    fn freq(&self) -> OpenFMBResult<f64> {
        Ok(self
            .resource_reading
            .as_ref()
            .context(NoMeterReading)?
            .reading_mmxu
            .as_ref()
            .context(NoReadingMmxu)?
            .hz
            .as_ref()
            .context(NoValue)?
            .mag)
    }

    fn va_net(&self) -> OpenFMBResult<f64> {
        Ok(self
            .resource_reading
            .as_ref()
            .context(NoMeterReading)?
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
            .mag)
    }
}
