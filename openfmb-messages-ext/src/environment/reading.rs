// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use environmentmodule::EnvironmentReadingProfile;
use openfmb_messages::{
    commonmodule::{MessageInfo, ReadingMessageInfo},
    *,
};
use snafu::{OptionExt, ResultExt};
use uuid::Uuid;

use crate::{error::*, OpenFMBExt, OpenFMBExtReading, ReadingProfileExt};

impl OpenFMBExt for EnvironmentReadingProfile {
    fn device_state(&self) -> OpenFMBResult<String> {
        Ok("".to_string())
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
        Ok("EnvironmentReadingProfile".to_string())
    }

    fn device_mrid(&self) -> OpenFMBResult<Uuid> {
        Ok(Uuid::from_str(
            &self
                .sensor
                .as_ref()
                .context(NoMeter)?
                .conducting_equipment
                .as_ref()
                .context(NoConductingEquipment)?
                .m_rid,
        )
        .context(UuidError)?)
    }

    fn device_name(&self) -> OpenFMBResult<String> {
        Ok(self
            .sensor
            .as_ref()
            .context(NoMeter)?
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

impl OpenFMBExtReading for EnvironmentReadingProfile {
    fn reading_message_info(&self) -> OpenFMBResult<&ReadingMessageInfo> {
        Ok(self
            .reading_message_info
            .as_ref()
            .context(NoStatusMessageInfo)?)
    }
}

pub trait EnvironmentReadingExt {
    // MENV
    fn co2_em(&self) -> OpenFMBResult<f64>;
    fn co_em(&self) -> OpenFMBResult<f64>;
    fn dust(&self) -> OpenFMBResult<f64>;
    fn n_ox_em(&self) -> OpenFMBResult<f64>;
    fn o2_cmbu_gas(&self) -> OpenFMBResult<f64>;
    fn o3_air(&self) -> OpenFMBResult<f64>;
    fn snd(&self) -> OpenFMBResult<f64>;
    fn s_ox_em(&self) -> OpenFMBResult<f64>;

    // MMET
    fn env_tmp(&self) -> OpenFMBResult<f64>;
    fn env_hum(&self) -> OpenFMBResult<f64>;
    fn env_pres(&self) -> OpenFMBResult<f64>;
}

impl EnvironmentReadingExt for EnvironmentReadingProfile {
    fn co2_em(&self) -> OpenFMBResult<f64> {
        Ok(self
            .environment_reading
            .as_ref()
            .context(NoValue)?
            .reading_menv
            .as_ref()
            .context(NoValue)?
            .co2_em
            .as_ref()
            .context(NoValue)?
            .mag)
    }

    fn co_em(&self) -> OpenFMBResult<f64> {
        Ok(self
            .environment_reading
            .as_ref()
            .context(NoValue)?
            .reading_menv
            .as_ref()
            .context(NoValue)?
            .co_em
            .as_ref()
            .context(NoValue)?
            .mag)
    }

    fn dust(&self) -> OpenFMBResult<f64> {
        Ok(self
            .environment_reading
            .as_ref()
            .context(NoValue)?
            .reading_menv
            .as_ref()
            .context(NoValue)?
            .dust
            .as_ref()
            .context(NoValue)?
            .mag)
    }

    fn n_ox_em(&self) -> OpenFMBResult<f64> {
        Ok(self
            .environment_reading
            .as_ref()
            .context(NoValue)?
            .reading_menv
            .as_ref()
            .context(NoValue)?
            .n_ox_em
            .as_ref()
            .context(NoValue)?
            .mag)
    }

    fn o2_cmbu_gas(&self) -> OpenFMBResult<f64> {
        Ok(self
            .environment_reading
            .as_ref()
            .context(NoValue)?
            .reading_menv
            .as_ref()
            .context(NoValue)?
            .o2_cmbu_gas
            .as_ref()
            .context(NoValue)?
            .mag)
    }

    fn o3_air(&self) -> OpenFMBResult<f64> {
        Ok(self
            .environment_reading
            .as_ref()
            .context(NoValue)?
            .reading_menv
            .as_ref()
            .context(NoValue)?
            .o3_air
            .as_ref()
            .context(NoValue)?
            .mag)
    }

    fn snd(&self) -> OpenFMBResult<f64> {
        Ok(self
            .environment_reading
            .as_ref()
            .context(NoValue)?
            .reading_menv
            .as_ref()
            .context(NoValue)?
            .snd
            .as_ref()
            .context(NoValue)?
            .mag)
    }

    fn s_ox_em(&self) -> OpenFMBResult<f64> {
        Ok(self
            .environment_reading
            .as_ref()
            .context(NoValue)?
            .reading_menv
            .as_ref()
            .context(NoValue)?
            .s_ox_em
            .as_ref()
            .context(NoValue)?
            .mag)
    }

    // MMET

    fn env_tmp(&self) -> OpenFMBResult<f64> {
        Ok(self
            .environment_reading
            .as_ref()
            .context(NoValue)?
            .reading_mmet
            .as_ref()
            .context(NoValue)?
            .env_tmp
            .as_ref()
            .context(NoValue)?
            .mag)
    }

    fn env_hum(&self) -> OpenFMBResult<f64> {
        Ok(self
            .environment_reading
            .as_ref()
            .context(NoValue)?
            .reading_mmet
            .as_ref()
            .context(NoValue)?
            .env_hum
            .as_ref()
            .context(NoValue)?
            .mag)
    }

    fn env_pres(&self) -> OpenFMBResult<f64> {
        Ok(self
            .environment_reading
            .as_ref()
            .context(NoValue)?
            .reading_mmet
            .as_ref()
            .context(NoValue)?
            .env_pres
            .as_ref()
            .context(NoValue)?
            .mag)
    }
}
