/// OpenFMB specialization for BreakerDiscreteControlProfile: Added blk  LN: Circuit breaker   Name:
/// XCBR
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct BreakerDiscreteControlXcbr {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub logical_node_for_control: ::std::option::Option<super::commonmodule::LogicalNodeForControl>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub pos: ::std::option::Option<super::commonmodule::ControlDpc>,
}
/// Breaker discrete control class
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct BreakerDiscreteControl {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub control_value: ::std::option::Option<super::commonmodule::ControlValue>,
    /// IEC61850 enhanced control parameters.
    #[prost(message, optional, tag="2")]
    pub check: ::std::option::Option<super::commonmodule::CheckConditions>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub breaker_discrete_control_xcbr: ::std::option::Option<BreakerDiscreteControlXcbr>,
}
/// A mechanical switching device capable of making, carrying, and breaking currents under normal
/// circuit conditions and also making, carrying for a specified time, and breaking currents under
/// specified abnormal circuit conditions e.g.  those of short circuit.
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Breaker {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub conducting_equipment: ::std::option::Option<super::commonmodule::ConductingEquipment>,
}
/// Instructs an end device (or an end device group) to perform a specified action.
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct BreakerDiscreteControlProfile {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub control_message_info: ::std::option::Option<super::commonmodule::ControlMessageInfo>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub breaker: ::std::option::Option<Breaker>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub breaker_discrete_control: ::std::option::Option<BreakerDiscreteControl>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="4")]
    pub ied: ::std::option::Option<super::commonmodule::Ied>,
}
/// Breaker event class
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct BreakerEvent {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub event_value: ::std::option::Option<super::commonmodule::EventValue>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub status_and_event_xcbr: ::std::option::Option<super::commonmodule::StatusAndEventXcbr>,
}
/// Breaker event profile
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct BreakerEventProfile {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub event_message_info: ::std::option::Option<super::commonmodule::EventMessageInfo>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub breaker: ::std::option::Option<Breaker>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub breaker_event: ::std::option::Option<BreakerEvent>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="4")]
    pub ied: ::std::option::Option<super::commonmodule::Ied>,
}
/// MISSING DOCUMENTATION!!!
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct BreakerReading {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub conducting_equipment_terminal_reading: ::std::option::Option<super::commonmodule::ConductingEquipmentTerminalReading>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub diff_reading_mmxu: ::std::option::Option<super::commonmodule::ReadingMmxu>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub phase_mmtn: ::std::option::Option<super::commonmodule::PhaseMmtn>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="4")]
    pub reading_mmtr: ::std::option::Option<super::commonmodule::ReadingMmtr>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="5")]
    pub reading_mmxu: ::std::option::Option<super::commonmodule::ReadingMmxu>,
}
/// Breaker reading profile
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct BreakerReadingProfile {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub reading_message_info: ::std::option::Option<super::commonmodule::ReadingMessageInfo>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub breaker: ::std::option::Option<Breaker>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, repeated, tag="3")]
    pub breaker_reading: ::std::vec::Vec<BreakerReading>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="4")]
    pub ied: ::std::option::Option<super::commonmodule::Ied>,
}
/// Status of a breaker
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct BreakerStatus {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub status_value: ::std::option::Option<super::commonmodule::StatusValue>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub status_and_event_xcbr: ::std::option::Option<super::commonmodule::StatusAndEventXcbr>,
}
/// Breaker status profile
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct BreakerStatusProfile {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub status_message_info: ::std::option::Option<super::commonmodule::StatusMessageInfo>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub breaker: ::std::option::Option<Breaker>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub breaker_status: ::std::option::Option<BreakerStatus>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="4")]
    pub ied: ::std::option::Option<super::commonmodule::Ied>,
}
