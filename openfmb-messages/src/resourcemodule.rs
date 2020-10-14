/// Resource reading value
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResourceReading {
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
mod resource_reading {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref CONDUCTING_EQUIPMENT_TERMINAL_READING: crate::commonmodule::ConductingEquipmentTerminalReading = Default::default();
        pub(super) static ref PHASE_MMTN: crate::commonmodule::PhaseMmtn = Default::default();
        pub(super) static ref READING_MMTR: crate::commonmodule::ReadingMmtr = Default::default();
        pub(super) static ref READING_MMXU: crate::commonmodule::ReadingMmxu = Default::default();
    }
}
pub trait IsResourceReading {
    fn _resource_reading(&self) -> &ResourceReading;
    fn conducting_equipment_terminal_reading(&self) -> &super::commonmodule::ConductingEquipmentTerminalReading {
        self._resource_reading().conducting_equipment_terminal_reading.as_ref().unwrap_or(&resource_reading::CONDUCTING_EQUIPMENT_TERMINAL_READING)
    }
    fn phase_mmtn(&self) -> &super::commonmodule::PhaseMmtn {
        self._resource_reading().phase_mmtn.as_ref().unwrap_or(&resource_reading::PHASE_MMTN)
    }
    fn reading_mmtr(&self) -> &super::commonmodule::ReadingMmtr {
        self._resource_reading().reading_mmtr.as_ref().unwrap_or(&resource_reading::READING_MMTR)
    }
    fn reading_mmxu(&self) -> &super::commonmodule::ReadingMmxu {
        self._resource_reading().reading_mmxu.as_ref().unwrap_or(&resource_reading::READING_MMXU)
    }
}
impl IsResourceReading for ResourceReading {
    fn _resource_reading(&self) -> &ResourceReading {
        self
    }
}
/// Resource reading profile
/// OpenFMB Profile Message: true
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResourceReadingProfile {
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
    pub conducting_equipment: ::std::option::Option<super::commonmodule::ConductingEquipment>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="3")]
    pub ied: ::std::option::Option<super::commonmodule::Ied>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="4")]
    pub resource_reading: ::std::option::Option<ResourceReading>,
}
mod resource_reading_profile {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref READING_MESSAGE_INFO: crate::commonmodule::ReadingMessageInfo = Default::default();
        pub(super) static ref CONDUCTING_EQUIPMENT: crate::commonmodule::ConductingEquipment = Default::default();
        pub(super) static ref IED: crate::commonmodule::Ied = Default::default();
        pub(super) static ref RESOURCE_READING: crate::resourcemodule::ResourceReading = Default::default();
    }
}
pub trait IsResourceReadingProfile {
    fn _resource_reading_profile(&self) -> &ResourceReadingProfile;
    fn reading_message_info(&self) -> &super::commonmodule::ReadingMessageInfo {
        self._resource_reading_profile().reading_message_info.as_ref().unwrap_or(&resource_reading_profile::READING_MESSAGE_INFO)
    }
    fn conducting_equipment(&self) -> &super::commonmodule::ConductingEquipment {
        self._resource_reading_profile().conducting_equipment.as_ref().unwrap_or(&resource_reading_profile::CONDUCTING_EQUIPMENT)
    }
    fn ied(&self) -> &super::commonmodule::Ied {
        self._resource_reading_profile().ied.as_ref().unwrap_or(&resource_reading_profile::IED)
    }
    fn resource_reading(&self) -> &ResourceReading {
        self._resource_reading_profile().resource_reading.as_ref().unwrap_or(&resource_reading_profile::RESOURCE_READING)
    }
}
impl IsResourceReadingProfile for ResourceReadingProfile {
    fn _resource_reading_profile(&self) -> &ResourceReadingProfile {
        self
    }
}
/// Current status information relevant to an entity.
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResourceStatus {
    /// UML inherited base object
    // parent_message: true
    // required_field: false
    // multiplicity_min: 0
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="1")]
    pub status_value: ::std::option::Option<super::commonmodule::StatusValue>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: false
    // multiplicity_min: 0
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, repeated, tag="2")]
    pub analog_status_ggio: ::std::vec::Vec<super::commonmodule::AnalogStatusGgio>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: false
    // multiplicity_min: 0
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, repeated, tag="3")]
    pub boolean_status_ggio: ::std::vec::Vec<super::commonmodule::BooleanStatusGgio>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: false
    // multiplicity_min: 0
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, repeated, tag="4")]
    pub integer_status_ggio: ::std::vec::Vec<super::commonmodule::IntegerStatusGgio>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: false
    // multiplicity_min: 0
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, repeated, tag="5")]
    pub string_status_ggio: ::std::vec::Vec<super::commonmodule::StringStatusGgio>,
}
mod resource_status {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref STATUS_VALUE: crate::commonmodule::StatusValue = Default::default();
    }
}
pub trait IsResourceStatus {
    fn _resource_status(&self) -> &ResourceStatus;
    fn status_value(&self) -> &super::commonmodule::StatusValue {
        self._resource_status().status_value.as_ref().unwrap_or(&resource_status::STATUS_VALUE)
    }
    fn analog_status_ggio(&self) -> &::std::vec::Vec<super::commonmodule::AnalogStatusGgio> {
        &self._resource_status().analog_status_ggio    }
    fn boolean_status_ggio(&self) -> &::std::vec::Vec<super::commonmodule::BooleanStatusGgio> {
        &self._resource_status().boolean_status_ggio    }
    fn integer_status_ggio(&self) -> &::std::vec::Vec<super::commonmodule::IntegerStatusGgio> {
        &self._resource_status().integer_status_ggio    }
    fn string_status_ggio(&self) -> &::std::vec::Vec<super::commonmodule::StringStatusGgio> {
        &self._resource_status().string_status_ggio    }
}
impl IsResourceStatus for ResourceStatus {
    fn _resource_status(&self) -> &ResourceStatus {
        self
    }
}
/// Resource status module
/// OpenFMB Profile Message: true
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResourceStatusProfile {
    /// UML inherited base object
    // parent_message: true
    // required_field: false
    // multiplicity_min: 0
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="1")]
    pub status_message_info: ::std::option::Option<super::commonmodule::StatusMessageInfo>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="2")]
    pub conducting_equipment: ::std::option::Option<super::commonmodule::ConductingEquipment>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="3")]
    pub ied: ::std::option::Option<super::commonmodule::Ied>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="4")]
    pub resource_status: ::std::option::Option<ResourceStatus>,
}
mod resource_status_profile {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref STATUS_MESSAGE_INFO: crate::commonmodule::StatusMessageInfo = Default::default();
        pub(super) static ref CONDUCTING_EQUIPMENT: crate::commonmodule::ConductingEquipment = Default::default();
        pub(super) static ref IED: crate::commonmodule::Ied = Default::default();
        pub(super) static ref RESOURCE_STATUS: crate::resourcemodule::ResourceStatus = Default::default();
    }
}
pub trait IsResourceStatusProfile {
    fn _resource_status_profile(&self) -> &ResourceStatusProfile;
    fn status_message_info(&self) -> &super::commonmodule::StatusMessageInfo {
        self._resource_status_profile().status_message_info.as_ref().unwrap_or(&resource_status_profile::STATUS_MESSAGE_INFO)
    }
    fn conducting_equipment(&self) -> &super::commonmodule::ConductingEquipment {
        self._resource_status_profile().conducting_equipment.as_ref().unwrap_or(&resource_status_profile::CONDUCTING_EQUIPMENT)
    }
    fn ied(&self) -> &super::commonmodule::Ied {
        self._resource_status_profile().ied.as_ref().unwrap_or(&resource_status_profile::IED)
    }
    fn resource_status(&self) -> &ResourceStatus {
        self._resource_status_profile().resource_status.as_ref().unwrap_or(&resource_status_profile::RESOURCE_STATUS)
    }
}
impl IsResourceStatusProfile for ResourceStatusProfile {
    fn _resource_status_profile(&self) -> &ResourceStatusProfile {
        self
    }
}
