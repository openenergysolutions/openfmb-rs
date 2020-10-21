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
pub trait IsSwitchDiscreteControlXswi {
    fn _switch_discrete_control_xswi(&self) -> &SwitchDiscreteControlXswi;
    fn _mut_switch_discrete_control_xswi(&mut self) -> &mut SwitchDiscreteControlXswi;
    fn logical_node_for_control(&self) -> &super::commonmodule::LogicalNodeForControl {
        self._switch_discrete_control_xswi().logical_node_for_control.as_ref().unwrap_or(&switch_discrete_control_xswi::LOGICAL_NODE_FOR_CONTROL)
    }
    fn mut_logical_node_for_control(&mut self) -> &mut super::commonmodule::LogicalNodeForControl {
        self._mut_switch_discrete_control_xswi().logical_node_for_control.get_or_insert(switch_discrete_control_xswi::LOGICAL_NODE_FOR_CONTROL.clone())
    }
    fn pos(&self) -> &super::commonmodule::ControlDpc {
        self._switch_discrete_control_xswi().pos.as_ref().unwrap_or(&switch_discrete_control_xswi::POS)
    }
    fn mut_pos(&mut self) -> &mut super::commonmodule::ControlDpc {
        self._mut_switch_discrete_control_xswi().pos.get_or_insert(switch_discrete_control_xswi::POS.clone())
    }
}
impl IsSwitchDiscreteControlXswi for SwitchDiscreteControlXswi {
    fn _switch_discrete_control_xswi(&self) -> &SwitchDiscreteControlXswi {
        self
    }
    fn _mut_switch_discrete_control_xswi(&mut self) -> &mut SwitchDiscreteControlXswi {
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
pub trait IsSwitchDiscreteControl {
    fn _switch_discrete_control(&self) -> &SwitchDiscreteControl;
    fn _mut_switch_discrete_control(&mut self) -> &mut SwitchDiscreteControl;
    fn control_value(&self) -> &super::commonmodule::ControlValue {
        self._switch_discrete_control().control_value.as_ref().unwrap_or(&switch_discrete_control::CONTROL_VALUE)
    }
    fn mut_control_value(&mut self) -> &mut super::commonmodule::ControlValue {
        self._mut_switch_discrete_control().control_value.get_or_insert(switch_discrete_control::CONTROL_VALUE.clone())
    }
    fn check(&self) -> &super::commonmodule::CheckConditions {
        self._switch_discrete_control().check.as_ref().unwrap_or(&switch_discrete_control::CHECK)
    }
    fn mut_check(&mut self) -> &mut super::commonmodule::CheckConditions {
        self._mut_switch_discrete_control().check.get_or_insert(switch_discrete_control::CHECK.clone())
    }
    fn switch_discrete_control_xswi(&self) -> &SwitchDiscreteControlXswi {
        self._switch_discrete_control().switch_discrete_control_xswi.as_ref().unwrap_or(&switch_discrete_control::SWITCH_DISCRETE_CONTROL_XSWI)
    }
    fn mut_switch_discrete_control_xswi(&mut self) -> &mut SwitchDiscreteControlXswi {
        self._mut_switch_discrete_control().switch_discrete_control_xswi.get_or_insert(switch_discrete_control::SWITCH_DISCRETE_CONTROL_XSWI.clone())
    }
}
impl IsSwitchDiscreteControl for SwitchDiscreteControl {
    fn _switch_discrete_control(&self) -> &SwitchDiscreteControl {
        self
    }
    fn _mut_switch_discrete_control(&mut self) -> &mut SwitchDiscreteControl {
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
pub trait IsProtectedSwitch {
    fn _protected_switch(&self) -> &ProtectedSwitch;
    fn _mut_protected_switch(&mut self) -> &mut ProtectedSwitch;
    fn conducting_equipment(&self) -> &super::commonmodule::ConductingEquipment {
        self._protected_switch().conducting_equipment.as_ref().unwrap_or(&protected_switch::CONDUCTING_EQUIPMENT)
    }
    fn mut_conducting_equipment(&mut self) -> &mut super::commonmodule::ConductingEquipment {
        self._mut_protected_switch().conducting_equipment.get_or_insert(protected_switch::CONDUCTING_EQUIPMENT.clone())
    }
}
impl IsProtectedSwitch for ProtectedSwitch {
    fn _protected_switch(&self) -> &ProtectedSwitch {
        self
    }
    fn _mut_protected_switch(&mut self) -> &mut ProtectedSwitch {
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
pub trait IsSwitchDiscreteControlProfile {
    fn _switch_discrete_control_profile(&self) -> &SwitchDiscreteControlProfile;
    fn _mut_switch_discrete_control_profile(&mut self) -> &mut SwitchDiscreteControlProfile;
    fn control_message_info(&self) -> &super::commonmodule::ControlMessageInfo {
        self._switch_discrete_control_profile().control_message_info.as_ref().unwrap_or(&switch_discrete_control_profile::CONTROL_MESSAGE_INFO)
    }
    fn mut_control_message_info(&mut self) -> &mut super::commonmodule::ControlMessageInfo {
        self._mut_switch_discrete_control_profile().control_message_info.get_or_insert(switch_discrete_control_profile::CONTROL_MESSAGE_INFO.clone())
    }
    fn ied(&self) -> &super::commonmodule::Ied {
        self._switch_discrete_control_profile().ied.as_ref().unwrap_or(&switch_discrete_control_profile::IED)
    }
    fn mut_ied(&mut self) -> &mut super::commonmodule::Ied {
        self._mut_switch_discrete_control_profile().ied.get_or_insert(switch_discrete_control_profile::IED.clone())
    }
    fn protected_switch(&self) -> &ProtectedSwitch {
        self._switch_discrete_control_profile().protected_switch.as_ref().unwrap_or(&switch_discrete_control_profile::PROTECTED_SWITCH)
    }
    fn mut_protected_switch(&mut self) -> &mut ProtectedSwitch {
        self._mut_switch_discrete_control_profile().protected_switch.get_or_insert(switch_discrete_control_profile::PROTECTED_SWITCH.clone())
    }
    fn switch_discrete_control(&self) -> &SwitchDiscreteControl {
        self._switch_discrete_control_profile().switch_discrete_control.as_ref().unwrap_or(&switch_discrete_control_profile::SWITCH_DISCRETE_CONTROL)
    }
    fn mut_switch_discrete_control(&mut self) -> &mut SwitchDiscreteControl {
        self._mut_switch_discrete_control_profile().switch_discrete_control.get_or_insert(switch_discrete_control_profile::SWITCH_DISCRETE_CONTROL.clone())
    }
}
impl IsSwitchDiscreteControlProfile for SwitchDiscreteControlProfile {
    fn _switch_discrete_control_profile(&self) -> &SwitchDiscreteControlProfile {
        self
    }
    fn _mut_switch_discrete_control_profile(&mut self) -> &mut SwitchDiscreteControlProfile {
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
pub trait IsSwitchEventXswi {
    fn _switch_event_xswi(&self) -> &SwitchEventXswi;
    fn _mut_switch_event_xswi(&mut self) -> &mut SwitchEventXswi;
    fn logical_node_for_event_and_status(&self) -> &super::commonmodule::LogicalNodeForEventAndStatus {
        self._switch_event_xswi().logical_node_for_event_and_status.as_ref().unwrap_or(&switch_event_xswi::LOGICAL_NODE_FOR_EVENT_AND_STATUS)
    }
    fn mut_logical_node_for_event_and_status(&mut self) -> &mut super::commonmodule::LogicalNodeForEventAndStatus {
        self._mut_switch_event_xswi().logical_node_for_event_and_status.get_or_insert(switch_event_xswi::LOGICAL_NODE_FOR_EVENT_AND_STATUS.clone())
    }
    fn dynamic_test(&self) -> &super::commonmodule::EnsDynamicTestKind {
        self._switch_event_xswi().dynamic_test.as_ref().unwrap_or(&switch_event_xswi::DYNAMIC_TEST)
    }
    fn mut_dynamic_test(&mut self) -> &mut super::commonmodule::EnsDynamicTestKind {
        self._mut_switch_event_xswi().dynamic_test.get_or_insert(switch_event_xswi::DYNAMIC_TEST.clone())
    }
    fn pos(&self) -> &super::commonmodule::StatusDps {
        self._switch_event_xswi().pos.as_ref().unwrap_or(&switch_event_xswi::POS)
    }
    fn mut_pos(&mut self) -> &mut super::commonmodule::StatusDps {
        self._mut_switch_event_xswi().pos.get_or_insert(switch_event_xswi::POS.clone())
    }
}
impl IsSwitchEventXswi for SwitchEventXswi {
    fn _switch_event_xswi(&self) -> &SwitchEventXswi {
        self
    }
    fn _mut_switch_event_xswi(&mut self) -> &mut SwitchEventXswi {
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
pub trait IsSwitchEvent {
    fn _switch_event(&self) -> &SwitchEvent;
    fn _mut_switch_event(&mut self) -> &mut SwitchEvent;
    fn event_value(&self) -> &super::commonmodule::EventValue {
        self._switch_event().event_value.as_ref().unwrap_or(&switch_event::EVENT_VALUE)
    }
    fn mut_event_value(&mut self) -> &mut super::commonmodule::EventValue {
        self._mut_switch_event().event_value.get_or_insert(switch_event::EVENT_VALUE.clone())
    }
    fn switch_event_xswi(&self) -> &SwitchEventXswi {
        self._switch_event().switch_event_xswi.as_ref().unwrap_or(&switch_event::SWITCH_EVENT_XSWI)
    }
    fn mut_switch_event_xswi(&mut self) -> &mut SwitchEventXswi {
        self._mut_switch_event().switch_event_xswi.get_or_insert(switch_event::SWITCH_EVENT_XSWI.clone())
    }
}
impl IsSwitchEvent for SwitchEvent {
    fn _switch_event(&self) -> &SwitchEvent {
        self
    }
    fn _mut_switch_event(&mut self) -> &mut SwitchEvent {
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
pub trait IsSwitchEventProfile {
    fn _switch_event_profile(&self) -> &SwitchEventProfile;
    fn _mut_switch_event_profile(&mut self) -> &mut SwitchEventProfile;
    fn event_message_info(&self) -> &super::commonmodule::EventMessageInfo {
        self._switch_event_profile().event_message_info.as_ref().unwrap_or(&switch_event_profile::EVENT_MESSAGE_INFO)
    }
    fn mut_event_message_info(&mut self) -> &mut super::commonmodule::EventMessageInfo {
        self._mut_switch_event_profile().event_message_info.get_or_insert(switch_event_profile::EVENT_MESSAGE_INFO.clone())
    }
    fn ied(&self) -> &super::commonmodule::Ied {
        self._switch_event_profile().ied.as_ref().unwrap_or(&switch_event_profile::IED)
    }
    fn mut_ied(&mut self) -> &mut super::commonmodule::Ied {
        self._mut_switch_event_profile().ied.get_or_insert(switch_event_profile::IED.clone())
    }
    fn protected_switch(&self) -> &ProtectedSwitch {
        self._switch_event_profile().protected_switch.as_ref().unwrap_or(&switch_event_profile::PROTECTED_SWITCH)
    }
    fn mut_protected_switch(&mut self) -> &mut ProtectedSwitch {
        self._mut_switch_event_profile().protected_switch.get_or_insert(switch_event_profile::PROTECTED_SWITCH.clone())
    }
    fn switch_event(&self) -> &SwitchEvent {
        self._switch_event_profile().switch_event.as_ref().unwrap_or(&switch_event_profile::SWITCH_EVENT)
    }
    fn mut_switch_event(&mut self) -> &mut SwitchEvent {
        self._mut_switch_event_profile().switch_event.get_or_insert(switch_event_profile::SWITCH_EVENT.clone())
    }
}
impl IsSwitchEventProfile for SwitchEventProfile {
    fn _switch_event_profile(&self) -> &SwitchEventProfile {
        self
    }
    fn _mut_switch_event_profile(&mut self) -> &mut SwitchEventProfile {
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
pub trait IsSwitchReading {
    fn _switch_reading(&self) -> &SwitchReading;
    fn _mut_switch_reading(&mut self) -> &mut SwitchReading;
    fn conducting_equipment_terminal_reading(&self) -> &super::commonmodule::ConductingEquipmentTerminalReading {
        self._switch_reading().conducting_equipment_terminal_reading.as_ref().unwrap_or(&switch_reading::CONDUCTING_EQUIPMENT_TERMINAL_READING)
    }
    fn mut_conducting_equipment_terminal_reading(&mut self) -> &mut super::commonmodule::ConductingEquipmentTerminalReading {
        self._mut_switch_reading().conducting_equipment_terminal_reading.get_or_insert(switch_reading::CONDUCTING_EQUIPMENT_TERMINAL_READING.clone())
    }
    fn diff_reading_mmxu(&self) -> &super::commonmodule::ReadingMmxu {
        self._switch_reading().diff_reading_mmxu.as_ref().unwrap_or(&switch_reading::DIFF_READING_MMXU)
    }
    fn mut_diff_reading_mmxu(&mut self) -> &mut super::commonmodule::ReadingMmxu {
        self._mut_switch_reading().diff_reading_mmxu.get_or_insert(switch_reading::DIFF_READING_MMXU.clone())
    }
    fn phase_mmtn(&self) -> &super::commonmodule::PhaseMmtn {
        self._switch_reading().phase_mmtn.as_ref().unwrap_or(&switch_reading::PHASE_MMTN)
    }
    fn mut_phase_mmtn(&mut self) -> &mut super::commonmodule::PhaseMmtn {
        self._mut_switch_reading().phase_mmtn.get_or_insert(switch_reading::PHASE_MMTN.clone())
    }
    fn reading_mmtr(&self) -> &super::commonmodule::ReadingMmtr {
        self._switch_reading().reading_mmtr.as_ref().unwrap_or(&switch_reading::READING_MMTR)
    }
    fn mut_reading_mmtr(&mut self) -> &mut super::commonmodule::ReadingMmtr {
        self._mut_switch_reading().reading_mmtr.get_or_insert(switch_reading::READING_MMTR.clone())
    }
    fn reading_mmxu(&self) -> &super::commonmodule::ReadingMmxu {
        self._switch_reading().reading_mmxu.as_ref().unwrap_or(&switch_reading::READING_MMXU)
    }
    fn mut_reading_mmxu(&mut self) -> &mut super::commonmodule::ReadingMmxu {
        self._mut_switch_reading().reading_mmxu.get_or_insert(switch_reading::READING_MMXU.clone())
    }
}
impl IsSwitchReading for SwitchReading {
    fn _switch_reading(&self) -> &SwitchReading {
        self
    }
    fn _mut_switch_reading(&mut self) -> &mut SwitchReading {
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
pub trait IsSwitchReadingProfile {
    fn _switch_reading_profile(&self) -> &SwitchReadingProfile;
    fn _mut_switch_reading_profile(&mut self) -> &mut SwitchReadingProfile;
    fn reading_message_info(&self) -> &super::commonmodule::ReadingMessageInfo {
        self._switch_reading_profile().reading_message_info.as_ref().unwrap_or(&switch_reading_profile::READING_MESSAGE_INFO)
    }
    fn mut_reading_message_info(&mut self) -> &mut super::commonmodule::ReadingMessageInfo {
        self._mut_switch_reading_profile().reading_message_info.get_or_insert(switch_reading_profile::READING_MESSAGE_INFO.clone())
    }
    fn ied(&self) -> &super::commonmodule::Ied {
        self._switch_reading_profile().ied.as_ref().unwrap_or(&switch_reading_profile::IED)
    }
    fn mut_ied(&mut self) -> &mut super::commonmodule::Ied {
        self._mut_switch_reading_profile().ied.get_or_insert(switch_reading_profile::IED.clone())
    }
    fn protected_switch(&self) -> &ProtectedSwitch {
        self._switch_reading_profile().protected_switch.as_ref().unwrap_or(&switch_reading_profile::PROTECTED_SWITCH)
    }
    fn mut_protected_switch(&mut self) -> &mut ProtectedSwitch {
        self._mut_switch_reading_profile().protected_switch.get_or_insert(switch_reading_profile::PROTECTED_SWITCH.clone())
    }
    fn switch_reading(&self) -> &::std::vec::Vec<SwitchReading> {
        &self._switch_reading_profile().switch_reading    }
    fn mut_switch_reading(&mut self) -> &mut ::std::vec::Vec<SwitchReading> {
        &mut self._mut_switch_reading_profile().switch_reading    }
}
impl IsSwitchReadingProfile for SwitchReadingProfile {
    fn _switch_reading_profile(&self) -> &SwitchReadingProfile {
        self
    }
    fn _mut_switch_reading_profile(&mut self) -> &mut SwitchReadingProfile {
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
pub trait IsSwitchStatusXswi {
    fn _switch_status_xswi(&self) -> &SwitchStatusXswi;
    fn _mut_switch_status_xswi(&mut self) -> &mut SwitchStatusXswi;
    fn logical_node_for_event_and_status(&self) -> &super::commonmodule::LogicalNodeForEventAndStatus {
        self._switch_status_xswi().logical_node_for_event_and_status.as_ref().unwrap_or(&switch_status_xswi::LOGICAL_NODE_FOR_EVENT_AND_STATUS)
    }
    fn mut_logical_node_for_event_and_status(&mut self) -> &mut super::commonmodule::LogicalNodeForEventAndStatus {
        self._mut_switch_status_xswi().logical_node_for_event_and_status.get_or_insert(switch_status_xswi::LOGICAL_NODE_FOR_EVENT_AND_STATUS.clone())
    }
    fn dynamic_test(&self) -> &super::commonmodule::EnsDynamicTestKind {
        self._switch_status_xswi().dynamic_test.as_ref().unwrap_or(&switch_status_xswi::DYNAMIC_TEST)
    }
    fn mut_dynamic_test(&mut self) -> &mut super::commonmodule::EnsDynamicTestKind {
        self._mut_switch_status_xswi().dynamic_test.get_or_insert(switch_status_xswi::DYNAMIC_TEST.clone())
    }
    fn pos(&self) -> &super::commonmodule::StatusDps {
        self._switch_status_xswi().pos.as_ref().unwrap_or(&switch_status_xswi::POS)
    }
    fn mut_pos(&mut self) -> &mut super::commonmodule::StatusDps {
        self._mut_switch_status_xswi().pos.get_or_insert(switch_status_xswi::POS.clone())
    }
}
impl IsSwitchStatusXswi for SwitchStatusXswi {
    fn _switch_status_xswi(&self) -> &SwitchStatusXswi {
        self
    }
    fn _mut_switch_status_xswi(&mut self) -> &mut SwitchStatusXswi {
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
pub trait IsSwitchStatus {
    fn _switch_status(&self) -> &SwitchStatus;
    fn _mut_switch_status(&mut self) -> &mut SwitchStatus;
    fn status_value(&self) -> &super::commonmodule::StatusValue {
        self._switch_status().status_value.as_ref().unwrap_or(&switch_status::STATUS_VALUE)
    }
    fn mut_status_value(&mut self) -> &mut super::commonmodule::StatusValue {
        self._mut_switch_status().status_value.get_or_insert(switch_status::STATUS_VALUE.clone())
    }
    fn switch_status_xswi(&self) -> &SwitchStatusXswi {
        self._switch_status().switch_status_xswi.as_ref().unwrap_or(&switch_status::SWITCH_STATUS_XSWI)
    }
    fn mut_switch_status_xswi(&mut self) -> &mut SwitchStatusXswi {
        self._mut_switch_status().switch_status_xswi.get_or_insert(switch_status::SWITCH_STATUS_XSWI.clone())
    }
}
impl IsSwitchStatus for SwitchStatus {
    fn _switch_status(&self) -> &SwitchStatus {
        self
    }
    fn _mut_switch_status(&mut self) -> &mut SwitchStatus {
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
pub trait IsSwitchStatusProfile {
    fn _switch_status_profile(&self) -> &SwitchStatusProfile;
    fn _mut_switch_status_profile(&mut self) -> &mut SwitchStatusProfile;
    fn status_message_info(&self) -> &super::commonmodule::StatusMessageInfo {
        self._switch_status_profile().status_message_info.as_ref().unwrap_or(&switch_status_profile::STATUS_MESSAGE_INFO)
    }
    fn mut_status_message_info(&mut self) -> &mut super::commonmodule::StatusMessageInfo {
        self._mut_switch_status_profile().status_message_info.get_or_insert(switch_status_profile::STATUS_MESSAGE_INFO.clone())
    }
    fn ied(&self) -> &super::commonmodule::Ied {
        self._switch_status_profile().ied.as_ref().unwrap_or(&switch_status_profile::IED)
    }
    fn mut_ied(&mut self) -> &mut super::commonmodule::Ied {
        self._mut_switch_status_profile().ied.get_or_insert(switch_status_profile::IED.clone())
    }
    fn protected_switch(&self) -> &ProtectedSwitch {
        self._switch_status_profile().protected_switch.as_ref().unwrap_or(&switch_status_profile::PROTECTED_SWITCH)
    }
    fn mut_protected_switch(&mut self) -> &mut ProtectedSwitch {
        self._mut_switch_status_profile().protected_switch.get_or_insert(switch_status_profile::PROTECTED_SWITCH.clone())
    }
    fn switch_status(&self) -> &SwitchStatus {
        self._switch_status_profile().switch_status.as_ref().unwrap_or(&switch_status_profile::SWITCH_STATUS)
    }
    fn mut_switch_status(&mut self) -> &mut SwitchStatus {
        self._mut_switch_status_profile().switch_status.get_or_insert(switch_status_profile::SWITCH_STATUS.clone())
    }
}
impl IsSwitchStatusProfile for SwitchStatusProfile {
    fn _switch_status_profile(&self) -> &SwitchStatusProfile {
        self
    }
    fn _mut_switch_status_profile(&mut self) -> &mut SwitchStatusProfile {
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
pub trait IsSwitchControlFscc {
    fn _switch_control_fscc(&self) -> &SwitchControlFscc;
    fn _mut_switch_control_fscc(&mut self) -> &mut SwitchControlFscc;
    fn logical_node_for_control(&self) -> &super::commonmodule::LogicalNodeForControl {
        self._switch_control_fscc().logical_node_for_control.as_ref().unwrap_or(&switch_control_fscc::LOGICAL_NODE_FOR_CONTROL)
    }
    fn mut_logical_node_for_control(&mut self) -> &mut super::commonmodule::LogicalNodeForControl {
        self._mut_switch_control_fscc().logical_node_for_control.get_or_insert(switch_control_fscc::LOGICAL_NODE_FOR_CONTROL.clone())
    }
    fn switch_control_schedule_fsch(&self) -> &super::commonmodule::SwitchControlScheduleFsch {
        self._switch_control_fscc().switch_control_schedule_fsch.as_ref().unwrap_or(&switch_control_fscc::SWITCH_CONTROL_SCHEDULE_FSCH)
    }
    fn mut_switch_control_schedule_fsch(&mut self) -> &mut super::commonmodule::SwitchControlScheduleFsch {
        self._mut_switch_control_fscc().switch_control_schedule_fsch.get_or_insert(switch_control_fscc::SWITCH_CONTROL_SCHEDULE_FSCH.clone())
    }
}
impl IsSwitchControlFscc for SwitchControlFscc {
    fn _switch_control_fscc(&self) -> &SwitchControlFscc {
        self
    }
    fn _mut_switch_control_fscc(&mut self) -> &mut SwitchControlFscc {
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
pub trait IsSwitchControl {
    fn _switch_control(&self) -> &SwitchControl;
    fn _mut_switch_control(&mut self) -> &mut SwitchControl;
    fn control_value(&self) -> &super::commonmodule::ControlValue {
        self._switch_control().control_value.as_ref().unwrap_or(&switch_control::CONTROL_VALUE)
    }
    fn mut_control_value(&mut self) -> &mut super::commonmodule::ControlValue {
        self._mut_switch_control().control_value.get_or_insert(switch_control::CONTROL_VALUE.clone())
    }
    fn check(&self) -> &super::commonmodule::CheckConditions {
        self._switch_control().check.as_ref().unwrap_or(&switch_control::CHECK)
    }
    fn mut_check(&mut self) -> &mut super::commonmodule::CheckConditions {
        self._mut_switch_control().check.get_or_insert(switch_control::CHECK.clone())
    }
    fn switch_control_fscc(&self) -> &SwitchControlFscc {
        self._switch_control().switch_control_fscc.as_ref().unwrap_or(&switch_control::SWITCH_CONTROL_FSCC)
    }
    fn mut_switch_control_fscc(&mut self) -> &mut SwitchControlFscc {
        self._mut_switch_control().switch_control_fscc.get_or_insert(switch_control::SWITCH_CONTROL_FSCC.clone())
    }
}
impl IsSwitchControl for SwitchControl {
    fn _switch_control(&self) -> &SwitchControl {
        self
    }
    fn _mut_switch_control(&mut self) -> &mut SwitchControl {
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
pub trait IsSwitchControlProfile {
    fn _switch_control_profile(&self) -> &SwitchControlProfile;
    fn _mut_switch_control_profile(&mut self) -> &mut SwitchControlProfile;
    fn control_message_info(&self) -> &super::commonmodule::ControlMessageInfo {
        self._switch_control_profile().control_message_info.as_ref().unwrap_or(&switch_control_profile::CONTROL_MESSAGE_INFO)
    }
    fn mut_control_message_info(&mut self) -> &mut super::commonmodule::ControlMessageInfo {
        self._mut_switch_control_profile().control_message_info.get_or_insert(switch_control_profile::CONTROL_MESSAGE_INFO.clone())
    }
    fn ied(&self) -> &super::commonmodule::Ied {
        self._switch_control_profile().ied.as_ref().unwrap_or(&switch_control_profile::IED)
    }
    fn mut_ied(&mut self) -> &mut super::commonmodule::Ied {
        self._mut_switch_control_profile().ied.get_or_insert(switch_control_profile::IED.clone())
    }
    fn protected_switch(&self) -> &ProtectedSwitch {
        self._switch_control_profile().protected_switch.as_ref().unwrap_or(&switch_control_profile::PROTECTED_SWITCH)
    }
    fn mut_protected_switch(&mut self) -> &mut ProtectedSwitch {
        self._mut_switch_control_profile().protected_switch.get_or_insert(switch_control_profile::PROTECTED_SWITCH.clone())
    }
    fn switch_control(&self) -> &SwitchControl {
        self._switch_control_profile().switch_control.as_ref().unwrap_or(&switch_control_profile::SWITCH_CONTROL)
    }
    fn mut_switch_control(&mut self) -> &mut SwitchControl {
        self._mut_switch_control_profile().switch_control.get_or_insert(switch_control_profile::SWITCH_CONTROL.clone())
    }
}
impl IsSwitchControlProfile for SwitchControlProfile {
    fn _switch_control_profile(&self) -> &SwitchControlProfile {
        self
    }
    fn _mut_switch_control_profile(&mut self) -> &mut SwitchControlProfile {
        self
    }
}
