#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct OptionalCoordinationServiceModeKind {
    #[prost(enumeration="CoordinationServiceModeKind", tag="1")]
    pub value: i32,
}
/// Coordination service mode kind
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct EngCoordinationServiceModeKind {
    /// The value of the coordination service mode.
    #[prost(enumeration="CoordinationServiceModeKind", tag="1")]
    pub set_val: i32,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub set_val_extension: ::std::option::Option<::std::string::String>,
}
/// OpenFMB specialization for coordination service control, DCSC (Distributed Coordination Service
/// Control), following 61850 naming convention.
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct CoordinationControlDcsc {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub logical_node_for_control: ::std::option::Option<super::commonmodule::LogicalNodeForControl>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub coordination_service_mode: ::std::option::Option<EngCoordinationServiceModeKind>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub island: ::std::option::Option<super::commonmodule::ControlDpc>,
}
/// Switch discrete control
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct CoordinationControl {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub control_value: ::std::option::Option<super::commonmodule::ControlValue>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub check: ::std::option::Option<super::commonmodule::CheckConditions>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub coordination_control_dcsc: ::std::option::Option<CoordinationControlDcsc>,
}
/// Switch control profile
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct CoordinationControlProfile {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub control_message_info: ::std::option::Option<super::commonmodule::ControlMessageInfo>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub application_system: ::std::option::Option<super::commonmodule::ApplicationSystem>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub coordination_control: ::std::option::Option<CoordinationControl>,
}
/// OpenFMB specialization for coordination service control, DCSC (Distributed Coordination Service
/// Control), following 61850 naming convention.
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct CoordinationEventDcsc {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub logical_node_for_event_and_status: ::std::option::Option<super::commonmodule::LogicalNodeForEventAndStatus>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub coordination_service_mode: ::std::option::Option<EngCoordinationServiceModeKind>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub island: ::std::option::Option<super::commonmodule::StatusSps>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="4")]
    pub permissible_auto: ::std::option::Option<super::commonmodule::StatusSps>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="5")]
    pub permissible_manual: ::std::option::Option<super::commonmodule::StatusSps>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="6")]
    pub permissible_netzero: ::std::option::Option<super::commonmodule::StatusSps>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="7")]
    pub permissible_start: ::std::option::Option<super::commonmodule::StatusSps>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="8")]
    pub permissible_stop: ::std::option::Option<super::commonmodule::StatusSps>,
}
/// Switch event
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct CoordinationEvent {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub event_value: ::std::option::Option<super::commonmodule::EventValue>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub coordination_event_dcsc: ::std::option::Option<CoordinationEventDcsc>,
}
/// Switch event profile
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct CoordinationEventProfile {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub event_message_info: ::std::option::Option<super::commonmodule::EventMessageInfo>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub application_system: ::std::option::Option<super::commonmodule::ApplicationSystem>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub coordination_event: ::std::option::Option<CoordinationEvent>,
}
/// OpenFMB specialization for coordination service control, DCSC (Distributed Coordination Service
/// Control), following 61850 naming convention.
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct CoordinationStatusDcsc {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub logical_node_for_event_and_status: ::std::option::Option<super::commonmodule::LogicalNodeForEventAndStatus>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub coordination_service_mode: ::std::option::Option<EngCoordinationServiceModeKind>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub island: ::std::option::Option<super::commonmodule::StatusSps>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="4")]
    pub permissible_auto: ::std::option::Option<super::commonmodule::StatusSps>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="5")]
    pub permissible_manual: ::std::option::Option<super::commonmodule::StatusSps>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="6")]
    pub permissible_netzero: ::std::option::Option<super::commonmodule::StatusSps>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="7")]
    pub permissible_start: ::std::option::Option<super::commonmodule::StatusSps>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="8")]
    pub permissible_stop: ::std::option::Option<super::commonmodule::StatusSps>,
}
/// Switch event
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct CoordinationStatus {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub event_value: ::std::option::Option<super::commonmodule::EventValue>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub coordination_status_dcsc: ::std::option::Option<CoordinationStatusDcsc>,
}
/// Switch event profile
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct CoordinationStatusProfile {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub event_message_info: ::std::option::Option<super::commonmodule::EventMessageInfo>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub application_system: ::std::option::Option<super::commonmodule::ApplicationSystem>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub coordination_status: ::std::option::Option<CoordinationStatus>,
}
/// State kind
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum CoordinationServiceModeKind {
    /// MISSING DOCUMENTATION!!!
    None = 0,
    /// MISSING DOCUMENTATION!!!
    Auto = 1,
    /// MISSING DOCUMENTATION!!!
    Manual = 2,
    /// MISSING DOCUMENTATION!!!
    Netzero = 3,
    /// MISSING DOCUMENTATION!!!
    Start = 4,
    /// MISSING DOCUMENTATION!!!
    Stop = 5,
}
