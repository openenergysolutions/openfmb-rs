use crate::{error::*, OpenFMBExt, OpenFMBExtReading, ReadingProfileExt};
use openfmb_messages::{
    commonmodule::{MessageInfo, ReadingMessageInfo},
    switchmodule::SwitchReadingProfile,
};
use snafu::{OptionExt, ResultExt};
use std::str::FromStr;
use uuid::Uuid;

impl OpenFMBExt for SwitchReadingProfile {
    fn device_state(&self) -> OpenFMBResult<String> {
        Ok(self
            .switch_reading
            .first()
            .unwrap()
            .reading_mmxu
            .clone()
            .unwrap()
            .w
            .unwrap()
            .net
            .unwrap()
            .c_val
            .unwrap()
            .mag
            .to_string())
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
        Ok("SwitchReadingProfile".to_string())
    }

    fn device_mrid(&self) -> OpenFMBResult<Uuid> {
        Ok(Uuid::from_str(
            &self
                .protected_switch
                .clone()
                .context(NoGeneratingUnit)?
                .conducting_equipment
                .context(NoConductingEquipment)?
                .m_rid,
        )
        .context(UuidError)?)
    }

    fn device_name(&self) -> OpenFMBResult<String> {
        Ok(self
            .protected_switch
            .clone()
            .context(NoProtectedSwitch)?
            .conducting_equipment
            .context(NoConductingEquipment)?
            .named_object
            .context(NoNamedObject)?
            .name
            .context(NoName)?)
    }
}

impl OpenFMBExtReading for SwitchReadingProfile {
    fn reading_message_info(&self) -> OpenFMBResult<&ReadingMessageInfo> {
        Ok(self
            .reading_message_info
            .as_ref()
            .context(NoStatusMessageInfo)?)
    }
}

pub trait SwitchReadingExt: ReadingProfileExt {
    fn switch_reading(&self) -> f64;
    fn get_current_phsa(&self) -> f64;
    fn get_current_phsb(&self) -> f64;
    fn get_current_phsc(&self) -> f64;
    fn get_ph_v(&self) -> &openfmb_messages::commonmodule::Wye;
    fn get_ppv(&self) -> &openfmb_messages::commonmodule::Del;
    fn get_freq(&self, side: u32) -> f64;
}

impl SwitchReadingExt for SwitchReadingProfile {
    fn switch_reading(&self) -> f64 {
        self.switch_reading
            .first()
            .unwrap()
            .reading_mmxu
            .as_ref()
            .unwrap()
            .w
            .as_ref()
            .unwrap()
            .net
            .as_ref()
            .unwrap()
            .c_val
            .as_ref()
            .unwrap()
            .mag
    }
    fn get_current_phsa(&self) -> f64 {
        self.switch_reading
            .first()
            .unwrap()
            .reading_mmxu
            .as_ref()
            .unwrap()
            .a
            .as_ref()
            .unwrap()
            .phs_a
            .as_ref()
            .unwrap()
            .c_val
            .as_ref()
            .unwrap()
            .mag
    }
    fn get_current_phsb(&self) -> f64 {
        self.switch_reading
            .first()
            .unwrap()
            .reading_mmxu
            .as_ref()
            .unwrap()
            .a
            .as_ref()
            .unwrap()
            .phs_b
            .as_ref()
            .unwrap()
            .c_val
            .as_ref()
            .unwrap()
            .mag
    }
    fn get_current_phsc(&self) -> f64 {
        self.switch_reading
            .first()
            .unwrap()
            .reading_mmxu
            .as_ref()
            .unwrap()
            .a
            .as_ref()
            .unwrap()
            .phs_c
            .as_ref()
            .unwrap()
            .c_val
            .as_ref()
            .unwrap()
            .mag
    }
    fn get_freq(&self, side: u32) -> f64 {
        if side == 0 {
            self.switch_reading
                .first()
                .unwrap()
                .reading_mmxu
                .as_ref()
                .unwrap()
                .hz
                .as_ref()
                .unwrap()
                .mag
        } else {
            self.switch_reading[1]                            
                .reading_mmxu
                .as_ref()
                .unwrap()
                .hz
                .as_ref()
                .unwrap()
                .mag
        }
            
    }
    fn get_ph_v(&self) -> &openfmb_messages::commonmodule::Wye {
        self.switch_reading
            .first()
            .unwrap()
            .reading_mmxu
            .as_ref()
            .unwrap()
            .ph_v
            .as_ref()
            .unwrap()
    }
    fn get_ppv(&self) -> &openfmb_messages::commonmodule::Del {
        self.switch_reading
            .first()
            .unwrap()
            .reading_mmxu
            .as_ref()
            .unwrap()
            .ppv
            .as_ref()
            .unwrap()
    }
}

impl ReadingProfileExt for SwitchReadingProfile {}
