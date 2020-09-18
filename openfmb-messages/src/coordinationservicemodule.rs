#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct OptionalCoordinationServiceModeKind {
    #[prost(enumeration="CoordinationServiceModeKind", tag="1")]
    pub value: i32,
}
mod optional_coordination_service_mode_kind {
    use lazy_static::lazy_static;
    lazy_static! {
    }
}
trait IsOptionalCoordinationServiceModeKind {
    fn _optional_coordination_service_mode_kind(&self) -> &OptionalCoordinationServiceModeKind;
    fn value(&self) -> &i32 {
        &self._optional_coordination_service_mode_kind().value    }
}
impl IsOptionalCoordinationServiceModeKind for OptionalCoordinationServiceModeKind {
    fn _optional_coordination_service_mode_kind(&self) -> &OptionalCoordinationServiceModeKind {
        self
    }
}
/// Coordination service mode kind
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct EngCoordinationServiceModeKind {
    /// The value of the coordination service mode.
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(enumeration="CoordinationServiceModeKind", tag="1")]
    pub set_val: i32,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub set_val_extension: ::std::option::Option<::std::string::String>,
}
mod eng_coordination_service_mode_kind {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref SET_VAL_EXTENSION: ::std::string::String = Default::default();
    }
}
trait IsEngCoordinationServiceModeKind {
    fn _eng_coordination_service_mode_kind(&self) -> &EngCoordinationServiceModeKind;
    fn set_val(&self) -> &i32 {
        &self._eng_coordination_service_mode_kind().set_val    }
    fn set_val_extension(&self) -> &::std::string::String {
        self._eng_coordination_service_mode_kind().set_val_extension.as_ref().unwrap_or(&eng_coordination_service_mode_kind::SET_VAL_EXTENSION)
    }
}
impl IsEngCoordinationServiceModeKind for EngCoordinationServiceModeKind {
    fn _eng_coordination_service_mode_kind(&self) -> &EngCoordinationServiceModeKind {
        self
    }
}
/// OpenFMB specialization for coordination service control, DCSC (Distributed Coordination Service
/// Control), following 61850 naming convention.
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct CoordinationControlDcsc {
    /// UML inherited base object
    // parent_message: true
    // required_field: false
    // multiplicity_min: 0
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="1")]
    pub logical_node_for_control: ::std::option::Option<super::commonmodule::LogicalNodeForControl>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub coordination_service_mode: ::std::option::Option<EngCoordinationServiceModeKind>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub island: ::std::option::Option<super::commonmodule::ControlDpc>,
}
mod coordination_control_dcsc {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref LOGICAL_NODE_FOR_CONTROL: crate::commonmodule::LogicalNodeForControl = Default::default();
        pub(super) static ref COORDINATION_SERVICE_MODE: crate::coordinationservicemodule::EngCoordinationServiceModeKind = Default::default();
        pub(super) static ref ISLAND: crate::commonmodule::ControlDpc = Default::default();
    }
}
trait IsCoordinationControlDcsc {
    fn _coordination_control_dcsc(&self) -> &CoordinationControlDcsc;
    fn logical_node_for_control(&self) -> &super::commonmodule::LogicalNodeForControl {
        self._coordination_control_dcsc().logical_node_for_control.as_ref().unwrap_or(&coordination_control_dcsc::LOGICAL_NODE_FOR_CONTROL)
    }
    fn coordination_service_mode(&self) -> &EngCoordinationServiceModeKind {
        self._coordination_control_dcsc().coordination_service_mode.as_ref().unwrap_or(&coordination_control_dcsc::COORDINATION_SERVICE_MODE)
    }
    fn island(&self) -> &super::commonmodule::ControlDpc {
        self._coordination_control_dcsc().island.as_ref().unwrap_or(&coordination_control_dcsc::ISLAND)
    }
}
impl IsCoordinationControlDcsc for CoordinationControlDcsc {
    fn _coordination_control_dcsc(&self) -> &CoordinationControlDcsc {
        self
    }
}
/// Switch discrete control
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct CoordinationControl {
    /// UML inherited base object
    // parent_message: true
    // required_field: false
    // multiplicity_min: 0
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="1")]
    pub control_value: ::std::option::Option<super::commonmodule::ControlValue>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub check: ::std::option::Option<super::commonmodule::CheckConditions>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="3")]
    pub coordination_control_dcsc: ::std::option::Option<CoordinationControlDcsc>,
}
mod coordination_control {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref CONTROL_VALUE: crate::commonmodule::ControlValue = Default::default();
        pub(super) static ref CHECK: crate::commonmodule::CheckConditions = Default::default();
        pub(super) static ref COORDINATION_CONTROL_DCSC: crate::coordinationservicemodule::CoordinationControlDcsc = Default::default();
    }
}
trait IsCoordinationControl {
    fn _coordination_control(&self) -> &CoordinationControl;
    fn control_value(&self) -> &super::commonmodule::ControlValue {
        self._coordination_control().control_value.as_ref().unwrap_or(&coordination_control::CONTROL_VALUE)
    }
    fn check(&self) -> &super::commonmodule::CheckConditions {
        self._coordination_control().check.as_ref().unwrap_or(&coordination_control::CHECK)
    }
    fn coordination_control_dcsc(&self) -> &CoordinationControlDcsc {
        self._coordination_control().coordination_control_dcsc.as_ref().unwrap_or(&coordination_control::COORDINATION_CONTROL_DCSC)
    }
}
impl IsCoordinationControl for CoordinationControl {
    fn _coordination_control(&self) -> &CoordinationControl {
        self
    }
}
/// Switch control profile
/// OpenFMB Profile Message: true
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct CoordinationControlProfile {
    /// UML inherited base object
    // parent_message: true
    // required_field: false
    // multiplicity_min: 0
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="1")]
    pub control_message_info: ::std::option::Option<super::commonmodule::ControlMessageInfo>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="2")]
    pub application_system: ::std::option::Option<super::commonmodule::ApplicationSystem>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="3")]
    pub coordination_control: ::std::option::Option<CoordinationControl>,
}
mod coordination_control_profile {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref CONTROL_MESSAGE_INFO: crate::commonmodule::ControlMessageInfo = Default::default();
        pub(super) static ref APPLICATION_SYSTEM: crate::commonmodule::ApplicationSystem = Default::default();
        pub(super) static ref COORDINATION_CONTROL: crate::coordinationservicemodule::CoordinationControl = Default::default();
    }
}
trait IsCoordinationControlProfile {
    fn _coordination_control_profile(&self) -> &CoordinationControlProfile;
    fn control_message_info(&self) -> &super::commonmodule::ControlMessageInfo {
        self._coordination_control_profile().control_message_info.as_ref().unwrap_or(&coordination_control_profile::CONTROL_MESSAGE_INFO)
    }
    fn application_system(&self) -> &super::commonmodule::ApplicationSystem {
        self._coordination_control_profile().application_system.as_ref().unwrap_or(&coordination_control_profile::APPLICATION_SYSTEM)
    }
    fn coordination_control(&self) -> &CoordinationControl {
        self._coordination_control_profile().coordination_control.as_ref().unwrap_or(&coordination_control_profile::COORDINATION_CONTROL)
    }
}
impl IsCoordinationControlProfile for CoordinationControlProfile {
    fn _coordination_control_profile(&self) -> &CoordinationControlProfile {
        self
    }
}
/// OpenFMB specialization for coordination service control, DCSC (Distributed Coordination Service
/// Control), following 61850 naming convention.
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct CoordinationEventDcsc {
    /// UML inherited base object
    // parent_message: true
    // required_field: false
    // multiplicity_min: 0
    // multiplicity_max: 0
    // uuid: false
    // key: false
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
mod coordination_event_dcsc {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref LOGICAL_NODE_FOR_EVENT_AND_STATUS: crate::commonmodule::LogicalNodeForEventAndStatus = Default::default();
        pub(super) static ref COORDINATION_SERVICE_MODE: crate::coordinationservicemodule::EngCoordinationServiceModeKind = Default::default();
        pub(super) static ref ISLAND: crate::commonmodule::StatusSps = Default::default();
        pub(super) static ref PERMISSIBLE_AUTO: crate::commonmodule::StatusSps = Default::default();
        pub(super) static ref PERMISSIBLE_MANUAL: crate::commonmodule::StatusSps = Default::default();
        pub(super) static ref PERMISSIBLE_NETZERO: crate::commonmodule::StatusSps = Default::default();
        pub(super) static ref PERMISSIBLE_START: crate::commonmodule::StatusSps = Default::default();
        pub(super) static ref PERMISSIBLE_STOP: crate::commonmodule::StatusSps = Default::default();
    }
}
trait IsCoordinationEventDcsc {
    fn _coordination_event_dcsc(&self) -> &CoordinationEventDcsc;
    fn logical_node_for_event_and_status(&self) -> &super::commonmodule::LogicalNodeForEventAndStatus {
        self._coordination_event_dcsc().logical_node_for_event_and_status.as_ref().unwrap_or(&coordination_event_dcsc::LOGICAL_NODE_FOR_EVENT_AND_STATUS)
    }
    fn coordination_service_mode(&self) -> &EngCoordinationServiceModeKind {
        self._coordination_event_dcsc().coordination_service_mode.as_ref().unwrap_or(&coordination_event_dcsc::COORDINATION_SERVICE_MODE)
    }
    fn island(&self) -> &super::commonmodule::StatusSps {
        self._coordination_event_dcsc().island.as_ref().unwrap_or(&coordination_event_dcsc::ISLAND)
    }
    fn permissible_auto(&self) -> &super::commonmodule::StatusSps {
        self._coordination_event_dcsc().permissible_auto.as_ref().unwrap_or(&coordination_event_dcsc::PERMISSIBLE_AUTO)
    }
    fn permissible_manual(&self) -> &super::commonmodule::StatusSps {
        self._coordination_event_dcsc().permissible_manual.as_ref().unwrap_or(&coordination_event_dcsc::PERMISSIBLE_MANUAL)
    }
    fn permissible_netzero(&self) -> &super::commonmodule::StatusSps {
        self._coordination_event_dcsc().permissible_netzero.as_ref().unwrap_or(&coordination_event_dcsc::PERMISSIBLE_NETZERO)
    }
    fn permissible_start(&self) -> &super::commonmodule::StatusSps {
        self._coordination_event_dcsc().permissible_start.as_ref().unwrap_or(&coordination_event_dcsc::PERMISSIBLE_START)
    }
    fn permissible_stop(&self) -> &super::commonmodule::StatusSps {
        self._coordination_event_dcsc().permissible_stop.as_ref().unwrap_or(&coordination_event_dcsc::PERMISSIBLE_STOP)
    }
}
impl IsCoordinationEventDcsc for CoordinationEventDcsc {
    fn _coordination_event_dcsc(&self) -> &CoordinationEventDcsc {
        self
    }
}
/// Switch event
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct CoordinationEvent {
    /// UML inherited base object
    // parent_message: true
    // required_field: false
    // multiplicity_min: 0
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="1")]
    pub event_value: ::std::option::Option<super::commonmodule::EventValue>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="2")]
    pub coordination_event_dcsc: ::std::option::Option<CoordinationEventDcsc>,
}
mod coordination_event {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref EVENT_VALUE: crate::commonmodule::EventValue = Default::default();
        pub(super) static ref COORDINATION_EVENT_DCSC: crate::coordinationservicemodule::CoordinationEventDcsc = Default::default();
    }
}
trait IsCoordinationEvent {
    fn _coordination_event(&self) -> &CoordinationEvent;
    fn event_value(&self) -> &super::commonmodule::EventValue {
        self._coordination_event().event_value.as_ref().unwrap_or(&coordination_event::EVENT_VALUE)
    }
    fn coordination_event_dcsc(&self) -> &CoordinationEventDcsc {
        self._coordination_event().coordination_event_dcsc.as_ref().unwrap_or(&coordination_event::COORDINATION_EVENT_DCSC)
    }
}
impl IsCoordinationEvent for CoordinationEvent {
    fn _coordination_event(&self) -> &CoordinationEvent {
        self
    }
}
/// Switch event profile
/// OpenFMB Profile Message: true
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct CoordinationEventProfile {
    /// UML inherited base object
    // parent_message: true
    // required_field: false
    // multiplicity_min: 0
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="1")]
    pub event_message_info: ::std::option::Option<super::commonmodule::EventMessageInfo>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="2")]
    pub application_system: ::std::option::Option<super::commonmodule::ApplicationSystem>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="3")]
    pub coordination_event: ::std::option::Option<CoordinationEvent>,
}
mod coordination_event_profile {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref EVENT_MESSAGE_INFO: crate::commonmodule::EventMessageInfo = Default::default();
        pub(super) static ref APPLICATION_SYSTEM: crate::commonmodule::ApplicationSystem = Default::default();
        pub(super) static ref COORDINATION_EVENT: crate::coordinationservicemodule::CoordinationEvent = Default::default();
    }
}
trait IsCoordinationEventProfile {
    fn _coordination_event_profile(&self) -> &CoordinationEventProfile;
    fn event_message_info(&self) -> &super::commonmodule::EventMessageInfo {
        self._coordination_event_profile().event_message_info.as_ref().unwrap_or(&coordination_event_profile::EVENT_MESSAGE_INFO)
    }
    fn application_system(&self) -> &super::commonmodule::ApplicationSystem {
        self._coordination_event_profile().application_system.as_ref().unwrap_or(&coordination_event_profile::APPLICATION_SYSTEM)
    }
    fn coordination_event(&self) -> &CoordinationEvent {
        self._coordination_event_profile().coordination_event.as_ref().unwrap_or(&coordination_event_profile::COORDINATION_EVENT)
    }
}
impl IsCoordinationEventProfile for CoordinationEventProfile {
    fn _coordination_event_profile(&self) -> &CoordinationEventProfile {
        self
    }
}
/// OpenFMB specialization for coordination service control, DCSC (Distributed Coordination Service
/// Control), following 61850 naming convention.
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct CoordinationStatusDcsc {
    /// UML inherited base object
    // parent_message: true
    // required_field: false
    // multiplicity_min: 0
    // multiplicity_max: 0
    // uuid: false
    // key: false
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
mod coordination_status_dcsc {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref LOGICAL_NODE_FOR_EVENT_AND_STATUS: crate::commonmodule::LogicalNodeForEventAndStatus = Default::default();
        pub(super) static ref COORDINATION_SERVICE_MODE: crate::coordinationservicemodule::EngCoordinationServiceModeKind = Default::default();
        pub(super) static ref ISLAND: crate::commonmodule::StatusSps = Default::default();
        pub(super) static ref PERMISSIBLE_AUTO: crate::commonmodule::StatusSps = Default::default();
        pub(super) static ref PERMISSIBLE_MANUAL: crate::commonmodule::StatusSps = Default::default();
        pub(super) static ref PERMISSIBLE_NETZERO: crate::commonmodule::StatusSps = Default::default();
        pub(super) static ref PERMISSIBLE_START: crate::commonmodule::StatusSps = Default::default();
        pub(super) static ref PERMISSIBLE_STOP: crate::commonmodule::StatusSps = Default::default();
    }
}
trait IsCoordinationStatusDcsc {
    fn _coordination_status_dcsc(&self) -> &CoordinationStatusDcsc;
    fn logical_node_for_event_and_status(&self) -> &super::commonmodule::LogicalNodeForEventAndStatus {
        self._coordination_status_dcsc().logical_node_for_event_and_status.as_ref().unwrap_or(&coordination_status_dcsc::LOGICAL_NODE_FOR_EVENT_AND_STATUS)
    }
    fn coordination_service_mode(&self) -> &EngCoordinationServiceModeKind {
        self._coordination_status_dcsc().coordination_service_mode.as_ref().unwrap_or(&coordination_status_dcsc::COORDINATION_SERVICE_MODE)
    }
    fn island(&self) -> &super::commonmodule::StatusSps {
        self._coordination_status_dcsc().island.as_ref().unwrap_or(&coordination_status_dcsc::ISLAND)
    }
    fn permissible_auto(&self) -> &super::commonmodule::StatusSps {
        self._coordination_status_dcsc().permissible_auto.as_ref().unwrap_or(&coordination_status_dcsc::PERMISSIBLE_AUTO)
    }
    fn permissible_manual(&self) -> &super::commonmodule::StatusSps {
        self._coordination_status_dcsc().permissible_manual.as_ref().unwrap_or(&coordination_status_dcsc::PERMISSIBLE_MANUAL)
    }
    fn permissible_netzero(&self) -> &super::commonmodule::StatusSps {
        self._coordination_status_dcsc().permissible_netzero.as_ref().unwrap_or(&coordination_status_dcsc::PERMISSIBLE_NETZERO)
    }
    fn permissible_start(&self) -> &super::commonmodule::StatusSps {
        self._coordination_status_dcsc().permissible_start.as_ref().unwrap_or(&coordination_status_dcsc::PERMISSIBLE_START)
    }
    fn permissible_stop(&self) -> &super::commonmodule::StatusSps {
        self._coordination_status_dcsc().permissible_stop.as_ref().unwrap_or(&coordination_status_dcsc::PERMISSIBLE_STOP)
    }
}
impl IsCoordinationStatusDcsc for CoordinationStatusDcsc {
    fn _coordination_status_dcsc(&self) -> &CoordinationStatusDcsc {
        self
    }
}
/// Switch event
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct CoordinationStatus {
    /// UML inherited base object
    // parent_message: true
    // required_field: false
    // multiplicity_min: 0
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="1")]
    pub event_value: ::std::option::Option<super::commonmodule::EventValue>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="2")]
    pub coordination_status_dcsc: ::std::option::Option<CoordinationStatusDcsc>,
}
mod coordination_status {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref EVENT_VALUE: crate::commonmodule::EventValue = Default::default();
        pub(super) static ref COORDINATION_STATUS_DCSC: crate::coordinationservicemodule::CoordinationStatusDcsc = Default::default();
    }
}
trait IsCoordinationStatus {
    fn _coordination_status(&self) -> &CoordinationStatus;
    fn event_value(&self) -> &super::commonmodule::EventValue {
        self._coordination_status().event_value.as_ref().unwrap_or(&coordination_status::EVENT_VALUE)
    }
    fn coordination_status_dcsc(&self) -> &CoordinationStatusDcsc {
        self._coordination_status().coordination_status_dcsc.as_ref().unwrap_or(&coordination_status::COORDINATION_STATUS_DCSC)
    }
}
impl IsCoordinationStatus for CoordinationStatus {
    fn _coordination_status(&self) -> &CoordinationStatus {
        self
    }
}
/// Switch event profile
/// OpenFMB Profile Message: true
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct CoordinationStatusProfile {
    /// UML inherited base object
    // parent_message: true
    // required_field: false
    // multiplicity_min: 0
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="1")]
    pub event_message_info: ::std::option::Option<super::commonmodule::EventMessageInfo>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="2")]
    pub application_system: ::std::option::Option<super::commonmodule::ApplicationSystem>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="3")]
    pub coordination_status: ::std::option::Option<CoordinationStatus>,
}
mod coordination_status_profile {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref EVENT_MESSAGE_INFO: crate::commonmodule::EventMessageInfo = Default::default();
        pub(super) static ref APPLICATION_SYSTEM: crate::commonmodule::ApplicationSystem = Default::default();
        pub(super) static ref COORDINATION_STATUS: crate::coordinationservicemodule::CoordinationStatus = Default::default();
    }
}
trait IsCoordinationStatusProfile {
    fn _coordination_status_profile(&self) -> &CoordinationStatusProfile;
    fn event_message_info(&self) -> &super::commonmodule::EventMessageInfo {
        self._coordination_status_profile().event_message_info.as_ref().unwrap_or(&coordination_status_profile::EVENT_MESSAGE_INFO)
    }
    fn application_system(&self) -> &super::commonmodule::ApplicationSystem {
        self._coordination_status_profile().application_system.as_ref().unwrap_or(&coordination_status_profile::APPLICATION_SYSTEM)
    }
    fn coordination_status(&self) -> &CoordinationStatus {
        self._coordination_status_profile().coordination_status.as_ref().unwrap_or(&coordination_status_profile::COORDINATION_STATUS)
    }
}
impl IsCoordinationStatusProfile for CoordinationStatusProfile {
    fn _coordination_status_profile(&self) -> &CoordinationStatusProfile {
        self
    }
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
