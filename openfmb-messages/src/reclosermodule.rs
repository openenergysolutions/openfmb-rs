/// Specialized 61850 FSCC class.  LN: Schedule controller   Name: FSCC
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct RecloserControlFscc {
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
mod recloser_control_fscc {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref LOGICAL_NODE_FOR_CONTROL: crate::commonmodule::LogicalNodeForControl = Default::default();
        pub(super) static ref SWITCH_CONTROL_SCHEDULE_FSCH: crate::commonmodule::SwitchControlScheduleFsch = Default::default();
    }
}
pub trait IsRecloserControlFscc {
    fn _recloser_control_fscc(&self) -> &RecloserControlFscc;
    fn _mut_recloser_control_fscc(&mut self) -> &mut RecloserControlFscc;
    fn logical_node_for_control(&self) -> &super::commonmodule::LogicalNodeForControl {
        self._recloser_control_fscc().logical_node_for_control.as_ref().unwrap_or(&recloser_control_fscc::LOGICAL_NODE_FOR_CONTROL)
    }
    fn mut_logical_node_for_control(&mut self) -> &mut super::commonmodule::LogicalNodeForControl {
        self._mut_recloser_control_fscc().logical_node_for_control.get_or_insert(recloser_control_fscc::LOGICAL_NODE_FOR_CONTROL.clone())
    }
    fn switch_control_schedule_fsch(&self) -> &super::commonmodule::SwitchControlScheduleFsch {
        self._recloser_control_fscc().switch_control_schedule_fsch.as_ref().unwrap_or(&recloser_control_fscc::SWITCH_CONTROL_SCHEDULE_FSCH)
    }
    fn mut_switch_control_schedule_fsch(&mut self) -> &mut super::commonmodule::SwitchControlScheduleFsch {
        self._mut_recloser_control_fscc().switch_control_schedule_fsch.get_or_insert(recloser_control_fscc::SWITCH_CONTROL_SCHEDULE_FSCH.clone())
    }
}
impl IsRecloserControlFscc for RecloserControlFscc {
    fn _recloser_control_fscc(&self) -> &RecloserControlFscc {
        self
    }
    fn _mut_recloser_control_fscc(&mut self) -> &mut RecloserControlFscc {
        self
    }
}
/// Recloser discrete control
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct RecloserControl {
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
    pub recloser_control_fscc: ::std::option::Option<RecloserControlFscc>,
}
mod recloser_control {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref CONTROL_VALUE: crate::commonmodule::ControlValue = Default::default();
        pub(super) static ref CHECK: crate::commonmodule::CheckConditions = Default::default();
        pub(super) static ref RECLOSER_CONTROL_FSCC: crate::reclosermodule::RecloserControlFscc = Default::default();
    }
}
pub trait IsRecloserControl {
    fn _recloser_control(&self) -> &RecloserControl;
    fn _mut_recloser_control(&mut self) -> &mut RecloserControl;
    fn control_value(&self) -> &super::commonmodule::ControlValue {
        self._recloser_control().control_value.as_ref().unwrap_or(&recloser_control::CONTROL_VALUE)
    }
    fn mut_control_value(&mut self) -> &mut super::commonmodule::ControlValue {
        self._mut_recloser_control().control_value.get_or_insert(recloser_control::CONTROL_VALUE.clone())
    }
    fn check(&self) -> &super::commonmodule::CheckConditions {
        self._recloser_control().check.as_ref().unwrap_or(&recloser_control::CHECK)
    }
    fn mut_check(&mut self) -> &mut super::commonmodule::CheckConditions {
        self._mut_recloser_control().check.get_or_insert(recloser_control::CHECK.clone())
    }
    fn recloser_control_fscc(&self) -> &RecloserControlFscc {
        self._recloser_control().recloser_control_fscc.as_ref().unwrap_or(&recloser_control::RECLOSER_CONTROL_FSCC)
    }
    fn mut_recloser_control_fscc(&mut self) -> &mut RecloserControlFscc {
        self._mut_recloser_control().recloser_control_fscc.get_or_insert(recloser_control::RECLOSER_CONTROL_FSCC.clone())
    }
}
impl IsRecloserControl for RecloserControl {
    fn _recloser_control(&self) -> &RecloserControl {
        self
    }
    fn _mut_recloser_control(&mut self) -> &mut RecloserControl {
        self
    }
}
/// Pole-mounted fault interrupter with built-in phase and ground relays, current transformer (CT),
/// and supplemental controls.
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Recloser {
    /// UML inherited base object
    // parent_message: true
    // required_field: false
    // multiplicity_min: 0
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="1")]
    pub conducting_equipment: ::std::option::Option<super::commonmodule::ConductingEquipment>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub normal_open: ::std::option::Option<bool>,
}
mod recloser {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref CONDUCTING_EQUIPMENT: crate::commonmodule::ConductingEquipment = Default::default();
        pub(super) static ref NORMAL_OPEN: bool = Default::default();
    }
}
pub trait IsRecloser {
    fn _recloser(&self) -> &Recloser;
    fn _mut_recloser(&mut self) -> &mut Recloser;
    fn conducting_equipment(&self) -> &super::commonmodule::ConductingEquipment {
        self._recloser().conducting_equipment.as_ref().unwrap_or(&recloser::CONDUCTING_EQUIPMENT)
    }
    fn mut_conducting_equipment(&mut self) -> &mut super::commonmodule::ConductingEquipment {
        self._mut_recloser().conducting_equipment.get_or_insert(recloser::CONDUCTING_EQUIPMENT.clone())
    }
    fn normal_open(&self) -> &bool {
        self._recloser().normal_open.as_ref().unwrap_or(&recloser::NORMAL_OPEN)
    }
    fn mut_normal_open(&mut self) -> &mut bool {
        self._mut_recloser().normal_open.get_or_insert(recloser::NORMAL_OPEN.clone())
    }
}
impl IsRecloser for Recloser {
    fn _recloser(&self) -> &Recloser {
        self
    }
    fn _mut_recloser(&mut self) -> &mut Recloser {
        self
    }
}
/// Recloser control profile.  Instructs an end device (or an end device group) to perform a
/// specified action.
/// OpenFMB Profile Message: true
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct RecloserControlProfile {
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
    pub recloser: ::std::option::Option<Recloser>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="4")]
    pub recloser_control: ::std::option::Option<RecloserControl>,
}
mod recloser_control_profile {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref CONTROL_MESSAGE_INFO: crate::commonmodule::ControlMessageInfo = Default::default();
        pub(super) static ref IED: crate::commonmodule::Ied = Default::default();
        pub(super) static ref RECLOSER: crate::reclosermodule::Recloser = Default::default();
        pub(super) static ref RECLOSER_CONTROL: crate::reclosermodule::RecloserControl = Default::default();
    }
}
pub trait IsRecloserControlProfile {
    fn _recloser_control_profile(&self) -> &RecloserControlProfile;
    fn _mut_recloser_control_profile(&mut self) -> &mut RecloserControlProfile;
    fn control_message_info(&self) -> &super::commonmodule::ControlMessageInfo {
        self._recloser_control_profile().control_message_info.as_ref().unwrap_or(&recloser_control_profile::CONTROL_MESSAGE_INFO)
    }
    fn mut_control_message_info(&mut self) -> &mut super::commonmodule::ControlMessageInfo {
        self._mut_recloser_control_profile().control_message_info.get_or_insert(recloser_control_profile::CONTROL_MESSAGE_INFO.clone())
    }
    fn ied(&self) -> &super::commonmodule::Ied {
        self._recloser_control_profile().ied.as_ref().unwrap_or(&recloser_control_profile::IED)
    }
    fn mut_ied(&mut self) -> &mut super::commonmodule::Ied {
        self._mut_recloser_control_profile().ied.get_or_insert(recloser_control_profile::IED.clone())
    }
    fn recloser(&self) -> &Recloser {
        self._recloser_control_profile().recloser.as_ref().unwrap_or(&recloser_control_profile::RECLOSER)
    }
    fn mut_recloser(&mut self) -> &mut Recloser {
        self._mut_recloser_control_profile().recloser.get_or_insert(recloser_control_profile::RECLOSER.clone())
    }
    fn recloser_control(&self) -> &RecloserControl {
        self._recloser_control_profile().recloser_control.as_ref().unwrap_or(&recloser_control_profile::RECLOSER_CONTROL)
    }
    fn mut_recloser_control(&mut self) -> &mut RecloserControl {
        self._mut_recloser_control_profile().recloser_control.get_or_insert(recloser_control_profile::RECLOSER_CONTROL.clone())
    }
}
impl IsRecloserControlProfile for RecloserControlProfile {
    fn _recloser_control_profile(&self) -> &RecloserControlProfile {
        self
    }
    fn _mut_recloser_control_profile(&mut self) -> &mut RecloserControlProfile {
        self
    }
}
/// OpenFMB specialization for RecloserDiscreteControlProfile: Added blk  LN: Circuit
/// breaker   Name: XCBR
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct RecloserDiscreteControlXcbr {
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
mod recloser_discrete_control_xcbr {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref LOGICAL_NODE_FOR_CONTROL: crate::commonmodule::LogicalNodeForControl = Default::default();
        pub(super) static ref POS: crate::commonmodule::ControlDpc = Default::default();
    }
}
pub trait IsRecloserDiscreteControlXcbr {
    fn _recloser_discrete_control_xcbr(&self) -> &RecloserDiscreteControlXcbr;
    fn _mut_recloser_discrete_control_xcbr(&mut self) -> &mut RecloserDiscreteControlXcbr;
    fn logical_node_for_control(&self) -> &super::commonmodule::LogicalNodeForControl {
        self._recloser_discrete_control_xcbr().logical_node_for_control.as_ref().unwrap_or(&recloser_discrete_control_xcbr::LOGICAL_NODE_FOR_CONTROL)
    }
    fn mut_logical_node_for_control(&mut self) -> &mut super::commonmodule::LogicalNodeForControl {
        self._mut_recloser_discrete_control_xcbr().logical_node_for_control.get_or_insert(recloser_discrete_control_xcbr::LOGICAL_NODE_FOR_CONTROL.clone())
    }
    fn pos(&self) -> &super::commonmodule::ControlDpc {
        self._recloser_discrete_control_xcbr().pos.as_ref().unwrap_or(&recloser_discrete_control_xcbr::POS)
    }
    fn mut_pos(&mut self) -> &mut super::commonmodule::ControlDpc {
        self._mut_recloser_discrete_control_xcbr().pos.get_or_insert(recloser_discrete_control_xcbr::POS.clone())
    }
}
impl IsRecloserDiscreteControlXcbr for RecloserDiscreteControlXcbr {
    fn _recloser_discrete_control_xcbr(&self) -> &RecloserDiscreteControlXcbr {
        self
    }
    fn _mut_recloser_discrete_control_xcbr(&mut self) -> &mut RecloserDiscreteControlXcbr {
        self
    }
}
/// Recloser discrete control
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct RecloserDiscreteControl {
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
    pub recloser_discrete_control_xcbr: ::std::option::Option<RecloserDiscreteControlXcbr>,
}
mod recloser_discrete_control {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref CONTROL_VALUE: crate::commonmodule::ControlValue = Default::default();
        pub(super) static ref CHECK: crate::commonmodule::CheckConditions = Default::default();
        pub(super) static ref RECLOSER_DISCRETE_CONTROL_XCBR: crate::reclosermodule::RecloserDiscreteControlXcbr = Default::default();
    }
}
pub trait IsRecloserDiscreteControl {
    fn _recloser_discrete_control(&self) -> &RecloserDiscreteControl;
    fn _mut_recloser_discrete_control(&mut self) -> &mut RecloserDiscreteControl;
    fn control_value(&self) -> &super::commonmodule::ControlValue {
        self._recloser_discrete_control().control_value.as_ref().unwrap_or(&recloser_discrete_control::CONTROL_VALUE)
    }
    fn mut_control_value(&mut self) -> &mut super::commonmodule::ControlValue {
        self._mut_recloser_discrete_control().control_value.get_or_insert(recloser_discrete_control::CONTROL_VALUE.clone())
    }
    fn check(&self) -> &super::commonmodule::CheckConditions {
        self._recloser_discrete_control().check.as_ref().unwrap_or(&recloser_discrete_control::CHECK)
    }
    fn mut_check(&mut self) -> &mut super::commonmodule::CheckConditions {
        self._mut_recloser_discrete_control().check.get_or_insert(recloser_discrete_control::CHECK.clone())
    }
    fn recloser_discrete_control_xcbr(&self) -> &RecloserDiscreteControlXcbr {
        self._recloser_discrete_control().recloser_discrete_control_xcbr.as_ref().unwrap_or(&recloser_discrete_control::RECLOSER_DISCRETE_CONTROL_XCBR)
    }
    fn mut_recloser_discrete_control_xcbr(&mut self) -> &mut RecloserDiscreteControlXcbr {
        self._mut_recloser_discrete_control().recloser_discrete_control_xcbr.get_or_insert(recloser_discrete_control::RECLOSER_DISCRETE_CONTROL_XCBR.clone())
    }
}
impl IsRecloserDiscreteControl for RecloserDiscreteControl {
    fn _recloser_discrete_control(&self) -> &RecloserDiscreteControl {
        self
    }
    fn _mut_recloser_discrete_control(&mut self) -> &mut RecloserDiscreteControl {
        self
    }
}
/// Recloser control profile.  Instructs an end device (or an end device group) to perform a
/// specified action.
/// OpenFMB Profile Message: true
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct RecloserDiscreteControlProfile {
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
    pub recloser: ::std::option::Option<Recloser>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="4")]
    pub recloser_discrete_control: ::std::option::Option<RecloserDiscreteControl>,
}
mod recloser_discrete_control_profile {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref CONTROL_MESSAGE_INFO: crate::commonmodule::ControlMessageInfo = Default::default();
        pub(super) static ref IED: crate::commonmodule::Ied = Default::default();
        pub(super) static ref RECLOSER: crate::reclosermodule::Recloser = Default::default();
        pub(super) static ref RECLOSER_DISCRETE_CONTROL: crate::reclosermodule::RecloserDiscreteControl = Default::default();
    }
}
pub trait IsRecloserDiscreteControlProfile {
    fn _recloser_discrete_control_profile(&self) -> &RecloserDiscreteControlProfile;
    fn _mut_recloser_discrete_control_profile(&mut self) -> &mut RecloserDiscreteControlProfile;
    fn control_message_info(&self) -> &super::commonmodule::ControlMessageInfo {
        self._recloser_discrete_control_profile().control_message_info.as_ref().unwrap_or(&recloser_discrete_control_profile::CONTROL_MESSAGE_INFO)
    }
    fn mut_control_message_info(&mut self) -> &mut super::commonmodule::ControlMessageInfo {
        self._mut_recloser_discrete_control_profile().control_message_info.get_or_insert(recloser_discrete_control_profile::CONTROL_MESSAGE_INFO.clone())
    }
    fn ied(&self) -> &super::commonmodule::Ied {
        self._recloser_discrete_control_profile().ied.as_ref().unwrap_or(&recloser_discrete_control_profile::IED)
    }
    fn mut_ied(&mut self) -> &mut super::commonmodule::Ied {
        self._mut_recloser_discrete_control_profile().ied.get_or_insert(recloser_discrete_control_profile::IED.clone())
    }
    fn recloser(&self) -> &Recloser {
        self._recloser_discrete_control_profile().recloser.as_ref().unwrap_or(&recloser_discrete_control_profile::RECLOSER)
    }
    fn mut_recloser(&mut self) -> &mut Recloser {
        self._mut_recloser_discrete_control_profile().recloser.get_or_insert(recloser_discrete_control_profile::RECLOSER.clone())
    }
    fn recloser_discrete_control(&self) -> &RecloserDiscreteControl {
        self._recloser_discrete_control_profile().recloser_discrete_control.as_ref().unwrap_or(&recloser_discrete_control_profile::RECLOSER_DISCRETE_CONTROL)
    }
    fn mut_recloser_discrete_control(&mut self) -> &mut RecloserDiscreteControl {
        self._mut_recloser_discrete_control_profile().recloser_discrete_control.get_or_insert(recloser_discrete_control_profile::RECLOSER_DISCRETE_CONTROL.clone())
    }
}
impl IsRecloserDiscreteControlProfile for RecloserDiscreteControlProfile {
    fn _recloser_discrete_control_profile(&self) -> &RecloserDiscreteControlProfile {
        self
    }
    fn _mut_recloser_discrete_control_profile(&mut self) -> &mut RecloserDiscreteControlProfile {
        self
    }
}
/// Recloser event
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct RecloserEvent {
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
mod recloser_event {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref EVENT_VALUE: crate::commonmodule::EventValue = Default::default();
        pub(super) static ref STATUS_AND_EVENT_XCBR: crate::commonmodule::StatusAndEventXcbr = Default::default();
    }
}
pub trait IsRecloserEvent {
    fn _recloser_event(&self) -> &RecloserEvent;
    fn _mut_recloser_event(&mut self) -> &mut RecloserEvent;
    fn event_value(&self) -> &super::commonmodule::EventValue {
        self._recloser_event().event_value.as_ref().unwrap_or(&recloser_event::EVENT_VALUE)
    }
    fn mut_event_value(&mut self) -> &mut super::commonmodule::EventValue {
        self._mut_recloser_event().event_value.get_or_insert(recloser_event::EVENT_VALUE.clone())
    }
    fn status_and_event_xcbr(&self) -> &super::commonmodule::StatusAndEventXcbr {
        self._recloser_event().status_and_event_xcbr.as_ref().unwrap_or(&recloser_event::STATUS_AND_EVENT_XCBR)
    }
    fn mut_status_and_event_xcbr(&mut self) -> &mut super::commonmodule::StatusAndEventXcbr {
        self._mut_recloser_event().status_and_event_xcbr.get_or_insert(recloser_event::STATUS_AND_EVENT_XCBR.clone())
    }
}
impl IsRecloserEvent for RecloserEvent {
    fn _recloser_event(&self) -> &RecloserEvent {
        self
    }
    fn _mut_recloser_event(&mut self) -> &mut RecloserEvent {
        self
    }
}
/// Recloser event profile
/// OpenFMB Profile Message: true
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct RecloserEventProfile {
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
    pub recloser: ::std::option::Option<Recloser>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="4")]
    pub recloser_event: ::std::option::Option<RecloserEvent>,
}
mod recloser_event_profile {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref EVENT_MESSAGE_INFO: crate::commonmodule::EventMessageInfo = Default::default();
        pub(super) static ref IED: crate::commonmodule::Ied = Default::default();
        pub(super) static ref RECLOSER: crate::reclosermodule::Recloser = Default::default();
        pub(super) static ref RECLOSER_EVENT: crate::reclosermodule::RecloserEvent = Default::default();
    }
}
pub trait IsRecloserEventProfile {
    fn _recloser_event_profile(&self) -> &RecloserEventProfile;
    fn _mut_recloser_event_profile(&mut self) -> &mut RecloserEventProfile;
    fn event_message_info(&self) -> &super::commonmodule::EventMessageInfo {
        self._recloser_event_profile().event_message_info.as_ref().unwrap_or(&recloser_event_profile::EVENT_MESSAGE_INFO)
    }
    fn mut_event_message_info(&mut self) -> &mut super::commonmodule::EventMessageInfo {
        self._mut_recloser_event_profile().event_message_info.get_or_insert(recloser_event_profile::EVENT_MESSAGE_INFO.clone())
    }
    fn ied(&self) -> &super::commonmodule::Ied {
        self._recloser_event_profile().ied.as_ref().unwrap_or(&recloser_event_profile::IED)
    }
    fn mut_ied(&mut self) -> &mut super::commonmodule::Ied {
        self._mut_recloser_event_profile().ied.get_or_insert(recloser_event_profile::IED.clone())
    }
    fn recloser(&self) -> &Recloser {
        self._recloser_event_profile().recloser.as_ref().unwrap_or(&recloser_event_profile::RECLOSER)
    }
    fn mut_recloser(&mut self) -> &mut Recloser {
        self._mut_recloser_event_profile().recloser.get_or_insert(recloser_event_profile::RECLOSER.clone())
    }
    fn recloser_event(&self) -> &RecloserEvent {
        self._recloser_event_profile().recloser_event.as_ref().unwrap_or(&recloser_event_profile::RECLOSER_EVENT)
    }
    fn mut_recloser_event(&mut self) -> &mut RecloserEvent {
        self._mut_recloser_event_profile().recloser_event.get_or_insert(recloser_event_profile::RECLOSER_EVENT.clone())
    }
}
impl IsRecloserEventProfile for RecloserEventProfile {
    fn _recloser_event_profile(&self) -> &RecloserEventProfile {
        self
    }
    fn _mut_recloser_event_profile(&mut self) -> &mut RecloserEventProfile {
        self
    }
}
/// Recloser reading value
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct RecloserReading {
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
mod recloser_reading {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref CONDUCTING_EQUIPMENT_TERMINAL_READING: crate::commonmodule::ConductingEquipmentTerminalReading = Default::default();
        pub(super) static ref DIFF_READING_MMXU: crate::commonmodule::ReadingMmxu = Default::default();
        pub(super) static ref PHASE_MMTN: crate::commonmodule::PhaseMmtn = Default::default();
        pub(super) static ref READING_MMTR: crate::commonmodule::ReadingMmtr = Default::default();
        pub(super) static ref READING_MMXU: crate::commonmodule::ReadingMmxu = Default::default();
    }
}
pub trait IsRecloserReading {
    fn _recloser_reading(&self) -> &RecloserReading;
    fn _mut_recloser_reading(&mut self) -> &mut RecloserReading;
    fn conducting_equipment_terminal_reading(&self) -> &super::commonmodule::ConductingEquipmentTerminalReading {
        self._recloser_reading().conducting_equipment_terminal_reading.as_ref().unwrap_or(&recloser_reading::CONDUCTING_EQUIPMENT_TERMINAL_READING)
    }
    fn mut_conducting_equipment_terminal_reading(&mut self) -> &mut super::commonmodule::ConductingEquipmentTerminalReading {
        self._mut_recloser_reading().conducting_equipment_terminal_reading.get_or_insert(recloser_reading::CONDUCTING_EQUIPMENT_TERMINAL_READING.clone())
    }
    fn diff_reading_mmxu(&self) -> &super::commonmodule::ReadingMmxu {
        self._recloser_reading().diff_reading_mmxu.as_ref().unwrap_or(&recloser_reading::DIFF_READING_MMXU)
    }
    fn mut_diff_reading_mmxu(&mut self) -> &mut super::commonmodule::ReadingMmxu {
        self._mut_recloser_reading().diff_reading_mmxu.get_or_insert(recloser_reading::DIFF_READING_MMXU.clone())
    }
    fn phase_mmtn(&self) -> &super::commonmodule::PhaseMmtn {
        self._recloser_reading().phase_mmtn.as_ref().unwrap_or(&recloser_reading::PHASE_MMTN)
    }
    fn mut_phase_mmtn(&mut self) -> &mut super::commonmodule::PhaseMmtn {
        self._mut_recloser_reading().phase_mmtn.get_or_insert(recloser_reading::PHASE_MMTN.clone())
    }
    fn reading_mmtr(&self) -> &super::commonmodule::ReadingMmtr {
        self._recloser_reading().reading_mmtr.as_ref().unwrap_or(&recloser_reading::READING_MMTR)
    }
    fn mut_reading_mmtr(&mut self) -> &mut super::commonmodule::ReadingMmtr {
        self._mut_recloser_reading().reading_mmtr.get_or_insert(recloser_reading::READING_MMTR.clone())
    }
    fn reading_mmxu(&self) -> &super::commonmodule::ReadingMmxu {
        self._recloser_reading().reading_mmxu.as_ref().unwrap_or(&recloser_reading::READING_MMXU)
    }
    fn mut_reading_mmxu(&mut self) -> &mut super::commonmodule::ReadingMmxu {
        self._mut_recloser_reading().reading_mmxu.get_or_insert(recloser_reading::READING_MMXU.clone())
    }
}
impl IsRecloserReading for RecloserReading {
    fn _recloser_reading(&self) -> &RecloserReading {
        self
    }
    fn _mut_recloser_reading(&mut self) -> &mut RecloserReading {
        self
    }
}
/// Recloser reading profile
/// OpenFMB Profile Message: true
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct RecloserReadingProfile {
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
    pub recloser: ::std::option::Option<Recloser>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 2
    // uuid: false
    // key: false
    #[prost(message, repeated, tag="4")]
    pub recloser_reading: ::std::vec::Vec<RecloserReading>,
}
mod recloser_reading_profile {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref READING_MESSAGE_INFO: crate::commonmodule::ReadingMessageInfo = Default::default();
        pub(super) static ref IED: crate::commonmodule::Ied = Default::default();
        pub(super) static ref RECLOSER: crate::reclosermodule::Recloser = Default::default();
    }
}
pub trait IsRecloserReadingProfile {
    fn _recloser_reading_profile(&self) -> &RecloserReadingProfile;
    fn _mut_recloser_reading_profile(&mut self) -> &mut RecloserReadingProfile;
    fn reading_message_info(&self) -> &super::commonmodule::ReadingMessageInfo {
        self._recloser_reading_profile().reading_message_info.as_ref().unwrap_or(&recloser_reading_profile::READING_MESSAGE_INFO)
    }
    fn mut_reading_message_info(&mut self) -> &mut super::commonmodule::ReadingMessageInfo {
        self._mut_recloser_reading_profile().reading_message_info.get_or_insert(recloser_reading_profile::READING_MESSAGE_INFO.clone())
    }
    fn ied(&self) -> &super::commonmodule::Ied {
        self._recloser_reading_profile().ied.as_ref().unwrap_or(&recloser_reading_profile::IED)
    }
    fn mut_ied(&mut self) -> &mut super::commonmodule::Ied {
        self._mut_recloser_reading_profile().ied.get_or_insert(recloser_reading_profile::IED.clone())
    }
    fn recloser(&self) -> &Recloser {
        self._recloser_reading_profile().recloser.as_ref().unwrap_or(&recloser_reading_profile::RECLOSER)
    }
    fn mut_recloser(&mut self) -> &mut Recloser {
        self._mut_recloser_reading_profile().recloser.get_or_insert(recloser_reading_profile::RECLOSER.clone())
    }
    fn recloser_reading(&self) -> &::std::vec::Vec<RecloserReading> {
        &self._recloser_reading_profile().recloser_reading    }
    fn mut_recloser_reading(&mut self) -> &mut ::std::vec::Vec<RecloserReading> {
        &mut self._mut_recloser_reading_profile().recloser_reading    }
}
impl IsRecloserReadingProfile for RecloserReadingProfile {
    fn _recloser_reading_profile(&self) -> &RecloserReadingProfile {
        self
    }
    fn _mut_recloser_reading_profile(&mut self) -> &mut RecloserReadingProfile {
        self
    }
}
/// Recloser status
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct RecloserStatus {
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
mod recloser_status {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref STATUS_VALUE: crate::commonmodule::StatusValue = Default::default();
        pub(super) static ref STATUS_AND_EVENT_XCBR: crate::commonmodule::StatusAndEventXcbr = Default::default();
    }
}
pub trait IsRecloserStatus {
    fn _recloser_status(&self) -> &RecloserStatus;
    fn _mut_recloser_status(&mut self) -> &mut RecloserStatus;
    fn status_value(&self) -> &super::commonmodule::StatusValue {
        self._recloser_status().status_value.as_ref().unwrap_or(&recloser_status::STATUS_VALUE)
    }
    fn mut_status_value(&mut self) -> &mut super::commonmodule::StatusValue {
        self._mut_recloser_status().status_value.get_or_insert(recloser_status::STATUS_VALUE.clone())
    }
    fn status_and_event_xcbr(&self) -> &super::commonmodule::StatusAndEventXcbr {
        self._recloser_status().status_and_event_xcbr.as_ref().unwrap_or(&recloser_status::STATUS_AND_EVENT_XCBR)
    }
    fn mut_status_and_event_xcbr(&mut self) -> &mut super::commonmodule::StatusAndEventXcbr {
        self._mut_recloser_status().status_and_event_xcbr.get_or_insert(recloser_status::STATUS_AND_EVENT_XCBR.clone())
    }
}
impl IsRecloserStatus for RecloserStatus {
    fn _recloser_status(&self) -> &RecloserStatus {
        self
    }
    fn _mut_recloser_status(&mut self) -> &mut RecloserStatus {
        self
    }
}
/// Recloser status profile
/// OpenFMB Profile Message: true
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct RecloserStatusProfile {
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
    pub recloser: ::std::option::Option<Recloser>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="4")]
    pub recloser_status: ::std::option::Option<RecloserStatus>,
}
mod recloser_status_profile {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref STATUS_MESSAGE_INFO: crate::commonmodule::StatusMessageInfo = Default::default();
        pub(super) static ref IED: crate::commonmodule::Ied = Default::default();
        pub(super) static ref RECLOSER: crate::reclosermodule::Recloser = Default::default();
        pub(super) static ref RECLOSER_STATUS: crate::reclosermodule::RecloserStatus = Default::default();
    }
}
pub trait IsRecloserStatusProfile {
    fn _recloser_status_profile(&self) -> &RecloserStatusProfile;
    fn _mut_recloser_status_profile(&mut self) -> &mut RecloserStatusProfile;
    fn status_message_info(&self) -> &super::commonmodule::StatusMessageInfo {
        self._recloser_status_profile().status_message_info.as_ref().unwrap_or(&recloser_status_profile::STATUS_MESSAGE_INFO)
    }
    fn mut_status_message_info(&mut self) -> &mut super::commonmodule::StatusMessageInfo {
        self._mut_recloser_status_profile().status_message_info.get_or_insert(recloser_status_profile::STATUS_MESSAGE_INFO.clone())
    }
    fn ied(&self) -> &super::commonmodule::Ied {
        self._recloser_status_profile().ied.as_ref().unwrap_or(&recloser_status_profile::IED)
    }
    fn mut_ied(&mut self) -> &mut super::commonmodule::Ied {
        self._mut_recloser_status_profile().ied.get_or_insert(recloser_status_profile::IED.clone())
    }
    fn recloser(&self) -> &Recloser {
        self._recloser_status_profile().recloser.as_ref().unwrap_or(&recloser_status_profile::RECLOSER)
    }
    fn mut_recloser(&mut self) -> &mut Recloser {
        self._mut_recloser_status_profile().recloser.get_or_insert(recloser_status_profile::RECLOSER.clone())
    }
    fn recloser_status(&self) -> &RecloserStatus {
        self._recloser_status_profile().recloser_status.as_ref().unwrap_or(&recloser_status_profile::RECLOSER_STATUS)
    }
    fn mut_recloser_status(&mut self) -> &mut RecloserStatus {
        self._mut_recloser_status_profile().recloser_status.get_or_insert(recloser_status_profile::RECLOSER_STATUS.clone())
    }
}
impl IsRecloserStatusProfile for RecloserStatusProfile {
    fn _recloser_status_profile(&self) -> &RecloserStatusProfile {
        self
    }
    fn _mut_recloser_status_profile(&mut self) -> &mut RecloserStatusProfile {
        self
    }
}
