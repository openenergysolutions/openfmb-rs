use std::str::FromStr;

use openfmb_messages::{
    commonmodule::{LogicalNodeForEventAndStatus, MessageInfo},
    essmodule::{EssReadingProfile, EssStatusZbat, EssStatusZgen},
};
use snafu::{OptionExt, ResultExt};
use uuid::Uuid;

use crate::{common::*, error::*, OpenFMBExt, ReadingProfileExt};

impl HasLogicalNodeForEventAndStatus for EssStatusZbat {
    fn get_logical_node_for_event_and_status(
        &self,
    ) -> OpenFMBResult<&LogicalNodeForEventAndStatus> {
        Ok(self
            .logical_node_for_event_and_status
            .as_ref()
            .context(NoLogicalNodeForEventAndStatus)?)
    }
}

impl HasLogicalNodeForEventAndStatus for EssStatusZgen {
    fn get_logical_node_for_event_and_status(
        &self,
    ) -> OpenFMBResult<&LogicalNodeForEventAndStatus> {
        Ok(self
            .e_ss_event_and_status_zgen
            .as_ref()
            .context(NoEssEventAndStatusZGen)?
            .logical_node_for_event_and_status
            .as_ref()
            .context(NoLogicalNodeForEventAndStatus)?)
    }
}

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

pub trait EssReadingExt: ReadingProfileExt {
    fn ess_reading(&self) -> OpenFMBResult<f32>;
}

impl EssReadingExt for EssReadingProfile {
    fn ess_reading(&self) -> OpenFMBResult<f32> {
        {
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
                .mag
                .as_ref()
                .context(NoMag)?
                .f
                .context(NoF)?)
        }
    }
}

impl ReadingProfileExt for EssReadingProfile {}
