// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

use crate::commonmodule::*;
/// Reclose enabled
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize)]
pub struct BreakerDiscreteControlXcbr {
    /// UML inherited base object
    // parent_message: true
    // required_field: false
    // multiplicity_min: None
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, optional, tag="1")]
    #[serde(default, rename = "discreteControlXCBR")]
    pub discrete_control_xcbr: ::std::option::Option<super::commonmodule::DiscreteControlXcbr>,
}
mod breaker_discrete_control_xcbr {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref DISCRETE_CONTROL_XCBR: crate::commonmodule::DiscreteControlXcbr = Default::default();
    }
}
impl BreakerDiscreteControlXcbr {
    pub(crate) fn parent(&self) -> &super::commonmodule::DiscreteControlXcbr {
        self.discrete_control_xcbr.as_ref().unwrap_or(&breaker_discrete_control_xcbr::DISCRETE_CONTROL_XCBR)
    }
    pub(crate) fn parent_mut(&mut self) -> &mut super::commonmodule::DiscreteControlXcbr {
        self.discrete_control_xcbr.get_or_insert(Default::default())
    }
}
pub trait IsBreakerDiscreteControlXcbr {
    fn _breaker_discrete_control_xcbr(&self) -> &BreakerDiscreteControlXcbr;
    fn _breaker_discrete_control_xcbr_mut(&mut self) -> &mut BreakerDiscreteControlXcbr;
    fn discrete_control_xcbr(&self) -> &super::commonmodule::DiscreteControlXcbr {
        self._breaker_discrete_control_xcbr().discrete_control_xcbr.as_ref().unwrap_or(&breaker_discrete_control_xcbr::DISCRETE_CONTROL_XCBR)
    }
    fn discrete_control_xcbr_mut(&mut self) -> &mut super::commonmodule::DiscreteControlXcbr {
        self._breaker_discrete_control_xcbr_mut().discrete_control_xcbr.get_or_insert(Default::default())
    }
}
impl IsBreakerDiscreteControlXcbr for BreakerDiscreteControlXcbr {
    fn _breaker_discrete_control_xcbr(&self) -> &BreakerDiscreteControlXcbr {
        self
    }
    fn _breaker_discrete_control_xcbr_mut(&mut self) -> &mut BreakerDiscreteControlXcbr {
        self
    }
}
impl IsDiscreteControlXcbr for BreakerDiscreteControlXcbr {
    fn _discrete_control_xcbr(&self) -> &super::commonmodule::DiscreteControlXcbr {
        self.parent()
    }
    fn _discrete_control_xcbr_mut(&mut self) -> &mut DiscreteControlXcbr {
        self.parent_mut()
    }
}
impl IsLogicalNodeForControl for BreakerDiscreteControlXcbr {
    fn _logical_node_for_control(&self) -> &super::commonmodule::LogicalNodeForControl {
        self.parent().parent()
    }
    fn _logical_node_for_control_mut(&mut self) -> &mut LogicalNodeForControl {
        self.parent_mut().parent_mut()
    }
}
impl IsLogicalNode for BreakerDiscreteControlXcbr {
    fn _logical_node(&self) -> &super::commonmodule::LogicalNode {
        self.parent().parent().parent()
    }
    fn _logical_node_mut(&mut self) -> &mut LogicalNode {
        self.parent_mut().parent_mut().parent_mut()
    }
}
impl IsIdentifiedObject for BreakerDiscreteControlXcbr {
    fn _identified_object(&self) -> &super::commonmodule::IdentifiedObject {
        self.parent().parent().parent().parent()
    }
    fn _identified_object_mut(&mut self) -> &mut IdentifiedObject {
        self.parent_mut().parent_mut().parent_mut().parent_mut()
    }
}
/// Breaker discrete control class
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize)]
pub struct BreakerDiscreteControl {
    /// UML inherited base object
    // parent_message: true
    // required_field: false
    // multiplicity_min: None
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, optional, tag="1")]
    #[serde(default, rename = "controlValue")]
    pub control_value: ::std::option::Option<super::commonmodule::ControlValue>,
    /// IEC61850 enhanced control parameters.
    #[prost(message, optional, tag="2")]
    #[serde(default)]
    pub check: ::std::option::Option<super::commonmodule::CheckConditions>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    #[serde(default, rename = "breakerDiscreteControlXCBR")]
    pub breaker_discrete_control_xcbr: ::std::option::Option<BreakerDiscreteControlXcbr>,
}
mod breaker_discrete_control {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref CONTROL_VALUE: crate::commonmodule::ControlValue = Default::default();
        pub(super) static ref CHECK: crate::commonmodule::CheckConditions = Default::default();
        pub(super) static ref BREAKER_DISCRETE_CONTROL_XCBR: crate::breakermodule::BreakerDiscreteControlXcbr = Default::default();
    }
}
impl BreakerDiscreteControl {
    pub(crate) fn parent(&self) -> &super::commonmodule::ControlValue {
        self.control_value.as_ref().unwrap_or(&breaker_discrete_control::CONTROL_VALUE)
    }
    pub(crate) fn parent_mut(&mut self) -> &mut super::commonmodule::ControlValue {
        self.control_value.get_or_insert(Default::default())
    }
}
pub trait IsBreakerDiscreteControl {
    fn _breaker_discrete_control(&self) -> &BreakerDiscreteControl;
    fn _breaker_discrete_control_mut(&mut self) -> &mut BreakerDiscreteControl;
    fn control_value(&self) -> &super::commonmodule::ControlValue {
        self._breaker_discrete_control().control_value.as_ref().unwrap_or(&breaker_discrete_control::CONTROL_VALUE)
    }
    fn control_value_mut(&mut self) -> &mut super::commonmodule::ControlValue {
        self._breaker_discrete_control_mut().control_value.get_or_insert(Default::default())
    }
    fn check(&self) -> &super::commonmodule::CheckConditions {
        self._breaker_discrete_control().check.as_ref().unwrap_or(&breaker_discrete_control::CHECK)
    }
    fn check_mut(&mut self) -> &mut super::commonmodule::CheckConditions {
        self._breaker_discrete_control_mut().check.get_or_insert(Default::default())
    }
    fn breaker_discrete_control_xcbr(&self) -> &BreakerDiscreteControlXcbr {
        self._breaker_discrete_control().breaker_discrete_control_xcbr.as_ref().unwrap_or(&breaker_discrete_control::BREAKER_DISCRETE_CONTROL_XCBR)
    }
    fn breaker_discrete_control_xcbr_mut(&mut self) -> &mut BreakerDiscreteControlXcbr {
        self._breaker_discrete_control_mut().breaker_discrete_control_xcbr.get_or_insert(Default::default())
    }
}
impl IsBreakerDiscreteControl for BreakerDiscreteControl {
    fn _breaker_discrete_control(&self) -> &BreakerDiscreteControl {
        self
    }
    fn _breaker_discrete_control_mut(&mut self) -> &mut BreakerDiscreteControl {
        self
    }
}
impl IsControlValue for BreakerDiscreteControl {
    fn _control_value(&self) -> &super::commonmodule::ControlValue {
        self.parent()
    }
    fn _control_value_mut(&mut self) -> &mut ControlValue {
        self.parent_mut()
    }
}
impl IsIdentifiedObject for BreakerDiscreteControl {
    fn _identified_object(&self) -> &super::commonmodule::IdentifiedObject {
        self.parent().parent()
    }
    fn _identified_object_mut(&mut self) -> &mut IdentifiedObject {
        self.parent_mut().parent_mut()
    }
}
/// A mechanical switching device capable of making, carrying, and breaking currents under normal
/// circuit conditions and also making, carrying for a specified time, and breaking currents under
/// specified abnormal circuit conditions e.g.  those of short circuit.
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize)]
pub struct Breaker {
    /// UML inherited base object
    // parent_message: true
    // required_field: false
    // multiplicity_min: None
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, optional, tag="1")]
    #[serde(default, rename = "conductingEquipment")]
    pub conducting_equipment: ::std::option::Option<super::commonmodule::ConductingEquipment>,
}
mod breaker {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref CONDUCTING_EQUIPMENT: crate::commonmodule::ConductingEquipment = Default::default();
    }
}
impl Breaker {
    pub(crate) fn parent(&self) -> &super::commonmodule::ConductingEquipment {
        self.conducting_equipment.as_ref().unwrap_or(&breaker::CONDUCTING_EQUIPMENT)
    }
    pub(crate) fn parent_mut(&mut self) -> &mut super::commonmodule::ConductingEquipment {
        self.conducting_equipment.get_or_insert(Default::default())
    }
}
pub trait IsBreaker {
    fn _breaker(&self) -> &Breaker;
    fn _breaker_mut(&mut self) -> &mut Breaker;
    fn conducting_equipment(&self) -> &super::commonmodule::ConductingEquipment {
        self._breaker().conducting_equipment.as_ref().unwrap_or(&breaker::CONDUCTING_EQUIPMENT)
    }
    fn conducting_equipment_mut(&mut self) -> &mut super::commonmodule::ConductingEquipment {
        self._breaker_mut().conducting_equipment.get_or_insert(Default::default())
    }
}
impl IsBreaker for Breaker {
    fn _breaker(&self) -> &Breaker {
        self
    }
    fn _breaker_mut(&mut self) -> &mut Breaker {
        self
    }
}
impl IsConductingEquipment for Breaker {
    fn _conducting_equipment(&self) -> &super::commonmodule::ConductingEquipment {
        self.parent()
    }
    fn _conducting_equipment_mut(&mut self) -> &mut ConductingEquipment {
        self.parent_mut()
    }
}
impl IsNamedObject for Breaker {
    fn _named_object(&self) -> &super::commonmodule::NamedObject {
        self.parent().parent()
    }
    fn _named_object_mut(&mut self) -> &mut NamedObject {
        self.parent_mut().parent_mut()
    }
}
/// Instructs an end device (or an end device group) to perform a specified action.
/// OpenFMB Profile Message: true
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize)]
pub struct BreakerDiscreteControlProfile {
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
    #[serde(default)]
    pub breaker: ::std::option::Option<Breaker>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: Some(1)
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, optional, tag="3")]
    #[serde(default, rename = "breakerDiscreteControl")]
    pub breaker_discrete_control: ::std::option::Option<BreakerDiscreteControl>,
}
mod breaker_discrete_control_profile {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref CONTROL_MESSAGE_INFO: crate::commonmodule::ControlMessageInfo = Default::default();
        pub(super) static ref BREAKER: crate::breakermodule::Breaker = Default::default();
        pub(super) static ref BREAKER_DISCRETE_CONTROL: crate::breakermodule::BreakerDiscreteControl = Default::default();
    }
}
impl BreakerDiscreteControlProfile {
    pub(crate) fn parent(&self) -> &super::commonmodule::ControlMessageInfo {
        self.control_message_info.as_ref().unwrap_or(&breaker_discrete_control_profile::CONTROL_MESSAGE_INFO)
    }
    pub(crate) fn parent_mut(&mut self) -> &mut super::commonmodule::ControlMessageInfo {
        self.control_message_info.get_or_insert(Default::default())
    }
}
pub trait IsBreakerDiscreteControlProfile {
    fn _breaker_discrete_control_profile(&self) -> &BreakerDiscreteControlProfile;
    fn _breaker_discrete_control_profile_mut(&mut self) -> &mut BreakerDiscreteControlProfile;
    fn control_message_info(&self) -> &super::commonmodule::ControlMessageInfo {
        self._breaker_discrete_control_profile().control_message_info.as_ref().unwrap_or(&breaker_discrete_control_profile::CONTROL_MESSAGE_INFO)
    }
    fn control_message_info_mut(&mut self) -> &mut super::commonmodule::ControlMessageInfo {
        self._breaker_discrete_control_profile_mut().control_message_info.get_or_insert(Default::default())
    }
    fn breaker(&self) -> &Breaker {
        self._breaker_discrete_control_profile().breaker.as_ref().unwrap_or(&breaker_discrete_control_profile::BREAKER)
    }
    fn breaker_mut(&mut self) -> &mut Breaker {
        self._breaker_discrete_control_profile_mut().breaker.get_or_insert(Default::default())
    }
    fn breaker_discrete_control(&self) -> &BreakerDiscreteControl {
        self._breaker_discrete_control_profile().breaker_discrete_control.as_ref().unwrap_or(&breaker_discrete_control_profile::BREAKER_DISCRETE_CONTROL)
    }
    fn breaker_discrete_control_mut(&mut self) -> &mut BreakerDiscreteControl {
        self._breaker_discrete_control_profile_mut().breaker_discrete_control.get_or_insert(Default::default())
    }
}
impl IsBreakerDiscreteControlProfile for BreakerDiscreteControlProfile {
    fn _breaker_discrete_control_profile(&self) -> &BreakerDiscreteControlProfile {
        self
    }
    fn _breaker_discrete_control_profile_mut(&mut self) -> &mut BreakerDiscreteControlProfile {
        self
    }
}
impl IsControlMessageInfo for BreakerDiscreteControlProfile {
    fn _control_message_info(&self) -> &super::commonmodule::ControlMessageInfo {
        self.parent()
    }
    fn _control_message_info_mut(&mut self) -> &mut ControlMessageInfo {
        self.parent_mut()
    }
}
impl IsMessageInfo for BreakerDiscreteControlProfile {
    fn _message_info(&self) -> &super::commonmodule::MessageInfo {
        self.parent().parent()
    }
    fn _message_info_mut(&mut self) -> &mut MessageInfo {
        self.parent_mut().parent_mut()
    }
}
impl IsIdentifiedObject for BreakerDiscreteControlProfile {
    fn _identified_object(&self) -> &super::commonmodule::IdentifiedObject {
        self.parent().parent().parent()
    }
    fn _identified_object_mut(&mut self) -> &mut IdentifiedObject {
        self.parent_mut().parent_mut().parent_mut()
    }
}
/// Breaker event class
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize)]
pub struct BreakerEvent {
    /// UML inherited base object
    // parent_message: true
    // required_field: false
    // multiplicity_min: None
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, optional, tag="1")]
    #[serde(default, rename = "eventValue")]
    pub event_value: ::std::option::Option<super::commonmodule::EventValue>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    #[serde(default, rename = "statusAndEventXCBR")]
    pub status_and_event_xcbr: ::std::option::Option<super::commonmodule::StatusAndEventXcbr>,
}
mod breaker_event {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref EVENT_VALUE: crate::commonmodule::EventValue = Default::default();
        pub(super) static ref STATUS_AND_EVENT_XCBR: crate::commonmodule::StatusAndEventXcbr = Default::default();
    }
}
impl BreakerEvent {
    pub(crate) fn parent(&self) -> &super::commonmodule::EventValue {
        self.event_value.as_ref().unwrap_or(&breaker_event::EVENT_VALUE)
    }
    pub(crate) fn parent_mut(&mut self) -> &mut super::commonmodule::EventValue {
        self.event_value.get_or_insert(Default::default())
    }
}
pub trait IsBreakerEvent {
    fn _breaker_event(&self) -> &BreakerEvent;
    fn _breaker_event_mut(&mut self) -> &mut BreakerEvent;
    fn event_value(&self) -> &super::commonmodule::EventValue {
        self._breaker_event().event_value.as_ref().unwrap_or(&breaker_event::EVENT_VALUE)
    }
    fn event_value_mut(&mut self) -> &mut super::commonmodule::EventValue {
        self._breaker_event_mut().event_value.get_or_insert(Default::default())
    }
    fn status_and_event_xcbr(&self) -> &super::commonmodule::StatusAndEventXcbr {
        self._breaker_event().status_and_event_xcbr.as_ref().unwrap_or(&breaker_event::STATUS_AND_EVENT_XCBR)
    }
    fn status_and_event_xcbr_mut(&mut self) -> &mut super::commonmodule::StatusAndEventXcbr {
        self._breaker_event_mut().status_and_event_xcbr.get_or_insert(Default::default())
    }
}
impl IsBreakerEvent for BreakerEvent {
    fn _breaker_event(&self) -> &BreakerEvent {
        self
    }
    fn _breaker_event_mut(&mut self) -> &mut BreakerEvent {
        self
    }
}
impl IsEventValue for BreakerEvent {
    fn _event_value(&self) -> &super::commonmodule::EventValue {
        self.parent()
    }
    fn _event_value_mut(&mut self) -> &mut EventValue {
        self.parent_mut()
    }
}
impl IsIdentifiedObject for BreakerEvent {
    fn _identified_object(&self) -> &super::commonmodule::IdentifiedObject {
        self.parent().parent()
    }
    fn _identified_object_mut(&mut self) -> &mut IdentifiedObject {
        self.parent_mut().parent_mut()
    }
}
/// Breaker event profile
/// OpenFMB Profile Message: true
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize)]
pub struct BreakerEventProfile {
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
    #[serde(default)]
    pub breaker: ::std::option::Option<Breaker>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: Some(1)
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, optional, tag="3")]
    #[serde(default, rename = "breakerEvent")]
    pub breaker_event: ::std::option::Option<BreakerEvent>,
}
mod breaker_event_profile {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref EVENT_MESSAGE_INFO: crate::commonmodule::EventMessageInfo = Default::default();
        pub(super) static ref BREAKER: crate::breakermodule::Breaker = Default::default();
        pub(super) static ref BREAKER_EVENT: crate::breakermodule::BreakerEvent = Default::default();
    }
}
impl BreakerEventProfile {
    pub(crate) fn parent(&self) -> &super::commonmodule::EventMessageInfo {
        self.event_message_info.as_ref().unwrap_or(&breaker_event_profile::EVENT_MESSAGE_INFO)
    }
    pub(crate) fn parent_mut(&mut self) -> &mut super::commonmodule::EventMessageInfo {
        self.event_message_info.get_or_insert(Default::default())
    }
}
pub trait IsBreakerEventProfile {
    fn _breaker_event_profile(&self) -> &BreakerEventProfile;
    fn _breaker_event_profile_mut(&mut self) -> &mut BreakerEventProfile;
    fn event_message_info(&self) -> &super::commonmodule::EventMessageInfo {
        self._breaker_event_profile().event_message_info.as_ref().unwrap_or(&breaker_event_profile::EVENT_MESSAGE_INFO)
    }
    fn event_message_info_mut(&mut self) -> &mut super::commonmodule::EventMessageInfo {
        self._breaker_event_profile_mut().event_message_info.get_or_insert(Default::default())
    }
    fn breaker(&self) -> &Breaker {
        self._breaker_event_profile().breaker.as_ref().unwrap_or(&breaker_event_profile::BREAKER)
    }
    fn breaker_mut(&mut self) -> &mut Breaker {
        self._breaker_event_profile_mut().breaker.get_or_insert(Default::default())
    }
    fn breaker_event(&self) -> &BreakerEvent {
        self._breaker_event_profile().breaker_event.as_ref().unwrap_or(&breaker_event_profile::BREAKER_EVENT)
    }
    fn breaker_event_mut(&mut self) -> &mut BreakerEvent {
        self._breaker_event_profile_mut().breaker_event.get_or_insert(Default::default())
    }
}
impl IsBreakerEventProfile for BreakerEventProfile {
    fn _breaker_event_profile(&self) -> &BreakerEventProfile {
        self
    }
    fn _breaker_event_profile_mut(&mut self) -> &mut BreakerEventProfile {
        self
    }
}
impl IsEventMessageInfo for BreakerEventProfile {
    fn _event_message_info(&self) -> &super::commonmodule::EventMessageInfo {
        self.parent()
    }
    fn _event_message_info_mut(&mut self) -> &mut EventMessageInfo {
        self.parent_mut()
    }
}
impl IsMessageInfo for BreakerEventProfile {
    fn _message_info(&self) -> &super::commonmodule::MessageInfo {
        self.parent().parent()
    }
    fn _message_info_mut(&mut self) -> &mut MessageInfo {
        self.parent_mut().parent_mut()
    }
}
impl IsIdentifiedObject for BreakerEventProfile {
    fn _identified_object(&self) -> &super::commonmodule::IdentifiedObject {
        self.parent().parent().parent()
    }
    fn _identified_object_mut(&mut self) -> &mut IdentifiedObject {
        self.parent_mut().parent_mut().parent_mut()
    }
}
/// MISSING DOCUMENTATION!!!
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize)]
pub struct BreakerReading {
    /// UML inherited base object
    // parent_message: true
    // required_field: false
    // multiplicity_min: None
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, optional, tag="1")]
    #[serde(default, rename = "conductingEquipmentTerminalReading")]
    pub conducting_equipment_terminal_reading: ::std::option::Option<super::commonmodule::ConductingEquipmentTerminalReading>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    #[serde(default, rename = "diffReadingMMXU")]
    pub diff_reading_mmxu: ::std::option::Option<super::commonmodule::ReadingMmxu>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    #[serde(default, rename = "phaseMMTN")]
    pub phase_mmtn: ::std::option::Option<super::commonmodule::PhaseMmtn>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="4")]
    #[serde(default, rename = "readingMMTR")]
    pub reading_mmtr: ::std::option::Option<super::commonmodule::ReadingMmtr>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="5")]
    #[serde(default, rename = "readingMMXU")]
    pub reading_mmxu: ::std::option::Option<super::commonmodule::ReadingMmxu>,
}
mod breaker_reading {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref CONDUCTING_EQUIPMENT_TERMINAL_READING: crate::commonmodule::ConductingEquipmentTerminalReading = Default::default();
        pub(super) static ref DIFF_READING_MMXU: crate::commonmodule::ReadingMmxu = Default::default();
        pub(super) static ref PHASE_MMTN: crate::commonmodule::PhaseMmtn = Default::default();
        pub(super) static ref READING_MMTR: crate::commonmodule::ReadingMmtr = Default::default();
        pub(super) static ref READING_MMXU: crate::commonmodule::ReadingMmxu = Default::default();
    }
}
impl BreakerReading {
    pub(crate) fn parent(&self) -> &super::commonmodule::ConductingEquipmentTerminalReading {
        self.conducting_equipment_terminal_reading.as_ref().unwrap_or(&breaker_reading::CONDUCTING_EQUIPMENT_TERMINAL_READING)
    }
    pub(crate) fn parent_mut(&mut self) -> &mut super::commonmodule::ConductingEquipmentTerminalReading {
        self.conducting_equipment_terminal_reading.get_or_insert(Default::default())
    }
}
pub trait IsBreakerReading {
    fn _breaker_reading(&self) -> &BreakerReading;
    fn _breaker_reading_mut(&mut self) -> &mut BreakerReading;
    fn conducting_equipment_terminal_reading(&self) -> &super::commonmodule::ConductingEquipmentTerminalReading {
        self._breaker_reading().conducting_equipment_terminal_reading.as_ref().unwrap_or(&breaker_reading::CONDUCTING_EQUIPMENT_TERMINAL_READING)
    }
    fn conducting_equipment_terminal_reading_mut(&mut self) -> &mut super::commonmodule::ConductingEquipmentTerminalReading {
        self._breaker_reading_mut().conducting_equipment_terminal_reading.get_or_insert(Default::default())
    }
    fn diff_reading_mmxu(&self) -> &super::commonmodule::ReadingMmxu {
        self._breaker_reading().diff_reading_mmxu.as_ref().unwrap_or(&breaker_reading::DIFF_READING_MMXU)
    }
    fn diff_reading_mmxu_mut(&mut self) -> &mut super::commonmodule::ReadingMmxu {
        self._breaker_reading_mut().diff_reading_mmxu.get_or_insert(Default::default())
    }
    fn phase_mmtn(&self) -> &super::commonmodule::PhaseMmtn {
        self._breaker_reading().phase_mmtn.as_ref().unwrap_or(&breaker_reading::PHASE_MMTN)
    }
    fn phase_mmtn_mut(&mut self) -> &mut super::commonmodule::PhaseMmtn {
        self._breaker_reading_mut().phase_mmtn.get_or_insert(Default::default())
    }
    fn reading_mmtr(&self) -> &super::commonmodule::ReadingMmtr {
        self._breaker_reading().reading_mmtr.as_ref().unwrap_or(&breaker_reading::READING_MMTR)
    }
    fn reading_mmtr_mut(&mut self) -> &mut super::commonmodule::ReadingMmtr {
        self._breaker_reading_mut().reading_mmtr.get_or_insert(Default::default())
    }
    fn reading_mmxu(&self) -> &super::commonmodule::ReadingMmxu {
        self._breaker_reading().reading_mmxu.as_ref().unwrap_or(&breaker_reading::READING_MMXU)
    }
    fn reading_mmxu_mut(&mut self) -> &mut super::commonmodule::ReadingMmxu {
        self._breaker_reading_mut().reading_mmxu.get_or_insert(Default::default())
    }
}
impl IsBreakerReading for BreakerReading {
    fn _breaker_reading(&self) -> &BreakerReading {
        self
    }
    fn _breaker_reading_mut(&mut self) -> &mut BreakerReading {
        self
    }
}
impl IsConductingEquipmentTerminalReading for BreakerReading {
    fn _conducting_equipment_terminal_reading(&self) -> &super::commonmodule::ConductingEquipmentTerminalReading {
        self.parent()
    }
    fn _conducting_equipment_terminal_reading_mut(&mut self) -> &mut ConductingEquipmentTerminalReading {
        self.parent_mut()
    }
}
/// Breaker reading profile
/// OpenFMB Profile Message: true
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize)]
pub struct BreakerReadingProfile {
    /// UML inherited base object
    // parent_message: true
    // required_field: false
    // multiplicity_min: None
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, optional, tag="1")]
    #[serde(default, rename = "readingMessageInfo")]
    pub reading_message_info: ::std::option::Option<super::commonmodule::ReadingMessageInfo>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: Some(1)
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, optional, tag="2")]
    #[serde(default)]
    pub breaker: ::std::option::Option<Breaker>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: Some(1)
    // multiplicity_max: Some(2)
    // uuid: false
    // key: false
    #[prost(message, repeated, tag="3")]
    #[serde(default, rename = "breakerReading")]
    pub breaker_reading: ::std::vec::Vec<BreakerReading>,
}
mod breaker_reading_profile {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref READING_MESSAGE_INFO: crate::commonmodule::ReadingMessageInfo = Default::default();
        pub(super) static ref BREAKER: crate::breakermodule::Breaker = Default::default();
    }
}
impl BreakerReadingProfile {
    pub(crate) fn parent(&self) -> &super::commonmodule::ReadingMessageInfo {
        self.reading_message_info.as_ref().unwrap_or(&breaker_reading_profile::READING_MESSAGE_INFO)
    }
    pub(crate) fn parent_mut(&mut self) -> &mut super::commonmodule::ReadingMessageInfo {
        self.reading_message_info.get_or_insert(Default::default())
    }
}
pub trait IsBreakerReadingProfile {
    fn _breaker_reading_profile(&self) -> &BreakerReadingProfile;
    fn _breaker_reading_profile_mut(&mut self) -> &mut BreakerReadingProfile;
    fn reading_message_info(&self) -> &super::commonmodule::ReadingMessageInfo {
        self._breaker_reading_profile().reading_message_info.as_ref().unwrap_or(&breaker_reading_profile::READING_MESSAGE_INFO)
    }
    fn reading_message_info_mut(&mut self) -> &mut super::commonmodule::ReadingMessageInfo {
        self._breaker_reading_profile_mut().reading_message_info.get_or_insert(Default::default())
    }
    fn breaker(&self) -> &Breaker {
        self._breaker_reading_profile().breaker.as_ref().unwrap_or(&breaker_reading_profile::BREAKER)
    }
    fn breaker_mut(&mut self) -> &mut Breaker {
        self._breaker_reading_profile_mut().breaker.get_or_insert(Default::default())
    }
    fn breaker_reading(&self) -> &::std::vec::Vec<BreakerReading> {
        &self._breaker_reading_profile().breaker_reading
    }
    fn breaker_reading_mut(&mut self) -> &mut ::std::vec::Vec<BreakerReading> {
        &mut self._breaker_reading_profile_mut().breaker_reading
    }
}
impl IsBreakerReadingProfile for BreakerReadingProfile {
    fn _breaker_reading_profile(&self) -> &BreakerReadingProfile {
        self
    }
    fn _breaker_reading_profile_mut(&mut self) -> &mut BreakerReadingProfile {
        self
    }
}
impl IsReadingMessageInfo for BreakerReadingProfile {
    fn _reading_message_info(&self) -> &super::commonmodule::ReadingMessageInfo {
        self.parent()
    }
    fn _reading_message_info_mut(&mut self) -> &mut ReadingMessageInfo {
        self.parent_mut()
    }
}
impl IsMessageInfo for BreakerReadingProfile {
    fn _message_info(&self) -> &super::commonmodule::MessageInfo {
        self.parent().parent()
    }
    fn _message_info_mut(&mut self) -> &mut MessageInfo {
        self.parent_mut().parent_mut()
    }
}
impl IsIdentifiedObject for BreakerReadingProfile {
    fn _identified_object(&self) -> &super::commonmodule::IdentifiedObject {
        self.parent().parent().parent()
    }
    fn _identified_object_mut(&mut self) -> &mut IdentifiedObject {
        self.parent_mut().parent_mut().parent_mut()
    }
}
/// Status of a breaker
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize)]
pub struct BreakerStatus {
    /// UML inherited base object
    // parent_message: true
    // required_field: false
    // multiplicity_min: None
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, optional, tag="1")]
    #[serde(default, rename = "statusValue")]
    pub status_value: ::std::option::Option<super::commonmodule::StatusValue>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    #[serde(default, rename = "statusAndEventXCBR")]
    pub status_and_event_xcbr: ::std::option::Option<super::commonmodule::StatusAndEventXcbr>,
}
mod breaker_status {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref STATUS_VALUE: crate::commonmodule::StatusValue = Default::default();
        pub(super) static ref STATUS_AND_EVENT_XCBR: crate::commonmodule::StatusAndEventXcbr = Default::default();
    }
}
impl BreakerStatus {
    pub(crate) fn parent(&self) -> &super::commonmodule::StatusValue {
        self.status_value.as_ref().unwrap_or(&breaker_status::STATUS_VALUE)
    }
    pub(crate) fn parent_mut(&mut self) -> &mut super::commonmodule::StatusValue {
        self.status_value.get_or_insert(Default::default())
    }
}
pub trait IsBreakerStatus {
    fn _breaker_status(&self) -> &BreakerStatus;
    fn _breaker_status_mut(&mut self) -> &mut BreakerStatus;
    fn status_value(&self) -> &super::commonmodule::StatusValue {
        self._breaker_status().status_value.as_ref().unwrap_or(&breaker_status::STATUS_VALUE)
    }
    fn status_value_mut(&mut self) -> &mut super::commonmodule::StatusValue {
        self._breaker_status_mut().status_value.get_or_insert(Default::default())
    }
    fn status_and_event_xcbr(&self) -> &super::commonmodule::StatusAndEventXcbr {
        self._breaker_status().status_and_event_xcbr.as_ref().unwrap_or(&breaker_status::STATUS_AND_EVENT_XCBR)
    }
    fn status_and_event_xcbr_mut(&mut self) -> &mut super::commonmodule::StatusAndEventXcbr {
        self._breaker_status_mut().status_and_event_xcbr.get_or_insert(Default::default())
    }
}
impl IsBreakerStatus for BreakerStatus {
    fn _breaker_status(&self) -> &BreakerStatus {
        self
    }
    fn _breaker_status_mut(&mut self) -> &mut BreakerStatus {
        self
    }
}
impl IsStatusValue for BreakerStatus {
    fn _status_value(&self) -> &super::commonmodule::StatusValue {
        self.parent()
    }
    fn _status_value_mut(&mut self) -> &mut StatusValue {
        self.parent_mut()
    }
}
impl IsIdentifiedObject for BreakerStatus {
    fn _identified_object(&self) -> &super::commonmodule::IdentifiedObject {
        self.parent().parent()
    }
    fn _identified_object_mut(&mut self) -> &mut IdentifiedObject {
        self.parent_mut().parent_mut()
    }
}
/// Breaker status profile
/// OpenFMB Profile Message: true
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize)]
pub struct BreakerStatusProfile {
    /// UML inherited base object
    // parent_message: true
    // required_field: false
    // multiplicity_min: None
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, optional, tag="1")]
    #[serde(default, rename = "statusMessageInfo")]
    pub status_message_info: ::std::option::Option<super::commonmodule::StatusMessageInfo>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: Some(1)
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, optional, tag="2")]
    #[serde(default)]
    pub breaker: ::std::option::Option<Breaker>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: Some(1)
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, optional, tag="3")]
    #[serde(default, rename = "breakerStatus")]
    pub breaker_status: ::std::option::Option<BreakerStatus>,
}
mod breaker_status_profile {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref STATUS_MESSAGE_INFO: crate::commonmodule::StatusMessageInfo = Default::default();
        pub(super) static ref BREAKER: crate::breakermodule::Breaker = Default::default();
        pub(super) static ref BREAKER_STATUS: crate::breakermodule::BreakerStatus = Default::default();
    }
}
impl BreakerStatusProfile {
    pub(crate) fn parent(&self) -> &super::commonmodule::StatusMessageInfo {
        self.status_message_info.as_ref().unwrap_or(&breaker_status_profile::STATUS_MESSAGE_INFO)
    }
    pub(crate) fn parent_mut(&mut self) -> &mut super::commonmodule::StatusMessageInfo {
        self.status_message_info.get_or_insert(Default::default())
    }
}
pub trait IsBreakerStatusProfile {
    fn _breaker_status_profile(&self) -> &BreakerStatusProfile;
    fn _breaker_status_profile_mut(&mut self) -> &mut BreakerStatusProfile;
    fn status_message_info(&self) -> &super::commonmodule::StatusMessageInfo {
        self._breaker_status_profile().status_message_info.as_ref().unwrap_or(&breaker_status_profile::STATUS_MESSAGE_INFO)
    }
    fn status_message_info_mut(&mut self) -> &mut super::commonmodule::StatusMessageInfo {
        self._breaker_status_profile_mut().status_message_info.get_or_insert(Default::default())
    }
    fn breaker(&self) -> &Breaker {
        self._breaker_status_profile().breaker.as_ref().unwrap_or(&breaker_status_profile::BREAKER)
    }
    fn breaker_mut(&mut self) -> &mut Breaker {
        self._breaker_status_profile_mut().breaker.get_or_insert(Default::default())
    }
    fn breaker_status(&self) -> &BreakerStatus {
        self._breaker_status_profile().breaker_status.as_ref().unwrap_or(&breaker_status_profile::BREAKER_STATUS)
    }
    fn breaker_status_mut(&mut self) -> &mut BreakerStatus {
        self._breaker_status_profile_mut().breaker_status.get_or_insert(Default::default())
    }
}
impl IsBreakerStatusProfile for BreakerStatusProfile {
    fn _breaker_status_profile(&self) -> &BreakerStatusProfile {
        self
    }
    fn _breaker_status_profile_mut(&mut self) -> &mut BreakerStatusProfile {
        self
    }
}
impl IsStatusMessageInfo for BreakerStatusProfile {
    fn _status_message_info(&self) -> &super::commonmodule::StatusMessageInfo {
        self.parent()
    }
    fn _status_message_info_mut(&mut self) -> &mut StatusMessageInfo {
        self.parent_mut()
    }
}
impl IsMessageInfo for BreakerStatusProfile {
    fn _message_info(&self) -> &super::commonmodule::MessageInfo {
        self.parent().parent()
    }
    fn _message_info_mut(&mut self) -> &mut MessageInfo {
        self.parent_mut().parent_mut()
    }
}
impl IsIdentifiedObject for BreakerStatusProfile {
    fn _identified_object(&self) -> &super::commonmodule::IdentifiedObject {
        self.parent().parent().parent()
    }
    fn _identified_object_mut(&mut self) -> &mut IdentifiedObject {
        self.parent_mut().parent_mut().parent_mut()
    }
}
