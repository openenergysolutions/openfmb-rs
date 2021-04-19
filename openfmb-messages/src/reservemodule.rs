// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

use crate::commonmodule::*;
/// [OpenFMB LN extension] Reserve margin in A, W, VAr and VA.
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize)]
pub struct ReserveMargin {
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
    /// Phase to ground/phase to neutral three phase currents.
    #[prost(message, optional, tag="2")]
    #[serde(default, rename = "A")]
    pub a: ::std::option::Option<super::commonmodule::Pmg>,
    /// Phase to ground/phase to neutral apparent powers S.
    #[prost(message, optional, tag="3")]
    #[serde(default, rename = "VA")]
    pub va: ::std::option::Option<super::commonmodule::Pmg>,
    /// Phase to ground/phase to neutral reactive powers Q.
    #[prost(message, optional, tag="4")]
    #[serde(default, rename = "VAr")]
    pub v_ar: ::std::option::Option<super::commonmodule::Pmg>,
    /// Phase to ground/phase to neutral real powers P.
    #[prost(message, optional, tag="5")]
    #[serde(default, rename = "W")]
    pub w: ::std::option::Option<super::commonmodule::Pmg>,
}
mod reserve_margin {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref LOGICAL_NODE: crate::commonmodule::LogicalNode = Default::default();
        pub(super) static ref A: crate::commonmodule::Pmg = Default::default();
        pub(super) static ref VA: crate::commonmodule::Pmg = Default::default();
        pub(super) static ref V_AR: crate::commonmodule::Pmg = Default::default();
        pub(super) static ref W: crate::commonmodule::Pmg = Default::default();
    }
}
impl ReserveMargin {
    pub(crate) fn parent(&self) -> &super::commonmodule::LogicalNode {
        self.logical_node.as_ref().unwrap_or(&reserve_margin::LOGICAL_NODE)
    }
    pub(crate) fn parent_mut(&mut self) -> &mut super::commonmodule::LogicalNode {
        self.logical_node.get_or_insert(Default::default())
    }
}
pub trait IsReserveMargin {
    fn _reserve_margin(&self) -> &ReserveMargin;
    fn _reserve_margin_mut(&mut self) -> &mut ReserveMargin;
    fn logical_node(&self) -> &super::commonmodule::LogicalNode {
        self._reserve_margin().logical_node.as_ref().unwrap_or(&reserve_margin::LOGICAL_NODE)
    }
    fn logical_node_mut(&mut self) -> &mut super::commonmodule::LogicalNode {
        self._reserve_margin_mut().logical_node.get_or_insert(Default::default())
    }
    fn a(&self) -> &super::commonmodule::Pmg {
        self._reserve_margin().a.as_ref().unwrap_or(&reserve_margin::A)
    }
    fn a_mut(&mut self) -> &mut super::commonmodule::Pmg {
        self._reserve_margin_mut().a.get_or_insert(Default::default())
    }
    fn va(&self) -> &super::commonmodule::Pmg {
        self._reserve_margin().va.as_ref().unwrap_or(&reserve_margin::VA)
    }
    fn va_mut(&mut self) -> &mut super::commonmodule::Pmg {
        self._reserve_margin_mut().va.get_or_insert(Default::default())
    }
    fn v_ar(&self) -> &super::commonmodule::Pmg {
        self._reserve_margin().v_ar.as_ref().unwrap_or(&reserve_margin::V_AR)
    }
    fn v_ar_mut(&mut self) -> &mut super::commonmodule::Pmg {
        self._reserve_margin_mut().v_ar.get_or_insert(Default::default())
    }
    fn w(&self) -> &super::commonmodule::Pmg {
        self._reserve_margin().w.as_ref().unwrap_or(&reserve_margin::W)
    }
    fn w_mut(&mut self) -> &mut super::commonmodule::Pmg {
        self._reserve_margin_mut().w.get_or_insert(Default::default())
    }
}
impl IsReserveMargin for ReserveMargin {
    fn _reserve_margin(&self) -> &ReserveMargin {
        self
    }
    fn _reserve_margin_mut(&mut self) -> &mut ReserveMargin {
        self
    }
}
impl IsLogicalNode for ReserveMargin {
    fn _logical_node(&self) -> &super::commonmodule::LogicalNode {
        self.parent()
    }
    fn _logical_node_mut(&mut self) -> &mut LogicalNode {
        self.parent_mut()
    }
}
impl IsIdentifiedObject for ReserveMargin {
    fn _identified_object(&self) -> &super::commonmodule::IdentifiedObject {
        self.parent().parent()
    }
    fn _identified_object_mut(&mut self) -> &mut IdentifiedObject {
        self.parent_mut().parent_mut()
    }
}
/// MISSING DOCUMENTATION!!!
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize)]
pub struct ReserveAvailability {
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="1")]
    #[serde(default, rename = "incrementalMargin")]
    pub incremental_margin: ::std::option::Option<ReserveMargin>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    #[serde(default)]
    pub margin: ::std::option::Option<ReserveMargin>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    #[serde(default, rename = "standbyMargin")]
    pub standby_margin: ::std::option::Option<ReserveMargin>,
}
mod reserve_availability {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref INCREMENTAL_MARGIN: crate::reservemodule::ReserveMargin = Default::default();
        pub(super) static ref MARGIN: crate::reservemodule::ReserveMargin = Default::default();
        pub(super) static ref STANDBY_MARGIN: crate::reservemodule::ReserveMargin = Default::default();
    }
}
impl ReserveAvailability {
}
pub trait IsReserveAvailability {
    fn _reserve_availability(&self) -> &ReserveAvailability;
    fn _reserve_availability_mut(&mut self) -> &mut ReserveAvailability;
    fn incremental_margin(&self) -> &ReserveMargin {
        self._reserve_availability().incremental_margin.as_ref().unwrap_or(&reserve_availability::INCREMENTAL_MARGIN)
    }
    fn incremental_margin_mut(&mut self) -> &mut ReserveMargin {
        self._reserve_availability_mut().incremental_margin.get_or_insert(Default::default())
    }
    fn margin(&self) -> &ReserveMargin {
        self._reserve_availability().margin.as_ref().unwrap_or(&reserve_availability::MARGIN)
    }
    fn margin_mut(&mut self) -> &mut ReserveMargin {
        self._reserve_availability_mut().margin.get_or_insert(Default::default())
    }
    fn standby_margin(&self) -> &ReserveMargin {
        self._reserve_availability().standby_margin.as_ref().unwrap_or(&reserve_availability::STANDBY_MARGIN)
    }
    fn standby_margin_mut(&mut self) -> &mut ReserveMargin {
        self._reserve_availability_mut().standby_margin.get_or_insert(Default::default())
    }
}
impl IsReserveAvailability for ReserveAvailability {
    fn _reserve_availability(&self) -> &ReserveAvailability {
        self
    }
    fn _reserve_availability_mut(&mut self) -> &mut ReserveAvailability {
        self
    }
}
/// MISSING DOCUMENTATION!!!
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize)]
pub struct AllocatedMargin {
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: Some(1)
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(string, tag="1")]
    #[serde(default, rename = "requestID")]
    pub request_id: std::string::String,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    #[serde(default, rename = "allocatedMargin")]
    pub allocated_margin: ::std::option::Option<ReserveMargin>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    #[serde(default, rename = "allocatedStandbyMargin")]
    pub allocated_standby_margin: ::std::option::Option<ReserveMargin>,
}
mod allocated_margin {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref ALLOCATED_MARGIN: crate::reservemodule::ReserveMargin = Default::default();
        pub(super) static ref ALLOCATED_STANDBY_MARGIN: crate::reservemodule::ReserveMargin = Default::default();
    }
}
impl AllocatedMargin {
}
pub trait IsAllocatedMargin {
    fn _allocated_margin(&self) -> &AllocatedMargin;
    fn _allocated_margin_mut(&mut self) -> &mut AllocatedMargin;
    fn request_id(&self) -> &std::string::String {
        &self._allocated_margin().request_id
    }
    fn request_id_mut(&mut self) -> &mut std::string::String {
        &mut self._allocated_margin_mut().request_id
    }
    fn allocated_margin(&self) -> &ReserveMargin {
        self._allocated_margin().allocated_margin.as_ref().unwrap_or(&allocated_margin::ALLOCATED_MARGIN)
    }
    fn allocated_margin_mut(&mut self) -> &mut ReserveMargin {
        self._allocated_margin_mut().allocated_margin.get_or_insert(Default::default())
    }
    fn allocated_standby_margin(&self) -> &ReserveMargin {
        self._allocated_margin().allocated_standby_margin.as_ref().unwrap_or(&allocated_margin::ALLOCATED_STANDBY_MARGIN)
    }
    fn allocated_standby_margin_mut(&mut self) -> &mut ReserveMargin {
        self._allocated_margin_mut().allocated_standby_margin.get_or_insert(Default::default())
    }
}
impl IsAllocatedMargin for AllocatedMargin {
    fn _allocated_margin(&self) -> &AllocatedMargin {
        self
    }
    fn _allocated_margin_mut(&mut self) -> &mut AllocatedMargin {
        self
    }
}
/// Reserve availability profile
/// OpenFMB Profile Message: true
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize)]
pub struct ReserveAvailabilityProfile {
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
    #[prost(message, optional, tag="2")]
    #[serde(default, rename = "allocatedMargin")]
    pub allocated_margin: ::std::option::Option<AllocatedMargin>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: Some(1)
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, optional, tag="3")]
    #[serde(default, rename = "requesterCircuitSegmentService")]
    pub requester_circuit_segment_service: ::std::option::Option<super::commonmodule::ApplicationSystem>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: Some(1)
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, optional, tag="4")]
    #[serde(default, rename = "reserveAvailability")]
    pub reserve_availability: ::std::option::Option<ReserveAvailability>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: Some(1)
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, optional, tag="5")]
    #[serde(default, rename = "responderCircuitSegmentService")]
    pub responder_circuit_segment_service: ::std::option::Option<super::commonmodule::ApplicationSystem>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: Some(1)
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, optional, tag="6")]
    #[serde(default, rename = "tiePoint")]
    pub tie_point: ::std::option::Option<super::commonmodule::ConductingEquipment>,
}
mod reserve_availability_profile {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref CONTROL_MESSAGE_INFO: crate::commonmodule::ControlMessageInfo = Default::default();
        pub(super) static ref ALLOCATED_MARGIN: crate::reservemodule::AllocatedMargin = Default::default();
        pub(super) static ref REQUESTER_CIRCUIT_SEGMENT_SERVICE: crate::commonmodule::ApplicationSystem = Default::default();
        pub(super) static ref RESERVE_AVAILABILITY: crate::reservemodule::ReserveAvailability = Default::default();
        pub(super) static ref RESPONDER_CIRCUIT_SEGMENT_SERVICE: crate::commonmodule::ApplicationSystem = Default::default();
        pub(super) static ref TIE_POINT: crate::commonmodule::ConductingEquipment = Default::default();
    }
}
impl ReserveAvailabilityProfile {
    pub(crate) fn parent(&self) -> &super::commonmodule::ControlMessageInfo {
        self.control_message_info.as_ref().unwrap_or(&reserve_availability_profile::CONTROL_MESSAGE_INFO)
    }
    pub(crate) fn parent_mut(&mut self) -> &mut super::commonmodule::ControlMessageInfo {
        self.control_message_info.get_or_insert(Default::default())
    }
}
pub trait IsReserveAvailabilityProfile {
    fn _reserve_availability_profile(&self) -> &ReserveAvailabilityProfile;
    fn _reserve_availability_profile_mut(&mut self) -> &mut ReserveAvailabilityProfile;
    fn control_message_info(&self) -> &super::commonmodule::ControlMessageInfo {
        self._reserve_availability_profile().control_message_info.as_ref().unwrap_or(&reserve_availability_profile::CONTROL_MESSAGE_INFO)
    }
    fn control_message_info_mut(&mut self) -> &mut super::commonmodule::ControlMessageInfo {
        self._reserve_availability_profile_mut().control_message_info.get_or_insert(Default::default())
    }
    fn allocated_margin(&self) -> &AllocatedMargin {
        self._reserve_availability_profile().allocated_margin.as_ref().unwrap_or(&reserve_availability_profile::ALLOCATED_MARGIN)
    }
    fn allocated_margin_mut(&mut self) -> &mut AllocatedMargin {
        self._reserve_availability_profile_mut().allocated_margin.get_or_insert(Default::default())
    }
    fn requester_circuit_segment_service(&self) -> &super::commonmodule::ApplicationSystem {
        self._reserve_availability_profile().requester_circuit_segment_service.as_ref().unwrap_or(&reserve_availability_profile::REQUESTER_CIRCUIT_SEGMENT_SERVICE)
    }
    fn requester_circuit_segment_service_mut(&mut self) -> &mut super::commonmodule::ApplicationSystem {
        self._reserve_availability_profile_mut().requester_circuit_segment_service.get_or_insert(Default::default())
    }
    fn reserve_availability(&self) -> &ReserveAvailability {
        self._reserve_availability_profile().reserve_availability.as_ref().unwrap_or(&reserve_availability_profile::RESERVE_AVAILABILITY)
    }
    fn reserve_availability_mut(&mut self) -> &mut ReserveAvailability {
        self._reserve_availability_profile_mut().reserve_availability.get_or_insert(Default::default())
    }
    fn responder_circuit_segment_service(&self) -> &super::commonmodule::ApplicationSystem {
        self._reserve_availability_profile().responder_circuit_segment_service.as_ref().unwrap_or(&reserve_availability_profile::RESPONDER_CIRCUIT_SEGMENT_SERVICE)
    }
    fn responder_circuit_segment_service_mut(&mut self) -> &mut super::commonmodule::ApplicationSystem {
        self._reserve_availability_profile_mut().responder_circuit_segment_service.get_or_insert(Default::default())
    }
    fn tie_point(&self) -> &super::commonmodule::ConductingEquipment {
        self._reserve_availability_profile().tie_point.as_ref().unwrap_or(&reserve_availability_profile::TIE_POINT)
    }
    fn tie_point_mut(&mut self) -> &mut super::commonmodule::ConductingEquipment {
        self._reserve_availability_profile_mut().tie_point.get_or_insert(Default::default())
    }
}
impl IsReserveAvailabilityProfile for ReserveAvailabilityProfile {
    fn _reserve_availability_profile(&self) -> &ReserveAvailabilityProfile {
        self
    }
    fn _reserve_availability_profile_mut(&mut self) -> &mut ReserveAvailabilityProfile {
        self
    }
}
impl IsControlMessageInfo for ReserveAvailabilityProfile {
    fn _control_message_info(&self) -> &super::commonmodule::ControlMessageInfo {
        self.parent()
    }
    fn _control_message_info_mut(&mut self) -> &mut ControlMessageInfo {
        self.parent_mut()
    }
}
impl IsMessageInfo for ReserveAvailabilityProfile {
    fn _message_info(&self) -> &super::commonmodule::MessageInfo {
        self.parent().parent()
    }
    fn _message_info_mut(&mut self) -> &mut MessageInfo {
        self.parent_mut().parent_mut()
    }
}
impl IsIdentifiedObject for ReserveAvailabilityProfile {
    fn _identified_object(&self) -> &super::commonmodule::IdentifiedObject {
        self.parent().parent().parent()
    }
    fn _identified_object_mut(&mut self) -> &mut IdentifiedObject {
        self.parent_mut().parent_mut().parent_mut()
    }
}
/// MISSING DOCUMENTATION!!!
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize)]
pub struct ReserveRequest {
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: Some(1)
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(string, tag="1")]
    #[serde(default, rename = "requestID")]
    pub request_id: std::string::String,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    #[serde(default)]
    pub margin: ::std::option::Option<ReserveMargin>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    #[serde(default, rename = "standbyMargin")]
    pub standby_margin: ::std::option::Option<ReserveMargin>,
}
mod reserve_request {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref MARGIN: crate::reservemodule::ReserveMargin = Default::default();
        pub(super) static ref STANDBY_MARGIN: crate::reservemodule::ReserveMargin = Default::default();
    }
}
impl ReserveRequest {
}
pub trait IsReserveRequest {
    fn _reserve_request(&self) -> &ReserveRequest;
    fn _reserve_request_mut(&mut self) -> &mut ReserveRequest;
    fn request_id(&self) -> &std::string::String {
        &self._reserve_request().request_id
    }
    fn request_id_mut(&mut self) -> &mut std::string::String {
        &mut self._reserve_request_mut().request_id
    }
    fn margin(&self) -> &ReserveMargin {
        self._reserve_request().margin.as_ref().unwrap_or(&reserve_request::MARGIN)
    }
    fn margin_mut(&mut self) -> &mut ReserveMargin {
        self._reserve_request_mut().margin.get_or_insert(Default::default())
    }
    fn standby_margin(&self) -> &ReserveMargin {
        self._reserve_request().standby_margin.as_ref().unwrap_or(&reserve_request::STANDBY_MARGIN)
    }
    fn standby_margin_mut(&mut self) -> &mut ReserveMargin {
        self._reserve_request_mut().standby_margin.get_or_insert(Default::default())
    }
}
impl IsReserveRequest for ReserveRequest {
    fn _reserve_request(&self) -> &ReserveRequest {
        self
    }
    fn _reserve_request_mut(&mut self) -> &mut ReserveRequest {
        self
    }
}
/// Reserve availability profile
/// OpenFMB Profile Message: true
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize)]
pub struct ReserveRequestProfile {
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
    #[serde(default, rename = "requesterCircuitSegmentService")]
    pub requester_circuit_segment_service: ::std::option::Option<super::commonmodule::ApplicationSystem>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: Some(1)
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, optional, tag="3")]
    #[serde(default, rename = "reserveRequest")]
    pub reserve_request: ::std::option::Option<ReserveRequest>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: Some(1)
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, optional, tag="4")]
    #[serde(default, rename = "responderCircuitSegmentService")]
    pub responder_circuit_segment_service: ::std::option::Option<super::commonmodule::ApplicationSystem>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: Some(1)
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, optional, tag="5")]
    #[serde(default, rename = "tiePoint")]
    pub tie_point: ::std::option::Option<super::commonmodule::ConductingEquipment>,
}
mod reserve_request_profile {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref CONTROL_MESSAGE_INFO: crate::commonmodule::ControlMessageInfo = Default::default();
        pub(super) static ref REQUESTER_CIRCUIT_SEGMENT_SERVICE: crate::commonmodule::ApplicationSystem = Default::default();
        pub(super) static ref RESERVE_REQUEST: crate::reservemodule::ReserveRequest = Default::default();
        pub(super) static ref RESPONDER_CIRCUIT_SEGMENT_SERVICE: crate::commonmodule::ApplicationSystem = Default::default();
        pub(super) static ref TIE_POINT: crate::commonmodule::ConductingEquipment = Default::default();
    }
}
impl ReserveRequestProfile {
    pub(crate) fn parent(&self) -> &super::commonmodule::ControlMessageInfo {
        self.control_message_info.as_ref().unwrap_or(&reserve_request_profile::CONTROL_MESSAGE_INFO)
    }
    pub(crate) fn parent_mut(&mut self) -> &mut super::commonmodule::ControlMessageInfo {
        self.control_message_info.get_or_insert(Default::default())
    }
}
pub trait IsReserveRequestProfile {
    fn _reserve_request_profile(&self) -> &ReserveRequestProfile;
    fn _reserve_request_profile_mut(&mut self) -> &mut ReserveRequestProfile;
    fn control_message_info(&self) -> &super::commonmodule::ControlMessageInfo {
        self._reserve_request_profile().control_message_info.as_ref().unwrap_or(&reserve_request_profile::CONTROL_MESSAGE_INFO)
    }
    fn control_message_info_mut(&mut self) -> &mut super::commonmodule::ControlMessageInfo {
        self._reserve_request_profile_mut().control_message_info.get_or_insert(Default::default())
    }
    fn requester_circuit_segment_service(&self) -> &super::commonmodule::ApplicationSystem {
        self._reserve_request_profile().requester_circuit_segment_service.as_ref().unwrap_or(&reserve_request_profile::REQUESTER_CIRCUIT_SEGMENT_SERVICE)
    }
    fn requester_circuit_segment_service_mut(&mut self) -> &mut super::commonmodule::ApplicationSystem {
        self._reserve_request_profile_mut().requester_circuit_segment_service.get_or_insert(Default::default())
    }
    fn reserve_request(&self) -> &ReserveRequest {
        self._reserve_request_profile().reserve_request.as_ref().unwrap_or(&reserve_request_profile::RESERVE_REQUEST)
    }
    fn reserve_request_mut(&mut self) -> &mut ReserveRequest {
        self._reserve_request_profile_mut().reserve_request.get_or_insert(Default::default())
    }
    fn responder_circuit_segment_service(&self) -> &super::commonmodule::ApplicationSystem {
        self._reserve_request_profile().responder_circuit_segment_service.as_ref().unwrap_or(&reserve_request_profile::RESPONDER_CIRCUIT_SEGMENT_SERVICE)
    }
    fn responder_circuit_segment_service_mut(&mut self) -> &mut super::commonmodule::ApplicationSystem {
        self._reserve_request_profile_mut().responder_circuit_segment_service.get_or_insert(Default::default())
    }
    fn tie_point(&self) -> &super::commonmodule::ConductingEquipment {
        self._reserve_request_profile().tie_point.as_ref().unwrap_or(&reserve_request_profile::TIE_POINT)
    }
    fn tie_point_mut(&mut self) -> &mut super::commonmodule::ConductingEquipment {
        self._reserve_request_profile_mut().tie_point.get_or_insert(Default::default())
    }
}
impl IsReserveRequestProfile for ReserveRequestProfile {
    fn _reserve_request_profile(&self) -> &ReserveRequestProfile {
        self
    }
    fn _reserve_request_profile_mut(&mut self) -> &mut ReserveRequestProfile {
        self
    }
}
impl IsControlMessageInfo for ReserveRequestProfile {
    fn _control_message_info(&self) -> &super::commonmodule::ControlMessageInfo {
        self.parent()
    }
    fn _control_message_info_mut(&mut self) -> &mut ControlMessageInfo {
        self.parent_mut()
    }
}
impl IsMessageInfo for ReserveRequestProfile {
    fn _message_info(&self) -> &super::commonmodule::MessageInfo {
        self.parent().parent()
    }
    fn _message_info_mut(&mut self) -> &mut MessageInfo {
        self.parent_mut().parent_mut()
    }
}
impl IsIdentifiedObject for ReserveRequestProfile {
    fn _identified_object(&self) -> &super::commonmodule::IdentifiedObject {
        self.parent().parent().parent()
    }
    fn _identified_object_mut(&mut self) -> &mut IdentifiedObject {
        self.parent_mut().parent_mut().parent_mut()
    }
}
