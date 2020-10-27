/// Specialized 61850 FSCC class.  LN: Schedule controller   Name: FSCC
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct RecloserControlFscc {
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
    fn _recloser_control_fscc_mut(&mut self) -> &mut RecloserControlFscc;
    fn logical_node_for_control(&self) -> &super::commonmodule::LogicalNodeForControl {
        self._recloser_control_fscc().logical_node_for_control.as_ref().unwrap_or(&recloser_control_fscc::LOGICAL_NODE_FOR_CONTROL)
    }
    fn logical_node_for_control_mut(&mut self) -> &mut super::commonmodule::LogicalNodeForControl {
        self._recloser_control_fscc_mut().logical_node_for_control.get_or_insert(Default::default())
    }
    fn switch_control_schedule_fsch(&self) -> &super::commonmodule::SwitchControlScheduleFsch {
        self._recloser_control_fscc().switch_control_schedule_fsch.as_ref().unwrap_or(&recloser_control_fscc::SWITCH_CONTROL_SCHEDULE_FSCH)
    }
    fn switch_control_schedule_fsch_mut(&mut self) -> &mut super::commonmodule::SwitchControlScheduleFsch {
        self._recloser_control_fscc_mut().switch_control_schedule_fsch.get_or_insert(Default::default())
    }
}
impl IsRecloserControlFscc for RecloserControlFscc {
    fn _recloser_control_fscc(&self) -> &RecloserControlFscc {
        self
    }
    fn _recloser_control_fscc_mut(&mut self) -> &mut RecloserControlFscc {
        self
    }
}
//impl IsLogicalNodeForControl for RecloserControlFscc {
    //fn _logical_node_for_control(&self) -> &LogicalNodeForControl {
        //
    //}
//fn _mut_logical_node_for_control(&mut self) -> &mut LogicalNodeForControl {
        //
    //}
//}
//impl IsLogicalNode for RecloserControlFscc {
    //fn _logical_node(&self) -> &LogicalNode {
        //
    //}
//fn _mut_logical_node(&mut self) -> &mut LogicalNode {
        //
    //}
//}
//impl IsIdentifiedObject for RecloserControlFscc {
    //fn _identified_object(&self) -> &IdentifiedObject {
        //
    //}
//fn _mut_identified_object(&mut self) -> &mut IdentifiedObject {
        //
    //}
//}
/// Recloser discrete control
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct RecloserControl {
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
    // parent_message: false
    // required_field: true
    // multiplicity_min: Some(1)
    // multiplicity_max: None
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
    fn _recloser_control_mut(&mut self) -> &mut RecloserControl;
    fn control_value(&self) -> &super::commonmodule::ControlValue {
        self._recloser_control().control_value.as_ref().unwrap_or(&recloser_control::CONTROL_VALUE)
    }
    fn control_value_mut(&mut self) -> &mut super::commonmodule::ControlValue {
        self._recloser_control_mut().control_value.get_or_insert(Default::default())
    }
    fn check(&self) -> &super::commonmodule::CheckConditions {
        self._recloser_control().check.as_ref().unwrap_or(&recloser_control::CHECK)
    }
    fn check_mut(&mut self) -> &mut super::commonmodule::CheckConditions {
        self._recloser_control_mut().check.get_or_insert(Default::default())
    }
    fn recloser_control_fscc(&self) -> &RecloserControlFscc {
        self._recloser_control().recloser_control_fscc.as_ref().unwrap_or(&recloser_control::RECLOSER_CONTROL_FSCC)
    }
    fn recloser_control_fscc_mut(&mut self) -> &mut RecloserControlFscc {
        self._recloser_control_mut().recloser_control_fscc.get_or_insert(Default::default())
    }
}
impl IsRecloserControl for RecloserControl {
    fn _recloser_control(&self) -> &RecloserControl {
        self
    }
    fn _recloser_control_mut(&mut self) -> &mut RecloserControl {
        self
    }
}
//impl IsControlValue for RecloserControl {
    //fn _control_value(&self) -> &ControlValue {
        //
    //}
//fn _mut_control_value(&mut self) -> &mut ControlValue {
        //
    //}
//}
//impl IsIdentifiedObject for RecloserControl {
    //fn _identified_object(&self) -> &IdentifiedObject {
        //
    //}
//fn _mut_identified_object(&mut self) -> &mut IdentifiedObject {
        //
    //}
//}
/// Pole-mounted fault interrupter with built-in phase and ground relays, current transformer (CT),
/// and supplemental controls.
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Recloser {
    /// UML inherited base object
    // parent_message: true
    // required_field: false
    // multiplicity_min: None
    // multiplicity_max: None
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
    fn _recloser_mut(&mut self) -> &mut Recloser;
    fn conducting_equipment(&self) -> &super::commonmodule::ConductingEquipment {
        self._recloser().conducting_equipment.as_ref().unwrap_or(&recloser::CONDUCTING_EQUIPMENT)
    }
    fn conducting_equipment_mut(&mut self) -> &mut super::commonmodule::ConductingEquipment {
        self._recloser_mut().conducting_equipment.get_or_insert(Default::default())
    }
    fn normal_open(&self) -> &bool {
        self._recloser().normal_open.as_ref().unwrap_or(&recloser::NORMAL_OPEN)
    }
    fn normal_open_mut(&mut self) -> &mut bool {
        self._recloser_mut().normal_open.get_or_insert(Default::default())
    }
}
impl IsRecloser for Recloser {
    fn _recloser(&self) -> &Recloser {
        self
    }
    fn _recloser_mut(&mut self) -> &mut Recloser {
        self
    }
}
//impl IsConductingEquipment for Recloser {
    //fn _conducting_equipment(&self) -> &ConductingEquipment {
        //
    //}
//fn _mut_conducting_equipment(&mut self) -> &mut ConductingEquipment {
        //
    //}
//}
//impl IsNamedObject for Recloser {
    //fn _named_object(&self) -> &NamedObject {
        //
    //}
//fn _mut_named_object(&mut self) -> &mut NamedObject {
        //
    //}
//}
/// Recloser control profile.  Instructs an end device (or an end device group) to perform a
/// specified action.
/// OpenFMB Profile Message: true
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct RecloserControlProfile {
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
    pub ied: ::std::option::Option<super::commonmodule::Ied>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: Some(1)
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, optional, tag="3")]
    pub recloser: ::std::option::Option<Recloser>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: Some(1)
    // multiplicity_max: None
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
    fn _recloser_control_profile_mut(&mut self) -> &mut RecloserControlProfile;
    fn control_message_info(&self) -> &super::commonmodule::ControlMessageInfo {
        self._recloser_control_profile().control_message_info.as_ref().unwrap_or(&recloser_control_profile::CONTROL_MESSAGE_INFO)
    }
    fn control_message_info_mut(&mut self) -> &mut super::commonmodule::ControlMessageInfo {
        self._recloser_control_profile_mut().control_message_info.get_or_insert(Default::default())
    }
    fn ied(&self) -> &super::commonmodule::Ied {
        self._recloser_control_profile().ied.as_ref().unwrap_or(&recloser_control_profile::IED)
    }
    fn ied_mut(&mut self) -> &mut super::commonmodule::Ied {
        self._recloser_control_profile_mut().ied.get_or_insert(Default::default())
    }
    fn recloser(&self) -> &Recloser {
        self._recloser_control_profile().recloser.as_ref().unwrap_or(&recloser_control_profile::RECLOSER)
    }
    fn recloser_mut(&mut self) -> &mut Recloser {
        self._recloser_control_profile_mut().recloser.get_or_insert(Default::default())
    }
    fn recloser_control(&self) -> &RecloserControl {
        self._recloser_control_profile().recloser_control.as_ref().unwrap_or(&recloser_control_profile::RECLOSER_CONTROL)
    }
    fn recloser_control_mut(&mut self) -> &mut RecloserControl {
        self._recloser_control_profile_mut().recloser_control.get_or_insert(Default::default())
    }
}
impl IsRecloserControlProfile for RecloserControlProfile {
    fn _recloser_control_profile(&self) -> &RecloserControlProfile {
        self
    }
    fn _recloser_control_profile_mut(&mut self) -> &mut RecloserControlProfile {
        self
    }
}
//impl IsControlMessageInfo for RecloserControlProfile {
    //fn _control_message_info(&self) -> &ControlMessageInfo {
        //
    //}
//fn _mut_control_message_info(&mut self) -> &mut ControlMessageInfo {
        //
    //}
//}
//impl IsMessageInfo for RecloserControlProfile {
    //fn _message_info(&self) -> &MessageInfo {
        //
    //}
//fn _mut_message_info(&mut self) -> &mut MessageInfo {
        //
    //}
//}
//impl IsIdentifiedObject for RecloserControlProfile {
    //fn _identified_object(&self) -> &IdentifiedObject {
        //
    //}
//fn _mut_identified_object(&mut self) -> &mut IdentifiedObject {
        //
    //}
//}
/// OpenFMB specialization for RecloserDiscreteControlProfile: Added blk  LN: Circuit
/// breaker   Name: XCBR
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct RecloserDiscreteControlXcbr {
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
    fn _recloser_discrete_control_xcbr_mut(&mut self) -> &mut RecloserDiscreteControlXcbr;
    fn logical_node_for_control(&self) -> &super::commonmodule::LogicalNodeForControl {
        self._recloser_discrete_control_xcbr().logical_node_for_control.as_ref().unwrap_or(&recloser_discrete_control_xcbr::LOGICAL_NODE_FOR_CONTROL)
    }
    fn logical_node_for_control_mut(&mut self) -> &mut super::commonmodule::LogicalNodeForControl {
        self._recloser_discrete_control_xcbr_mut().logical_node_for_control.get_or_insert(Default::default())
    }
    fn pos(&self) -> &super::commonmodule::ControlDpc {
        self._recloser_discrete_control_xcbr().pos.as_ref().unwrap_or(&recloser_discrete_control_xcbr::POS)
    }
    fn pos_mut(&mut self) -> &mut super::commonmodule::ControlDpc {
        self._recloser_discrete_control_xcbr_mut().pos.get_or_insert(Default::default())
    }
}
impl IsRecloserDiscreteControlXcbr for RecloserDiscreteControlXcbr {
    fn _recloser_discrete_control_xcbr(&self) -> &RecloserDiscreteControlXcbr {
        self
    }
    fn _recloser_discrete_control_xcbr_mut(&mut self) -> &mut RecloserDiscreteControlXcbr {
        self
    }
}
//impl IsLogicalNodeForControl for RecloserDiscreteControlXcbr {
    //fn _logical_node_for_control(&self) -> &LogicalNodeForControl {
        //
    //}
//fn _mut_logical_node_for_control(&mut self) -> &mut LogicalNodeForControl {
        //
    //}
//}
//impl IsLogicalNode for RecloserDiscreteControlXcbr {
    //fn _logical_node(&self) -> &LogicalNode {
        //
    //}
//fn _mut_logical_node(&mut self) -> &mut LogicalNode {
        //
    //}
//}
//impl IsIdentifiedObject for RecloserDiscreteControlXcbr {
    //fn _identified_object(&self) -> &IdentifiedObject {
        //
    //}
//fn _mut_identified_object(&mut self) -> &mut IdentifiedObject {
        //
    //}
//}
/// Recloser discrete control
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct RecloserDiscreteControl {
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
    // parent_message: false
    // required_field: true
    // multiplicity_min: Some(1)
    // multiplicity_max: None
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
    fn _recloser_discrete_control_mut(&mut self) -> &mut RecloserDiscreteControl;
    fn control_value(&self) -> &super::commonmodule::ControlValue {
        self._recloser_discrete_control().control_value.as_ref().unwrap_or(&recloser_discrete_control::CONTROL_VALUE)
    }
    fn control_value_mut(&mut self) -> &mut super::commonmodule::ControlValue {
        self._recloser_discrete_control_mut().control_value.get_or_insert(Default::default())
    }
    fn check(&self) -> &super::commonmodule::CheckConditions {
        self._recloser_discrete_control().check.as_ref().unwrap_or(&recloser_discrete_control::CHECK)
    }
    fn check_mut(&mut self) -> &mut super::commonmodule::CheckConditions {
        self._recloser_discrete_control_mut().check.get_or_insert(Default::default())
    }
    fn recloser_discrete_control_xcbr(&self) -> &RecloserDiscreteControlXcbr {
        self._recloser_discrete_control().recloser_discrete_control_xcbr.as_ref().unwrap_or(&recloser_discrete_control::RECLOSER_DISCRETE_CONTROL_XCBR)
    }
    fn recloser_discrete_control_xcbr_mut(&mut self) -> &mut RecloserDiscreteControlXcbr {
        self._recloser_discrete_control_mut().recloser_discrete_control_xcbr.get_or_insert(Default::default())
    }
}
impl IsRecloserDiscreteControl for RecloserDiscreteControl {
    fn _recloser_discrete_control(&self) -> &RecloserDiscreteControl {
        self
    }
    fn _recloser_discrete_control_mut(&mut self) -> &mut RecloserDiscreteControl {
        self
    }
}
//impl IsControlValue for RecloserDiscreteControl {
    //fn _control_value(&self) -> &ControlValue {
        //
    //}
//fn _mut_control_value(&mut self) -> &mut ControlValue {
        //
    //}
//}
//impl IsIdentifiedObject for RecloserDiscreteControl {
    //fn _identified_object(&self) -> &IdentifiedObject {
        //
    //}
//fn _mut_identified_object(&mut self) -> &mut IdentifiedObject {
        //
    //}
//}
/// Recloser control profile.  Instructs an end device (or an end device group) to perform a
/// specified action.
/// OpenFMB Profile Message: true
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct RecloserDiscreteControlProfile {
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
    pub ied: ::std::option::Option<super::commonmodule::Ied>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: Some(1)
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, optional, tag="3")]
    pub recloser: ::std::option::Option<Recloser>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: Some(1)
    // multiplicity_max: None
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
    fn _recloser_discrete_control_profile_mut(&mut self) -> &mut RecloserDiscreteControlProfile;
    fn control_message_info(&self) -> &super::commonmodule::ControlMessageInfo {
        self._recloser_discrete_control_profile().control_message_info.as_ref().unwrap_or(&recloser_discrete_control_profile::CONTROL_MESSAGE_INFO)
    }
    fn control_message_info_mut(&mut self) -> &mut super::commonmodule::ControlMessageInfo {
        self._recloser_discrete_control_profile_mut().control_message_info.get_or_insert(Default::default())
    }
    fn ied(&self) -> &super::commonmodule::Ied {
        self._recloser_discrete_control_profile().ied.as_ref().unwrap_or(&recloser_discrete_control_profile::IED)
    }
    fn ied_mut(&mut self) -> &mut super::commonmodule::Ied {
        self._recloser_discrete_control_profile_mut().ied.get_or_insert(Default::default())
    }
    fn recloser(&self) -> &Recloser {
        self._recloser_discrete_control_profile().recloser.as_ref().unwrap_or(&recloser_discrete_control_profile::RECLOSER)
    }
    fn recloser_mut(&mut self) -> &mut Recloser {
        self._recloser_discrete_control_profile_mut().recloser.get_or_insert(Default::default())
    }
    fn recloser_discrete_control(&self) -> &RecloserDiscreteControl {
        self._recloser_discrete_control_profile().recloser_discrete_control.as_ref().unwrap_or(&recloser_discrete_control_profile::RECLOSER_DISCRETE_CONTROL)
    }
    fn recloser_discrete_control_mut(&mut self) -> &mut RecloserDiscreteControl {
        self._recloser_discrete_control_profile_mut().recloser_discrete_control.get_or_insert(Default::default())
    }
}
impl IsRecloserDiscreteControlProfile for RecloserDiscreteControlProfile {
    fn _recloser_discrete_control_profile(&self) -> &RecloserDiscreteControlProfile {
        self
    }
    fn _recloser_discrete_control_profile_mut(&mut self) -> &mut RecloserDiscreteControlProfile {
        self
    }
}
//impl IsControlMessageInfo for RecloserDiscreteControlProfile {
    //fn _control_message_info(&self) -> &ControlMessageInfo {
        //
    //}
//fn _mut_control_message_info(&mut self) -> &mut ControlMessageInfo {
        //
    //}
//}
//impl IsMessageInfo for RecloserDiscreteControlProfile {
    //fn _message_info(&self) -> &MessageInfo {
        //
    //}
//fn _mut_message_info(&mut self) -> &mut MessageInfo {
        //
    //}
//}
//impl IsIdentifiedObject for RecloserDiscreteControlProfile {
    //fn _identified_object(&self) -> &IdentifiedObject {
        //
    //}
//fn _mut_identified_object(&mut self) -> &mut IdentifiedObject {
        //
    //}
//}
/// Recloser event
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct RecloserEvent {
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
    // parent_message: false
    // required_field: true
    // multiplicity_min: Some(1)
    // multiplicity_max: None
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
    fn _recloser_event_mut(&mut self) -> &mut RecloserEvent;
    fn event_value(&self) -> &super::commonmodule::EventValue {
        self._recloser_event().event_value.as_ref().unwrap_or(&recloser_event::EVENT_VALUE)
    }
    fn event_value_mut(&mut self) -> &mut super::commonmodule::EventValue {
        self._recloser_event_mut().event_value.get_or_insert(Default::default())
    }
    fn status_and_event_xcbr(&self) -> &super::commonmodule::StatusAndEventXcbr {
        self._recloser_event().status_and_event_xcbr.as_ref().unwrap_or(&recloser_event::STATUS_AND_EVENT_XCBR)
    }
    fn status_and_event_xcbr_mut(&mut self) -> &mut super::commonmodule::StatusAndEventXcbr {
        self._recloser_event_mut().status_and_event_xcbr.get_or_insert(Default::default())
    }
}
impl IsRecloserEvent for RecloserEvent {
    fn _recloser_event(&self) -> &RecloserEvent {
        self
    }
    fn _recloser_event_mut(&mut self) -> &mut RecloserEvent {
        self
    }
}
//impl IsEventValue for RecloserEvent {
    //fn _event_value(&self) -> &EventValue {
        //
    //}
//fn _mut_event_value(&mut self) -> &mut EventValue {
        //
    //}
//}
//impl IsIdentifiedObject for RecloserEvent {
    //fn _identified_object(&self) -> &IdentifiedObject {
        //
    //}
//fn _mut_identified_object(&mut self) -> &mut IdentifiedObject {
        //
    //}
//}
/// Recloser event profile
/// OpenFMB Profile Message: true
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct RecloserEventProfile {
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
    pub ied: ::std::option::Option<super::commonmodule::Ied>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: Some(1)
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, optional, tag="3")]
    pub recloser: ::std::option::Option<Recloser>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: Some(1)
    // multiplicity_max: None
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
    fn _recloser_event_profile_mut(&mut self) -> &mut RecloserEventProfile;
    fn event_message_info(&self) -> &super::commonmodule::EventMessageInfo {
        self._recloser_event_profile().event_message_info.as_ref().unwrap_or(&recloser_event_profile::EVENT_MESSAGE_INFO)
    }
    fn event_message_info_mut(&mut self) -> &mut super::commonmodule::EventMessageInfo {
        self._recloser_event_profile_mut().event_message_info.get_or_insert(Default::default())
    }
    fn ied(&self) -> &super::commonmodule::Ied {
        self._recloser_event_profile().ied.as_ref().unwrap_or(&recloser_event_profile::IED)
    }
    fn ied_mut(&mut self) -> &mut super::commonmodule::Ied {
        self._recloser_event_profile_mut().ied.get_or_insert(Default::default())
    }
    fn recloser(&self) -> &Recloser {
        self._recloser_event_profile().recloser.as_ref().unwrap_or(&recloser_event_profile::RECLOSER)
    }
    fn recloser_mut(&mut self) -> &mut Recloser {
        self._recloser_event_profile_mut().recloser.get_or_insert(Default::default())
    }
    fn recloser_event(&self) -> &RecloserEvent {
        self._recloser_event_profile().recloser_event.as_ref().unwrap_or(&recloser_event_profile::RECLOSER_EVENT)
    }
    fn recloser_event_mut(&mut self) -> &mut RecloserEvent {
        self._recloser_event_profile_mut().recloser_event.get_or_insert(Default::default())
    }
}
impl IsRecloserEventProfile for RecloserEventProfile {
    fn _recloser_event_profile(&self) -> &RecloserEventProfile {
        self
    }
    fn _recloser_event_profile_mut(&mut self) -> &mut RecloserEventProfile {
        self
    }
}
//impl IsEventMessageInfo for RecloserEventProfile {
    //fn _event_message_info(&self) -> &EventMessageInfo {
        //
    //}
//fn _mut_event_message_info(&mut self) -> &mut EventMessageInfo {
        //
    //}
//}
//impl IsMessageInfo for RecloserEventProfile {
    //fn _message_info(&self) -> &MessageInfo {
        //
    //}
//fn _mut_message_info(&mut self) -> &mut MessageInfo {
        //
    //}
//}
//impl IsIdentifiedObject for RecloserEventProfile {
    //fn _identified_object(&self) -> &IdentifiedObject {
        //
    //}
//fn _mut_identified_object(&mut self) -> &mut IdentifiedObject {
        //
    //}
//}
/// Recloser reading value
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct RecloserReading {
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
    fn _recloser_reading_mut(&mut self) -> &mut RecloserReading;
    fn conducting_equipment_terminal_reading(&self) -> &super::commonmodule::ConductingEquipmentTerminalReading {
        self._recloser_reading().conducting_equipment_terminal_reading.as_ref().unwrap_or(&recloser_reading::CONDUCTING_EQUIPMENT_TERMINAL_READING)
    }
    fn conducting_equipment_terminal_reading_mut(&mut self) -> &mut super::commonmodule::ConductingEquipmentTerminalReading {
        self._recloser_reading_mut().conducting_equipment_terminal_reading.get_or_insert(Default::default())
    }
    fn diff_reading_mmxu(&self) -> &super::commonmodule::ReadingMmxu {
        self._recloser_reading().diff_reading_mmxu.as_ref().unwrap_or(&recloser_reading::DIFF_READING_MMXU)
    }
    fn diff_reading_mmxu_mut(&mut self) -> &mut super::commonmodule::ReadingMmxu {
        self._recloser_reading_mut().diff_reading_mmxu.get_or_insert(Default::default())
    }
    fn phase_mmtn(&self) -> &super::commonmodule::PhaseMmtn {
        self._recloser_reading().phase_mmtn.as_ref().unwrap_or(&recloser_reading::PHASE_MMTN)
    }
    fn phase_mmtn_mut(&mut self) -> &mut super::commonmodule::PhaseMmtn {
        self._recloser_reading_mut().phase_mmtn.get_or_insert(Default::default())
    }
    fn reading_mmtr(&self) -> &super::commonmodule::ReadingMmtr {
        self._recloser_reading().reading_mmtr.as_ref().unwrap_or(&recloser_reading::READING_MMTR)
    }
    fn reading_mmtr_mut(&mut self) -> &mut super::commonmodule::ReadingMmtr {
        self._recloser_reading_mut().reading_mmtr.get_or_insert(Default::default())
    }
    fn reading_mmxu(&self) -> &super::commonmodule::ReadingMmxu {
        self._recloser_reading().reading_mmxu.as_ref().unwrap_or(&recloser_reading::READING_MMXU)
    }
    fn reading_mmxu_mut(&mut self) -> &mut super::commonmodule::ReadingMmxu {
        self._recloser_reading_mut().reading_mmxu.get_or_insert(Default::default())
    }
}
impl IsRecloserReading for RecloserReading {
    fn _recloser_reading(&self) -> &RecloserReading {
        self
    }
    fn _recloser_reading_mut(&mut self) -> &mut RecloserReading {
        self
    }
}
//impl IsConductingEquipmentTerminalReading for RecloserReading {
    //fn _conducting_equipment_terminal_reading(&self) -> &ConductingEquipmentTerminalReading {
        //
    //}
//fn _mut_conducting_equipment_terminal_reading(&mut self) -> &mut ConductingEquipmentTerminalReading {
        //
    //}
//}
/// Recloser reading profile
/// OpenFMB Profile Message: true
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct RecloserReadingProfile {
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
    pub ied: ::std::option::Option<super::commonmodule::Ied>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: Some(1)
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, optional, tag="3")]
    pub recloser: ::std::option::Option<Recloser>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: Some(1)
    // multiplicity_max: Some(2)
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
    fn _recloser_reading_profile_mut(&mut self) -> &mut RecloserReadingProfile;
    fn reading_message_info(&self) -> &super::commonmodule::ReadingMessageInfo {
        self._recloser_reading_profile().reading_message_info.as_ref().unwrap_or(&recloser_reading_profile::READING_MESSAGE_INFO)
    }
    fn reading_message_info_mut(&mut self) -> &mut super::commonmodule::ReadingMessageInfo {
        self._recloser_reading_profile_mut().reading_message_info.get_or_insert(Default::default())
    }
    fn ied(&self) -> &super::commonmodule::Ied {
        self._recloser_reading_profile().ied.as_ref().unwrap_or(&recloser_reading_profile::IED)
    }
    fn ied_mut(&mut self) -> &mut super::commonmodule::Ied {
        self._recloser_reading_profile_mut().ied.get_or_insert(Default::default())
    }
    fn recloser(&self) -> &Recloser {
        self._recloser_reading_profile().recloser.as_ref().unwrap_or(&recloser_reading_profile::RECLOSER)
    }
    fn recloser_mut(&mut self) -> &mut Recloser {
        self._recloser_reading_profile_mut().recloser.get_or_insert(Default::default())
    }
    fn recloser_reading(&self) -> &::std::vec::Vec<RecloserReading> {
        &self._recloser_reading_profile().recloser_reading
    }
    fn recloser_reading_mut(&mut self) -> &mut ::std::vec::Vec<RecloserReading> {
        &mut self._recloser_reading_profile_mut().recloser_reading
    }
}
impl IsRecloserReadingProfile for RecloserReadingProfile {
    fn _recloser_reading_profile(&self) -> &RecloserReadingProfile {
        self
    }
    fn _recloser_reading_profile_mut(&mut self) -> &mut RecloserReadingProfile {
        self
    }
}
//impl IsReadingMessageInfo for RecloserReadingProfile {
    //fn _reading_message_info(&self) -> &ReadingMessageInfo {
        //
    //}
//fn _mut_reading_message_info(&mut self) -> &mut ReadingMessageInfo {
        //
    //}
//}
//impl IsMessageInfo for RecloserReadingProfile {
    //fn _message_info(&self) -> &MessageInfo {
        //
    //}
//fn _mut_message_info(&mut self) -> &mut MessageInfo {
        //
    //}
//}
//impl IsIdentifiedObject for RecloserReadingProfile {
    //fn _identified_object(&self) -> &IdentifiedObject {
        //
    //}
//fn _mut_identified_object(&mut self) -> &mut IdentifiedObject {
        //
    //}
//}
/// Recloser status
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct RecloserStatus {
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
    // parent_message: false
    // required_field: true
    // multiplicity_min: Some(1)
    // multiplicity_max: None
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
    fn _recloser_status_mut(&mut self) -> &mut RecloserStatus;
    fn status_value(&self) -> &super::commonmodule::StatusValue {
        self._recloser_status().status_value.as_ref().unwrap_or(&recloser_status::STATUS_VALUE)
    }
    fn status_value_mut(&mut self) -> &mut super::commonmodule::StatusValue {
        self._recloser_status_mut().status_value.get_or_insert(Default::default())
    }
    fn status_and_event_xcbr(&self) -> &super::commonmodule::StatusAndEventXcbr {
        self._recloser_status().status_and_event_xcbr.as_ref().unwrap_or(&recloser_status::STATUS_AND_EVENT_XCBR)
    }
    fn status_and_event_xcbr_mut(&mut self) -> &mut super::commonmodule::StatusAndEventXcbr {
        self._recloser_status_mut().status_and_event_xcbr.get_or_insert(Default::default())
    }
}
impl IsRecloserStatus for RecloserStatus {
    fn _recloser_status(&self) -> &RecloserStatus {
        self
    }
    fn _recloser_status_mut(&mut self) -> &mut RecloserStatus {
        self
    }
}
//impl IsStatusValue for RecloserStatus {
    //fn _status_value(&self) -> &StatusValue {
        //
    //}
//fn _mut_status_value(&mut self) -> &mut StatusValue {
        //
    //}
//}
//impl IsIdentifiedObject for RecloserStatus {
    //fn _identified_object(&self) -> &IdentifiedObject {
        //
    //}
//fn _mut_identified_object(&mut self) -> &mut IdentifiedObject {
        //
    //}
//}
/// Recloser status profile
/// OpenFMB Profile Message: true
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct RecloserStatusProfile {
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
    pub ied: ::std::option::Option<super::commonmodule::Ied>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: Some(1)
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, optional, tag="3")]
    pub recloser: ::std::option::Option<Recloser>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: Some(1)
    // multiplicity_max: None
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
    fn _recloser_status_profile_mut(&mut self) -> &mut RecloserStatusProfile;
    fn status_message_info(&self) -> &super::commonmodule::StatusMessageInfo {
        self._recloser_status_profile().status_message_info.as_ref().unwrap_or(&recloser_status_profile::STATUS_MESSAGE_INFO)
    }
    fn status_message_info_mut(&mut self) -> &mut super::commonmodule::StatusMessageInfo {
        self._recloser_status_profile_mut().status_message_info.get_or_insert(Default::default())
    }
    fn ied(&self) -> &super::commonmodule::Ied {
        self._recloser_status_profile().ied.as_ref().unwrap_or(&recloser_status_profile::IED)
    }
    fn ied_mut(&mut self) -> &mut super::commonmodule::Ied {
        self._recloser_status_profile_mut().ied.get_or_insert(Default::default())
    }
    fn recloser(&self) -> &Recloser {
        self._recloser_status_profile().recloser.as_ref().unwrap_or(&recloser_status_profile::RECLOSER)
    }
    fn recloser_mut(&mut self) -> &mut Recloser {
        self._recloser_status_profile_mut().recloser.get_or_insert(Default::default())
    }
    fn recloser_status(&self) -> &RecloserStatus {
        self._recloser_status_profile().recloser_status.as_ref().unwrap_or(&recloser_status_profile::RECLOSER_STATUS)
    }
    fn recloser_status_mut(&mut self) -> &mut RecloserStatus {
        self._recloser_status_profile_mut().recloser_status.get_or_insert(Default::default())
    }
}
impl IsRecloserStatusProfile for RecloserStatusProfile {
    fn _recloser_status_profile(&self) -> &RecloserStatusProfile {
        self
    }
    fn _recloser_status_profile_mut(&mut self) -> &mut RecloserStatusProfile {
        self
    }
}
//impl IsStatusMessageInfo for RecloserStatusProfile {
    //fn _status_message_info(&self) -> &StatusMessageInfo {
        //
    //}
//fn _mut_status_message_info(&mut self) -> &mut StatusMessageInfo {
        //
    //}
//}
//impl IsMessageInfo for RecloserStatusProfile {
    //fn _message_info(&self) -> &MessageInfo {
        //
    //}
//fn _mut_message_info(&mut self) -> &mut MessageInfo {
        //
    //}
//}
//impl IsIdentifiedObject for RecloserStatusProfile {
    //fn _identified_object(&self) -> &IdentifiedObject {
        //
    //}
//fn _mut_identified_object(&mut self) -> &mut IdentifiedObject {
        //
    //}
//}
