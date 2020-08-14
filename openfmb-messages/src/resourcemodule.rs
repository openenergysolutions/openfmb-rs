/// Resource reading value
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResourceReading {
    /// UML inherited base object
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
/// Resource reading profile
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResourceReadingProfile {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub reading_message_info: ::std::option::Option<super::commonmodule::ReadingMessageInfo>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub conducting_equipment: ::std::option::Option<super::commonmodule::ConductingEquipment>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub ied: ::std::option::Option<super::commonmodule::Ied>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="4")]
    pub resource_reading: ::std::option::Option<ResourceReading>,
}
/// Current status information relevant to an entity.
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResourceStatus {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub status_value: ::std::option::Option<super::commonmodule::StatusValue>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, repeated, tag="2")]
    pub analog_status_ggio: ::std::vec::Vec<super::commonmodule::AnalogStatusGgio>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, repeated, tag="3")]
    pub boolean_status_ggio: ::std::vec::Vec<super::commonmodule::BooleanStatusGgio>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, repeated, tag="4")]
    pub integer_status_ggio: ::std::vec::Vec<super::commonmodule::IntegerStatusGgio>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, repeated, tag="5")]
    pub string_status_ggio: ::std::vec::Vec<super::commonmodule::StringStatusGgio>,
}
/// Resource status module
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResourceStatusProfile {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub status_message_info: ::std::option::Option<super::commonmodule::StatusMessageInfo>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub conducting_equipment: ::std::option::Option<super::commonmodule::ConductingEquipment>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub ied: ::std::option::Option<super::commonmodule::Ied>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="4")]
    pub resource_status: ::std::option::Option<ResourceStatus>,
}
