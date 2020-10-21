/// OpenFMB specialization for BreakerDiscreteControlProfile: Added blk  LN: Circuit breaker   Name:
/// XCBR
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct BreakerDiscreteControlXcbr {
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
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="2")]
    pub pos: ::std::option::Option<super::commonmodule::ControlDpc>,
}
mod breaker_discrete_control_xcbr {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref LOGICAL_NODE_FOR_CONTROL: crate::commonmodule::LogicalNodeForControl = Default::default();
        pub(super) static ref POS: crate::commonmodule::ControlDpc = Default::default();
    }
}
pub trait IsBreakerDiscreteControlXcbr {
    fn _breaker_discrete_control_xcbr(&self) -> &BreakerDiscreteControlXcbr;
    fn _mut_breaker_discrete_control_xcbr(&mut self) -> &mut BreakerDiscreteControlXcbr;
    fn logical_node_for_control(&self) -> &super::commonmodule::LogicalNodeForControl {
        self._breaker_discrete_control_xcbr().logical_node_for_control.as_ref().unwrap_or(&breaker_discrete_control_xcbr::LOGICAL_NODE_FOR_CONTROL)
    }
    fn mut_logical_node_for_control(&mut self) -> &mut super::commonmodule::LogicalNodeForControl {
        self._mut_breaker_discrete_control_xcbr().logical_node_for_control.get_or_insert(breaker_discrete_control_xcbr::LOGICAL_NODE_FOR_CONTROL.clone())
    }
    fn pos(&self) -> &super::commonmodule::ControlDpc {
        self._breaker_discrete_control_xcbr().pos.as_ref().unwrap_or(&breaker_discrete_control_xcbr::POS)
    }
    fn mut_pos(&mut self) -> &mut super::commonmodule::ControlDpc {
        self._mut_breaker_discrete_control_xcbr().pos.get_or_insert(breaker_discrete_control_xcbr::POS.clone())
    }
}
impl IsBreakerDiscreteControlXcbr for BreakerDiscreteControlXcbr {
    fn _breaker_discrete_control_xcbr(&self) -> &BreakerDiscreteControlXcbr {
        self
    }
    fn _mut_breaker_discrete_control_xcbr(&mut self) -> &mut BreakerDiscreteControlXcbr {
        self
    }
}
/// Breaker discrete control class
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct BreakerDiscreteControl {
    /// UML inherited base object
    // parent_message: true
    // required_field: false
    // multiplicity_min: 0
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="1")]
    pub control_value: ::std::option::Option<super::commonmodule::ControlValue>,
    /// IEC61850 enhanced control parameters.
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
pub trait IsBreakerDiscreteControl {
    fn _breaker_discrete_control(&self) -> &BreakerDiscreteControl;
    fn _mut_breaker_discrete_control(&mut self) -> &mut BreakerDiscreteControl;
    fn control_value(&self) -> &super::commonmodule::ControlValue {
        self._breaker_discrete_control().control_value.as_ref().unwrap_or(&breaker_discrete_control::CONTROL_VALUE)
    }
    fn mut_control_value(&mut self) -> &mut super::commonmodule::ControlValue {
        self._mut_breaker_discrete_control().control_value.get_or_insert(breaker_discrete_control::CONTROL_VALUE.clone())
    }
    fn check(&self) -> &super::commonmodule::CheckConditions {
        self._breaker_discrete_control().check.as_ref().unwrap_or(&breaker_discrete_control::CHECK)
    }
    fn mut_check(&mut self) -> &mut super::commonmodule::CheckConditions {
        self._mut_breaker_discrete_control().check.get_or_insert(breaker_discrete_control::CHECK.clone())
    }
    fn breaker_discrete_control_xcbr(&self) -> &BreakerDiscreteControlXcbr {
        self._breaker_discrete_control().breaker_discrete_control_xcbr.as_ref().unwrap_or(&breaker_discrete_control::BREAKER_DISCRETE_CONTROL_XCBR)
    }
    fn mut_breaker_discrete_control_xcbr(&mut self) -> &mut BreakerDiscreteControlXcbr {
        self._mut_breaker_discrete_control().breaker_discrete_control_xcbr.get_or_insert(breaker_discrete_control::BREAKER_DISCRETE_CONTROL_XCBR.clone())
    }
}
impl IsBreakerDiscreteControl for BreakerDiscreteControl {
    fn _breaker_discrete_control(&self) -> &BreakerDiscreteControl {
        self
    }
    fn _mut_breaker_discrete_control(&mut self) -> &mut BreakerDiscreteControl {
        self
    }
}
/// A mechanical switching device capable of making, carrying, and breaking currents under normal
/// circuit conditions and also making, carrying for a specified time, and breaking currents under
/// specified abnormal circuit conditions e.g.  those of short circuit.
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Breaker {
    /// UML inherited base object
    // parent_message: true
    // required_field: false
    // multiplicity_min: 0
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="1")]
    pub conducting_equipment: ::std::option::Option<super::commonmodule::ConductingEquipment>,
}
mod breaker {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref CONDUCTING_EQUIPMENT: crate::commonmodule::ConductingEquipment = Default::default();
    }
}
pub trait IsBreaker {
    fn _breaker(&self) -> &Breaker;
    fn _mut_breaker(&mut self) -> &mut Breaker;
    fn conducting_equipment(&self) -> &super::commonmodule::ConductingEquipment {
        self._breaker().conducting_equipment.as_ref().unwrap_or(&breaker::CONDUCTING_EQUIPMENT)
    }
    fn mut_conducting_equipment(&mut self) -> &mut super::commonmodule::ConductingEquipment {
        self._mut_breaker().conducting_equipment.get_or_insert(breaker::CONDUCTING_EQUIPMENT.clone())
    }
}
impl IsBreaker for Breaker {
    fn _breaker(&self) -> &Breaker {
        self
    }
    fn _mut_breaker(&mut self) -> &mut Breaker {
        self
    }
}
/// Instructs an end device (or an end device group) to perform a specified action.
/// OpenFMB Profile Message: true
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct BreakerDiscreteControlProfile {
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
    pub breaker: ::std::option::Option<Breaker>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="3")]
    pub breaker_discrete_control: ::std::option::Option<BreakerDiscreteControl>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="4")]
    pub ied: ::std::option::Option<super::commonmodule::Ied>,
}
mod breaker_discrete_control_profile {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref CONTROL_MESSAGE_INFO: crate::commonmodule::ControlMessageInfo = Default::default();
        pub(super) static ref BREAKER: crate::breakermodule::Breaker = Default::default();
        pub(super) static ref BREAKER_DISCRETE_CONTROL: crate::breakermodule::BreakerDiscreteControl = Default::default();
        pub(super) static ref IED: crate::commonmodule::Ied = Default::default();
    }
}
pub trait IsBreakerDiscreteControlProfile {
    fn _breaker_discrete_control_profile(&self) -> &BreakerDiscreteControlProfile;
    fn _mut_breaker_discrete_control_profile(&mut self) -> &mut BreakerDiscreteControlProfile;
    fn control_message_info(&self) -> &super::commonmodule::ControlMessageInfo {
        self._breaker_discrete_control_profile().control_message_info.as_ref().unwrap_or(&breaker_discrete_control_profile::CONTROL_MESSAGE_INFO)
    }
    fn mut_control_message_info(&mut self) -> &mut super::commonmodule::ControlMessageInfo {
        self._mut_breaker_discrete_control_profile().control_message_info.get_or_insert(breaker_discrete_control_profile::CONTROL_MESSAGE_INFO.clone())
    }
    fn breaker(&self) -> &Breaker {
        self._breaker_discrete_control_profile().breaker.as_ref().unwrap_or(&breaker_discrete_control_profile::BREAKER)
    }
    fn mut_breaker(&mut self) -> &mut Breaker {
        self._mut_breaker_discrete_control_profile().breaker.get_or_insert(breaker_discrete_control_profile::BREAKER.clone())
    }
    fn breaker_discrete_control(&self) -> &BreakerDiscreteControl {
        self._breaker_discrete_control_profile().breaker_discrete_control.as_ref().unwrap_or(&breaker_discrete_control_profile::BREAKER_DISCRETE_CONTROL)
    }
    fn mut_breaker_discrete_control(&mut self) -> &mut BreakerDiscreteControl {
        self._mut_breaker_discrete_control_profile().breaker_discrete_control.get_or_insert(breaker_discrete_control_profile::BREAKER_DISCRETE_CONTROL.clone())
    }
    fn ied(&self) -> &super::commonmodule::Ied {
        self._breaker_discrete_control_profile().ied.as_ref().unwrap_or(&breaker_discrete_control_profile::IED)
    }
    fn mut_ied(&mut self) -> &mut super::commonmodule::Ied {
        self._mut_breaker_discrete_control_profile().ied.get_or_insert(breaker_discrete_control_profile::IED.clone())
    }
}
impl IsBreakerDiscreteControlProfile for BreakerDiscreteControlProfile {
    fn _breaker_discrete_control_profile(&self) -> &BreakerDiscreteControlProfile {
        self
    }
    fn _mut_breaker_discrete_control_profile(&mut self) -> &mut BreakerDiscreteControlProfile {
        self
    }
}
/// Breaker event class
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct BreakerEvent {
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
    pub status_and_event_xcbr: ::std::option::Option<super::commonmodule::StatusAndEventXcbr>,
}
mod breaker_event {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref EVENT_VALUE: crate::commonmodule::EventValue = Default::default();
        pub(super) static ref STATUS_AND_EVENT_XCBR: crate::commonmodule::StatusAndEventXcbr = Default::default();
    }
}
pub trait IsBreakerEvent {
    fn _breaker_event(&self) -> &BreakerEvent;
    fn _mut_breaker_event(&mut self) -> &mut BreakerEvent;
    fn event_value(&self) -> &super::commonmodule::EventValue {
        self._breaker_event().event_value.as_ref().unwrap_or(&breaker_event::EVENT_VALUE)
    }
    fn mut_event_value(&mut self) -> &mut super::commonmodule::EventValue {
        self._mut_breaker_event().event_value.get_or_insert(breaker_event::EVENT_VALUE.clone())
    }
    fn status_and_event_xcbr(&self) -> &super::commonmodule::StatusAndEventXcbr {
        self._breaker_event().status_and_event_xcbr.as_ref().unwrap_or(&breaker_event::STATUS_AND_EVENT_XCBR)
    }
    fn mut_status_and_event_xcbr(&mut self) -> &mut super::commonmodule::StatusAndEventXcbr {
        self._mut_breaker_event().status_and_event_xcbr.get_or_insert(breaker_event::STATUS_AND_EVENT_XCBR.clone())
    }
}
impl IsBreakerEvent for BreakerEvent {
    fn _breaker_event(&self) -> &BreakerEvent {
        self
    }
    fn _mut_breaker_event(&mut self) -> &mut BreakerEvent {
        self
    }
}
/// Breaker event profile
/// OpenFMB Profile Message: true
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct BreakerEventProfile {
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
    pub breaker: ::std::option::Option<Breaker>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="3")]
    pub breaker_event: ::std::option::Option<BreakerEvent>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="4")]
    pub ied: ::std::option::Option<super::commonmodule::Ied>,
}
mod breaker_event_profile {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref EVENT_MESSAGE_INFO: crate::commonmodule::EventMessageInfo = Default::default();
        pub(super) static ref BREAKER: crate::breakermodule::Breaker = Default::default();
        pub(super) static ref BREAKER_EVENT: crate::breakermodule::BreakerEvent = Default::default();
        pub(super) static ref IED: crate::commonmodule::Ied = Default::default();
    }
}
pub trait IsBreakerEventProfile {
    fn _breaker_event_profile(&self) -> &BreakerEventProfile;
    fn _mut_breaker_event_profile(&mut self) -> &mut BreakerEventProfile;
    fn event_message_info(&self) -> &super::commonmodule::EventMessageInfo {
        self._breaker_event_profile().event_message_info.as_ref().unwrap_or(&breaker_event_profile::EVENT_MESSAGE_INFO)
    }
    fn mut_event_message_info(&mut self) -> &mut super::commonmodule::EventMessageInfo {
        self._mut_breaker_event_profile().event_message_info.get_or_insert(breaker_event_profile::EVENT_MESSAGE_INFO.clone())
    }
    fn breaker(&self) -> &Breaker {
        self._breaker_event_profile().breaker.as_ref().unwrap_or(&breaker_event_profile::BREAKER)
    }
    fn mut_breaker(&mut self) -> &mut Breaker {
        self._mut_breaker_event_profile().breaker.get_or_insert(breaker_event_profile::BREAKER.clone())
    }
    fn breaker_event(&self) -> &BreakerEvent {
        self._breaker_event_profile().breaker_event.as_ref().unwrap_or(&breaker_event_profile::BREAKER_EVENT)
    }
    fn mut_breaker_event(&mut self) -> &mut BreakerEvent {
        self._mut_breaker_event_profile().breaker_event.get_or_insert(breaker_event_profile::BREAKER_EVENT.clone())
    }
    fn ied(&self) -> &super::commonmodule::Ied {
        self._breaker_event_profile().ied.as_ref().unwrap_or(&breaker_event_profile::IED)
    }
    fn mut_ied(&mut self) -> &mut super::commonmodule::Ied {
        self._mut_breaker_event_profile().ied.get_or_insert(breaker_event_profile::IED.clone())
    }
}
impl IsBreakerEventProfile for BreakerEventProfile {
    fn _breaker_event_profile(&self) -> &BreakerEventProfile {
        self
    }
    fn _mut_breaker_event_profile(&mut self) -> &mut BreakerEventProfile {
        self
    }
}
/// MISSING DOCUMENTATION!!!
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct BreakerReading {
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
pub trait IsBreakerReading {
    fn _breaker_reading(&self) -> &BreakerReading;
    fn _mut_breaker_reading(&mut self) -> &mut BreakerReading;
    fn conducting_equipment_terminal_reading(&self) -> &super::commonmodule::ConductingEquipmentTerminalReading {
        self._breaker_reading().conducting_equipment_terminal_reading.as_ref().unwrap_or(&breaker_reading::CONDUCTING_EQUIPMENT_TERMINAL_READING)
    }
    fn mut_conducting_equipment_terminal_reading(&mut self) -> &mut super::commonmodule::ConductingEquipmentTerminalReading {
        self._mut_breaker_reading().conducting_equipment_terminal_reading.get_or_insert(breaker_reading::CONDUCTING_EQUIPMENT_TERMINAL_READING.clone())
    }
    fn diff_reading_mmxu(&self) -> &super::commonmodule::ReadingMmxu {
        self._breaker_reading().diff_reading_mmxu.as_ref().unwrap_or(&breaker_reading::DIFF_READING_MMXU)
    }
    fn mut_diff_reading_mmxu(&mut self) -> &mut super::commonmodule::ReadingMmxu {
        self._mut_breaker_reading().diff_reading_mmxu.get_or_insert(breaker_reading::DIFF_READING_MMXU.clone())
    }
    fn phase_mmtn(&self) -> &super::commonmodule::PhaseMmtn {
        self._breaker_reading().phase_mmtn.as_ref().unwrap_or(&breaker_reading::PHASE_MMTN)
    }
    fn mut_phase_mmtn(&mut self) -> &mut super::commonmodule::PhaseMmtn {
        self._mut_breaker_reading().phase_mmtn.get_or_insert(breaker_reading::PHASE_MMTN.clone())
    }
    fn reading_mmtr(&self) -> &super::commonmodule::ReadingMmtr {
        self._breaker_reading().reading_mmtr.as_ref().unwrap_or(&breaker_reading::READING_MMTR)
    }
    fn mut_reading_mmtr(&mut self) -> &mut super::commonmodule::ReadingMmtr {
        self._mut_breaker_reading().reading_mmtr.get_or_insert(breaker_reading::READING_MMTR.clone())
    }
    fn reading_mmxu(&self) -> &super::commonmodule::ReadingMmxu {
        self._breaker_reading().reading_mmxu.as_ref().unwrap_or(&breaker_reading::READING_MMXU)
    }
    fn mut_reading_mmxu(&mut self) -> &mut super::commonmodule::ReadingMmxu {
        self._mut_breaker_reading().reading_mmxu.get_or_insert(breaker_reading::READING_MMXU.clone())
    }
}
impl IsBreakerReading for BreakerReading {
    fn _breaker_reading(&self) -> &BreakerReading {
        self
    }
    fn _mut_breaker_reading(&mut self) -> &mut BreakerReading {
        self
    }
}
/// Breaker reading profile
/// OpenFMB Profile Message: true
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct BreakerReadingProfile {
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
    pub breaker: ::std::option::Option<Breaker>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 2
    // uuid: false
    // key: false
    #[prost(message, repeated, tag="3")]
    pub breaker_reading: ::std::vec::Vec<BreakerReading>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="4")]
    pub ied: ::std::option::Option<super::commonmodule::Ied>,
}
mod breaker_reading_profile {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref READING_MESSAGE_INFO: crate::commonmodule::ReadingMessageInfo = Default::default();
        pub(super) static ref BREAKER: crate::breakermodule::Breaker = Default::default();
        pub(super) static ref IED: crate::commonmodule::Ied = Default::default();
    }
}
pub trait IsBreakerReadingProfile {
    fn _breaker_reading_profile(&self) -> &BreakerReadingProfile;
    fn _mut_breaker_reading_profile(&mut self) -> &mut BreakerReadingProfile;
    fn reading_message_info(&self) -> &super::commonmodule::ReadingMessageInfo {
        self._breaker_reading_profile().reading_message_info.as_ref().unwrap_or(&breaker_reading_profile::READING_MESSAGE_INFO)
    }
    fn mut_reading_message_info(&mut self) -> &mut super::commonmodule::ReadingMessageInfo {
        self._mut_breaker_reading_profile().reading_message_info.get_or_insert(breaker_reading_profile::READING_MESSAGE_INFO.clone())
    }
    fn breaker(&self) -> &Breaker {
        self._breaker_reading_profile().breaker.as_ref().unwrap_or(&breaker_reading_profile::BREAKER)
    }
    fn mut_breaker(&mut self) -> &mut Breaker {
        self._mut_breaker_reading_profile().breaker.get_or_insert(breaker_reading_profile::BREAKER.clone())
    }
    fn breaker_reading(&self) -> &::std::vec::Vec<BreakerReading> {
        &self._breaker_reading_profile().breaker_reading    }
    fn mut_breaker_reading(&mut self) -> &mut ::std::vec::Vec<BreakerReading> {
        &mut self._mut_breaker_reading_profile().breaker_reading    }
    fn ied(&self) -> &super::commonmodule::Ied {
        self._breaker_reading_profile().ied.as_ref().unwrap_or(&breaker_reading_profile::IED)
    }
    fn mut_ied(&mut self) -> &mut super::commonmodule::Ied {
        self._mut_breaker_reading_profile().ied.get_or_insert(breaker_reading_profile::IED.clone())
    }
}
impl IsBreakerReadingProfile for BreakerReadingProfile {
    fn _breaker_reading_profile(&self) -> &BreakerReadingProfile {
        self
    }
    fn _mut_breaker_reading_profile(&mut self) -> &mut BreakerReadingProfile {
        self
    }
}
/// Status of a breaker
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct BreakerStatus {
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
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="2")]
    pub status_and_event_xcbr: ::std::option::Option<super::commonmodule::StatusAndEventXcbr>,
}
mod breaker_status {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref STATUS_VALUE: crate::commonmodule::StatusValue = Default::default();
        pub(super) static ref STATUS_AND_EVENT_XCBR: crate::commonmodule::StatusAndEventXcbr = Default::default();
    }
}
pub trait IsBreakerStatus {
    fn _breaker_status(&self) -> &BreakerStatus;
    fn _mut_breaker_status(&mut self) -> &mut BreakerStatus;
    fn status_value(&self) -> &super::commonmodule::StatusValue {
        self._breaker_status().status_value.as_ref().unwrap_or(&breaker_status::STATUS_VALUE)
    }
    fn mut_status_value(&mut self) -> &mut super::commonmodule::StatusValue {
        self._mut_breaker_status().status_value.get_or_insert(breaker_status::STATUS_VALUE.clone())
    }
    fn status_and_event_xcbr(&self) -> &super::commonmodule::StatusAndEventXcbr {
        self._breaker_status().status_and_event_xcbr.as_ref().unwrap_or(&breaker_status::STATUS_AND_EVENT_XCBR)
    }
    fn mut_status_and_event_xcbr(&mut self) -> &mut super::commonmodule::StatusAndEventXcbr {
        self._mut_breaker_status().status_and_event_xcbr.get_or_insert(breaker_status::STATUS_AND_EVENT_XCBR.clone())
    }
}
impl IsBreakerStatus for BreakerStatus {
    fn _breaker_status(&self) -> &BreakerStatus {
        self
    }
    fn _mut_breaker_status(&mut self) -> &mut BreakerStatus {
        self
    }
}
/// Breaker status profile
/// OpenFMB Profile Message: true
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct BreakerStatusProfile {
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
    pub breaker: ::std::option::Option<Breaker>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="3")]
    pub breaker_status: ::std::option::Option<BreakerStatus>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="4")]
    pub ied: ::std::option::Option<super::commonmodule::Ied>,
}
mod breaker_status_profile {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref STATUS_MESSAGE_INFO: crate::commonmodule::StatusMessageInfo = Default::default();
        pub(super) static ref BREAKER: crate::breakermodule::Breaker = Default::default();
        pub(super) static ref BREAKER_STATUS: crate::breakermodule::BreakerStatus = Default::default();
        pub(super) static ref IED: crate::commonmodule::Ied = Default::default();
    }
}
pub trait IsBreakerStatusProfile {
    fn _breaker_status_profile(&self) -> &BreakerStatusProfile;
    fn _mut_breaker_status_profile(&mut self) -> &mut BreakerStatusProfile;
    fn status_message_info(&self) -> &super::commonmodule::StatusMessageInfo {
        self._breaker_status_profile().status_message_info.as_ref().unwrap_or(&breaker_status_profile::STATUS_MESSAGE_INFO)
    }
    fn mut_status_message_info(&mut self) -> &mut super::commonmodule::StatusMessageInfo {
        self._mut_breaker_status_profile().status_message_info.get_or_insert(breaker_status_profile::STATUS_MESSAGE_INFO.clone())
    }
    fn breaker(&self) -> &Breaker {
        self._breaker_status_profile().breaker.as_ref().unwrap_or(&breaker_status_profile::BREAKER)
    }
    fn mut_breaker(&mut self) -> &mut Breaker {
        self._mut_breaker_status_profile().breaker.get_or_insert(breaker_status_profile::BREAKER.clone())
    }
    fn breaker_status(&self) -> &BreakerStatus {
        self._breaker_status_profile().breaker_status.as_ref().unwrap_or(&breaker_status_profile::BREAKER_STATUS)
    }
    fn mut_breaker_status(&mut self) -> &mut BreakerStatus {
        self._mut_breaker_status_profile().breaker_status.get_or_insert(breaker_status_profile::BREAKER_STATUS.clone())
    }
    fn ied(&self) -> &super::commonmodule::Ied {
        self._breaker_status_profile().ied.as_ref().unwrap_or(&breaker_status_profile::IED)
    }
    fn mut_ied(&mut self) -> &mut super::commonmodule::Ied {
        self._mut_breaker_status_profile().ied.get_or_insert(breaker_status_profile::IED.clone())
    }
}
impl IsBreakerStatusProfile for BreakerStatusProfile {
    fn _breaker_status_profile(&self) -> &BreakerStatusProfile {
        self
    }
    fn _mut_breaker_status_profile(&mut self) -> &mut BreakerStatusProfile {
        self
    }
}
