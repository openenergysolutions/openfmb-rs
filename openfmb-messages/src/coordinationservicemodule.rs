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
pub trait IsOptionalCoordinationServiceModeKind {
    fn _optional_coordination_service_mode_kind(&self) -> &OptionalCoordinationServiceModeKind;
    fn _mut_optional_coordination_service_mode_kind(&mut self) -> &mut OptionalCoordinationServiceModeKind;
    fn value(&self) -> &i32 {
        &self._optional_coordination_service_mode_kind().value    }
    fn mut_value(&mut self) -> &mut i32 {
        &mut self._mut_optional_coordination_service_mode_kind().value    }
}
impl IsOptionalCoordinationServiceModeKind for OptionalCoordinationServiceModeKind {
    fn _optional_coordination_service_mode_kind(&self) -> &OptionalCoordinationServiceModeKind {
        self
    }
    fn _mut_optional_coordination_service_mode_kind(&mut self) -> &mut OptionalCoordinationServiceModeKind {
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
pub trait IsEngCoordinationServiceModeKind {
    fn _eng_coordination_service_mode_kind(&self) -> &EngCoordinationServiceModeKind;
    fn _mut_eng_coordination_service_mode_kind(&mut self) -> &mut EngCoordinationServiceModeKind;
    fn set_val(&self) -> &i32 {
        &self._eng_coordination_service_mode_kind().set_val    }
    fn mut_set_val(&mut self) -> &mut i32 {
        &mut self._mut_eng_coordination_service_mode_kind().set_val    }
    fn set_val_extension(&self) -> &::std::string::String {
        self._eng_coordination_service_mode_kind().set_val_extension.as_ref().unwrap_or(&eng_coordination_service_mode_kind::SET_VAL_EXTENSION)
    }
    fn mut_set_val_extension(&mut self) -> &mut ::std::string::String {
        self._mut_eng_coordination_service_mode_kind().set_val_extension.get_or_insert(eng_coordination_service_mode_kind::SET_VAL_EXTENSION.clone())
    }
}
impl IsEngCoordinationServiceModeKind for EngCoordinationServiceModeKind {
    fn _eng_coordination_service_mode_kind(&self) -> &EngCoordinationServiceModeKind {
        self
    }
    fn _mut_eng_coordination_service_mode_kind(&mut self) -> &mut EngCoordinationServiceModeKind {
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
pub trait IsCoordinationControlDcsc {
    fn _coordination_control_dcsc(&self) -> &CoordinationControlDcsc;
    fn _mut_coordination_control_dcsc(&mut self) -> &mut CoordinationControlDcsc;
    fn logical_node_for_control(&self) -> &super::commonmodule::LogicalNodeForControl {
        self._coordination_control_dcsc().logical_node_for_control.as_ref().unwrap_or(&coordination_control_dcsc::LOGICAL_NODE_FOR_CONTROL)
    }
    fn mut_logical_node_for_control(&mut self) -> &mut super::commonmodule::LogicalNodeForControl {
        self._mut_coordination_control_dcsc().logical_node_for_control.get_or_insert(coordination_control_dcsc::LOGICAL_NODE_FOR_CONTROL.clone())
    }
    fn coordination_service_mode(&self) -> &EngCoordinationServiceModeKind {
        self._coordination_control_dcsc().coordination_service_mode.as_ref().unwrap_or(&coordination_control_dcsc::COORDINATION_SERVICE_MODE)
    }
    fn mut_coordination_service_mode(&mut self) -> &mut EngCoordinationServiceModeKind {
        self._mut_coordination_control_dcsc().coordination_service_mode.get_or_insert(coordination_control_dcsc::COORDINATION_SERVICE_MODE.clone())
    }
    fn island(&self) -> &super::commonmodule::ControlDpc {
        self._coordination_control_dcsc().island.as_ref().unwrap_or(&coordination_control_dcsc::ISLAND)
    }
    fn mut_island(&mut self) -> &mut super::commonmodule::ControlDpc {
        self._mut_coordination_control_dcsc().island.get_or_insert(coordination_control_dcsc::ISLAND.clone())
    }
}
impl IsCoordinationControlDcsc for CoordinationControlDcsc {
    fn _coordination_control_dcsc(&self) -> &CoordinationControlDcsc {
        self
    }
    fn _mut_coordination_control_dcsc(&mut self) -> &mut CoordinationControlDcsc {
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
pub trait IsCoordinationControl {
    fn _coordination_control(&self) -> &CoordinationControl;
    fn _mut_coordination_control(&mut self) -> &mut CoordinationControl;
    fn control_value(&self) -> &super::commonmodule::ControlValue {
        self._coordination_control().control_value.as_ref().unwrap_or(&coordination_control::CONTROL_VALUE)
    }
    fn mut_control_value(&mut self) -> &mut super::commonmodule::ControlValue {
        self._mut_coordination_control().control_value.get_or_insert(coordination_control::CONTROL_VALUE.clone())
    }
    fn check(&self) -> &super::commonmodule::CheckConditions {
        self._coordination_control().check.as_ref().unwrap_or(&coordination_control::CHECK)
    }
    fn mut_check(&mut self) -> &mut super::commonmodule::CheckConditions {
        self._mut_coordination_control().check.get_or_insert(coordination_control::CHECK.clone())
    }
    fn coordination_control_dcsc(&self) -> &CoordinationControlDcsc {
        self._coordination_control().coordination_control_dcsc.as_ref().unwrap_or(&coordination_control::COORDINATION_CONTROL_DCSC)
    }
    fn mut_coordination_control_dcsc(&mut self) -> &mut CoordinationControlDcsc {
        self._mut_coordination_control().coordination_control_dcsc.get_or_insert(coordination_control::COORDINATION_CONTROL_DCSC.clone())
    }
}
impl IsCoordinationControl for CoordinationControl {
    fn _coordination_control(&self) -> &CoordinationControl {
        self
    }
    fn _mut_coordination_control(&mut self) -> &mut CoordinationControl {
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
pub trait IsCoordinationControlProfile {
    fn _coordination_control_profile(&self) -> &CoordinationControlProfile;
    fn _mut_coordination_control_profile(&mut self) -> &mut CoordinationControlProfile;
    fn control_message_info(&self) -> &super::commonmodule::ControlMessageInfo {
        self._coordination_control_profile().control_message_info.as_ref().unwrap_or(&coordination_control_profile::CONTROL_MESSAGE_INFO)
    }
    fn mut_control_message_info(&mut self) -> &mut super::commonmodule::ControlMessageInfo {
        self._mut_coordination_control_profile().control_message_info.get_or_insert(coordination_control_profile::CONTROL_MESSAGE_INFO.clone())
    }
    fn application_system(&self) -> &super::commonmodule::ApplicationSystem {
        self._coordination_control_profile().application_system.as_ref().unwrap_or(&coordination_control_profile::APPLICATION_SYSTEM)
    }
    fn mut_application_system(&mut self) -> &mut super::commonmodule::ApplicationSystem {
        self._mut_coordination_control_profile().application_system.get_or_insert(coordination_control_profile::APPLICATION_SYSTEM.clone())
    }
    fn coordination_control(&self) -> &CoordinationControl {
        self._coordination_control_profile().coordination_control.as_ref().unwrap_or(&coordination_control_profile::COORDINATION_CONTROL)
    }
    fn mut_coordination_control(&mut self) -> &mut CoordinationControl {
        self._mut_coordination_control_profile().coordination_control.get_or_insert(coordination_control_profile::COORDINATION_CONTROL.clone())
    }
}
impl IsCoordinationControlProfile for CoordinationControlProfile {
    fn _coordination_control_profile(&self) -> &CoordinationControlProfile {
        self
    }
    fn _mut_coordination_control_profile(&mut self) -> &mut CoordinationControlProfile {
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
pub trait IsCoordinationEventDcsc {
    fn _coordination_event_dcsc(&self) -> &CoordinationEventDcsc;
    fn _mut_coordination_event_dcsc(&mut self) -> &mut CoordinationEventDcsc;
    fn logical_node_for_event_and_status(&self) -> &super::commonmodule::LogicalNodeForEventAndStatus {
        self._coordination_event_dcsc().logical_node_for_event_and_status.as_ref().unwrap_or(&coordination_event_dcsc::LOGICAL_NODE_FOR_EVENT_AND_STATUS)
    }
    fn mut_logical_node_for_event_and_status(&mut self) -> &mut super::commonmodule::LogicalNodeForEventAndStatus {
        self._mut_coordination_event_dcsc().logical_node_for_event_and_status.get_or_insert(coordination_event_dcsc::LOGICAL_NODE_FOR_EVENT_AND_STATUS.clone())
    }
    fn coordination_service_mode(&self) -> &EngCoordinationServiceModeKind {
        self._coordination_event_dcsc().coordination_service_mode.as_ref().unwrap_or(&coordination_event_dcsc::COORDINATION_SERVICE_MODE)
    }
    fn mut_coordination_service_mode(&mut self) -> &mut EngCoordinationServiceModeKind {
        self._mut_coordination_event_dcsc().coordination_service_mode.get_or_insert(coordination_event_dcsc::COORDINATION_SERVICE_MODE.clone())
    }
    fn island(&self) -> &super::commonmodule::StatusSps {
        self._coordination_event_dcsc().island.as_ref().unwrap_or(&coordination_event_dcsc::ISLAND)
    }
    fn mut_island(&mut self) -> &mut super::commonmodule::StatusSps {
        self._mut_coordination_event_dcsc().island.get_or_insert(coordination_event_dcsc::ISLAND.clone())
    }
    fn permissible_auto(&self) -> &super::commonmodule::StatusSps {
        self._coordination_event_dcsc().permissible_auto.as_ref().unwrap_or(&coordination_event_dcsc::PERMISSIBLE_AUTO)
    }
    fn mut_permissible_auto(&mut self) -> &mut super::commonmodule::StatusSps {
        self._mut_coordination_event_dcsc().permissible_auto.get_or_insert(coordination_event_dcsc::PERMISSIBLE_AUTO.clone())
    }
    fn permissible_manual(&self) -> &super::commonmodule::StatusSps {
        self._coordination_event_dcsc().permissible_manual.as_ref().unwrap_or(&coordination_event_dcsc::PERMISSIBLE_MANUAL)
    }
    fn mut_permissible_manual(&mut self) -> &mut super::commonmodule::StatusSps {
        self._mut_coordination_event_dcsc().permissible_manual.get_or_insert(coordination_event_dcsc::PERMISSIBLE_MANUAL.clone())
    }
    fn permissible_netzero(&self) -> &super::commonmodule::StatusSps {
        self._coordination_event_dcsc().permissible_netzero.as_ref().unwrap_or(&coordination_event_dcsc::PERMISSIBLE_NETZERO)
    }
    fn mut_permissible_netzero(&mut self) -> &mut super::commonmodule::StatusSps {
        self._mut_coordination_event_dcsc().permissible_netzero.get_or_insert(coordination_event_dcsc::PERMISSIBLE_NETZERO.clone())
    }
    fn permissible_start(&self) -> &super::commonmodule::StatusSps {
        self._coordination_event_dcsc().permissible_start.as_ref().unwrap_or(&coordination_event_dcsc::PERMISSIBLE_START)
    }
    fn mut_permissible_start(&mut self) -> &mut super::commonmodule::StatusSps {
        self._mut_coordination_event_dcsc().permissible_start.get_or_insert(coordination_event_dcsc::PERMISSIBLE_START.clone())
    }
    fn permissible_stop(&self) -> &super::commonmodule::StatusSps {
        self._coordination_event_dcsc().permissible_stop.as_ref().unwrap_or(&coordination_event_dcsc::PERMISSIBLE_STOP)
    }
    fn mut_permissible_stop(&mut self) -> &mut super::commonmodule::StatusSps {
        self._mut_coordination_event_dcsc().permissible_stop.get_or_insert(coordination_event_dcsc::PERMISSIBLE_STOP.clone())
    }
}
impl IsCoordinationEventDcsc for CoordinationEventDcsc {
    fn _coordination_event_dcsc(&self) -> &CoordinationEventDcsc {
        self
    }
    fn _mut_coordination_event_dcsc(&mut self) -> &mut CoordinationEventDcsc {
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
pub trait IsCoordinationEvent {
    fn _coordination_event(&self) -> &CoordinationEvent;
    fn _mut_coordination_event(&mut self) -> &mut CoordinationEvent;
    fn event_value(&self) -> &super::commonmodule::EventValue {
        self._coordination_event().event_value.as_ref().unwrap_or(&coordination_event::EVENT_VALUE)
    }
    fn mut_event_value(&mut self) -> &mut super::commonmodule::EventValue {
        self._mut_coordination_event().event_value.get_or_insert(coordination_event::EVENT_VALUE.clone())
    }
    fn coordination_event_dcsc(&self) -> &CoordinationEventDcsc {
        self._coordination_event().coordination_event_dcsc.as_ref().unwrap_or(&coordination_event::COORDINATION_EVENT_DCSC)
    }
    fn mut_coordination_event_dcsc(&mut self) -> &mut CoordinationEventDcsc {
        self._mut_coordination_event().coordination_event_dcsc.get_or_insert(coordination_event::COORDINATION_EVENT_DCSC.clone())
    }
}
impl IsCoordinationEvent for CoordinationEvent {
    fn _coordination_event(&self) -> &CoordinationEvent {
        self
    }
    fn _mut_coordination_event(&mut self) -> &mut CoordinationEvent {
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
pub trait IsCoordinationEventProfile {
    fn _coordination_event_profile(&self) -> &CoordinationEventProfile;
    fn _mut_coordination_event_profile(&mut self) -> &mut CoordinationEventProfile;
    fn event_message_info(&self) -> &super::commonmodule::EventMessageInfo {
        self._coordination_event_profile().event_message_info.as_ref().unwrap_or(&coordination_event_profile::EVENT_MESSAGE_INFO)
    }
    fn mut_event_message_info(&mut self) -> &mut super::commonmodule::EventMessageInfo {
        self._mut_coordination_event_profile().event_message_info.get_or_insert(coordination_event_profile::EVENT_MESSAGE_INFO.clone())
    }
    fn application_system(&self) -> &super::commonmodule::ApplicationSystem {
        self._coordination_event_profile().application_system.as_ref().unwrap_or(&coordination_event_profile::APPLICATION_SYSTEM)
    }
    fn mut_application_system(&mut self) -> &mut super::commonmodule::ApplicationSystem {
        self._mut_coordination_event_profile().application_system.get_or_insert(coordination_event_profile::APPLICATION_SYSTEM.clone())
    }
    fn coordination_event(&self) -> &CoordinationEvent {
        self._coordination_event_profile().coordination_event.as_ref().unwrap_or(&coordination_event_profile::COORDINATION_EVENT)
    }
    fn mut_coordination_event(&mut self) -> &mut CoordinationEvent {
        self._mut_coordination_event_profile().coordination_event.get_or_insert(coordination_event_profile::COORDINATION_EVENT.clone())
    }
}
impl IsCoordinationEventProfile for CoordinationEventProfile {
    fn _coordination_event_profile(&self) -> &CoordinationEventProfile {
        self
    }
    fn _mut_coordination_event_profile(&mut self) -> &mut CoordinationEventProfile {
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
pub trait IsCoordinationStatusDcsc {
    fn _coordination_status_dcsc(&self) -> &CoordinationStatusDcsc;
    fn _mut_coordination_status_dcsc(&mut self) -> &mut CoordinationStatusDcsc;
    fn logical_node_for_event_and_status(&self) -> &super::commonmodule::LogicalNodeForEventAndStatus {
        self._coordination_status_dcsc().logical_node_for_event_and_status.as_ref().unwrap_or(&coordination_status_dcsc::LOGICAL_NODE_FOR_EVENT_AND_STATUS)
    }
    fn mut_logical_node_for_event_and_status(&mut self) -> &mut super::commonmodule::LogicalNodeForEventAndStatus {
        self._mut_coordination_status_dcsc().logical_node_for_event_and_status.get_or_insert(coordination_status_dcsc::LOGICAL_NODE_FOR_EVENT_AND_STATUS.clone())
    }
    fn coordination_service_mode(&self) -> &EngCoordinationServiceModeKind {
        self._coordination_status_dcsc().coordination_service_mode.as_ref().unwrap_or(&coordination_status_dcsc::COORDINATION_SERVICE_MODE)
    }
    fn mut_coordination_service_mode(&mut self) -> &mut EngCoordinationServiceModeKind {
        self._mut_coordination_status_dcsc().coordination_service_mode.get_or_insert(coordination_status_dcsc::COORDINATION_SERVICE_MODE.clone())
    }
    fn island(&self) -> &super::commonmodule::StatusSps {
        self._coordination_status_dcsc().island.as_ref().unwrap_or(&coordination_status_dcsc::ISLAND)
    }
    fn mut_island(&mut self) -> &mut super::commonmodule::StatusSps {
        self._mut_coordination_status_dcsc().island.get_or_insert(coordination_status_dcsc::ISLAND.clone())
    }
    fn permissible_auto(&self) -> &super::commonmodule::StatusSps {
        self._coordination_status_dcsc().permissible_auto.as_ref().unwrap_or(&coordination_status_dcsc::PERMISSIBLE_AUTO)
    }
    fn mut_permissible_auto(&mut self) -> &mut super::commonmodule::StatusSps {
        self._mut_coordination_status_dcsc().permissible_auto.get_or_insert(coordination_status_dcsc::PERMISSIBLE_AUTO.clone())
    }
    fn permissible_manual(&self) -> &super::commonmodule::StatusSps {
        self._coordination_status_dcsc().permissible_manual.as_ref().unwrap_or(&coordination_status_dcsc::PERMISSIBLE_MANUAL)
    }
    fn mut_permissible_manual(&mut self) -> &mut super::commonmodule::StatusSps {
        self._mut_coordination_status_dcsc().permissible_manual.get_or_insert(coordination_status_dcsc::PERMISSIBLE_MANUAL.clone())
    }
    fn permissible_netzero(&self) -> &super::commonmodule::StatusSps {
        self._coordination_status_dcsc().permissible_netzero.as_ref().unwrap_or(&coordination_status_dcsc::PERMISSIBLE_NETZERO)
    }
    fn mut_permissible_netzero(&mut self) -> &mut super::commonmodule::StatusSps {
        self._mut_coordination_status_dcsc().permissible_netzero.get_or_insert(coordination_status_dcsc::PERMISSIBLE_NETZERO.clone())
    }
    fn permissible_start(&self) -> &super::commonmodule::StatusSps {
        self._coordination_status_dcsc().permissible_start.as_ref().unwrap_or(&coordination_status_dcsc::PERMISSIBLE_START)
    }
    fn mut_permissible_start(&mut self) -> &mut super::commonmodule::StatusSps {
        self._mut_coordination_status_dcsc().permissible_start.get_or_insert(coordination_status_dcsc::PERMISSIBLE_START.clone())
    }
    fn permissible_stop(&self) -> &super::commonmodule::StatusSps {
        self._coordination_status_dcsc().permissible_stop.as_ref().unwrap_or(&coordination_status_dcsc::PERMISSIBLE_STOP)
    }
    fn mut_permissible_stop(&mut self) -> &mut super::commonmodule::StatusSps {
        self._mut_coordination_status_dcsc().permissible_stop.get_or_insert(coordination_status_dcsc::PERMISSIBLE_STOP.clone())
    }
}
impl IsCoordinationStatusDcsc for CoordinationStatusDcsc {
    fn _coordination_status_dcsc(&self) -> &CoordinationStatusDcsc {
        self
    }
    fn _mut_coordination_status_dcsc(&mut self) -> &mut CoordinationStatusDcsc {
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
pub trait IsCoordinationStatus {
    fn _coordination_status(&self) -> &CoordinationStatus;
    fn _mut_coordination_status(&mut self) -> &mut CoordinationStatus;
    fn event_value(&self) -> &super::commonmodule::EventValue {
        self._coordination_status().event_value.as_ref().unwrap_or(&coordination_status::EVENT_VALUE)
    }
    fn mut_event_value(&mut self) -> &mut super::commonmodule::EventValue {
        self._mut_coordination_status().event_value.get_or_insert(coordination_status::EVENT_VALUE.clone())
    }
    fn coordination_status_dcsc(&self) -> &CoordinationStatusDcsc {
        self._coordination_status().coordination_status_dcsc.as_ref().unwrap_or(&coordination_status::COORDINATION_STATUS_DCSC)
    }
    fn mut_coordination_status_dcsc(&mut self) -> &mut CoordinationStatusDcsc {
        self._mut_coordination_status().coordination_status_dcsc.get_or_insert(coordination_status::COORDINATION_STATUS_DCSC.clone())
    }
}
impl IsCoordinationStatus for CoordinationStatus {
    fn _coordination_status(&self) -> &CoordinationStatus {
        self
    }
    fn _mut_coordination_status(&mut self) -> &mut CoordinationStatus {
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
pub trait IsCoordinationStatusProfile {
    fn _coordination_status_profile(&self) -> &CoordinationStatusProfile;
    fn _mut_coordination_status_profile(&mut self) -> &mut CoordinationStatusProfile;
    fn event_message_info(&self) -> &super::commonmodule::EventMessageInfo {
        self._coordination_status_profile().event_message_info.as_ref().unwrap_or(&coordination_status_profile::EVENT_MESSAGE_INFO)
    }
    fn mut_event_message_info(&mut self) -> &mut super::commonmodule::EventMessageInfo {
        self._mut_coordination_status_profile().event_message_info.get_or_insert(coordination_status_profile::EVENT_MESSAGE_INFO.clone())
    }
    fn application_system(&self) -> &super::commonmodule::ApplicationSystem {
        self._coordination_status_profile().application_system.as_ref().unwrap_or(&coordination_status_profile::APPLICATION_SYSTEM)
    }
    fn mut_application_system(&mut self) -> &mut super::commonmodule::ApplicationSystem {
        self._mut_coordination_status_profile().application_system.get_or_insert(coordination_status_profile::APPLICATION_SYSTEM.clone())
    }
    fn coordination_status(&self) -> &CoordinationStatus {
        self._coordination_status_profile().coordination_status.as_ref().unwrap_or(&coordination_status_profile::COORDINATION_STATUS)
    }
    fn mut_coordination_status(&mut self) -> &mut CoordinationStatus {
        self._mut_coordination_status_profile().coordination_status.get_or_insert(coordination_status_profile::COORDINATION_STATUS.clone())
    }
}
impl IsCoordinationStatusProfile for CoordinationStatusProfile {
    fn _coordination_status_profile(&self) -> &CoordinationStatusProfile {
        self
    }
    fn _mut_coordination_status_profile(&mut self) -> &mut CoordinationStatusProfile {
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
