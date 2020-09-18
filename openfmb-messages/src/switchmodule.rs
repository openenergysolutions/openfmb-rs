/// OpenFMB specialization for switch control:  LN: Circuit switch   Name: XSWI
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SwitchDiscreteControlXswi {
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
    pub pos: ::std::option::Option<super::commonmodule::ControlDpc>,
}
mod switch_discrete_control_xswi {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref LOGICAL_NODE_FOR_CONTROL: crate::commonmodule::LogicalNodeForControl = Default::default();
        pub(super) static ref POS: crate::commonmodule::ControlDpc = Default::default();
    }
}
trait IsSwitchDiscreteControlXswi {
    fn _switch_discrete_control_xswi(&self) -> &SwitchDiscreteControlXswi;
    fn logical_node_for_control(&self) -> &super::commonmodule::LogicalNodeForControl {
        self._switch_discrete_control_xswi().logical_node_for_control.as_ref().unwrap_or(&switch_discrete_control_xswi::LOGICAL_NODE_FOR_CONTROL)
    }
    fn pos(&self) -> &super::commonmodule::ControlDpc {
        self._switch_discrete_control_xswi().pos.as_ref().unwrap_or(&switch_discrete_control_xswi::POS)
    }
}
impl IsSwitchDiscreteControlXswi for SwitchDiscreteControlXswi {
    fn _switch_discrete_control_xswi(&self) -> &SwitchDiscreteControlXswi {
        self
    }
}
/// Switch discrete control
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SwitchDiscreteControl {
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
    pub switch_discrete_control_xswi: ::std::option::Option<SwitchDiscreteControlXswi>,
}
mod switch_discrete_control {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref CONTROL_VALUE: crate::commonmodule::ControlValue = Default::default();
        pub(super) static ref CHECK: crate::commonmodule::CheckConditions = Default::default();
        pub(super) static ref SWITCH_DISCRETE_CONTROL_XSWI: crate::switchmodule::SwitchDiscreteControlXswi = Default::default();
    }
}
trait IsSwitchDiscreteControl {
    fn _switch_discrete_control(&self) -> &SwitchDiscreteControl;
    fn control_value(&self) -> &super::commonmodule::ControlValue {
        self._switch_discrete_control().control_value.as_ref().unwrap_or(&switch_discrete_control::CONTROL_VALUE)
    }
    fn check(&self) -> &super::commonmodule::CheckConditions {
        self._switch_discrete_control().check.as_ref().unwrap_or(&switch_discrete_control::CHECK)
    }
    fn switch_discrete_control_xswi(&self) -> &SwitchDiscreteControlXswi {
        self._switch_discrete_control().switch_discrete_control_xswi.as_ref().unwrap_or(&switch_discrete_control::SWITCH_DISCRETE_CONTROL_XSWI)
    }
}
impl IsSwitchDiscreteControl for SwitchDiscreteControl {
    fn _switch_discrete_control(&self) -> &SwitchDiscreteControl {
        self
    }
}
/// A ProtectedSwitch is a switching device that can be operated by ProtectionEquipment.
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ProtectedSwitch {
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
mod protected_switch {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref CONDUCTING_EQUIPMENT: crate::commonmodule::ConductingEquipment = Default::default();
    }
}
trait IsProtectedSwitch {
    fn _protected_switch(&self) -> &ProtectedSwitch;
    fn conducting_equipment(&self) -> &super::commonmodule::ConductingEquipment {
        self._protected_switch().conducting_equipment.as_ref().unwrap_or(&protected_switch::CONDUCTING_EQUIPMENT)
    }
}
impl IsProtectedSwitch for ProtectedSwitch {
    fn _protected_switch(&self) -> &ProtectedSwitch {
        self
    }
}
/// Switch control profile
/// OpenFMB Profile Message: true
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SwitchDiscreteControlProfile {
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
    pub ied: ::std::option::Option<super::commonmodule::Ied>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="3")]
    pub protected_switch: ::std::option::Option<ProtectedSwitch>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="4")]
    pub switch_discrete_control: ::std::option::Option<SwitchDiscreteControl>,
}
mod switch_discrete_control_profile {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref CONTROL_MESSAGE_INFO: crate::commonmodule::ControlMessageInfo = Default::default();
        pub(super) static ref IED: crate::commonmodule::Ied = Default::default();
        pub(super) static ref PROTECTED_SWITCH: crate::switchmodule::ProtectedSwitch = Default::default();
        pub(super) static ref SWITCH_DISCRETE_CONTROL: crate::switchmodule::SwitchDiscreteControl = Default::default();
    }
}
trait IsSwitchDiscreteControlProfile {
    fn _switch_discrete_control_profile(&self) -> &SwitchDiscreteControlProfile;
    fn control_message_info(&self) -> &super::commonmodule::ControlMessageInfo {
        self._switch_discrete_control_profile().control_message_info.as_ref().unwrap_or(&switch_discrete_control_profile::CONTROL_MESSAGE_INFO)
    }
    fn ied(&self) -> &super::commonmodule::Ied {
        self._switch_discrete_control_profile().ied.as_ref().unwrap_or(&switch_discrete_control_profile::IED)
    }
    fn protected_switch(&self) -> &ProtectedSwitch {
        self._switch_discrete_control_profile().protected_switch.as_ref().unwrap_or(&switch_discrete_control_profile::PROTECTED_SWITCH)
    }
    fn switch_discrete_control(&self) -> &SwitchDiscreteControl {
        self._switch_discrete_control_profile().switch_discrete_control.as_ref().unwrap_or(&switch_discrete_control_profile::SWITCH_DISCRETE_CONTROL)
    }
}
impl IsSwitchDiscreteControlProfile for SwitchDiscreteControlProfile {
    fn _switch_discrete_control_profile(&self) -> &SwitchDiscreteControlProfile {
        self
    }
}
/// OpenFMB specialization for SwitchEventProfile
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SwitchEventXswi {
    /// UML inherited base object
    // parent_message: true
    // required_field: false
    // multiplicity_min: 0
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="1")]
    pub logical_node_for_event_and_status: ::std::option::Option<super::commonmodule::LogicalNodeForEventAndStatus>,
    /// Dynamic test status
    #[prost(message, optional, tag="2")]
    pub dynamic_test: ::std::option::Option<super::commonmodule::EnsDynamicTestKind>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="3")]
    pub pos: ::std::option::Option<super::commonmodule::StatusDps>,
}
mod switch_event_xswi {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref LOGICAL_NODE_FOR_EVENT_AND_STATUS: crate::commonmodule::LogicalNodeForEventAndStatus = Default::default();
        pub(super) static ref DYNAMIC_TEST: crate::commonmodule::EnsDynamicTestKind = Default::default();
        pub(super) static ref POS: crate::commonmodule::StatusDps = Default::default();
    }
}
trait IsSwitchEventXswi {
    fn _switch_event_xswi(&self) -> &SwitchEventXswi;
    fn logical_node_for_event_and_status(&self) -> &super::commonmodule::LogicalNodeForEventAndStatus {
        self._switch_event_xswi().logical_node_for_event_and_status.as_ref().unwrap_or(&switch_event_xswi::LOGICAL_NODE_FOR_EVENT_AND_STATUS)
    }
    fn dynamic_test(&self) -> &super::commonmodule::EnsDynamicTestKind {
        self._switch_event_xswi().dynamic_test.as_ref().unwrap_or(&switch_event_xswi::DYNAMIC_TEST)
    }
    fn pos(&self) -> &super::commonmodule::StatusDps {
        self._switch_event_xswi().pos.as_ref().unwrap_or(&switch_event_xswi::POS)
    }
}
impl IsSwitchEventXswi for SwitchEventXswi {
    fn _switch_event_xswi(&self) -> &SwitchEventXswi {
        self
    }
}
/// Switch event
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SwitchEvent {
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
    pub switch_event_xswi: ::std::option::Option<SwitchEventXswi>,
}
mod switch_event {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref EVENT_VALUE: crate::commonmodule::EventValue = Default::default();
        pub(super) static ref SWITCH_EVENT_XSWI: crate::switchmodule::SwitchEventXswi = Default::default();
    }
}
trait IsSwitchEvent {
    fn _switch_event(&self) -> &SwitchEvent;
    fn event_value(&self) -> &super::commonmodule::EventValue {
        self._switch_event().event_value.as_ref().unwrap_or(&switch_event::EVENT_VALUE)
    }
    fn switch_event_xswi(&self) -> &SwitchEventXswi {
        self._switch_event().switch_event_xswi.as_ref().unwrap_or(&switch_event::SWITCH_EVENT_XSWI)
    }
}
impl IsSwitchEvent for SwitchEvent {
    fn _switch_event(&self) -> &SwitchEvent {
        self
    }
}
/// Switch event profile
/// OpenFMB Profile Message: true
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SwitchEventProfile {
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
    pub ied: ::std::option::Option<super::commonmodule::Ied>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="3")]
    pub protected_switch: ::std::option::Option<ProtectedSwitch>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="4")]
    pub switch_event: ::std::option::Option<SwitchEvent>,
}
mod switch_event_profile {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref EVENT_MESSAGE_INFO: crate::commonmodule::EventMessageInfo = Default::default();
        pub(super) static ref IED: crate::commonmodule::Ied = Default::default();
        pub(super) static ref PROTECTED_SWITCH: crate::switchmodule::ProtectedSwitch = Default::default();
        pub(super) static ref SWITCH_EVENT: crate::switchmodule::SwitchEvent = Default::default();
    }
}
trait IsSwitchEventProfile {
    fn _switch_event_profile(&self) -> &SwitchEventProfile;
    fn event_message_info(&self) -> &super::commonmodule::EventMessageInfo {
        self._switch_event_profile().event_message_info.as_ref().unwrap_or(&switch_event_profile::EVENT_MESSAGE_INFO)
    }
    fn ied(&self) -> &super::commonmodule::Ied {
        self._switch_event_profile().ied.as_ref().unwrap_or(&switch_event_profile::IED)
    }
    fn protected_switch(&self) -> &ProtectedSwitch {
        self._switch_event_profile().protected_switch.as_ref().unwrap_or(&switch_event_profile::PROTECTED_SWITCH)
    }
    fn switch_event(&self) -> &SwitchEvent {
        self._switch_event_profile().switch_event.as_ref().unwrap_or(&switch_event_profile::SWITCH_EVENT)
    }
}
impl IsSwitchEventProfile for SwitchEventProfile {
    fn _switch_event_profile(&self) -> &SwitchEventProfile {
        self
    }
}
/// Switch reading value
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SwitchReading {
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
mod switch_reading {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref CONDUCTING_EQUIPMENT_TERMINAL_READING: crate::commonmodule::ConductingEquipmentTerminalReading = Default::default();
        pub(super) static ref DIFF_READING_MMXU: crate::commonmodule::ReadingMmxu = Default::default();
        pub(super) static ref PHASE_MMTN: crate::commonmodule::PhaseMmtn = Default::default();
        pub(super) static ref READING_MMTR: crate::commonmodule::ReadingMmtr = Default::default();
        pub(super) static ref READING_MMXU: crate::commonmodule::ReadingMmxu = Default::default();
    }
}
trait IsSwitchReading {
    fn _switch_reading(&self) -> &SwitchReading;
    fn conducting_equipment_terminal_reading(&self) -> &super::commonmodule::ConductingEquipmentTerminalReading {
        self._switch_reading().conducting_equipment_terminal_reading.as_ref().unwrap_or(&switch_reading::CONDUCTING_EQUIPMENT_TERMINAL_READING)
    }
    fn diff_reading_mmxu(&self) -> &super::commonmodule::ReadingMmxu {
        self._switch_reading().diff_reading_mmxu.as_ref().unwrap_or(&switch_reading::DIFF_READING_MMXU)
    }
    fn phase_mmtn(&self) -> &super::commonmodule::PhaseMmtn {
        self._switch_reading().phase_mmtn.as_ref().unwrap_or(&switch_reading::PHASE_MMTN)
    }
    fn reading_mmtr(&self) -> &super::commonmodule::ReadingMmtr {
        self._switch_reading().reading_mmtr.as_ref().unwrap_or(&switch_reading::READING_MMTR)
    }
    fn reading_mmxu(&self) -> &super::commonmodule::ReadingMmxu {
        self._switch_reading().reading_mmxu.as_ref().unwrap_or(&switch_reading::READING_MMXU)
    }
}
impl IsSwitchReading for SwitchReading {
    fn _switch_reading(&self) -> &SwitchReading {
        self
    }
}
/// Switch reading profile
/// OpenFMB Profile Message: true
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SwitchReadingProfile {
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
    pub ied: ::std::option::Option<super::commonmodule::Ied>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="3")]
    pub protected_switch: ::std::option::Option<ProtectedSwitch>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 2
    // uuid: false
    // key: false
    #[prost(message, repeated, tag="4")]
    pub switch_reading: ::std::vec::Vec<SwitchReading>,
}
mod switch_reading_profile {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref READING_MESSAGE_INFO: crate::commonmodule::ReadingMessageInfo = Default::default();
        pub(super) static ref IED: crate::commonmodule::Ied = Default::default();
        pub(super) static ref PROTECTED_SWITCH: crate::switchmodule::ProtectedSwitch = Default::default();
    }
}
trait IsSwitchReadingProfile {
    fn _switch_reading_profile(&self) -> &SwitchReadingProfile;
    fn reading_message_info(&self) -> &super::commonmodule::ReadingMessageInfo {
        self._switch_reading_profile().reading_message_info.as_ref().unwrap_or(&switch_reading_profile::READING_MESSAGE_INFO)
    }
    fn ied(&self) -> &super::commonmodule::Ied {
        self._switch_reading_profile().ied.as_ref().unwrap_or(&switch_reading_profile::IED)
    }
    fn protected_switch(&self) -> &ProtectedSwitch {
        self._switch_reading_profile().protected_switch.as_ref().unwrap_or(&switch_reading_profile::PROTECTED_SWITCH)
    }
    fn switch_reading(&self) -> &::std::vec::Vec<SwitchReading> {
        &self._switch_reading_profile().switch_reading    }
}
impl IsSwitchReadingProfile for SwitchReadingProfile {
    fn _switch_reading_profile(&self) -> &SwitchReadingProfile {
        self
    }
}
/// OpenFMB specialization for SwitchStatusProfile
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SwitchStatusXswi {
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
    pub dynamic_test: ::std::option::Option<super::commonmodule::EnsDynamicTestKind>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="3")]
    pub pos: ::std::option::Option<super::commonmodule::StatusDps>,
}
mod switch_status_xswi {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref LOGICAL_NODE_FOR_EVENT_AND_STATUS: crate::commonmodule::LogicalNodeForEventAndStatus = Default::default();
        pub(super) static ref DYNAMIC_TEST: crate::commonmodule::EnsDynamicTestKind = Default::default();
        pub(super) static ref POS: crate::commonmodule::StatusDps = Default::default();
    }
}
trait IsSwitchStatusXswi {
    fn _switch_status_xswi(&self) -> &SwitchStatusXswi;
    fn logical_node_for_event_and_status(&self) -> &super::commonmodule::LogicalNodeForEventAndStatus {
        self._switch_status_xswi().logical_node_for_event_and_status.as_ref().unwrap_or(&switch_status_xswi::LOGICAL_NODE_FOR_EVENT_AND_STATUS)
    }
    fn dynamic_test(&self) -> &super::commonmodule::EnsDynamicTestKind {
        self._switch_status_xswi().dynamic_test.as_ref().unwrap_or(&switch_status_xswi::DYNAMIC_TEST)
    }
    fn pos(&self) -> &super::commonmodule::StatusDps {
        self._switch_status_xswi().pos.as_ref().unwrap_or(&switch_status_xswi::POS)
    }
}
impl IsSwitchStatusXswi for SwitchStatusXswi {
    fn _switch_status_xswi(&self) -> &SwitchStatusXswi {
        self
    }
}
/// Switch status
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SwitchStatus {
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
    pub switch_status_xswi: ::std::option::Option<SwitchStatusXswi>,
}
mod switch_status {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref STATUS_VALUE: crate::commonmodule::StatusValue = Default::default();
        pub(super) static ref SWITCH_STATUS_XSWI: crate::switchmodule::SwitchStatusXswi = Default::default();
    }
}
trait IsSwitchStatus {
    fn _switch_status(&self) -> &SwitchStatus;
    fn status_value(&self) -> &super::commonmodule::StatusValue {
        self._switch_status().status_value.as_ref().unwrap_or(&switch_status::STATUS_VALUE)
    }
    fn switch_status_xswi(&self) -> &SwitchStatusXswi {
        self._switch_status().switch_status_xswi.as_ref().unwrap_or(&switch_status::SWITCH_STATUS_XSWI)
    }
}
impl IsSwitchStatus for SwitchStatus {
    fn _switch_status(&self) -> &SwitchStatus {
        self
    }
}
/// Switch status profile
/// OpenFMB Profile Message: true
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SwitchStatusProfile {
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
    pub ied: ::std::option::Option<super::commonmodule::Ied>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="3")]
    pub protected_switch: ::std::option::Option<ProtectedSwitch>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="4")]
    pub switch_status: ::std::option::Option<SwitchStatus>,
}
mod switch_status_profile {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref STATUS_MESSAGE_INFO: crate::commonmodule::StatusMessageInfo = Default::default();
        pub(super) static ref IED: crate::commonmodule::Ied = Default::default();
        pub(super) static ref PROTECTED_SWITCH: crate::switchmodule::ProtectedSwitch = Default::default();
        pub(super) static ref SWITCH_STATUS: crate::switchmodule::SwitchStatus = Default::default();
    }
}
trait IsSwitchStatusProfile {
    fn _switch_status_profile(&self) -> &SwitchStatusProfile;
    fn status_message_info(&self) -> &super::commonmodule::StatusMessageInfo {
        self._switch_status_profile().status_message_info.as_ref().unwrap_or(&switch_status_profile::STATUS_MESSAGE_INFO)
    }
    fn ied(&self) -> &super::commonmodule::Ied {
        self._switch_status_profile().ied.as_ref().unwrap_or(&switch_status_profile::IED)
    }
    fn protected_switch(&self) -> &ProtectedSwitch {
        self._switch_status_profile().protected_switch.as_ref().unwrap_or(&switch_status_profile::PROTECTED_SWITCH)
    }
    fn switch_status(&self) -> &SwitchStatus {
        self._switch_status_profile().switch_status.as_ref().unwrap_or(&switch_status_profile::SWITCH_STATUS)
    }
}
impl IsSwitchStatusProfile for SwitchStatusProfile {
    fn _switch_status_profile(&self) -> &SwitchStatusProfile {
        self
    }
}
/// Specialized 61850 FSCC class.  LN: Schedule controller   Name: FSCC
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SwitchControlFscc {
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
    pub switch_control_schedule_fsch: ::std::option::Option<super::commonmodule::SwitchControlScheduleFsch>,
}
mod switch_control_fscc {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref LOGICAL_NODE_FOR_CONTROL: crate::commonmodule::LogicalNodeForControl = Default::default();
        pub(super) static ref SWITCH_CONTROL_SCHEDULE_FSCH: crate::commonmodule::SwitchControlScheduleFsch = Default::default();
    }
}
trait IsSwitchControlFscc {
    fn _switch_control_fscc(&self) -> &SwitchControlFscc;
    fn logical_node_for_control(&self) -> &super::commonmodule::LogicalNodeForControl {
        self._switch_control_fscc().logical_node_for_control.as_ref().unwrap_or(&switch_control_fscc::LOGICAL_NODE_FOR_CONTROL)
    }
    fn switch_control_schedule_fsch(&self) -> &super::commonmodule::SwitchControlScheduleFsch {
        self._switch_control_fscc().switch_control_schedule_fsch.as_ref().unwrap_or(&switch_control_fscc::SWITCH_CONTROL_SCHEDULE_FSCH)
    }
}
impl IsSwitchControlFscc for SwitchControlFscc {
    fn _switch_control_fscc(&self) -> &SwitchControlFscc {
        self
    }
}
/// Switch discrete control
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SwitchControl {
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
    pub switch_control_fscc: ::std::option::Option<SwitchControlFscc>,
}
mod switch_control {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref CONTROL_VALUE: crate::commonmodule::ControlValue = Default::default();
        pub(super) static ref CHECK: crate::commonmodule::CheckConditions = Default::default();
        pub(super) static ref SWITCH_CONTROL_FSCC: crate::switchmodule::SwitchControlFscc = Default::default();
    }
}
trait IsSwitchControl {
    fn _switch_control(&self) -> &SwitchControl;
    fn control_value(&self) -> &super::commonmodule::ControlValue {
        self._switch_control().control_value.as_ref().unwrap_or(&switch_control::CONTROL_VALUE)
    }
    fn check(&self) -> &super::commonmodule::CheckConditions {
        self._switch_control().check.as_ref().unwrap_or(&switch_control::CHECK)
    }
    fn switch_control_fscc(&self) -> &SwitchControlFscc {
        self._switch_control().switch_control_fscc.as_ref().unwrap_or(&switch_control::SWITCH_CONTROL_FSCC)
    }
}
impl IsSwitchControl for SwitchControl {
    fn _switch_control(&self) -> &SwitchControl {
        self
    }
}
/// Switch control profile
/// OpenFMB Profile Message: true
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SwitchControlProfile {
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
    pub ied: ::std::option::Option<super::commonmodule::Ied>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="3")]
    pub protected_switch: ::std::option::Option<ProtectedSwitch>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="4")]
    pub switch_control: ::std::option::Option<SwitchControl>,
}
mod switch_control_profile {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref CONTROL_MESSAGE_INFO: crate::commonmodule::ControlMessageInfo = Default::default();
        pub(super) static ref IED: crate::commonmodule::Ied = Default::default();
        pub(super) static ref PROTECTED_SWITCH: crate::switchmodule::ProtectedSwitch = Default::default();
        pub(super) static ref SWITCH_CONTROL: crate::switchmodule::SwitchControl = Default::default();
    }
}
trait IsSwitchControlProfile {
    fn _switch_control_profile(&self) -> &SwitchControlProfile;
    fn control_message_info(&self) -> &super::commonmodule::ControlMessageInfo {
        self._switch_control_profile().control_message_info.as_ref().unwrap_or(&switch_control_profile::CONTROL_MESSAGE_INFO)
    }
    fn ied(&self) -> &super::commonmodule::Ied {
        self._switch_control_profile().ied.as_ref().unwrap_or(&switch_control_profile::IED)
    }
    fn protected_switch(&self) -> &ProtectedSwitch {
        self._switch_control_profile().protected_switch.as_ref().unwrap_or(&switch_control_profile::PROTECTED_SWITCH)
    }
    fn switch_control(&self) -> &SwitchControl {
        self._switch_control_profile().switch_control.as_ref().unwrap_or(&switch_control_profile::SWITCH_CONTROL)
    }
}
impl IsSwitchControlProfile for SwitchControlProfile {
    fn _switch_control_profile(&self) -> &SwitchControlProfile {
        self
    }
}
