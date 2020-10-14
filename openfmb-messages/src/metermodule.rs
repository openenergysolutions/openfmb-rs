/// Resource reading value
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct MeterReading {
    /// UML inherited base object
    // parent_message: true
    // required_field: false
    // multiplicity_min: 0
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="1")]
    pub conducting_equipment_terminal_reading: ::std::option::Option<super::commonmodule::ConductingEquipmentTerminalReading>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub phase_mmtn: ::std::option::Option<super::commonmodule::PhaseMmtn>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub reading_mmtr: ::std::option::Option<super::commonmodule::ReadingMmtr>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="4")]
    pub reading_mmxu: ::std::option::Option<super::commonmodule::ReadingMmxu>,
}
mod meter_reading {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref CONDUCTING_EQUIPMENT_TERMINAL_READING: crate::commonmodule::ConductingEquipmentTerminalReading = Default::default();
        pub(super) static ref PHASE_MMTN: crate::commonmodule::PhaseMmtn = Default::default();
        pub(super) static ref READING_MMTR: crate::commonmodule::ReadingMmtr = Default::default();
        pub(super) static ref READING_MMXU: crate::commonmodule::ReadingMmxu = Default::default();
    }
}
pub trait IsMeterReading {
    fn _meter_reading(&self) -> &MeterReading;
    fn conducting_equipment_terminal_reading(&self) -> &super::commonmodule::ConductingEquipmentTerminalReading {
        self._meter_reading().conducting_equipment_terminal_reading.as_ref().unwrap_or(&meter_reading::CONDUCTING_EQUIPMENT_TERMINAL_READING)
    }
    fn phase_mmtn(&self) -> &super::commonmodule::PhaseMmtn {
        self._meter_reading().phase_mmtn.as_ref().unwrap_or(&meter_reading::PHASE_MMTN)
    }
    fn reading_mmtr(&self) -> &super::commonmodule::ReadingMmtr {
        self._meter_reading().reading_mmtr.as_ref().unwrap_or(&meter_reading::READING_MMTR)
    }
    fn reading_mmxu(&self) -> &super::commonmodule::ReadingMmxu {
        self._meter_reading().reading_mmxu.as_ref().unwrap_or(&meter_reading::READING_MMXU)
    }
}
impl IsMeterReading for MeterReading {
    fn _meter_reading(&self) -> &MeterReading {
        self
    }
}
/// Resource reading profile
/// OpenFMB Profile Message: true
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct MeterReadingProfile {
    /// UML inherited base object
    // parent_message: true
    // required_field: false
    // multiplicity_min: 0
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="1")]
    pub reading_message_info: ::std::option::Option<super::commonmodule::ReadingMessageInfo>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="2")]
    pub ied: ::std::option::Option<super::commonmodule::Ied>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="3")]
    pub meter: ::std::option::Option<super::commonmodule::Meter>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="4")]
    pub meter_reading: ::std::option::Option<MeterReading>,
}
mod meter_reading_profile {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref READING_MESSAGE_INFO: crate::commonmodule::ReadingMessageInfo = Default::default();
        pub(super) static ref IED: crate::commonmodule::Ied = Default::default();
        pub(super) static ref METER: crate::commonmodule::Meter = Default::default();
        pub(super) static ref METER_READING: crate::metermodule::MeterReading = Default::default();
    }
}
pub trait IsMeterReadingProfile {
    fn _meter_reading_profile(&self) -> &MeterReadingProfile;
    fn reading_message_info(&self) -> &super::commonmodule::ReadingMessageInfo {
        self._meter_reading_profile().reading_message_info.as_ref().unwrap_or(&meter_reading_profile::READING_MESSAGE_INFO)
    }
    fn ied(&self) -> &super::commonmodule::Ied {
        self._meter_reading_profile().ied.as_ref().unwrap_or(&meter_reading_profile::IED)
    }
    fn meter(&self) -> &super::commonmodule::Meter {
        self._meter_reading_profile().meter.as_ref().unwrap_or(&meter_reading_profile::METER)
    }
    fn meter_reading(&self) -> &MeterReading {
        self._meter_reading_profile().meter_reading.as_ref().unwrap_or(&meter_reading_profile::METER_READING)
    }
}
impl IsMeterReadingProfile for MeterReadingProfile {
    fn _meter_reading_profile(&self) -> &MeterReadingProfile {
        self
    }
}
