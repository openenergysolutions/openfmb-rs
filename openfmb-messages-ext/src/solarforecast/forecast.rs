// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;
use crate::{error::*, OpenFMBExt};
use openfmb_messages::commonmodule::StateKind;
use openfmb_messages::{
    commonmodule::{MessageInfo},
    solarforecastmodule::SolarForecastProfile,
};
use snafu::{OptionExt, ResultExt};
use uuid::Uuid;
// I think the default defined for OpenFMBExt should work for us
impl OpenFMBExt for SolarForecastProfile {

    fn message_info(&self) -> OpenFMBResult<&MessageInfo> {
        Ok(self
            .message_info
            .as_ref()
            .context(NoMessageInfo)?)
    }

    fn message_type(&self) -> OpenFMBResult<String> {
        Ok("SolarForecastProfile".to_string())
    }

    fn device_mrid(&self) -> OpenFMBResult<Uuid> {
        Ok(Uuid::from_str(
            &self
                .message_info
                .as_ref()
                .context(NoMessageInfo)?
                .identified_object
                .as_ref()
                .context(NoNamedObject)?
                .m_rid
                .clone()
                .context(NoMRID)?
        )
        .context(UuidError)?)
    }

    fn device_name(&self) -> OpenFMBResult<String> {
        Ok(self
            .message_info
            .as_ref()
            .context(NoMessageInfo)?
            .identified_object
            .as_ref()
            .context(NoMessageInfo)?
            .name
            .clone()
            .context(NoName)?
        )
    }


    fn device_state(&self) -> OpenFMBResult<String> {
        Ok("Undefined".into())
    }


}

