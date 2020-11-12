use crate::commonmodule::*;
/// Cap bank compensator system
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct CapBankSystem {
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
mod cap_bank_system {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref CONDUCTING_EQUIPMENT: crate::commonmodule::ConductingEquipment = Default::default();
    }
}
impl CapBankSystem {
    pub(crate) fn parent(&self) -> &super::commonmodule::ConductingEquipment {
        self.conducting_equipment.as_ref().unwrap_or(&cap_bank_system::CONDUCTING_EQUIPMENT)
    }
    pub(crate) fn parent_mut(&mut self) -> &mut super::commonmodule::ConductingEquipment {
        self._cap_bank_system_mut().conducting_equipment.get_or_insert(Default::default())
    }
}
pub trait IsCapBankSystem {
    fn _cap_bank_system(&self) -> &CapBankSystem;
    fn _cap_bank_system_mut(&mut self) -> &mut CapBankSystem;
    fn conducting_equipment(&self) -> &super::commonmodule::ConductingEquipment {
        self._cap_bank_system().conducting_equipment.as_ref().unwrap_or(&cap_bank_system::CONDUCTING_EQUIPMENT)
    }
    fn conducting_equipment_mut(&mut self) -> &mut super::commonmodule::ConductingEquipment {
        self._cap_bank_system_mut().conducting_equipment.get_or_insert(Default::default())
    }
}
impl IsCapBankSystem for CapBankSystem {
    fn _cap_bank_system(&self) -> &CapBankSystem {
        self
    }
    fn _cap_bank_system_mut(&mut self) -> &mut CapBankSystem {
        self
    }
}
impl IsConductingEquipment for CapBankSystem {
    fn _conducting_equipment(&self) -> &super::commonmodule::ConductingEquipment {
        self.parent()
    }
    fn _conducting_equipment_mut(&mut self) -> &mut ConductingEquipment {
        self.parent_mut()
    }
}
impl IsNamedObject for CapBankSystem {
    fn _named_object(&self) -> &super::commonmodule::NamedObject {
        self.parent().parent()
    }
    fn _named_object_mut(&mut self) -> &mut NamedObject {
        self.parent_mut().parent_mut()
    }
}
/// LN: Power cap bank  Name: YPSH
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct CapBankControlYpsh {
    /// (controllable) Position of the switch of power shunt.
    #[prost(message, optional, tag="1")]
    pub pos: ::std::option::Option<super::commonmodule::PhaseDpc>,
}
mod cap_bank_control_ypsh {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref POS: crate::commonmodule::PhaseDpc = Default::default();
    }
}
impl CapBankControlYpsh {
}
pub trait IsCapBankControlYpsh {
    fn _cap_bank_control_ypsh(&self) -> &CapBankControlYpsh;
    fn _cap_bank_control_ypsh_mut(&mut self) -> &mut CapBankControlYpsh;
    fn pos(&self) -> &super::commonmodule::PhaseDpc {
        self._cap_bank_control_ypsh().pos.as_ref().unwrap_or(&cap_bank_control_ypsh::POS)
    }
    fn pos_mut(&mut self) -> &mut super::commonmodule::PhaseDpc {
        self._cap_bank_control_ypsh_mut().pos.get_or_insert(Default::default())
    }
}
impl IsCapBankControlYpsh for CapBankControlYpsh {
    fn _cap_bank_control_ypsh(&self) -> &CapBankControlYpsh {
        self
    }
    fn _cap_bank_control_ypsh_mut(&mut self) -> &mut CapBankControlYpsh {
        self
    }
}
/// Point definition (Point)
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct CapBankPoint {
    /// Regulator control
    #[prost(message, optional, tag="1")]
    pub control: ::std::option::Option<CapBankControlYpsh>,
    /// Start time
    // parent_message: false
    // required_field: true
    // multiplicity_min: Some(1)
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, optional, tag="2")]
    pub start_time: ::std::option::Option<super::commonmodule::Timestamp>,
}
mod cap_bank_point {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref CONTROL: crate::capbankmodule::CapBankControlYpsh = Default::default();
        pub(super) static ref START_TIME: crate::commonmodule::Timestamp = Default::default();
    }
}
impl CapBankPoint {
}
pub trait IsCapBankPoint {
    fn _cap_bank_point(&self) -> &CapBankPoint;
    fn _cap_bank_point_mut(&mut self) -> &mut CapBankPoint;
    fn control(&self) -> &CapBankControlYpsh {
        self._cap_bank_point().control.as_ref().unwrap_or(&cap_bank_point::CONTROL)
    }
    fn control_mut(&mut self) -> &mut CapBankControlYpsh {
        self._cap_bank_point_mut().control.get_or_insert(Default::default())
    }
    fn start_time(&self) -> &super::commonmodule::Timestamp {
        self._cap_bank_point().start_time.as_ref().unwrap_or(&cap_bank_point::START_TIME)
    }
    fn start_time_mut(&mut self) -> &mut super::commonmodule::Timestamp {
        self._cap_bank_point_mut().start_time.get_or_insert(Default::default())
    }
}
impl IsCapBankPoint for CapBankPoint {
    fn _cap_bank_point(&self) -> &CapBankPoint {
        self
    }
    fn _cap_bank_point_mut(&mut self) -> &mut CapBankPoint {
        self
    }
}
/// Curve shape setting (FC=SP) (CSG_SP)
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct CapBankCsg {
    /// The array with the points specifying a curve shape.
    // parent_message: false
    // required_field: true
    // multiplicity_min: Some(1)
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, repeated, tag="1")]
    pub crv_pts: ::std::vec::Vec<CapBankPoint>,
}
mod cap_bank_csg {
    use lazy_static::lazy_static;
    lazy_static! {
    }
}
impl CapBankCsg {
}
pub trait IsCapBankCsg {
    fn _cap_bank_csg(&self) -> &CapBankCsg;
    fn _cap_bank_csg_mut(&mut self) -> &mut CapBankCsg;
    fn crv_pts(&self) -> &::std::vec::Vec<CapBankPoint> {
        &self._cap_bank_csg().crv_pts
    }
    fn crv_pts_mut(&mut self) -> &mut ::std::vec::Vec<CapBankPoint> {
        &mut self._cap_bank_csg_mut().crv_pts
    }
}
impl IsCapBankCsg for CapBankCsg {
    fn _cap_bank_csg(&self) -> &CapBankCsg {
        self
    }
    fn _cap_bank_csg_mut(&mut self) -> &mut CapBankCsg {
        self
    }
}
/// OpenFMB specialization for control schedule using:  LN: Schedule   Name: FSCH
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct CapBankControlScheduleFsch {
    /// Control value in CSG type
    // parent_message: false
    // required_field: true
    // multiplicity_min: Some(1)
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, optional, tag="1")]
    pub val_csg: ::std::option::Option<CapBankCsg>,
}
mod cap_bank_control_schedule_fsch {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref VAL_CSG: crate::capbankmodule::CapBankCsg = Default::default();
    }
}
impl CapBankControlScheduleFsch {
}
pub trait IsCapBankControlScheduleFsch {
    fn _cap_bank_control_schedule_fsch(&self) -> &CapBankControlScheduleFsch;
    fn _cap_bank_control_schedule_fsch_mut(&mut self) -> &mut CapBankControlScheduleFsch;
    fn val_csg(&self) -> &CapBankCsg {
        self._cap_bank_control_schedule_fsch().val_csg.as_ref().unwrap_or(&cap_bank_control_schedule_fsch::VAL_CSG)
    }
    fn val_csg_mut(&mut self) -> &mut CapBankCsg {
        self._cap_bank_control_schedule_fsch_mut().val_csg.get_or_insert(Default::default())
    }
}
impl IsCapBankControlScheduleFsch for CapBankControlScheduleFsch {
    fn _cap_bank_control_schedule_fsch(&self) -> &CapBankControlScheduleFsch {
        self
    }
    fn _cap_bank_control_schedule_fsch_mut(&mut self) -> &mut CapBankControlScheduleFsch {
        self
    }
}
/// Using 61850 FSCC for cap bank control
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct CapBankControlFscc {
    /// UML inherited base object
    // parent_message: true
    // required_field: false
    // multiplicity_min: None
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, optional, tag="1")]
    pub control_fscc: ::std::option::Option<super::commonmodule::ControlFscc>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub cap_bank_control_schedule_fsch: ::std::option::Option<CapBankControlScheduleFsch>,
}
mod cap_bank_control_fscc {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref CONTROL_FSCC: crate::commonmodule::ControlFscc = Default::default();
        pub(super) static ref CAP_BANK_CONTROL_SCHEDULE_FSCH: crate::capbankmodule::CapBankControlScheduleFsch = Default::default();
    }
}
impl CapBankControlFscc {
    pub(crate) fn parent(&self) -> &super::commonmodule::ControlFscc {
        self.control_fscc.as_ref().unwrap_or(&cap_bank_control_fscc::CONTROL_FSCC)
    }
    pub(crate) fn parent_mut(&mut self) -> &mut super::commonmodule::ControlFscc {
        self._cap_bank_control_fscc_mut().control_fscc.get_or_insert(Default::default())
    }
}
pub trait IsCapBankControlFscc {
    fn _cap_bank_control_fscc(&self) -> &CapBankControlFscc;
    fn _cap_bank_control_fscc_mut(&mut self) -> &mut CapBankControlFscc;
    fn control_fscc(&self) -> &super::commonmodule::ControlFscc {
        self._cap_bank_control_fscc().control_fscc.as_ref().unwrap_or(&cap_bank_control_fscc::CONTROL_FSCC)
    }
    fn control_fscc_mut(&mut self) -> &mut super::commonmodule::ControlFscc {
        self._cap_bank_control_fscc_mut().control_fscc.get_or_insert(Default::default())
    }
    fn cap_bank_control_schedule_fsch(&self) -> &CapBankControlScheduleFsch {
        self._cap_bank_control_fscc().cap_bank_control_schedule_fsch.as_ref().unwrap_or(&cap_bank_control_fscc::CAP_BANK_CONTROL_SCHEDULE_FSCH)
    }
    fn cap_bank_control_schedule_fsch_mut(&mut self) -> &mut CapBankControlScheduleFsch {
        self._cap_bank_control_fscc_mut().cap_bank_control_schedule_fsch.get_or_insert(Default::default())
    }
}
impl IsCapBankControlFscc for CapBankControlFscc {
    fn _cap_bank_control_fscc(&self) -> &CapBankControlFscc {
        self
    }
    fn _cap_bank_control_fscc_mut(&mut self) -> &mut CapBankControlFscc {
        self
    }
}
impl IsControlFscc for CapBankControlFscc {
    fn _control_fscc(&self) -> &super::commonmodule::ControlFscc {
        self.parent()
    }
    fn _control_fscc_mut(&mut self) -> &mut ControlFscc {
        self.parent_mut()
    }
}
impl IsLogicalNodeForControl for CapBankControlFscc {
    fn _logical_node_for_control(&self) -> &super::commonmodule::LogicalNodeForControl {
        self.parent().parent()
    }
    fn _logical_node_for_control_mut(&mut self) -> &mut LogicalNodeForControl {
        self.parent_mut().parent_mut()
    }
}
impl IsLogicalNode for CapBankControlFscc {
    fn _logical_node(&self) -> &super::commonmodule::LogicalNode {
        self.parent().parent().parent()
    }
    fn _logical_node_mut(&mut self) -> &mut LogicalNode {
        self.parent_mut().parent_mut().parent_mut()
    }
}
impl IsIdentifiedObject for CapBankControlFscc {
    fn _identified_object(&self) -> &super::commonmodule::IdentifiedObject {
        self.parent().parent().parent().parent()
    }
    fn _identified_object_mut(&mut self) -> &mut IdentifiedObject {
        self.parent_mut().parent_mut().parent_mut().parent_mut()
    }
}
/// CapBank control
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct CapBankControl {
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
    pub cap_bank_control_fscc: ::std::option::Option<CapBankControlFscc>,
}
mod cap_bank_control {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref CONTROL_VALUE: crate::commonmodule::ControlValue = Default::default();
        pub(super) static ref CHECK: crate::commonmodule::CheckConditions = Default::default();
        pub(super) static ref CAP_BANK_CONTROL_FSCC: crate::capbankmodule::CapBankControlFscc = Default::default();
    }
}
impl CapBankControl {
    pub(crate) fn parent(&self) -> &super::commonmodule::ControlValue {
        self.control_value.as_ref().unwrap_or(&cap_bank_control::CONTROL_VALUE)
    }
    pub(crate) fn parent_mut(&mut self) -> &mut super::commonmodule::ControlValue {
        self._cap_bank_control_mut().control_value.get_or_insert(Default::default())
    }
}
pub trait IsCapBankControl {
    fn _cap_bank_control(&self) -> &CapBankControl;
    fn _cap_bank_control_mut(&mut self) -> &mut CapBankControl;
    fn control_value(&self) -> &super::commonmodule::ControlValue {
        self._cap_bank_control().control_value.as_ref().unwrap_or(&cap_bank_control::CONTROL_VALUE)
    }
    fn control_value_mut(&mut self) -> &mut super::commonmodule::ControlValue {
        self._cap_bank_control_mut().control_value.get_or_insert(Default::default())
    }
    fn check(&self) -> &super::commonmodule::CheckConditions {
        self._cap_bank_control().check.as_ref().unwrap_or(&cap_bank_control::CHECK)
    }
    fn check_mut(&mut self) -> &mut super::commonmodule::CheckConditions {
        self._cap_bank_control_mut().check.get_or_insert(Default::default())
    }
    fn cap_bank_control_fscc(&self) -> &CapBankControlFscc {
        self._cap_bank_control().cap_bank_control_fscc.as_ref().unwrap_or(&cap_bank_control::CAP_BANK_CONTROL_FSCC)
    }
    fn cap_bank_control_fscc_mut(&mut self) -> &mut CapBankControlFscc {
        self._cap_bank_control_mut().cap_bank_control_fscc.get_or_insert(Default::default())
    }
}
impl IsCapBankControl for CapBankControl {
    fn _cap_bank_control(&self) -> &CapBankControl {
        self
    }
    fn _cap_bank_control_mut(&mut self) -> &mut CapBankControl {
        self
    }
}
impl IsControlValue for CapBankControl {
    fn _control_value(&self) -> &super::commonmodule::ControlValue {
        self.parent()
    }
    fn _control_value_mut(&mut self) -> &mut ControlValue {
        self.parent_mut()
    }
}
impl IsIdentifiedObject for CapBankControl {
    fn _identified_object(&self) -> &super::commonmodule::IdentifiedObject {
        self.parent().parent()
    }
    fn _identified_object_mut(&mut self) -> &mut IdentifiedObject {
        self.parent_mut().parent_mut()
    }
}
/// Cap bank control profile.  Instructs an end device (or an end device group) to perform a
/// specified action.
/// OpenFMB Profile Message: true
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct CapBankControlProfile {
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
    pub cap_bank_control: ::std::option::Option<CapBankControl>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: Some(1)
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, optional, tag="3")]
    pub cap_bank_system: ::std::option::Option<CapBankSystem>,
}
mod cap_bank_control_profile {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref CONTROL_MESSAGE_INFO: crate::commonmodule::ControlMessageInfo = Default::default();
        pub(super) static ref CAP_BANK_CONTROL: crate::capbankmodule::CapBankControl = Default::default();
        pub(super) static ref CAP_BANK_SYSTEM: crate::capbankmodule::CapBankSystem = Default::default();
    }
}
impl CapBankControlProfile {
    pub(crate) fn parent(&self) -> &super::commonmodule::ControlMessageInfo {
        self.control_message_info.as_ref().unwrap_or(&cap_bank_control_profile::CONTROL_MESSAGE_INFO)
    }
    pub(crate) fn parent_mut(&mut self) -> &mut super::commonmodule::ControlMessageInfo {
        self._cap_bank_control_profile_mut().control_message_info.get_or_insert(Default::default())
    }
}
pub trait IsCapBankControlProfile {
    fn _cap_bank_control_profile(&self) -> &CapBankControlProfile;
    fn _cap_bank_control_profile_mut(&mut self) -> &mut CapBankControlProfile;
    fn control_message_info(&self) -> &super::commonmodule::ControlMessageInfo {
        self._cap_bank_control_profile().control_message_info.as_ref().unwrap_or(&cap_bank_control_profile::CONTROL_MESSAGE_INFO)
    }
    fn control_message_info_mut(&mut self) -> &mut super::commonmodule::ControlMessageInfo {
        self._cap_bank_control_profile_mut().control_message_info.get_or_insert(Default::default())
    }
    fn cap_bank_control(&self) -> &CapBankControl {
        self._cap_bank_control_profile().cap_bank_control.as_ref().unwrap_or(&cap_bank_control_profile::CAP_BANK_CONTROL)
    }
    fn cap_bank_control_mut(&mut self) -> &mut CapBankControl {
        self._cap_bank_control_profile_mut().cap_bank_control.get_or_insert(Default::default())
    }
    fn cap_bank_system(&self) -> &CapBankSystem {
        self._cap_bank_control_profile().cap_bank_system.as_ref().unwrap_or(&cap_bank_control_profile::CAP_BANK_SYSTEM)
    }
    fn cap_bank_system_mut(&mut self) -> &mut CapBankSystem {
        self._cap_bank_control_profile_mut().cap_bank_system.get_or_insert(Default::default())
    }
}
impl IsCapBankControlProfile for CapBankControlProfile {
    fn _cap_bank_control_profile(&self) -> &CapBankControlProfile {
        self
    }
    fn _cap_bank_control_profile_mut(&mut self) -> &mut CapBankControlProfile {
        self
    }
}
impl IsControlMessageInfo for CapBankControlProfile {
    fn _control_message_info(&self) -> &super::commonmodule::ControlMessageInfo {
        self.parent()
    }
    fn _control_message_info_mut(&mut self) -> &mut ControlMessageInfo {
        self.parent_mut()
    }
}
impl IsMessageInfo for CapBankControlProfile {
    fn _message_info(&self) -> &super::commonmodule::MessageInfo {
        self.parent().parent()
    }
    fn _message_info_mut(&mut self) -> &mut MessageInfo {
        self.parent_mut().parent_mut()
    }
}
impl IsIdentifiedObject for CapBankControlProfile {
    fn _identified_object(&self) -> &super::commonmodule::IdentifiedObject {
        self.parent().parent().parent()
    }
    fn _identified_object_mut(&mut self) -> &mut IdentifiedObject {
        self.parent_mut().parent_mut().parent_mut()
    }
}
/// OpenFMB specialization for cap bank discrete control:
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct CapBankDiscreteControlYpsh {
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
}
mod cap_bank_discrete_control_ypsh {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref LOGICAL_NODE_FOR_CONTROL: crate::commonmodule::LogicalNodeForControl = Default::default();
        pub(super) static ref POS: crate::commonmodule::PhaseDpc = Default::default();
    }
}
impl CapBankDiscreteControlYpsh {
    pub(crate) fn parent(&self) -> &super::commonmodule::LogicalNodeForControl {
        self.logical_node_for_control.as_ref().unwrap_or(&cap_bank_discrete_control_ypsh::LOGICAL_NODE_FOR_CONTROL)
    }
    pub(crate) fn parent_mut(&mut self) -> &mut super::commonmodule::LogicalNodeForControl {
        self._cap_bank_discrete_control_ypsh_mut().logical_node_for_control.get_or_insert(Default::default())
    }
}
pub trait IsCapBankDiscreteControlYpsh {
    fn _cap_bank_discrete_control_ypsh(&self) -> &CapBankDiscreteControlYpsh;
    fn _cap_bank_discrete_control_ypsh_mut(&mut self) -> &mut CapBankDiscreteControlYpsh;
    fn logical_node_for_control(&self) -> &super::commonmodule::LogicalNodeForControl {
        self._cap_bank_discrete_control_ypsh().logical_node_for_control.as_ref().unwrap_or(&cap_bank_discrete_control_ypsh::LOGICAL_NODE_FOR_CONTROL)
    }
    fn logical_node_for_control_mut(&mut self) -> &mut super::commonmodule::LogicalNodeForControl {
        self._cap_bank_discrete_control_ypsh_mut().logical_node_for_control.get_or_insert(Default::default())
    }
    fn pos(&self) -> &super::commonmodule::PhaseDpc {
        self._cap_bank_discrete_control_ypsh().pos.as_ref().unwrap_or(&cap_bank_discrete_control_ypsh::POS)
    }
    fn pos_mut(&mut self) -> &mut super::commonmodule::PhaseDpc {
        self._cap_bank_discrete_control_ypsh_mut().pos.get_or_insert(Default::default())
    }
}
impl IsCapBankDiscreteControlYpsh for CapBankDiscreteControlYpsh {
    fn _cap_bank_discrete_control_ypsh(&self) -> &CapBankDiscreteControlYpsh {
        self
    }
    fn _cap_bank_discrete_control_ypsh_mut(&mut self) -> &mut CapBankDiscreteControlYpsh {
        self
    }
}
impl IsLogicalNodeForControl for CapBankDiscreteControlYpsh {
    fn _logical_node_for_control(&self) -> &super::commonmodule::LogicalNodeForControl {
        self.parent()
    }
    fn _logical_node_for_control_mut(&mut self) -> &mut LogicalNodeForControl {
        self.parent_mut()
    }
}
impl IsLogicalNode for CapBankDiscreteControlYpsh {
    fn _logical_node(&self) -> &super::commonmodule::LogicalNode {
        self.parent().parent()
    }
    fn _logical_node_mut(&mut self) -> &mut LogicalNode {
        self.parent_mut().parent_mut()
    }
}
impl IsIdentifiedObject for CapBankDiscreteControlYpsh {
    fn _identified_object(&self) -> &super::commonmodule::IdentifiedObject {
        self.parent().parent().parent()
    }
    fn _identified_object_mut(&mut self) -> &mut IdentifiedObject {
        self.parent_mut().parent_mut().parent_mut()
    }
}
/// Cap bank discrete control
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct CapBankDiscreteControl {
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
    pub cap_bank_discrete_control_ypsh: ::std::option::Option<CapBankDiscreteControlYpsh>,
}
mod cap_bank_discrete_control {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref CONTROL_VALUE: crate::commonmodule::ControlValue = Default::default();
        pub(super) static ref CHECK: crate::commonmodule::CheckConditions = Default::default();
        pub(super) static ref CAP_BANK_DISCRETE_CONTROL_YPSH: crate::capbankmodule::CapBankDiscreteControlYpsh = Default::default();
    }
}
impl CapBankDiscreteControl {
    pub(crate) fn parent(&self) -> &super::commonmodule::ControlValue {
        self.control_value.as_ref().unwrap_or(&cap_bank_discrete_control::CONTROL_VALUE)
    }
    pub(crate) fn parent_mut(&mut self) -> &mut super::commonmodule::ControlValue {
        self._cap_bank_discrete_control_mut().control_value.get_or_insert(Default::default())
    }
}
pub trait IsCapBankDiscreteControl {
    fn _cap_bank_discrete_control(&self) -> &CapBankDiscreteControl;
    fn _cap_bank_discrete_control_mut(&mut self) -> &mut CapBankDiscreteControl;
    fn control_value(&self) -> &super::commonmodule::ControlValue {
        self._cap_bank_discrete_control().control_value.as_ref().unwrap_or(&cap_bank_discrete_control::CONTROL_VALUE)
    }
    fn control_value_mut(&mut self) -> &mut super::commonmodule::ControlValue {
        self._cap_bank_discrete_control_mut().control_value.get_or_insert(Default::default())
    }
    fn check(&self) -> &super::commonmodule::CheckConditions {
        self._cap_bank_discrete_control().check.as_ref().unwrap_or(&cap_bank_discrete_control::CHECK)
    }
    fn check_mut(&mut self) -> &mut super::commonmodule::CheckConditions {
        self._cap_bank_discrete_control_mut().check.get_or_insert(Default::default())
    }
    fn cap_bank_discrete_control_ypsh(&self) -> &CapBankDiscreteControlYpsh {
        self._cap_bank_discrete_control().cap_bank_discrete_control_ypsh.as_ref().unwrap_or(&cap_bank_discrete_control::CAP_BANK_DISCRETE_CONTROL_YPSH)
    }
    fn cap_bank_discrete_control_ypsh_mut(&mut self) -> &mut CapBankDiscreteControlYpsh {
        self._cap_bank_discrete_control_mut().cap_bank_discrete_control_ypsh.get_or_insert(Default::default())
    }
}
impl IsCapBankDiscreteControl for CapBankDiscreteControl {
    fn _cap_bank_discrete_control(&self) -> &CapBankDiscreteControl {
        self
    }
    fn _cap_bank_discrete_control_mut(&mut self) -> &mut CapBankDiscreteControl {
        self
    }
}
impl IsControlValue for CapBankDiscreteControl {
    fn _control_value(&self) -> &super::commonmodule::ControlValue {
        self.parent()
    }
    fn _control_value_mut(&mut self) -> &mut ControlValue {
        self.parent_mut()
    }
}
impl IsIdentifiedObject for CapBankDiscreteControl {
    fn _identified_object(&self) -> &super::commonmodule::IdentifiedObject {
        self.parent().parent()
    }
    fn _identified_object_mut(&mut self) -> &mut IdentifiedObject {
        self.parent_mut().parent_mut()
    }
}
/// Cap bank discrete control profile.  Instructs an end device (or an end device group) to perform
/// a specified action.
/// OpenFMB Profile Message: true
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct CapBankDiscreteControlProfile {
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
    pub cap_bank_control: ::std::option::Option<CapBankDiscreteControl>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: Some(1)
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, optional, tag="3")]
    pub cap_bank_system: ::std::option::Option<CapBankSystem>,
}
mod cap_bank_discrete_control_profile {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref CONTROL_MESSAGE_INFO: crate::commonmodule::ControlMessageInfo = Default::default();
        pub(super) static ref CAP_BANK_CONTROL: crate::capbankmodule::CapBankDiscreteControl = Default::default();
        pub(super) static ref CAP_BANK_SYSTEM: crate::capbankmodule::CapBankSystem = Default::default();
    }
}
impl CapBankDiscreteControlProfile {
    pub(crate) fn parent(&self) -> &super::commonmodule::ControlMessageInfo {
        self.control_message_info.as_ref().unwrap_or(&cap_bank_discrete_control_profile::CONTROL_MESSAGE_INFO)
    }
    pub(crate) fn parent_mut(&mut self) -> &mut super::commonmodule::ControlMessageInfo {
        self._cap_bank_discrete_control_profile_mut().control_message_info.get_or_insert(Default::default())
    }
}
pub trait IsCapBankDiscreteControlProfile {
    fn _cap_bank_discrete_control_profile(&self) -> &CapBankDiscreteControlProfile;
    fn _cap_bank_discrete_control_profile_mut(&mut self) -> &mut CapBankDiscreteControlProfile;
    fn control_message_info(&self) -> &super::commonmodule::ControlMessageInfo {
        self._cap_bank_discrete_control_profile().control_message_info.as_ref().unwrap_or(&cap_bank_discrete_control_profile::CONTROL_MESSAGE_INFO)
    }
    fn control_message_info_mut(&mut self) -> &mut super::commonmodule::ControlMessageInfo {
        self._cap_bank_discrete_control_profile_mut().control_message_info.get_or_insert(Default::default())
    }
    fn cap_bank_control(&self) -> &CapBankDiscreteControl {
        self._cap_bank_discrete_control_profile().cap_bank_control.as_ref().unwrap_or(&cap_bank_discrete_control_profile::CAP_BANK_CONTROL)
    }
    fn cap_bank_control_mut(&mut self) -> &mut CapBankDiscreteControl {
        self._cap_bank_discrete_control_profile_mut().cap_bank_control.get_or_insert(Default::default())
    }
    fn cap_bank_system(&self) -> &CapBankSystem {
        self._cap_bank_discrete_control_profile().cap_bank_system.as_ref().unwrap_or(&cap_bank_discrete_control_profile::CAP_BANK_SYSTEM)
    }
    fn cap_bank_system_mut(&mut self) -> &mut CapBankSystem {
        self._cap_bank_discrete_control_profile_mut().cap_bank_system.get_or_insert(Default::default())
    }
}
impl IsCapBankDiscreteControlProfile for CapBankDiscreteControlProfile {
    fn _cap_bank_discrete_control_profile(&self) -> &CapBankDiscreteControlProfile {
        self
    }
    fn _cap_bank_discrete_control_profile_mut(&mut self) -> &mut CapBankDiscreteControlProfile {
        self
    }
}
impl IsControlMessageInfo for CapBankDiscreteControlProfile {
    fn _control_message_info(&self) -> &super::commonmodule::ControlMessageInfo {
        self.parent()
    }
    fn _control_message_info_mut(&mut self) -> &mut ControlMessageInfo {
        self.parent_mut()
    }
}
impl IsMessageInfo for CapBankDiscreteControlProfile {
    fn _message_info(&self) -> &super::commonmodule::MessageInfo {
        self.parent().parent()
    }
    fn _message_info_mut(&mut self) -> &mut MessageInfo {
        self.parent_mut().parent_mut()
    }
}
impl IsIdentifiedObject for CapBankDiscreteControlProfile {
    fn _identified_object(&self) -> &super::commonmodule::IdentifiedObject {
        self.parent().parent().parent()
    }
    fn _identified_object_mut(&mut self) -> &mut IdentifiedObject {
        self.parent_mut().parent_mut().parent_mut()
    }
}
/// LN: Power cap bank  Name: YPSH
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct CapBankEventAndStatusYpsh {
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
    /// (controllable) Position of the switch of power shunt.
    #[prost(message, optional, tag="3")]
    pub pos: ::std::option::Option<super::commonmodule::PhaseDps>,
}
mod cap_bank_event_and_status_ypsh {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref LOGICAL_NODE_FOR_EVENT_AND_STATUS: crate::commonmodule::LogicalNodeForEventAndStatus = Default::default();
        pub(super) static ref DYNAMIC_TEST: crate::commonmodule::EnsDynamicTestKind = Default::default();
        pub(super) static ref POS: crate::commonmodule::PhaseDps = Default::default();
    }
}
impl CapBankEventAndStatusYpsh {
    pub(crate) fn parent(&self) -> &super::commonmodule::LogicalNodeForEventAndStatus {
        self.logical_node_for_event_and_status.as_ref().unwrap_or(&cap_bank_event_and_status_ypsh::LOGICAL_NODE_FOR_EVENT_AND_STATUS)
    }
    pub(crate) fn parent_mut(&mut self) -> &mut super::commonmodule::LogicalNodeForEventAndStatus {
        self._cap_bank_event_and_status_ypsh_mut().logical_node_for_event_and_status.get_or_insert(Default::default())
    }
}
pub trait IsCapBankEventAndStatusYpsh {
    fn _cap_bank_event_and_status_ypsh(&self) -> &CapBankEventAndStatusYpsh;
    fn _cap_bank_event_and_status_ypsh_mut(&mut self) -> &mut CapBankEventAndStatusYpsh;
    fn logical_node_for_event_and_status(&self) -> &super::commonmodule::LogicalNodeForEventAndStatus {
        self._cap_bank_event_and_status_ypsh().logical_node_for_event_and_status.as_ref().unwrap_or(&cap_bank_event_and_status_ypsh::LOGICAL_NODE_FOR_EVENT_AND_STATUS)
    }
    fn logical_node_for_event_and_status_mut(&mut self) -> &mut super::commonmodule::LogicalNodeForEventAndStatus {
        self._cap_bank_event_and_status_ypsh_mut().logical_node_for_event_and_status.get_or_insert(Default::default())
    }
    fn dynamic_test(&self) -> &super::commonmodule::EnsDynamicTestKind {
        self._cap_bank_event_and_status_ypsh().dynamic_test.as_ref().unwrap_or(&cap_bank_event_and_status_ypsh::DYNAMIC_TEST)
    }
    fn dynamic_test_mut(&mut self) -> &mut super::commonmodule::EnsDynamicTestKind {
        self._cap_bank_event_and_status_ypsh_mut().dynamic_test.get_or_insert(Default::default())
    }
    fn pos(&self) -> &super::commonmodule::PhaseDps {
        self._cap_bank_event_and_status_ypsh().pos.as_ref().unwrap_or(&cap_bank_event_and_status_ypsh::POS)
    }
    fn pos_mut(&mut self) -> &mut super::commonmodule::PhaseDps {
        self._cap_bank_event_and_status_ypsh_mut().pos.get_or_insert(Default::default())
    }
}
impl IsCapBankEventAndStatusYpsh for CapBankEventAndStatusYpsh {
    fn _cap_bank_event_and_status_ypsh(&self) -> &CapBankEventAndStatusYpsh {
        self
    }
    fn _cap_bank_event_and_status_ypsh_mut(&mut self) -> &mut CapBankEventAndStatusYpsh {
        self
    }
}
impl IsLogicalNodeForEventAndStatus for CapBankEventAndStatusYpsh {
    fn _logical_node_for_event_and_status(&self) -> &super::commonmodule::LogicalNodeForEventAndStatus {
        self.parent()
    }
    fn _logical_node_for_event_and_status_mut(&mut self) -> &mut LogicalNodeForEventAndStatus {
        self.parent_mut()
    }
}
impl IsLogicalNode for CapBankEventAndStatusYpsh {
    fn _logical_node(&self) -> &super::commonmodule::LogicalNode {
        self.parent().parent()
    }
    fn _logical_node_mut(&mut self) -> &mut LogicalNode {
        self.parent_mut().parent_mut()
    }
}
impl IsIdentifiedObject for CapBankEventAndStatusYpsh {
    fn _identified_object(&self) -> &super::commonmodule::IdentifiedObject {
        self.parent().parent().parent()
    }
    fn _identified_object_mut(&mut self) -> &mut IdentifiedObject {
        self.parent_mut().parent_mut().parent_mut()
    }
}
/// Cap bank event
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct CapBankEvent {
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
    pub cap_bank_event_and_status_ypsh: ::std::option::Option<CapBankEventAndStatusYpsh>,
}
mod cap_bank_event {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref EVENT_VALUE: crate::commonmodule::EventValue = Default::default();
        pub(super) static ref CAP_BANK_EVENT_AND_STATUS_YPSH: crate::capbankmodule::CapBankEventAndStatusYpsh = Default::default();
    }
}
impl CapBankEvent {
    pub(crate) fn parent(&self) -> &super::commonmodule::EventValue {
        self.event_value.as_ref().unwrap_or(&cap_bank_event::EVENT_VALUE)
    }
    pub(crate) fn parent_mut(&mut self) -> &mut super::commonmodule::EventValue {
        self._cap_bank_event_mut().event_value.get_or_insert(Default::default())
    }
}
pub trait IsCapBankEvent {
    fn _cap_bank_event(&self) -> &CapBankEvent;
    fn _cap_bank_event_mut(&mut self) -> &mut CapBankEvent;
    fn event_value(&self) -> &super::commonmodule::EventValue {
        self._cap_bank_event().event_value.as_ref().unwrap_or(&cap_bank_event::EVENT_VALUE)
    }
    fn event_value_mut(&mut self) -> &mut super::commonmodule::EventValue {
        self._cap_bank_event_mut().event_value.get_or_insert(Default::default())
    }
    fn cap_bank_event_and_status_ypsh(&self) -> &CapBankEventAndStatusYpsh {
        self._cap_bank_event().cap_bank_event_and_status_ypsh.as_ref().unwrap_or(&cap_bank_event::CAP_BANK_EVENT_AND_STATUS_YPSH)
    }
    fn cap_bank_event_and_status_ypsh_mut(&mut self) -> &mut CapBankEventAndStatusYpsh {
        self._cap_bank_event_mut().cap_bank_event_and_status_ypsh.get_or_insert(Default::default())
    }
}
impl IsCapBankEvent for CapBankEvent {
    fn _cap_bank_event(&self) -> &CapBankEvent {
        self
    }
    fn _cap_bank_event_mut(&mut self) -> &mut CapBankEvent {
        self
    }
}
impl IsEventValue for CapBankEvent {
    fn _event_value(&self) -> &super::commonmodule::EventValue {
        self.parent()
    }
    fn _event_value_mut(&mut self) -> &mut EventValue {
        self.parent_mut()
    }
}
impl IsIdentifiedObject for CapBankEvent {
    fn _identified_object(&self) -> &super::commonmodule::IdentifiedObject {
        self.parent().parent()
    }
    fn _identified_object_mut(&mut self) -> &mut IdentifiedObject {
        self.parent_mut().parent_mut()
    }
}
/// Cap bank status profile
/// OpenFMB Profile Message: true
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct CapBankEventProfile {
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
    pub cap_bank_event: ::std::option::Option<CapBankEvent>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: Some(1)
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, optional, tag="3")]
    pub cap_bank_system: ::std::option::Option<CapBankSystem>,
}
mod cap_bank_event_profile {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref EVENT_MESSAGE_INFO: crate::commonmodule::EventMessageInfo = Default::default();
        pub(super) static ref CAP_BANK_EVENT: crate::capbankmodule::CapBankEvent = Default::default();
        pub(super) static ref CAP_BANK_SYSTEM: crate::capbankmodule::CapBankSystem = Default::default();
    }
}
impl CapBankEventProfile {
    pub(crate) fn parent(&self) -> &super::commonmodule::EventMessageInfo {
        self.event_message_info.as_ref().unwrap_or(&cap_bank_event_profile::EVENT_MESSAGE_INFO)
    }
    pub(crate) fn parent_mut(&mut self) -> &mut super::commonmodule::EventMessageInfo {
        self._cap_bank_event_profile_mut().event_message_info.get_or_insert(Default::default())
    }
}
pub trait IsCapBankEventProfile {
    fn _cap_bank_event_profile(&self) -> &CapBankEventProfile;
    fn _cap_bank_event_profile_mut(&mut self) -> &mut CapBankEventProfile;
    fn event_message_info(&self) -> &super::commonmodule::EventMessageInfo {
        self._cap_bank_event_profile().event_message_info.as_ref().unwrap_or(&cap_bank_event_profile::EVENT_MESSAGE_INFO)
    }
    fn event_message_info_mut(&mut self) -> &mut super::commonmodule::EventMessageInfo {
        self._cap_bank_event_profile_mut().event_message_info.get_or_insert(Default::default())
    }
    fn cap_bank_event(&self) -> &CapBankEvent {
        self._cap_bank_event_profile().cap_bank_event.as_ref().unwrap_or(&cap_bank_event_profile::CAP_BANK_EVENT)
    }
    fn cap_bank_event_mut(&mut self) -> &mut CapBankEvent {
        self._cap_bank_event_profile_mut().cap_bank_event.get_or_insert(Default::default())
    }
    fn cap_bank_system(&self) -> &CapBankSystem {
        self._cap_bank_event_profile().cap_bank_system.as_ref().unwrap_or(&cap_bank_event_profile::CAP_BANK_SYSTEM)
    }
    fn cap_bank_system_mut(&mut self) -> &mut CapBankSystem {
        self._cap_bank_event_profile_mut().cap_bank_system.get_or_insert(Default::default())
    }
}
impl IsCapBankEventProfile for CapBankEventProfile {
    fn _cap_bank_event_profile(&self) -> &CapBankEventProfile {
        self
    }
    fn _cap_bank_event_profile_mut(&mut self) -> &mut CapBankEventProfile {
        self
    }
}
impl IsEventMessageInfo for CapBankEventProfile {
    fn _event_message_info(&self) -> &super::commonmodule::EventMessageInfo {
        self.parent()
    }
    fn _event_message_info_mut(&mut self) -> &mut EventMessageInfo {
        self.parent_mut()
    }
}
impl IsMessageInfo for CapBankEventProfile {
    fn _message_info(&self) -> &super::commonmodule::MessageInfo {
        self.parent().parent()
    }
    fn _message_info_mut(&mut self) -> &mut MessageInfo {
        self.parent_mut().parent_mut()
    }
}
impl IsIdentifiedObject for CapBankEventProfile {
    fn _identified_object(&self) -> &super::commonmodule::IdentifiedObject {
        self.parent().parent().parent()
    }
    fn _identified_object_mut(&mut self) -> &mut IdentifiedObject {
        self.parent_mut().parent_mut().parent_mut()
    }
}
/// Cap bank reading value
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct CapBankReading {
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
    pub phase_mmtn: ::std::option::Option<super::commonmodule::PhaseMmtn>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub reading_mmtr: ::std::option::Option<super::commonmodule::ReadingMmtr>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="4")]
    pub reading_mmxu: ::std::option::Option<super::commonmodule::ReadingMmxu>,
}
mod cap_bank_reading {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref CONDUCTING_EQUIPMENT_TERMINAL_READING: crate::commonmodule::ConductingEquipmentTerminalReading = Default::default();
        pub(super) static ref PHASE_MMTN: crate::commonmodule::PhaseMmtn = Default::default();
        pub(super) static ref READING_MMTR: crate::commonmodule::ReadingMmtr = Default::default();
        pub(super) static ref READING_MMXU: crate::commonmodule::ReadingMmxu = Default::default();
    }
}
impl CapBankReading {
    pub(crate) fn parent(&self) -> &super::commonmodule::ConductingEquipmentTerminalReading {
        self.conducting_equipment_terminal_reading.as_ref().unwrap_or(&cap_bank_reading::CONDUCTING_EQUIPMENT_TERMINAL_READING)
    }
    pub(crate) fn parent_mut(&mut self) -> &mut super::commonmodule::ConductingEquipmentTerminalReading {
        self._cap_bank_reading_mut().conducting_equipment_terminal_reading.get_or_insert(Default::default())
    }
}
pub trait IsCapBankReading {
    fn _cap_bank_reading(&self) -> &CapBankReading;
    fn _cap_bank_reading_mut(&mut self) -> &mut CapBankReading;
    fn conducting_equipment_terminal_reading(&self) -> &super::commonmodule::ConductingEquipmentTerminalReading {
        self._cap_bank_reading().conducting_equipment_terminal_reading.as_ref().unwrap_or(&cap_bank_reading::CONDUCTING_EQUIPMENT_TERMINAL_READING)
    }
    fn conducting_equipment_terminal_reading_mut(&mut self) -> &mut super::commonmodule::ConductingEquipmentTerminalReading {
        self._cap_bank_reading_mut().conducting_equipment_terminal_reading.get_or_insert(Default::default())
    }
    fn phase_mmtn(&self) -> &super::commonmodule::PhaseMmtn {
        self._cap_bank_reading().phase_mmtn.as_ref().unwrap_or(&cap_bank_reading::PHASE_MMTN)
    }
    fn phase_mmtn_mut(&mut self) -> &mut super::commonmodule::PhaseMmtn {
        self._cap_bank_reading_mut().phase_mmtn.get_or_insert(Default::default())
    }
    fn reading_mmtr(&self) -> &super::commonmodule::ReadingMmtr {
        self._cap_bank_reading().reading_mmtr.as_ref().unwrap_or(&cap_bank_reading::READING_MMTR)
    }
    fn reading_mmtr_mut(&mut self) -> &mut super::commonmodule::ReadingMmtr {
        self._cap_bank_reading_mut().reading_mmtr.get_or_insert(Default::default())
    }
    fn reading_mmxu(&self) -> &super::commonmodule::ReadingMmxu {
        self._cap_bank_reading().reading_mmxu.as_ref().unwrap_or(&cap_bank_reading::READING_MMXU)
    }
    fn reading_mmxu_mut(&mut self) -> &mut super::commonmodule::ReadingMmxu {
        self._cap_bank_reading_mut().reading_mmxu.get_or_insert(Default::default())
    }
}
impl IsCapBankReading for CapBankReading {
    fn _cap_bank_reading(&self) -> &CapBankReading {
        self
    }
    fn _cap_bank_reading_mut(&mut self) -> &mut CapBankReading {
        self
    }
}
impl IsConductingEquipmentTerminalReading for CapBankReading {
    fn _conducting_equipment_terminal_reading(&self) -> &super::commonmodule::ConductingEquipmentTerminalReading {
        self.parent()
    }
    fn _conducting_equipment_terminal_reading_mut(&mut self) -> &mut ConductingEquipmentTerminalReading {
        self.parent_mut()
    }
}
/// Cap bank reading profile
/// OpenFMB Profile Message: true
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct CapBankReadingProfile {
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
    pub cap_bank_reading: ::std::option::Option<CapBankReading>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: Some(1)
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, optional, tag="3")]
    pub cap_bank_system: ::std::option::Option<CapBankSystem>,
}
mod cap_bank_reading_profile {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref READING_MESSAGE_INFO: crate::commonmodule::ReadingMessageInfo = Default::default();
        pub(super) static ref CAP_BANK_READING: crate::capbankmodule::CapBankReading = Default::default();
        pub(super) static ref CAP_BANK_SYSTEM: crate::capbankmodule::CapBankSystem = Default::default();
    }
}
impl CapBankReadingProfile {
    pub(crate) fn parent(&self) -> &super::commonmodule::ReadingMessageInfo {
        self.reading_message_info.as_ref().unwrap_or(&cap_bank_reading_profile::READING_MESSAGE_INFO)
    }
    pub(crate) fn parent_mut(&mut self) -> &mut super::commonmodule::ReadingMessageInfo {
        self._cap_bank_reading_profile_mut().reading_message_info.get_or_insert(Default::default())
    }
}
pub trait IsCapBankReadingProfile {
    fn _cap_bank_reading_profile(&self) -> &CapBankReadingProfile;
    fn _cap_bank_reading_profile_mut(&mut self) -> &mut CapBankReadingProfile;
    fn reading_message_info(&self) -> &super::commonmodule::ReadingMessageInfo {
        self._cap_bank_reading_profile().reading_message_info.as_ref().unwrap_or(&cap_bank_reading_profile::READING_MESSAGE_INFO)
    }
    fn reading_message_info_mut(&mut self) -> &mut super::commonmodule::ReadingMessageInfo {
        self._cap_bank_reading_profile_mut().reading_message_info.get_or_insert(Default::default())
    }
    fn cap_bank_reading(&self) -> &CapBankReading {
        self._cap_bank_reading_profile().cap_bank_reading.as_ref().unwrap_or(&cap_bank_reading_profile::CAP_BANK_READING)
    }
    fn cap_bank_reading_mut(&mut self) -> &mut CapBankReading {
        self._cap_bank_reading_profile_mut().cap_bank_reading.get_or_insert(Default::default())
    }
    fn cap_bank_system(&self) -> &CapBankSystem {
        self._cap_bank_reading_profile().cap_bank_system.as_ref().unwrap_or(&cap_bank_reading_profile::CAP_BANK_SYSTEM)
    }
    fn cap_bank_system_mut(&mut self) -> &mut CapBankSystem {
        self._cap_bank_reading_profile_mut().cap_bank_system.get_or_insert(Default::default())
    }
}
impl IsCapBankReadingProfile for CapBankReadingProfile {
    fn _cap_bank_reading_profile(&self) -> &CapBankReadingProfile {
        self
    }
    fn _cap_bank_reading_profile_mut(&mut self) -> &mut CapBankReadingProfile {
        self
    }
}
impl IsReadingMessageInfo for CapBankReadingProfile {
    fn _reading_message_info(&self) -> &super::commonmodule::ReadingMessageInfo {
        self.parent()
    }
    fn _reading_message_info_mut(&mut self) -> &mut ReadingMessageInfo {
        self.parent_mut()
    }
}
impl IsMessageInfo for CapBankReadingProfile {
    fn _message_info(&self) -> &super::commonmodule::MessageInfo {
        self.parent().parent()
    }
    fn _message_info_mut(&mut self) -> &mut MessageInfo {
        self.parent_mut().parent_mut()
    }
}
impl IsIdentifiedObject for CapBankReadingProfile {
    fn _identified_object(&self) -> &super::commonmodule::IdentifiedObject {
        self.parent().parent().parent()
    }
    fn _identified_object_mut(&mut self) -> &mut IdentifiedObject {
        self.parent_mut().parent_mut().parent_mut()
    }
}
/// Cap bank status
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct CapBankStatus {
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
    pub cap_bank_event_and_status_ypsh: ::std::option::Option<CapBankEventAndStatusYpsh>,
}
mod cap_bank_status {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref STATUS_VALUE: crate::commonmodule::StatusValue = Default::default();
        pub(super) static ref CAP_BANK_EVENT_AND_STATUS_YPSH: crate::capbankmodule::CapBankEventAndStatusYpsh = Default::default();
    }
}
impl CapBankStatus {
    pub(crate) fn parent(&self) -> &super::commonmodule::StatusValue {
        self.status_value.as_ref().unwrap_or(&cap_bank_status::STATUS_VALUE)
    }
    pub(crate) fn parent_mut(&mut self) -> &mut super::commonmodule::StatusValue {
        self._cap_bank_status_mut().status_value.get_or_insert(Default::default())
    }
}
pub trait IsCapBankStatus {
    fn _cap_bank_status(&self) -> &CapBankStatus;
    fn _cap_bank_status_mut(&mut self) -> &mut CapBankStatus;
    fn status_value(&self) -> &super::commonmodule::StatusValue {
        self._cap_bank_status().status_value.as_ref().unwrap_or(&cap_bank_status::STATUS_VALUE)
    }
    fn status_value_mut(&mut self) -> &mut super::commonmodule::StatusValue {
        self._cap_bank_status_mut().status_value.get_or_insert(Default::default())
    }
    fn cap_bank_event_and_status_ypsh(&self) -> &CapBankEventAndStatusYpsh {
        self._cap_bank_status().cap_bank_event_and_status_ypsh.as_ref().unwrap_or(&cap_bank_status::CAP_BANK_EVENT_AND_STATUS_YPSH)
    }
    fn cap_bank_event_and_status_ypsh_mut(&mut self) -> &mut CapBankEventAndStatusYpsh {
        self._cap_bank_status_mut().cap_bank_event_and_status_ypsh.get_or_insert(Default::default())
    }
}
impl IsCapBankStatus for CapBankStatus {
    fn _cap_bank_status(&self) -> &CapBankStatus {
        self
    }
    fn _cap_bank_status_mut(&mut self) -> &mut CapBankStatus {
        self
    }
}
impl IsStatusValue for CapBankStatus {
    fn _status_value(&self) -> &super::commonmodule::StatusValue {
        self.parent()
    }
    fn _status_value_mut(&mut self) -> &mut StatusValue {
        self.parent_mut()
    }
}
impl IsIdentifiedObject for CapBankStatus {
    fn _identified_object(&self) -> &super::commonmodule::IdentifiedObject {
        self.parent().parent()
    }
    fn _identified_object_mut(&mut self) -> &mut IdentifiedObject {
        self.parent_mut().parent_mut()
    }
}
/// Cap bank status profile
/// OpenFMB Profile Message: true
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct CapBankStatusProfile {
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
    pub cap_bank_status: ::std::option::Option<CapBankStatus>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: Some(1)
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, optional, tag="3")]
    pub cap_bank_system: ::std::option::Option<CapBankSystem>,
}
mod cap_bank_status_profile {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref STATUS_MESSAGE_INFO: crate::commonmodule::StatusMessageInfo = Default::default();
        pub(super) static ref CAP_BANK_STATUS: crate::capbankmodule::CapBankStatus = Default::default();
        pub(super) static ref CAP_BANK_SYSTEM: crate::capbankmodule::CapBankSystem = Default::default();
    }
}
impl CapBankStatusProfile {
    pub(crate) fn parent(&self) -> &super::commonmodule::StatusMessageInfo {
        self.status_message_info.as_ref().unwrap_or(&cap_bank_status_profile::STATUS_MESSAGE_INFO)
    }
    pub(crate) fn parent_mut(&mut self) -> &mut super::commonmodule::StatusMessageInfo {
        self._cap_bank_status_profile_mut().status_message_info.get_or_insert(Default::default())
    }
}
pub trait IsCapBankStatusProfile {
    fn _cap_bank_status_profile(&self) -> &CapBankStatusProfile;
    fn _cap_bank_status_profile_mut(&mut self) -> &mut CapBankStatusProfile;
    fn status_message_info(&self) -> &super::commonmodule::StatusMessageInfo {
        self._cap_bank_status_profile().status_message_info.as_ref().unwrap_or(&cap_bank_status_profile::STATUS_MESSAGE_INFO)
    }
    fn status_message_info_mut(&mut self) -> &mut super::commonmodule::StatusMessageInfo {
        self._cap_bank_status_profile_mut().status_message_info.get_or_insert(Default::default())
    }
    fn cap_bank_status(&self) -> &CapBankStatus {
        self._cap_bank_status_profile().cap_bank_status.as_ref().unwrap_or(&cap_bank_status_profile::CAP_BANK_STATUS)
    }
    fn cap_bank_status_mut(&mut self) -> &mut CapBankStatus {
        self._cap_bank_status_profile_mut().cap_bank_status.get_or_insert(Default::default())
    }
    fn cap_bank_system(&self) -> &CapBankSystem {
        self._cap_bank_status_profile().cap_bank_system.as_ref().unwrap_or(&cap_bank_status_profile::CAP_BANK_SYSTEM)
    }
    fn cap_bank_system_mut(&mut self) -> &mut CapBankSystem {
        self._cap_bank_status_profile_mut().cap_bank_system.get_or_insert(Default::default())
    }
}
impl IsCapBankStatusProfile for CapBankStatusProfile {
    fn _cap_bank_status_profile(&self) -> &CapBankStatusProfile {
        self
    }
    fn _cap_bank_status_profile_mut(&mut self) -> &mut CapBankStatusProfile {
        self
    }
}
impl IsStatusMessageInfo for CapBankStatusProfile {
    fn _status_message_info(&self) -> &super::commonmodule::StatusMessageInfo {
        self.parent()
    }
    fn _status_message_info_mut(&mut self) -> &mut StatusMessageInfo {
        self.parent_mut()
    }
}
impl IsMessageInfo for CapBankStatusProfile {
    fn _message_info(&self) -> &super::commonmodule::MessageInfo {
        self.parent().parent()
    }
    fn _message_info_mut(&mut self) -> &mut MessageInfo {
        self.parent_mut().parent_mut()
    }
}
impl IsIdentifiedObject for CapBankStatusProfile {
    fn _identified_object(&self) -> &super::commonmodule::IdentifiedObject {
        self.parent().parent().parent()
    }
    fn _identified_object_mut(&mut self) -> &mut IdentifiedObject {
        self.parent_mut().parent_mut().parent_mut()
    }
}
