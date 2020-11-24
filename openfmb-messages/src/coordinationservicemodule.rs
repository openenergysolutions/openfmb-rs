use crate::commonmodule::*;
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize)]
pub struct OptionalCoordinationServiceModeKind {
    #[prost(enumeration="CoordinationServiceModeKind", tag="1")]
    #[serde(default)]
    pub value: i32,
}
mod optional_coordination_service_mode_kind {
    use lazy_static::lazy_static;
    lazy_static! {
    }
}
impl OptionalCoordinationServiceModeKind {
}
pub trait IsOptionalCoordinationServiceModeKind {
    fn _optional_coordination_service_mode_kind(&self) -> &OptionalCoordinationServiceModeKind;
    fn _optional_coordination_service_mode_kind_mut(&mut self) -> &mut OptionalCoordinationServiceModeKind;
    fn value(&self) -> i32 {
        self._optional_coordination_service_mode_kind().value
    }
    fn value_mut(&mut self) -> &mut i32 {
        &mut self._optional_coordination_service_mode_kind_mut().value
    }
}
impl IsOptionalCoordinationServiceModeKind for OptionalCoordinationServiceModeKind {
    fn _optional_coordination_service_mode_kind(&self) -> &OptionalCoordinationServiceModeKind {
        self
    }
    fn _optional_coordination_service_mode_kind_mut(&mut self) -> &mut OptionalCoordinationServiceModeKind {
        self
    }
}
/// Coordination service mode kind
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize)]
pub struct EngCoordinationServiceModeKind {
    /// The value of the coordination service mode.
    // parent_message: false
    // required_field: true
    // multiplicity_min: Some(1)
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(enumeration="CoordinationServiceModeKind", tag="1")]
    #[serde(default, rename = "setVal")]
    pub set_val: i32,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    #[serde(default, rename = "setValExtension")]
    pub set_val_extension: ::std::option::Option<::std::string::String>,
}
mod eng_coordination_service_mode_kind {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref SET_VAL_EXTENSION: ::std::string::String = Default::default();
    }
}
impl EngCoordinationServiceModeKind {
}
pub trait IsEngCoordinationServiceModeKind {
    fn _eng_coordination_service_mode_kind(&self) -> &EngCoordinationServiceModeKind;
    fn _eng_coordination_service_mode_kind_mut(&mut self) -> &mut EngCoordinationServiceModeKind;
    fn set_val(&self) -> i32 {
        self._eng_coordination_service_mode_kind().set_val
    }
    fn set_val_mut(&mut self) -> &mut i32 {
        &mut self._eng_coordination_service_mode_kind_mut().set_val
    }
    fn set_val_extension(&self) -> &::std::string::String {
        self._eng_coordination_service_mode_kind().set_val_extension.as_ref().unwrap_or(&eng_coordination_service_mode_kind::SET_VAL_EXTENSION)
    }
    fn set_val_extension_mut(&mut self) -> &mut ::std::string::String {
        self._eng_coordination_service_mode_kind_mut().set_val_extension.get_or_insert(Default::default())
    }
}
impl IsEngCoordinationServiceModeKind for EngCoordinationServiceModeKind {
    fn _eng_coordination_service_mode_kind(&self) -> &EngCoordinationServiceModeKind {
        self
    }
    fn _eng_coordination_service_mode_kind_mut(&mut self) -> &mut EngCoordinationServiceModeKind {
        self
    }
}
/// OpenFMB specialization for coordination service control, DCSC (Distributed Coordination Service
/// Control), following 61850 naming convention.
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize)]
pub struct CoordinationControlDcsc {
    /// UML inherited base object
    // parent_message: true
    // required_field: false
    // multiplicity_min: None
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, optional, tag="1")]
    #[serde(default, rename = "logicalNodeForControl")]
    pub logical_node_for_control: ::std::option::Option<super::commonmodule::LogicalNodeForControl>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    #[serde(default, rename = "CoordinationServiceMode")]
    pub coordination_service_mode: ::std::option::Option<EngCoordinationServiceModeKind>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    #[serde(default, rename = "Island")]
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
impl CoordinationControlDcsc {
    pub(crate) fn parent(&self) -> &super::commonmodule::LogicalNodeForControl {
        self.logical_node_for_control.as_ref().unwrap_or(&coordination_control_dcsc::LOGICAL_NODE_FOR_CONTROL)
    }
    pub(crate) fn parent_mut(&mut self) -> &mut super::commonmodule::LogicalNodeForControl {
        self.logical_node_for_control.get_or_insert(Default::default())
    }
}
pub trait IsCoordinationControlDcsc {
    fn _coordination_control_dcsc(&self) -> &CoordinationControlDcsc;
    fn _coordination_control_dcsc_mut(&mut self) -> &mut CoordinationControlDcsc;
    fn logical_node_for_control(&self) -> &super::commonmodule::LogicalNodeForControl {
        self._coordination_control_dcsc().logical_node_for_control.as_ref().unwrap_or(&coordination_control_dcsc::LOGICAL_NODE_FOR_CONTROL)
    }
    fn logical_node_for_control_mut(&mut self) -> &mut super::commonmodule::LogicalNodeForControl {
        self._coordination_control_dcsc_mut().logical_node_for_control.get_or_insert(Default::default())
    }
    fn coordination_service_mode(&self) -> &EngCoordinationServiceModeKind {
        self._coordination_control_dcsc().coordination_service_mode.as_ref().unwrap_or(&coordination_control_dcsc::COORDINATION_SERVICE_MODE)
    }
    fn coordination_service_mode_mut(&mut self) -> &mut EngCoordinationServiceModeKind {
        self._coordination_control_dcsc_mut().coordination_service_mode.get_or_insert(Default::default())
    }
    fn island(&self) -> &super::commonmodule::ControlDpc {
        self._coordination_control_dcsc().island.as_ref().unwrap_or(&coordination_control_dcsc::ISLAND)
    }
    fn island_mut(&mut self) -> &mut super::commonmodule::ControlDpc {
        self._coordination_control_dcsc_mut().island.get_or_insert(Default::default())
    }
}
impl IsCoordinationControlDcsc for CoordinationControlDcsc {
    fn _coordination_control_dcsc(&self) -> &CoordinationControlDcsc {
        self
    }
    fn _coordination_control_dcsc_mut(&mut self) -> &mut CoordinationControlDcsc {
        self
    }
}
impl IsLogicalNodeForControl for CoordinationControlDcsc {
    fn _logical_node_for_control(&self) -> &super::commonmodule::LogicalNodeForControl {
        self.parent()
    }
    fn _logical_node_for_control_mut(&mut self) -> &mut LogicalNodeForControl {
        self.parent_mut()
    }
}
impl IsLogicalNode for CoordinationControlDcsc {
    fn _logical_node(&self) -> &super::commonmodule::LogicalNode {
        self.parent().parent()
    }
    fn _logical_node_mut(&mut self) -> &mut LogicalNode {
        self.parent_mut().parent_mut()
    }
}
impl IsIdentifiedObject for CoordinationControlDcsc {
    fn _identified_object(&self) -> &super::commonmodule::IdentifiedObject {
        self.parent().parent().parent()
    }
    fn _identified_object_mut(&mut self) -> &mut IdentifiedObject {
        self.parent_mut().parent_mut().parent_mut()
    }
}
/// Switch discrete control
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize)]
pub struct CoordinationControl {
    /// UML inherited base object
    // parent_message: true
    // required_field: false
    // multiplicity_min: None
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, optional, tag="1")]
    #[serde(default, rename = "identifiedObject")]
    pub identified_object: ::std::option::Option<super::commonmodule::IdentifiedObject>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    #[serde(default)]
    pub check: ::std::option::Option<super::commonmodule::CheckConditions>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: Some(1)
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, optional, tag="3")]
    #[serde(default, rename = "coordinationControlDCSC")]
    pub coordination_control_dcsc: ::std::option::Option<CoordinationControlDcsc>,
}
mod coordination_control {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref IDENTIFIED_OBJECT: crate::commonmodule::IdentifiedObject = Default::default();
        pub(super) static ref CHECK: crate::commonmodule::CheckConditions = Default::default();
        pub(super) static ref COORDINATION_CONTROL_DCSC: crate::coordinationservicemodule::CoordinationControlDcsc = Default::default();
    }
}
impl CoordinationControl {
    pub(crate) fn parent(&self) -> &super::commonmodule::IdentifiedObject {
        self.identified_object.as_ref().unwrap_or(&coordination_control::IDENTIFIED_OBJECT)
    }
    pub(crate) fn parent_mut(&mut self) -> &mut super::commonmodule::IdentifiedObject {
        self.identified_object.get_or_insert(Default::default())
    }
}
pub trait IsCoordinationControl {
    fn _coordination_control(&self) -> &CoordinationControl;
    fn _coordination_control_mut(&mut self) -> &mut CoordinationControl;
    fn identified_object(&self) -> &super::commonmodule::IdentifiedObject {
        self._coordination_control().identified_object.as_ref().unwrap_or(&coordination_control::IDENTIFIED_OBJECT)
    }
    fn identified_object_mut(&mut self) -> &mut super::commonmodule::IdentifiedObject {
        self._coordination_control_mut().identified_object.get_or_insert(Default::default())
    }
    fn check(&self) -> &super::commonmodule::CheckConditions {
        self._coordination_control().check.as_ref().unwrap_or(&coordination_control::CHECK)
    }
    fn check_mut(&mut self) -> &mut super::commonmodule::CheckConditions {
        self._coordination_control_mut().check.get_or_insert(Default::default())
    }
    fn coordination_control_dcsc(&self) -> &CoordinationControlDcsc {
        self._coordination_control().coordination_control_dcsc.as_ref().unwrap_or(&coordination_control::COORDINATION_CONTROL_DCSC)
    }
    fn coordination_control_dcsc_mut(&mut self) -> &mut CoordinationControlDcsc {
        self._coordination_control_mut().coordination_control_dcsc.get_or_insert(Default::default())
    }
}
impl IsCoordinationControl for CoordinationControl {
    fn _coordination_control(&self) -> &CoordinationControl {
        self
    }
    fn _coordination_control_mut(&mut self) -> &mut CoordinationControl {
        self
    }
}
impl IsIdentifiedObject for CoordinationControl {
    fn _identified_object(&self) -> &super::commonmodule::IdentifiedObject {
        self.parent()
    }
    fn _identified_object_mut(&mut self) -> &mut IdentifiedObject {
        self.parent_mut()
    }
}
/// Switch control profile
/// OpenFMB Profile Message: true
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize)]
pub struct CoordinationControlProfile {
    /// UML inherited base object
    // parent_message: true
    // required_field: false
    // multiplicity_min: None
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, optional, tag="1")]
    #[serde(default, rename = "controlMessageInfo")]
    pub control_message_info: ::std::option::Option<super::commonmodule::ControlMessageInfo>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: Some(1)
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, optional, tag="2")]
    #[serde(default, rename = "applicationSystem")]
    pub application_system: ::std::option::Option<super::commonmodule::ApplicationSystem>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: Some(1)
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, optional, tag="3")]
    #[serde(default, rename = "coordinationControl")]
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
impl CoordinationControlProfile {
    pub(crate) fn parent(&self) -> &super::commonmodule::ControlMessageInfo {
        self.control_message_info.as_ref().unwrap_or(&coordination_control_profile::CONTROL_MESSAGE_INFO)
    }
    pub(crate) fn parent_mut(&mut self) -> &mut super::commonmodule::ControlMessageInfo {
        self.control_message_info.get_or_insert(Default::default())
    }
}
pub trait IsCoordinationControlProfile {
    fn _coordination_control_profile(&self) -> &CoordinationControlProfile;
    fn _coordination_control_profile_mut(&mut self) -> &mut CoordinationControlProfile;
    fn control_message_info(&self) -> &super::commonmodule::ControlMessageInfo {
        self._coordination_control_profile().control_message_info.as_ref().unwrap_or(&coordination_control_profile::CONTROL_MESSAGE_INFO)
    }
    fn control_message_info_mut(&mut self) -> &mut super::commonmodule::ControlMessageInfo {
        self._coordination_control_profile_mut().control_message_info.get_or_insert(Default::default())
    }
    fn application_system(&self) -> &super::commonmodule::ApplicationSystem {
        self._coordination_control_profile().application_system.as_ref().unwrap_or(&coordination_control_profile::APPLICATION_SYSTEM)
    }
    fn application_system_mut(&mut self) -> &mut super::commonmodule::ApplicationSystem {
        self._coordination_control_profile_mut().application_system.get_or_insert(Default::default())
    }
    fn coordination_control(&self) -> &CoordinationControl {
        self._coordination_control_profile().coordination_control.as_ref().unwrap_or(&coordination_control_profile::COORDINATION_CONTROL)
    }
    fn coordination_control_mut(&mut self) -> &mut CoordinationControl {
        self._coordination_control_profile_mut().coordination_control.get_or_insert(Default::default())
    }
}
impl IsCoordinationControlProfile for CoordinationControlProfile {
    fn _coordination_control_profile(&self) -> &CoordinationControlProfile {
        self
    }
    fn _coordination_control_profile_mut(&mut self) -> &mut CoordinationControlProfile {
        self
    }
}
impl IsControlMessageInfo for CoordinationControlProfile {
    fn _control_message_info(&self) -> &super::commonmodule::ControlMessageInfo {
        self.parent()
    }
    fn _control_message_info_mut(&mut self) -> &mut ControlMessageInfo {
        self.parent_mut()
    }
}
impl IsMessageInfo for CoordinationControlProfile {
    fn _message_info(&self) -> &super::commonmodule::MessageInfo {
        self.parent().parent()
    }
    fn _message_info_mut(&mut self) -> &mut MessageInfo {
        self.parent_mut().parent_mut()
    }
}
impl IsIdentifiedObject for CoordinationControlProfile {
    fn _identified_object(&self) -> &super::commonmodule::IdentifiedObject {
        self.parent().parent().parent()
    }
    fn _identified_object_mut(&mut self) -> &mut IdentifiedObject {
        self.parent_mut().parent_mut().parent_mut()
    }
}
/// OpenFMB specialization for coordination service control, DCSC (Distributed Coordination Service
/// Control), following 61850 naming convention.
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize)]
pub struct CoordinationEventDcsc {
    /// UML inherited base object
    // parent_message: true
    // required_field: false
    // multiplicity_min: None
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, optional, tag="1")]
    #[serde(default, rename = "logicalNode")]
    pub logical_node: ::std::option::Option<super::commonmodule::LogicalNode>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    #[serde(default, rename = "CoordinationServiceMode")]
    pub coordination_service_mode: ::std::option::Option<EngCoordinationServiceModeKind>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    #[serde(default, rename = "Island")]
    pub island: ::std::option::Option<super::commonmodule::StatusSps>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="4")]
    #[serde(default, rename = "PermissibleAuto")]
    pub permissible_auto: ::std::option::Option<super::commonmodule::StatusSps>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="5")]
    #[serde(default, rename = "PermissibleManual")]
    pub permissible_manual: ::std::option::Option<super::commonmodule::StatusSps>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="6")]
    #[serde(default, rename = "PermissibleNetzero")]
    pub permissible_netzero: ::std::option::Option<super::commonmodule::StatusSps>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="7")]
    #[serde(default, rename = "PermissibleStart")]
    pub permissible_start: ::std::option::Option<super::commonmodule::StatusSps>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="8")]
    #[serde(default, rename = "PermissibleStop")]
    pub permissible_stop: ::std::option::Option<super::commonmodule::StatusSps>,
}
mod coordination_event_dcsc {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref LOGICAL_NODE: crate::commonmodule::LogicalNode = Default::default();
        pub(super) static ref COORDINATION_SERVICE_MODE: crate::coordinationservicemodule::EngCoordinationServiceModeKind = Default::default();
        pub(super) static ref ISLAND: crate::commonmodule::StatusSps = Default::default();
        pub(super) static ref PERMISSIBLE_AUTO: crate::commonmodule::StatusSps = Default::default();
        pub(super) static ref PERMISSIBLE_MANUAL: crate::commonmodule::StatusSps = Default::default();
        pub(super) static ref PERMISSIBLE_NETZERO: crate::commonmodule::StatusSps = Default::default();
        pub(super) static ref PERMISSIBLE_START: crate::commonmodule::StatusSps = Default::default();
        pub(super) static ref PERMISSIBLE_STOP: crate::commonmodule::StatusSps = Default::default();
    }
}
impl CoordinationEventDcsc {
    pub(crate) fn parent(&self) -> &super::commonmodule::LogicalNode {
        self.logical_node.as_ref().unwrap_or(&coordination_event_dcsc::LOGICAL_NODE)
    }
    pub(crate) fn parent_mut(&mut self) -> &mut super::commonmodule::LogicalNode {
        self.logical_node.get_or_insert(Default::default())
    }
}
pub trait IsCoordinationEventDcsc {
    fn _coordination_event_dcsc(&self) -> &CoordinationEventDcsc;
    fn _coordination_event_dcsc_mut(&mut self) -> &mut CoordinationEventDcsc;
    fn logical_node(&self) -> &super::commonmodule::LogicalNode {
        self._coordination_event_dcsc().logical_node.as_ref().unwrap_or(&coordination_event_dcsc::LOGICAL_NODE)
    }
    fn logical_node_mut(&mut self) -> &mut super::commonmodule::LogicalNode {
        self._coordination_event_dcsc_mut().logical_node.get_or_insert(Default::default())
    }
    fn coordination_service_mode(&self) -> &EngCoordinationServiceModeKind {
        self._coordination_event_dcsc().coordination_service_mode.as_ref().unwrap_or(&coordination_event_dcsc::COORDINATION_SERVICE_MODE)
    }
    fn coordination_service_mode_mut(&mut self) -> &mut EngCoordinationServiceModeKind {
        self._coordination_event_dcsc_mut().coordination_service_mode.get_or_insert(Default::default())
    }
    fn island(&self) -> &super::commonmodule::StatusSps {
        self._coordination_event_dcsc().island.as_ref().unwrap_or(&coordination_event_dcsc::ISLAND)
    }
    fn island_mut(&mut self) -> &mut super::commonmodule::StatusSps {
        self._coordination_event_dcsc_mut().island.get_or_insert(Default::default())
    }
    fn permissible_auto(&self) -> &super::commonmodule::StatusSps {
        self._coordination_event_dcsc().permissible_auto.as_ref().unwrap_or(&coordination_event_dcsc::PERMISSIBLE_AUTO)
    }
    fn permissible_auto_mut(&mut self) -> &mut super::commonmodule::StatusSps {
        self._coordination_event_dcsc_mut().permissible_auto.get_or_insert(Default::default())
    }
    fn permissible_manual(&self) -> &super::commonmodule::StatusSps {
        self._coordination_event_dcsc().permissible_manual.as_ref().unwrap_or(&coordination_event_dcsc::PERMISSIBLE_MANUAL)
    }
    fn permissible_manual_mut(&mut self) -> &mut super::commonmodule::StatusSps {
        self._coordination_event_dcsc_mut().permissible_manual.get_or_insert(Default::default())
    }
    fn permissible_netzero(&self) -> &super::commonmodule::StatusSps {
        self._coordination_event_dcsc().permissible_netzero.as_ref().unwrap_or(&coordination_event_dcsc::PERMISSIBLE_NETZERO)
    }
    fn permissible_netzero_mut(&mut self) -> &mut super::commonmodule::StatusSps {
        self._coordination_event_dcsc_mut().permissible_netzero.get_or_insert(Default::default())
    }
    fn permissible_start(&self) -> &super::commonmodule::StatusSps {
        self._coordination_event_dcsc().permissible_start.as_ref().unwrap_or(&coordination_event_dcsc::PERMISSIBLE_START)
    }
    fn permissible_start_mut(&mut self) -> &mut super::commonmodule::StatusSps {
        self._coordination_event_dcsc_mut().permissible_start.get_or_insert(Default::default())
    }
    fn permissible_stop(&self) -> &super::commonmodule::StatusSps {
        self._coordination_event_dcsc().permissible_stop.as_ref().unwrap_or(&coordination_event_dcsc::PERMISSIBLE_STOP)
    }
    fn permissible_stop_mut(&mut self) -> &mut super::commonmodule::StatusSps {
        self._coordination_event_dcsc_mut().permissible_stop.get_or_insert(Default::default())
    }
}
impl IsCoordinationEventDcsc for CoordinationEventDcsc {
    fn _coordination_event_dcsc(&self) -> &CoordinationEventDcsc {
        self
    }
    fn _coordination_event_dcsc_mut(&mut self) -> &mut CoordinationEventDcsc {
        self
    }
}
impl IsLogicalNode for CoordinationEventDcsc {
    fn _logical_node(&self) -> &super::commonmodule::LogicalNode {
        self.parent()
    }
    fn _logical_node_mut(&mut self) -> &mut LogicalNode {
        self.parent_mut()
    }
}
impl IsIdentifiedObject for CoordinationEventDcsc {
    fn _identified_object(&self) -> &super::commonmodule::IdentifiedObject {
        self.parent().parent()
    }
    fn _identified_object_mut(&mut self) -> &mut IdentifiedObject {
        self.parent_mut().parent_mut()
    }
}
/// Switch event
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize)]
pub struct CoordinationEvent {
    /// UML inherited base object
    // parent_message: true
    // required_field: false
    // multiplicity_min: None
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, optional, tag="1")]
    #[serde(default, rename = "identifiedObject")]
    pub identified_object: ::std::option::Option<super::commonmodule::IdentifiedObject>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: Some(1)
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, optional, tag="2")]
    #[serde(default, rename = "coordinationEventDCSC")]
    pub coordination_event_dcsc: ::std::option::Option<CoordinationEventDcsc>,
}
mod coordination_event {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref IDENTIFIED_OBJECT: crate::commonmodule::IdentifiedObject = Default::default();
        pub(super) static ref COORDINATION_EVENT_DCSC: crate::coordinationservicemodule::CoordinationEventDcsc = Default::default();
    }
}
impl CoordinationEvent {
    pub(crate) fn parent(&self) -> &super::commonmodule::IdentifiedObject {
        self.identified_object.as_ref().unwrap_or(&coordination_event::IDENTIFIED_OBJECT)
    }
    pub(crate) fn parent_mut(&mut self) -> &mut super::commonmodule::IdentifiedObject {
        self.identified_object.get_or_insert(Default::default())
    }
}
pub trait IsCoordinationEvent {
    fn _coordination_event(&self) -> &CoordinationEvent;
    fn _coordination_event_mut(&mut self) -> &mut CoordinationEvent;
    fn identified_object(&self) -> &super::commonmodule::IdentifiedObject {
        self._coordination_event().identified_object.as_ref().unwrap_or(&coordination_event::IDENTIFIED_OBJECT)
    }
    fn identified_object_mut(&mut self) -> &mut super::commonmodule::IdentifiedObject {
        self._coordination_event_mut().identified_object.get_or_insert(Default::default())
    }
    fn coordination_event_dcsc(&self) -> &CoordinationEventDcsc {
        self._coordination_event().coordination_event_dcsc.as_ref().unwrap_or(&coordination_event::COORDINATION_EVENT_DCSC)
    }
    fn coordination_event_dcsc_mut(&mut self) -> &mut CoordinationEventDcsc {
        self._coordination_event_mut().coordination_event_dcsc.get_or_insert(Default::default())
    }
}
impl IsCoordinationEvent for CoordinationEvent {
    fn _coordination_event(&self) -> &CoordinationEvent {
        self
    }
    fn _coordination_event_mut(&mut self) -> &mut CoordinationEvent {
        self
    }
}
impl IsIdentifiedObject for CoordinationEvent {
    fn _identified_object(&self) -> &super::commonmodule::IdentifiedObject {
        self.parent()
    }
    fn _identified_object_mut(&mut self) -> &mut IdentifiedObject {
        self.parent_mut()
    }
}
/// Switch event profile
/// OpenFMB Profile Message: true
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize)]
pub struct CoordinationEventProfile {
    /// UML inherited base object
    // parent_message: true
    // required_field: false
    // multiplicity_min: None
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, optional, tag="1")]
    #[serde(default, rename = "eventMessageInfo")]
    pub event_message_info: ::std::option::Option<super::commonmodule::EventMessageInfo>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: Some(1)
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, optional, tag="2")]
    #[serde(default, rename = "applicationSystem")]
    pub application_system: ::std::option::Option<super::commonmodule::ApplicationSystem>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: Some(1)
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, optional, tag="3")]
    #[serde(default, rename = "coordinationEvent")]
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
impl CoordinationEventProfile {
    pub(crate) fn parent(&self) -> &super::commonmodule::EventMessageInfo {
        self.event_message_info.as_ref().unwrap_or(&coordination_event_profile::EVENT_MESSAGE_INFO)
    }
    pub(crate) fn parent_mut(&mut self) -> &mut super::commonmodule::EventMessageInfo {
        self.event_message_info.get_or_insert(Default::default())
    }
}
pub trait IsCoordinationEventProfile {
    fn _coordination_event_profile(&self) -> &CoordinationEventProfile;
    fn _coordination_event_profile_mut(&mut self) -> &mut CoordinationEventProfile;
    fn event_message_info(&self) -> &super::commonmodule::EventMessageInfo {
        self._coordination_event_profile().event_message_info.as_ref().unwrap_or(&coordination_event_profile::EVENT_MESSAGE_INFO)
    }
    fn event_message_info_mut(&mut self) -> &mut super::commonmodule::EventMessageInfo {
        self._coordination_event_profile_mut().event_message_info.get_or_insert(Default::default())
    }
    fn application_system(&self) -> &super::commonmodule::ApplicationSystem {
        self._coordination_event_profile().application_system.as_ref().unwrap_or(&coordination_event_profile::APPLICATION_SYSTEM)
    }
    fn application_system_mut(&mut self) -> &mut super::commonmodule::ApplicationSystem {
        self._coordination_event_profile_mut().application_system.get_or_insert(Default::default())
    }
    fn coordination_event(&self) -> &CoordinationEvent {
        self._coordination_event_profile().coordination_event.as_ref().unwrap_or(&coordination_event_profile::COORDINATION_EVENT)
    }
    fn coordination_event_mut(&mut self) -> &mut CoordinationEvent {
        self._coordination_event_profile_mut().coordination_event.get_or_insert(Default::default())
    }
}
impl IsCoordinationEventProfile for CoordinationEventProfile {
    fn _coordination_event_profile(&self) -> &CoordinationEventProfile {
        self
    }
    fn _coordination_event_profile_mut(&mut self) -> &mut CoordinationEventProfile {
        self
    }
}
impl IsEventMessageInfo for CoordinationEventProfile {
    fn _event_message_info(&self) -> &super::commonmodule::EventMessageInfo {
        self.parent()
    }
    fn _event_message_info_mut(&mut self) -> &mut EventMessageInfo {
        self.parent_mut()
    }
}
impl IsMessageInfo for CoordinationEventProfile {
    fn _message_info(&self) -> &super::commonmodule::MessageInfo {
        self.parent().parent()
    }
    fn _message_info_mut(&mut self) -> &mut MessageInfo {
        self.parent_mut().parent_mut()
    }
}
impl IsIdentifiedObject for CoordinationEventProfile {
    fn _identified_object(&self) -> &super::commonmodule::IdentifiedObject {
        self.parent().parent().parent()
    }
    fn _identified_object_mut(&mut self) -> &mut IdentifiedObject {
        self.parent_mut().parent_mut().parent_mut()
    }
}
/// OpenFMB specialization for coordination service control, DCSC (Distributed Coordination Service
/// Control), following 61850 naming convention.
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize)]
pub struct CoordinationStatusDcsc {
    /// UML inherited base object
    // parent_message: true
    // required_field: false
    // multiplicity_min: None
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, optional, tag="1")]
    #[serde(default, rename = "logicalNode")]
    pub logical_node: ::std::option::Option<super::commonmodule::LogicalNode>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    #[serde(default, rename = "CoordinationServiceMode")]
    pub coordination_service_mode: ::std::option::Option<EngCoordinationServiceModeKind>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    #[serde(default, rename = "Island")]
    pub island: ::std::option::Option<super::commonmodule::StatusSps>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="4")]
    #[serde(default, rename = "PermissibleAuto")]
    pub permissible_auto: ::std::option::Option<super::commonmodule::StatusSps>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="5")]
    #[serde(default, rename = "PermissibleManual")]
    pub permissible_manual: ::std::option::Option<super::commonmodule::StatusSps>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="6")]
    #[serde(default, rename = "PermissibleNetzero")]
    pub permissible_netzero: ::std::option::Option<super::commonmodule::StatusSps>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="7")]
    #[serde(default, rename = "PermissibleStart")]
    pub permissible_start: ::std::option::Option<super::commonmodule::StatusSps>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="8")]
    #[serde(default, rename = "PermissibleStop")]
    pub permissible_stop: ::std::option::Option<super::commonmodule::StatusSps>,
}
mod coordination_status_dcsc {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref LOGICAL_NODE: crate::commonmodule::LogicalNode = Default::default();
        pub(super) static ref COORDINATION_SERVICE_MODE: crate::coordinationservicemodule::EngCoordinationServiceModeKind = Default::default();
        pub(super) static ref ISLAND: crate::commonmodule::StatusSps = Default::default();
        pub(super) static ref PERMISSIBLE_AUTO: crate::commonmodule::StatusSps = Default::default();
        pub(super) static ref PERMISSIBLE_MANUAL: crate::commonmodule::StatusSps = Default::default();
        pub(super) static ref PERMISSIBLE_NETZERO: crate::commonmodule::StatusSps = Default::default();
        pub(super) static ref PERMISSIBLE_START: crate::commonmodule::StatusSps = Default::default();
        pub(super) static ref PERMISSIBLE_STOP: crate::commonmodule::StatusSps = Default::default();
    }
}
impl CoordinationStatusDcsc {
    pub(crate) fn parent(&self) -> &super::commonmodule::LogicalNode {
        self.logical_node.as_ref().unwrap_or(&coordination_status_dcsc::LOGICAL_NODE)
    }
    pub(crate) fn parent_mut(&mut self) -> &mut super::commonmodule::LogicalNode {
        self.logical_node.get_or_insert(Default::default())
    }
}
pub trait IsCoordinationStatusDcsc {
    fn _coordination_status_dcsc(&self) -> &CoordinationStatusDcsc;
    fn _coordination_status_dcsc_mut(&mut self) -> &mut CoordinationStatusDcsc;
    fn logical_node(&self) -> &super::commonmodule::LogicalNode {
        self._coordination_status_dcsc().logical_node.as_ref().unwrap_or(&coordination_status_dcsc::LOGICAL_NODE)
    }
    fn logical_node_mut(&mut self) -> &mut super::commonmodule::LogicalNode {
        self._coordination_status_dcsc_mut().logical_node.get_or_insert(Default::default())
    }
    fn coordination_service_mode(&self) -> &EngCoordinationServiceModeKind {
        self._coordination_status_dcsc().coordination_service_mode.as_ref().unwrap_or(&coordination_status_dcsc::COORDINATION_SERVICE_MODE)
    }
    fn coordination_service_mode_mut(&mut self) -> &mut EngCoordinationServiceModeKind {
        self._coordination_status_dcsc_mut().coordination_service_mode.get_or_insert(Default::default())
    }
    fn island(&self) -> &super::commonmodule::StatusSps {
        self._coordination_status_dcsc().island.as_ref().unwrap_or(&coordination_status_dcsc::ISLAND)
    }
    fn island_mut(&mut self) -> &mut super::commonmodule::StatusSps {
        self._coordination_status_dcsc_mut().island.get_or_insert(Default::default())
    }
    fn permissible_auto(&self) -> &super::commonmodule::StatusSps {
        self._coordination_status_dcsc().permissible_auto.as_ref().unwrap_or(&coordination_status_dcsc::PERMISSIBLE_AUTO)
    }
    fn permissible_auto_mut(&mut self) -> &mut super::commonmodule::StatusSps {
        self._coordination_status_dcsc_mut().permissible_auto.get_or_insert(Default::default())
    }
    fn permissible_manual(&self) -> &super::commonmodule::StatusSps {
        self._coordination_status_dcsc().permissible_manual.as_ref().unwrap_or(&coordination_status_dcsc::PERMISSIBLE_MANUAL)
    }
    fn permissible_manual_mut(&mut self) -> &mut super::commonmodule::StatusSps {
        self._coordination_status_dcsc_mut().permissible_manual.get_or_insert(Default::default())
    }
    fn permissible_netzero(&self) -> &super::commonmodule::StatusSps {
        self._coordination_status_dcsc().permissible_netzero.as_ref().unwrap_or(&coordination_status_dcsc::PERMISSIBLE_NETZERO)
    }
    fn permissible_netzero_mut(&mut self) -> &mut super::commonmodule::StatusSps {
        self._coordination_status_dcsc_mut().permissible_netzero.get_or_insert(Default::default())
    }
    fn permissible_start(&self) -> &super::commonmodule::StatusSps {
        self._coordination_status_dcsc().permissible_start.as_ref().unwrap_or(&coordination_status_dcsc::PERMISSIBLE_START)
    }
    fn permissible_start_mut(&mut self) -> &mut super::commonmodule::StatusSps {
        self._coordination_status_dcsc_mut().permissible_start.get_or_insert(Default::default())
    }
    fn permissible_stop(&self) -> &super::commonmodule::StatusSps {
        self._coordination_status_dcsc().permissible_stop.as_ref().unwrap_or(&coordination_status_dcsc::PERMISSIBLE_STOP)
    }
    fn permissible_stop_mut(&mut self) -> &mut super::commonmodule::StatusSps {
        self._coordination_status_dcsc_mut().permissible_stop.get_or_insert(Default::default())
    }
}
impl IsCoordinationStatusDcsc for CoordinationStatusDcsc {
    fn _coordination_status_dcsc(&self) -> &CoordinationStatusDcsc {
        self
    }
    fn _coordination_status_dcsc_mut(&mut self) -> &mut CoordinationStatusDcsc {
        self
    }
}
impl IsLogicalNode for CoordinationStatusDcsc {
    fn _logical_node(&self) -> &super::commonmodule::LogicalNode {
        self.parent()
    }
    fn _logical_node_mut(&mut self) -> &mut LogicalNode {
        self.parent_mut()
    }
}
impl IsIdentifiedObject for CoordinationStatusDcsc {
    fn _identified_object(&self) -> &super::commonmodule::IdentifiedObject {
        self.parent().parent()
    }
    fn _identified_object_mut(&mut self) -> &mut IdentifiedObject {
        self.parent_mut().parent_mut()
    }
}
/// Switch event
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize)]
pub struct CoordinationStatus {
    /// UML inherited base object
    // parent_message: true
    // required_field: false
    // multiplicity_min: None
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, optional, tag="1")]
    #[serde(default, rename = "identifiedObject")]
    pub identified_object: ::std::option::Option<super::commonmodule::IdentifiedObject>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: Some(1)
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, optional, tag="2")]
    #[serde(default, rename = "coordinationStatusDCSC")]
    pub coordination_status_dcsc: ::std::option::Option<CoordinationStatusDcsc>,
}
mod coordination_status {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref IDENTIFIED_OBJECT: crate::commonmodule::IdentifiedObject = Default::default();
        pub(super) static ref COORDINATION_STATUS_DCSC: crate::coordinationservicemodule::CoordinationStatusDcsc = Default::default();
    }
}
impl CoordinationStatus {
    pub(crate) fn parent(&self) -> &super::commonmodule::IdentifiedObject {
        self.identified_object.as_ref().unwrap_or(&coordination_status::IDENTIFIED_OBJECT)
    }
    pub(crate) fn parent_mut(&mut self) -> &mut super::commonmodule::IdentifiedObject {
        self.identified_object.get_or_insert(Default::default())
    }
}
pub trait IsCoordinationStatus {
    fn _coordination_status(&self) -> &CoordinationStatus;
    fn _coordination_status_mut(&mut self) -> &mut CoordinationStatus;
    fn identified_object(&self) -> &super::commonmodule::IdentifiedObject {
        self._coordination_status().identified_object.as_ref().unwrap_or(&coordination_status::IDENTIFIED_OBJECT)
    }
    fn identified_object_mut(&mut self) -> &mut super::commonmodule::IdentifiedObject {
        self._coordination_status_mut().identified_object.get_or_insert(Default::default())
    }
    fn coordination_status_dcsc(&self) -> &CoordinationStatusDcsc {
        self._coordination_status().coordination_status_dcsc.as_ref().unwrap_or(&coordination_status::COORDINATION_STATUS_DCSC)
    }
    fn coordination_status_dcsc_mut(&mut self) -> &mut CoordinationStatusDcsc {
        self._coordination_status_mut().coordination_status_dcsc.get_or_insert(Default::default())
    }
}
impl IsCoordinationStatus for CoordinationStatus {
    fn _coordination_status(&self) -> &CoordinationStatus {
        self
    }
    fn _coordination_status_mut(&mut self) -> &mut CoordinationStatus {
        self
    }
}
impl IsIdentifiedObject for CoordinationStatus {
    fn _identified_object(&self) -> &super::commonmodule::IdentifiedObject {
        self.parent()
    }
    fn _identified_object_mut(&mut self) -> &mut IdentifiedObject {
        self.parent_mut()
    }
}
/// Switch event profile
/// OpenFMB Profile Message: true
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize)]
pub struct CoordinationStatusProfile {
    /// UML inherited base object
    // parent_message: true
    // required_field: false
    // multiplicity_min: None
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, optional, tag="1")]
    #[serde(default, rename = "eventMessageInfo")]
    pub event_message_info: ::std::option::Option<super::commonmodule::EventMessageInfo>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: Some(1)
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, optional, tag="2")]
    #[serde(default, rename = "applicationSystem")]
    pub application_system: ::std::option::Option<super::commonmodule::ApplicationSystem>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: Some(1)
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, optional, tag="3")]
    #[serde(default, rename = "coordinationStatus")]
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
impl CoordinationStatusProfile {
    pub(crate) fn parent(&self) -> &super::commonmodule::EventMessageInfo {
        self.event_message_info.as_ref().unwrap_or(&coordination_status_profile::EVENT_MESSAGE_INFO)
    }
    pub(crate) fn parent_mut(&mut self) -> &mut super::commonmodule::EventMessageInfo {
        self.event_message_info.get_or_insert(Default::default())
    }
}
pub trait IsCoordinationStatusProfile {
    fn _coordination_status_profile(&self) -> &CoordinationStatusProfile;
    fn _coordination_status_profile_mut(&mut self) -> &mut CoordinationStatusProfile;
    fn event_message_info(&self) -> &super::commonmodule::EventMessageInfo {
        self._coordination_status_profile().event_message_info.as_ref().unwrap_or(&coordination_status_profile::EVENT_MESSAGE_INFO)
    }
    fn event_message_info_mut(&mut self) -> &mut super::commonmodule::EventMessageInfo {
        self._coordination_status_profile_mut().event_message_info.get_or_insert(Default::default())
    }
    fn application_system(&self) -> &super::commonmodule::ApplicationSystem {
        self._coordination_status_profile().application_system.as_ref().unwrap_or(&coordination_status_profile::APPLICATION_SYSTEM)
    }
    fn application_system_mut(&mut self) -> &mut super::commonmodule::ApplicationSystem {
        self._coordination_status_profile_mut().application_system.get_or_insert(Default::default())
    }
    fn coordination_status(&self) -> &CoordinationStatus {
        self._coordination_status_profile().coordination_status.as_ref().unwrap_or(&coordination_status_profile::COORDINATION_STATUS)
    }
    fn coordination_status_mut(&mut self) -> &mut CoordinationStatus {
        self._coordination_status_profile_mut().coordination_status.get_or_insert(Default::default())
    }
}
impl IsCoordinationStatusProfile for CoordinationStatusProfile {
    fn _coordination_status_profile(&self) -> &CoordinationStatusProfile {
        self
    }
    fn _coordination_status_profile_mut(&mut self) -> &mut CoordinationStatusProfile {
        self
    }
}
impl IsEventMessageInfo for CoordinationStatusProfile {
    fn _event_message_info(&self) -> &super::commonmodule::EventMessageInfo {
        self.parent()
    }
    fn _event_message_info_mut(&mut self) -> &mut EventMessageInfo {
        self.parent_mut()
    }
}
impl IsMessageInfo for CoordinationStatusProfile {
    fn _message_info(&self) -> &super::commonmodule::MessageInfo {
        self.parent().parent()
    }
    fn _message_info_mut(&mut self) -> &mut MessageInfo {
        self.parent_mut().parent_mut()
    }
}
impl IsIdentifiedObject for CoordinationStatusProfile {
    fn _identified_object(&self) -> &super::commonmodule::IdentifiedObject {
        self.parent().parent().parent()
    }
    fn _identified_object_mut(&mut self) -> &mut IdentifiedObject {
        self.parent_mut().parent_mut().parent_mut()
    }
}
/// State kind
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum CoordinationServiceModeKind {
    /// MISSING DOCUMENTATION!!!
    #[serde(rename = "CoordinationServiceModeKind_none")]
    None = 0,
    /// MISSING DOCUMENTATION!!!
    #[serde(rename = "CoordinationServiceModeKind_auto")]
    Auto = 1,
    /// MISSING DOCUMENTATION!!!
    #[serde(rename = "CoordinationServiceModeKind_manual")]
    Manual = 2,
    /// MISSING DOCUMENTATION!!!
    #[serde(rename = "CoordinationServiceModeKind_netzero")]
    Netzero = 3,
    /// MISSING DOCUMENTATION!!!
    #[serde(rename = "CoordinationServiceModeKind_start")]
    Start = 4,
    /// MISSING DOCUMENTATION!!!
    #[serde(rename = "CoordinationServiceModeKind_stop")]
    Stop = 5,
}
