/// OpenFMB specialization for switch control:  LN: Circuit switch   Name: XSWI
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SwitchDiscreteControlXswi {
    /// UML inherited base object
    // parent_message: true
    // required_field: false
    // multiplicity_min: None
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, optional, tag="1")]
    pub logical_node_for_control: ::std::option::Option<super::commonmodule::LogicalNodeForControl>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub pos: ::std::option::Option<super::commonmodule::PhaseDpc>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub reset_protection_pickup: ::std::option::Option<super::commonmodule::ControlSpc>,
}
mod switch_discrete_control_xswi {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref LOGICAL_NODE_FOR_CONTROL: crate::commonmodule::LogicalNodeForControl = Default::default();
        pub(super) static ref POS: crate::commonmodule::PhaseDpc = Default::default();
        pub(super) static ref RESET_PROTECTION_PICKUP: crate::commonmodule::ControlSpc = Default::default();
    }
}
pub trait IsSwitchDiscreteControlXswi {
    fn _switch_discrete_control_xswi(&self) -> &SwitchDiscreteControlXswi;
    fn _switch_discrete_control_xswi_mut(&mut self) -> &mut SwitchDiscreteControlXswi;
    fn logical_node_for_control(&self) -> &super::commonmodule::LogicalNodeForControl {
        self._switch_discrete_control_xswi().logical_node_for_control.as_ref().unwrap_or(&switch_discrete_control_xswi::LOGICAL_NODE_FOR_CONTROL)
    }
    fn logical_node_for_control_mut(&mut self) -> &mut super::commonmodule::LogicalNodeForControl {
        self._switch_discrete_control_xswi_mut().logical_node_for_control.get_or_insert(Default::default())
    }
    fn pos(&self) -> &super::commonmodule::PhaseDpc {
        self._switch_discrete_control_xswi().pos.as_ref().unwrap_or(&switch_discrete_control_xswi::POS)
    }
    fn pos_mut(&mut self) -> &mut super::commonmodule::PhaseDpc {
        self._switch_discrete_control_xswi_mut().pos.get_or_insert(Default::default())
    }
    fn reset_protection_pickup(&self) -> &super::commonmodule::ControlSpc {
        self._switch_discrete_control_xswi().reset_protection_pickup.as_ref().unwrap_or(&switch_discrete_control_xswi::RESET_PROTECTION_PICKUP)
    }
    fn reset_protection_pickup_mut(&mut self) -> &mut super::commonmodule::ControlSpc {
        self._switch_discrete_control_xswi_mut().reset_protection_pickup.get_or_insert(Default::default())
    }
}
impl IsSwitchDiscreteControlXswi for SwitchDiscreteControlXswi {
    fn _switch_discrete_control_xswi(&self) -> &SwitchDiscreteControlXswi {
        self
    }
    fn _switch_discrete_control_xswi_mut(&mut self) -> &mut SwitchDiscreteControlXswi {
        self
    }
}
//impl IsLogicalNodeForControl for SwitchDiscreteControlXswi {
    //fn _logical_node_for_control(&self) -> &LogicalNodeForControl {
        //
    //}
//fn _mut_logical_node_for_control(&mut self) -> &mut LogicalNodeForControl {
        //
    //}
//}
//impl IsLogicalNode for SwitchDiscreteControlXswi {
    //fn _logical_node(&self) -> &LogicalNode {
        //
    //}
//fn _mut_logical_node(&mut self) -> &mut LogicalNode {
        //
    //}
//}
//impl IsIdentifiedObject for SwitchDiscreteControlXswi {
    //fn _identified_object(&self) -> &IdentifiedObject {
        //
    //}
//fn _mut_identified_object(&mut self) -> &mut IdentifiedObject {
        //
    //}
//}
/// Switch discrete control
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SwitchDiscreteControl {
    /// UML inherited base object
    // parent_message: true
    // required_field: false
    // multiplicity_min: None
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, optional, tag="1")]
    pub control_value: ::std::option::Option<super::commonmodule::ControlValue>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub check: ::std::option::Option<super::commonmodule::CheckConditions>,
    /// MISSING DOCUMENTATION!!!
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
    fn _switch_discrete_control_mut(&mut self) -> &mut SwitchDiscreteControl;
    fn control_value(&self) -> &super::commonmodule::ControlValue {
        self._switch_discrete_control().control_value.as_ref().unwrap_or(&switch_discrete_control::CONTROL_VALUE)
    }
    fn control_value_mut(&mut self) -> &mut super::commonmodule::ControlValue {
        self._switch_discrete_control_mut().control_value.get_or_insert(Default::default())
    }
    fn check(&self) -> &super::commonmodule::CheckConditions {
        self._switch_discrete_control().check.as_ref().unwrap_or(&switch_discrete_control::CHECK)
    }
    fn check_mut(&mut self) -> &mut super::commonmodule::CheckConditions {
        self._switch_discrete_control_mut().check.get_or_insert(Default::default())
    }
    fn switch_discrete_control_xswi(&self) -> &SwitchDiscreteControlXswi {
        self._switch_discrete_control().switch_discrete_control_xswi.as_ref().unwrap_or(&switch_discrete_control::SWITCH_DISCRETE_CONTROL_XSWI)
    }
    fn switch_discrete_control_xswi_mut(&mut self) -> &mut SwitchDiscreteControlXswi {
        self._switch_discrete_control_mut().switch_discrete_control_xswi.get_or_insert(Default::default())
    }
}
impl IsSwitchDiscreteControl for SwitchDiscreteControl {
    fn _switch_discrete_control(&self) -> &SwitchDiscreteControl {
        self
    }
    fn _switch_discrete_control_mut(&mut self) -> &mut SwitchDiscreteControl {
        self
    }
}
//impl IsControlValue for SwitchDiscreteControl {
    //fn _control_value(&self) -> &ControlValue {
        //
    //}
//fn _mut_control_value(&mut self) -> &mut ControlValue {
        //
    //}
//}
//impl IsIdentifiedObject for SwitchDiscreteControl {
    //fn _identified_object(&self) -> &IdentifiedObject {
        //
    //}
//fn _mut_identified_object(&mut self) -> &mut IdentifiedObject {
        //
    //}
//}
/// A ProtectedSwitch is a switching device that can be operated by ProtectionEquipment.
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ProtectedSwitch {
    /// UML inherited base object
    // parent_message: true
    // required_field: false
    // multiplicity_min: None
    // multiplicity_max: None
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
    fn _protected_switch_mut(&mut self) -> &mut ProtectedSwitch;
    fn conducting_equipment(&self) -> &super::commonmodule::ConductingEquipment {
        self._protected_switch().conducting_equipment.as_ref().unwrap_or(&protected_switch::CONDUCTING_EQUIPMENT)
    }
    fn conducting_equipment_mut(&mut self) -> &mut super::commonmodule::ConductingEquipment {
        self._protected_switch_mut().conducting_equipment.get_or_insert(Default::default())
    }
}
impl IsProtectedSwitch for ProtectedSwitch {
    fn _protected_switch(&self) -> &ProtectedSwitch {
        self
    }
    fn _protected_switch_mut(&mut self) -> &mut ProtectedSwitch {
        self
    }
}
//impl IsConductingEquipment for ProtectedSwitch {
    //fn _conducting_equipment(&self) -> &ConductingEquipment {
        //
    //}
//fn _mut_conducting_equipment(&mut self) -> &mut ConductingEquipment {
        //
    //}
//}
//impl IsNamedObject for ProtectedSwitch {
    //fn _named_object(&self) -> &NamedObject {
        //
    //}
//fn _mut_named_object(&mut self) -> &mut NamedObject {
        //
    //}
//}
/// Switch control profile
/// OpenFMB Profile Message: true
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SwitchDiscreteControlProfile {
    /// UML inherited base object
    // parent_message: true
    // required_field: false
    // multiplicity_min: None
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, optional, tag="1")]
    pub control_message_info: ::std::option::Option<super::commonmodule::ControlMessageInfo>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: Some(1)
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, optional, tag="2")]
    pub protected_switch: ::std::option::Option<ProtectedSwitch>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: Some(1)
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, optional, tag="3")]
    pub switch_discrete_control: ::std::option::Option<SwitchDiscreteControl>,
}
mod switch_discrete_control_profile {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref CONTROL_MESSAGE_INFO: crate::commonmodule::ControlMessageInfo = Default::default();
        pub(super) static ref PROTECTED_SWITCH: crate::switchmodule::ProtectedSwitch = Default::default();
        pub(super) static ref SWITCH_DISCRETE_CONTROL: crate::switchmodule::SwitchDiscreteControl = Default::default();
    }
}
pub trait IsSwitchDiscreteControlProfile {
    fn _switch_discrete_control_profile(&self) -> &SwitchDiscreteControlProfile;
    fn _switch_discrete_control_profile_mut(&mut self) -> &mut SwitchDiscreteControlProfile;
    fn control_message_info(&self) -> &super::commonmodule::ControlMessageInfo {
        self._switch_discrete_control_profile().control_message_info.as_ref().unwrap_or(&switch_discrete_control_profile::CONTROL_MESSAGE_INFO)
    }
    fn control_message_info_mut(&mut self) -> &mut super::commonmodule::ControlMessageInfo {
        self._switch_discrete_control_profile_mut().control_message_info.get_or_insert(Default::default())
    }
    fn protected_switch(&self) -> &ProtectedSwitch {
        self._switch_discrete_control_profile().protected_switch.as_ref().unwrap_or(&switch_discrete_control_profile::PROTECTED_SWITCH)
    }
    fn protected_switch_mut(&mut self) -> &mut ProtectedSwitch {
        self._switch_discrete_control_profile_mut().protected_switch.get_or_insert(Default::default())
    }
    fn switch_discrete_control(&self) -> &SwitchDiscreteControl {
        self._switch_discrete_control_profile().switch_discrete_control.as_ref().unwrap_or(&switch_discrete_control_profile::SWITCH_DISCRETE_CONTROL)
    }
    fn switch_discrete_control_mut(&mut self) -> &mut SwitchDiscreteControl {
        self._switch_discrete_control_profile_mut().switch_discrete_control.get_or_insert(Default::default())
    }
}
impl IsSwitchDiscreteControlProfile for SwitchDiscreteControlProfile {
    fn _switch_discrete_control_profile(&self) -> &SwitchDiscreteControlProfile {
        self
    }
    fn _switch_discrete_control_profile_mut(&mut self) -> &mut SwitchDiscreteControlProfile {
        self
    }
}
//impl IsControlMessageInfo for SwitchDiscreteControlProfile {
    //fn _control_message_info(&self) -> &ControlMessageInfo {
        //
    //}
//fn _mut_control_message_info(&mut self) -> &mut ControlMessageInfo {
        //
    //}
//}
//impl IsMessageInfo for SwitchDiscreteControlProfile {
    //fn _message_info(&self) -> &MessageInfo {
        //
    //}
//fn _mut_message_info(&mut self) -> &mut MessageInfo {
        //
    //}
//}
//impl IsIdentifiedObject for SwitchDiscreteControlProfile {
    //fn _identified_object(&self) -> &IdentifiedObject {
        //
    //}
//fn _mut_identified_object(&mut self) -> &mut IdentifiedObject {
        //
    //}
//}
/// OpenFMB specialization for SwitchEventProfile
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SwitchEventXswi {
    /// UML inherited base object
    // parent_message: true
    // required_field: false
    // multiplicity_min: None
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, optional, tag="1")]
    pub logical_node_for_event_and_status: ::std::option::Option<super::commonmodule::LogicalNodeForEventAndStatus>,
    /// Dynamic test status
    #[prost(message, optional, tag="2")]
    pub dynamic_test: ::std::option::Option<super::commonmodule::EnsDynamicTestKind>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub pos: ::std::option::Option<super::commonmodule::PhaseDps>,
}
mod switch_event_xswi {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref LOGICAL_NODE_FOR_EVENT_AND_STATUS: crate::commonmodule::LogicalNodeForEventAndStatus = Default::default();
        pub(super) static ref DYNAMIC_TEST: crate::commonmodule::EnsDynamicTestKind = Default::default();
        pub(super) static ref POS: crate::commonmodule::PhaseDps = Default::default();
    }
}
pub trait IsSwitchEventXswi {
    fn _switch_event_xswi(&self) -> &SwitchEventXswi;
    fn _switch_event_xswi_mut(&mut self) -> &mut SwitchEventXswi;
    fn logical_node_for_event_and_status(&self) -> &super::commonmodule::LogicalNodeForEventAndStatus {
        self._switch_event_xswi().logical_node_for_event_and_status.as_ref().unwrap_or(&switch_event_xswi::LOGICAL_NODE_FOR_EVENT_AND_STATUS)
    }
    fn logical_node_for_event_and_status_mut(&mut self) -> &mut super::commonmodule::LogicalNodeForEventAndStatus {
        self._switch_event_xswi_mut().logical_node_for_event_and_status.get_or_insert(Default::default())
    }
    fn dynamic_test(&self) -> &super::commonmodule::EnsDynamicTestKind {
        self._switch_event_xswi().dynamic_test.as_ref().unwrap_or(&switch_event_xswi::DYNAMIC_TEST)
    }
    fn dynamic_test_mut(&mut self) -> &mut super::commonmodule::EnsDynamicTestKind {
        self._switch_event_xswi_mut().dynamic_test.get_or_insert(Default::default())
    }
    fn pos(&self) -> &super::commonmodule::PhaseDps {
        self._switch_event_xswi().pos.as_ref().unwrap_or(&switch_event_xswi::POS)
    }
    fn pos_mut(&mut self) -> &mut super::commonmodule::PhaseDps {
        self._switch_event_xswi_mut().pos.get_or_insert(Default::default())
    }
}
impl IsSwitchEventXswi for SwitchEventXswi {
    fn _switch_event_xswi(&self) -> &SwitchEventXswi {
        self
    }
    fn _switch_event_xswi_mut(&mut self) -> &mut SwitchEventXswi {
        self
    }
}
//impl IsLogicalNodeForEventAndStatus for SwitchEventXswi {
    //fn _logical_node_for_event_and_status(&self) -> &LogicalNodeForEventAndStatus {
        //
    //}
//fn _mut_logical_node_for_event_and_status(&mut self) -> &mut LogicalNodeForEventAndStatus {
        //
    //}
//}
//impl IsLogicalNode for SwitchEventXswi {
    //fn _logical_node(&self) -> &LogicalNode {
        //
    //}
//fn _mut_logical_node(&mut self) -> &mut LogicalNode {
        //
    //}
//}
//impl IsIdentifiedObject for SwitchEventXswi {
    //fn _identified_object(&self) -> &IdentifiedObject {
        //
    //}
//fn _mut_identified_object(&mut self) -> &mut IdentifiedObject {
        //
    //}
//}
/// Switch event
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SwitchEvent {
    /// UML inherited base object
    // parent_message: true
    // required_field: false
    // multiplicity_min: None
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, optional, tag="1")]
    pub event_value: ::std::option::Option<super::commonmodule::EventValue>,
    /// MISSING DOCUMENTATION!!!
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
    fn _switch_event_mut(&mut self) -> &mut SwitchEvent;
    fn event_value(&self) -> &super::commonmodule::EventValue {
        self._switch_event().event_value.as_ref().unwrap_or(&switch_event::EVENT_VALUE)
    }
    fn event_value_mut(&mut self) -> &mut super::commonmodule::EventValue {
        self._switch_event_mut().event_value.get_or_insert(Default::default())
    }
    fn switch_event_xswi(&self) -> &SwitchEventXswi {
        self._switch_event().switch_event_xswi.as_ref().unwrap_or(&switch_event::SWITCH_EVENT_XSWI)
    }
    fn switch_event_xswi_mut(&mut self) -> &mut SwitchEventXswi {
        self._switch_event_mut().switch_event_xswi.get_or_insert(Default::default())
    }
}
impl IsSwitchEvent for SwitchEvent {
    fn _switch_event(&self) -> &SwitchEvent {
        self
    }
    fn _switch_event_mut(&mut self) -> &mut SwitchEvent {
        self
    }
}
//impl IsEventValue for SwitchEvent {
    //fn _event_value(&self) -> &EventValue {
        //
    //}
//fn _mut_event_value(&mut self) -> &mut EventValue {
        //
    //}
//}
//impl IsIdentifiedObject for SwitchEvent {
    //fn _identified_object(&self) -> &IdentifiedObject {
        //
    //}
//fn _mut_identified_object(&mut self) -> &mut IdentifiedObject {
        //
    //}
//}
/// Switch event profile
/// OpenFMB Profile Message: true
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SwitchEventProfile {
    /// UML inherited base object
    // parent_message: true
    // required_field: false
    // multiplicity_min: None
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, optional, tag="1")]
    pub event_message_info: ::std::option::Option<super::commonmodule::EventMessageInfo>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: Some(1)
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, optional, tag="2")]
    pub protected_switch: ::std::option::Option<ProtectedSwitch>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: Some(1)
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, optional, tag="3")]
    pub switch_event: ::std::option::Option<SwitchEvent>,
}
mod switch_event_profile {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref EVENT_MESSAGE_INFO: crate::commonmodule::EventMessageInfo = Default::default();
        pub(super) static ref PROTECTED_SWITCH: crate::switchmodule::ProtectedSwitch = Default::default();
        pub(super) static ref SWITCH_EVENT: crate::switchmodule::SwitchEvent = Default::default();
    }
}
pub trait IsSwitchEventProfile {
    fn _switch_event_profile(&self) -> &SwitchEventProfile;
    fn _switch_event_profile_mut(&mut self) -> &mut SwitchEventProfile;
    fn event_message_info(&self) -> &super::commonmodule::EventMessageInfo {
        self._switch_event_profile().event_message_info.as_ref().unwrap_or(&switch_event_profile::EVENT_MESSAGE_INFO)
    }
    fn event_message_info_mut(&mut self) -> &mut super::commonmodule::EventMessageInfo {
        self._switch_event_profile_mut().event_message_info.get_or_insert(Default::default())
    }
    fn protected_switch(&self) -> &ProtectedSwitch {
        self._switch_event_profile().protected_switch.as_ref().unwrap_or(&switch_event_profile::PROTECTED_SWITCH)
    }
    fn protected_switch_mut(&mut self) -> &mut ProtectedSwitch {
        self._switch_event_profile_mut().protected_switch.get_or_insert(Default::default())
    }
    fn switch_event(&self) -> &SwitchEvent {
        self._switch_event_profile().switch_event.as_ref().unwrap_or(&switch_event_profile::SWITCH_EVENT)
    }
    fn switch_event_mut(&mut self) -> &mut SwitchEvent {
        self._switch_event_profile_mut().switch_event.get_or_insert(Default::default())
    }
}
impl IsSwitchEventProfile for SwitchEventProfile {
    fn _switch_event_profile(&self) -> &SwitchEventProfile {
        self
    }
    fn _switch_event_profile_mut(&mut self) -> &mut SwitchEventProfile {
        self
    }
}
//impl IsEventMessageInfo for SwitchEventProfile {
    //fn _event_message_info(&self) -> &EventMessageInfo {
        //
    //}
//fn _mut_event_message_info(&mut self) -> &mut EventMessageInfo {
        //
    //}
//}
//impl IsMessageInfo for SwitchEventProfile {
    //fn _message_info(&self) -> &MessageInfo {
        //
    //}
//fn _mut_message_info(&mut self) -> &mut MessageInfo {
        //
    //}
//}
//impl IsIdentifiedObject for SwitchEventProfile {
    //fn _identified_object(&self) -> &IdentifiedObject {
        //
    //}
//fn _mut_identified_object(&mut self) -> &mut IdentifiedObject {
        //
    //}
//}
/// Switch reading value
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SwitchReading {
    /// UML inherited base object
    // parent_message: true
    // required_field: false
    // multiplicity_min: None
    // multiplicity_max: None
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
    fn _switch_reading_mut(&mut self) -> &mut SwitchReading;
    fn conducting_equipment_terminal_reading(&self) -> &super::commonmodule::ConductingEquipmentTerminalReading {
        self._switch_reading().conducting_equipment_terminal_reading.as_ref().unwrap_or(&switch_reading::CONDUCTING_EQUIPMENT_TERMINAL_READING)
    }
    fn conducting_equipment_terminal_reading_mut(&mut self) -> &mut super::commonmodule::ConductingEquipmentTerminalReading {
        self._switch_reading_mut().conducting_equipment_terminal_reading.get_or_insert(Default::default())
    }
    fn diff_reading_mmxu(&self) -> &super::commonmodule::ReadingMmxu {
        self._switch_reading().diff_reading_mmxu.as_ref().unwrap_or(&switch_reading::DIFF_READING_MMXU)
    }
    fn diff_reading_mmxu_mut(&mut self) -> &mut super::commonmodule::ReadingMmxu {
        self._switch_reading_mut().diff_reading_mmxu.get_or_insert(Default::default())
    }
    fn phase_mmtn(&self) -> &super::commonmodule::PhaseMmtn {
        self._switch_reading().phase_mmtn.as_ref().unwrap_or(&switch_reading::PHASE_MMTN)
    }
    fn phase_mmtn_mut(&mut self) -> &mut super::commonmodule::PhaseMmtn {
        self._switch_reading_mut().phase_mmtn.get_or_insert(Default::default())
    }
    fn reading_mmtr(&self) -> &super::commonmodule::ReadingMmtr {
        self._switch_reading().reading_mmtr.as_ref().unwrap_or(&switch_reading::READING_MMTR)
    }
    fn reading_mmtr_mut(&mut self) -> &mut super::commonmodule::ReadingMmtr {
        self._switch_reading_mut().reading_mmtr.get_or_insert(Default::default())
    }
    fn reading_mmxu(&self) -> &super::commonmodule::ReadingMmxu {
        self._switch_reading().reading_mmxu.as_ref().unwrap_or(&switch_reading::READING_MMXU)
    }
    fn reading_mmxu_mut(&mut self) -> &mut super::commonmodule::ReadingMmxu {
        self._switch_reading_mut().reading_mmxu.get_or_insert(Default::default())
    }
}
impl IsSwitchReading for SwitchReading {
    fn _switch_reading(&self) -> &SwitchReading {
        self
    }
    fn _switch_reading_mut(&mut self) -> &mut SwitchReading {
        self
    }
}
//impl IsConductingEquipmentTerminalReading for SwitchReading {
    //fn _conducting_equipment_terminal_reading(&self) -> &ConductingEquipmentTerminalReading {
        //
    //}
//fn _mut_conducting_equipment_terminal_reading(&mut self) -> &mut ConductingEquipmentTerminalReading {
        //
    //}
//}
/// Switch reading profile
/// OpenFMB Profile Message: true
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SwitchReadingProfile {
    /// UML inherited base object
    // parent_message: true
    // required_field: false
    // multiplicity_min: None
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, optional, tag="1")]
    pub reading_message_info: ::std::option::Option<super::commonmodule::ReadingMessageInfo>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: Some(1)
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, optional, tag="2")]
    pub protected_switch: ::std::option::Option<ProtectedSwitch>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: Some(1)
    // multiplicity_max: Some(2)
    // uuid: false
    // key: false
    #[prost(message, repeated, tag="3")]
    pub switch_reading: ::std::vec::Vec<SwitchReading>,
}
mod switch_reading_profile {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref READING_MESSAGE_INFO: crate::commonmodule::ReadingMessageInfo = Default::default();
        pub(super) static ref PROTECTED_SWITCH: crate::switchmodule::ProtectedSwitch = Default::default();
    }
}
pub trait IsSwitchReadingProfile {
    fn _switch_reading_profile(&self) -> &SwitchReadingProfile;
    fn _switch_reading_profile_mut(&mut self) -> &mut SwitchReadingProfile;
    fn reading_message_info(&self) -> &super::commonmodule::ReadingMessageInfo {
        self._switch_reading_profile().reading_message_info.as_ref().unwrap_or(&switch_reading_profile::READING_MESSAGE_INFO)
    }
    fn reading_message_info_mut(&mut self) -> &mut super::commonmodule::ReadingMessageInfo {
        self._switch_reading_profile_mut().reading_message_info.get_or_insert(Default::default())
    }
    fn protected_switch(&self) -> &ProtectedSwitch {
        self._switch_reading_profile().protected_switch.as_ref().unwrap_or(&switch_reading_profile::PROTECTED_SWITCH)
    }
    fn protected_switch_mut(&mut self) -> &mut ProtectedSwitch {
        self._switch_reading_profile_mut().protected_switch.get_or_insert(Default::default())
    }
    fn switch_reading(&self) -> &::std::vec::Vec<SwitchReading> {
        &self._switch_reading_profile().switch_reading
    }
    fn switch_reading_mut(&mut self) -> &mut ::std::vec::Vec<SwitchReading> {
        &mut self._switch_reading_profile_mut().switch_reading
    }
}
impl IsSwitchReadingProfile for SwitchReadingProfile {
    fn _switch_reading_profile(&self) -> &SwitchReadingProfile {
        self
    }
    fn _switch_reading_profile_mut(&mut self) -> &mut SwitchReadingProfile {
        self
    }
}
//impl IsReadingMessageInfo for SwitchReadingProfile {
    //fn _reading_message_info(&self) -> &ReadingMessageInfo {
        //
    //}
//fn _mut_reading_message_info(&mut self) -> &mut ReadingMessageInfo {
        //
    //}
//}
//impl IsMessageInfo for SwitchReadingProfile {
    //fn _message_info(&self) -> &MessageInfo {
        //
    //}
//fn _mut_message_info(&mut self) -> &mut MessageInfo {
        //
    //}
//}
//impl IsIdentifiedObject for SwitchReadingProfile {
    //fn _identified_object(&self) -> &IdentifiedObject {
        //
    //}
//fn _mut_identified_object(&mut self) -> &mut IdentifiedObject {
        //
    //}
//}
/// OpenFMB specialization for SwitchStatusProfile
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SwitchStatusXswi {
    /// UML inherited base object
    // parent_message: true
    // required_field: false
    // multiplicity_min: None
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, optional, tag="1")]
    pub logical_node_for_event_and_status: ::std::option::Option<super::commonmodule::LogicalNodeForEventAndStatus>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub dynamic_test: ::std::option::Option<super::commonmodule::EnsDynamicTestKind>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="4")]
    pub pos: ::std::option::Option<super::commonmodule::PhaseDps>,
    /// Fault latch: LT01=51A OR 51B OR 51C
    // parent_message: false
    // required_field: true
    // multiplicity_min: Some(1)
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, optional, tag="5")]
    pub protection_pickup: ::std::option::Option<super::commonmodule::PhaseSps>,
}
mod switch_status_xswi {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref LOGICAL_NODE_FOR_EVENT_AND_STATUS: crate::commonmodule::LogicalNodeForEventAndStatus = Default::default();
        pub(super) static ref DYNAMIC_TEST: crate::commonmodule::EnsDynamicTestKind = Default::default();
        pub(super) static ref POS: crate::commonmodule::PhaseDps = Default::default();
        pub(super) static ref PROTECTION_PICKUP: crate::commonmodule::PhaseSps = Default::default();
    }
}
pub trait IsSwitchStatusXswi {
    fn _switch_status_xswi(&self) -> &SwitchStatusXswi;
    fn _switch_status_xswi_mut(&mut self) -> &mut SwitchStatusXswi;
    fn logical_node_for_event_and_status(&self) -> &super::commonmodule::LogicalNodeForEventAndStatus {
        self._switch_status_xswi().logical_node_for_event_and_status.as_ref().unwrap_or(&switch_status_xswi::LOGICAL_NODE_FOR_EVENT_AND_STATUS)
    }
    fn logical_node_for_event_and_status_mut(&mut self) -> &mut super::commonmodule::LogicalNodeForEventAndStatus {
        self._switch_status_xswi_mut().logical_node_for_event_and_status.get_or_insert(Default::default())
    }
    fn dynamic_test(&self) -> &super::commonmodule::EnsDynamicTestKind {
        self._switch_status_xswi().dynamic_test.as_ref().unwrap_or(&switch_status_xswi::DYNAMIC_TEST)
    }
    fn dynamic_test_mut(&mut self) -> &mut super::commonmodule::EnsDynamicTestKind {
        self._switch_status_xswi_mut().dynamic_test.get_or_insert(Default::default())
    }
    fn pos(&self) -> &super::commonmodule::PhaseDps {
        self._switch_status_xswi().pos.as_ref().unwrap_or(&switch_status_xswi::POS)
    }
    fn pos_mut(&mut self) -> &mut super::commonmodule::PhaseDps {
        self._switch_status_xswi_mut().pos.get_or_insert(Default::default())
    }
    fn protection_pickup(&self) -> &super::commonmodule::PhaseSps {
        self._switch_status_xswi().protection_pickup.as_ref().unwrap_or(&switch_status_xswi::PROTECTION_PICKUP)
    }
    fn protection_pickup_mut(&mut self) -> &mut super::commonmodule::PhaseSps {
        self._switch_status_xswi_mut().protection_pickup.get_or_insert(Default::default())
    }
}
impl IsSwitchStatusXswi for SwitchStatusXswi {
    fn _switch_status_xswi(&self) -> &SwitchStatusXswi {
        self
    }
    fn _switch_status_xswi_mut(&mut self) -> &mut SwitchStatusXswi {
        self
    }
}
//impl IsLogicalNodeForEventAndStatus for SwitchStatusXswi {
    //fn _logical_node_for_event_and_status(&self) -> &LogicalNodeForEventAndStatus {
        //
    //}
//fn _mut_logical_node_for_event_and_status(&mut self) -> &mut LogicalNodeForEventAndStatus {
        //
    //}
//}
//impl IsLogicalNode for SwitchStatusXswi {
    //fn _logical_node(&self) -> &LogicalNode {
        //
    //}
//fn _mut_logical_node(&mut self) -> &mut LogicalNode {
        //
    //}
//}
//impl IsIdentifiedObject for SwitchStatusXswi {
    //fn _identified_object(&self) -> &IdentifiedObject {
        //
    //}
//fn _mut_identified_object(&mut self) -> &mut IdentifiedObject {
        //
    //}
//}
/// Switch status
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SwitchStatus {
    /// UML inherited base object
    // parent_message: true
    // required_field: false
    // multiplicity_min: None
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, optional, tag="1")]
    pub status_value: ::std::option::Option<super::commonmodule::StatusValue>,
    /// MISSING DOCUMENTATION!!!
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
    fn _switch_status_mut(&mut self) -> &mut SwitchStatus;
    fn status_value(&self) -> &super::commonmodule::StatusValue {
        self._switch_status().status_value.as_ref().unwrap_or(&switch_status::STATUS_VALUE)
    }
    fn status_value_mut(&mut self) -> &mut super::commonmodule::StatusValue {
        self._switch_status_mut().status_value.get_or_insert(Default::default())
    }
    fn switch_status_xswi(&self) -> &SwitchStatusXswi {
        self._switch_status().switch_status_xswi.as_ref().unwrap_or(&switch_status::SWITCH_STATUS_XSWI)
    }
    fn switch_status_xswi_mut(&mut self) -> &mut SwitchStatusXswi {
        self._switch_status_mut().switch_status_xswi.get_or_insert(Default::default())
    }
}
impl IsSwitchStatus for SwitchStatus {
    fn _switch_status(&self) -> &SwitchStatus {
        self
    }
    fn _switch_status_mut(&mut self) -> &mut SwitchStatus {
        self
    }
}
//impl IsStatusValue for SwitchStatus {
    //fn _status_value(&self) -> &StatusValue {
        //
    //}
//fn _mut_status_value(&mut self) -> &mut StatusValue {
        //
    //}
//}
//impl IsIdentifiedObject for SwitchStatus {
    //fn _identified_object(&self) -> &IdentifiedObject {
        //
    //}
//fn _mut_identified_object(&mut self) -> &mut IdentifiedObject {
        //
    //}
//}
/// Switch status profile
/// OpenFMB Profile Message: true
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SwitchStatusProfile {
    /// UML inherited base object
    // parent_message: true
    // required_field: false
    // multiplicity_min: None
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, optional, tag="1")]
    pub status_message_info: ::std::option::Option<super::commonmodule::StatusMessageInfo>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: Some(1)
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, optional, tag="2")]
    pub protected_switch: ::std::option::Option<ProtectedSwitch>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: Some(1)
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, optional, tag="3")]
    pub switch_status: ::std::option::Option<SwitchStatus>,
}
mod switch_status_profile {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref STATUS_MESSAGE_INFO: crate::commonmodule::StatusMessageInfo = Default::default();
        pub(super) static ref PROTECTED_SWITCH: crate::switchmodule::ProtectedSwitch = Default::default();
        pub(super) static ref SWITCH_STATUS: crate::switchmodule::SwitchStatus = Default::default();
    }
}
pub trait IsSwitchStatusProfile {
    fn _switch_status_profile(&self) -> &SwitchStatusProfile;
    fn _switch_status_profile_mut(&mut self) -> &mut SwitchStatusProfile;
    fn status_message_info(&self) -> &super::commonmodule::StatusMessageInfo {
        self._switch_status_profile().status_message_info.as_ref().unwrap_or(&switch_status_profile::STATUS_MESSAGE_INFO)
    }
    fn status_message_info_mut(&mut self) -> &mut super::commonmodule::StatusMessageInfo {
        self._switch_status_profile_mut().status_message_info.get_or_insert(Default::default())
    }
    fn protected_switch(&self) -> &ProtectedSwitch {
        self._switch_status_profile().protected_switch.as_ref().unwrap_or(&switch_status_profile::PROTECTED_SWITCH)
    }
    fn protected_switch_mut(&mut self) -> &mut ProtectedSwitch {
        self._switch_status_profile_mut().protected_switch.get_or_insert(Default::default())
    }
    fn switch_status(&self) -> &SwitchStatus {
        self._switch_status_profile().switch_status.as_ref().unwrap_or(&switch_status_profile::SWITCH_STATUS)
    }
    fn switch_status_mut(&mut self) -> &mut SwitchStatus {
        self._switch_status_profile_mut().switch_status.get_or_insert(Default::default())
    }
}
impl IsSwitchStatusProfile for SwitchStatusProfile {
    fn _switch_status_profile(&self) -> &SwitchStatusProfile {
        self
    }
    fn _switch_status_profile_mut(&mut self) -> &mut SwitchStatusProfile {
        self
    }
}
//impl IsStatusMessageInfo for SwitchStatusProfile {
    //fn _status_message_info(&self) -> &StatusMessageInfo {
        //
    //}
//fn _mut_status_message_info(&mut self) -> &mut StatusMessageInfo {
        //
    //}
//}
//impl IsMessageInfo for SwitchStatusProfile {
    //fn _message_info(&self) -> &MessageInfo {
        //
    //}
//fn _mut_message_info(&mut self) -> &mut MessageInfo {
        //
    //}
//}
//impl IsIdentifiedObject for SwitchStatusProfile {
    //fn _identified_object(&self) -> &IdentifiedObject {
        //
    //}
//fn _mut_identified_object(&mut self) -> &mut IdentifiedObject {
        //
    //}
//}
