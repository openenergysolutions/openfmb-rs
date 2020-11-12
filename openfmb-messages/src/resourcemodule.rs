use crate::commonmodule::*;
/// LN: Generic process I/O   Name: GGIO
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct BooleanControlGgio {
    /// UML inherited base object
    // parent_message: true
    // required_field: false
    // multiplicity_min: None
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, optional, tag="1")]
    pub logical_node: ::std::option::Option<super::commonmodule::LogicalNode>,
    /// Phase code
    #[prost(message, optional, tag="2")]
    pub phase: ::std::option::Option<super::commonmodule::OptionalPhaseCodeKind>,
    /// (controllable) If true, generic single point controllable status output <i>n</i> has been
    /// enabled, otherwise it has been disabled.
    // parent_message: false
    // required_field: true
    // multiplicity_min: Some(1)
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, optional, tag="3")]
    pub spcso: ::std::option::Option<super::commonmodule::ControlSpc>,
}
mod boolean_control_ggio {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref LOGICAL_NODE: crate::commonmodule::LogicalNode = Default::default();
        pub(super) static ref PHASE: crate::commonmodule::OptionalPhaseCodeKind = Default::default();
        pub(super) static ref SPCSO: crate::commonmodule::ControlSpc = Default::default();
    }
}
impl BooleanControlGgio {
    pub(crate) fn parent(&self) -> &super::commonmodule::LogicalNode {
        self.logical_node.as_ref().unwrap_or(&boolean_control_ggio::LOGICAL_NODE)
    }
    pub(crate) fn parent_mut(&mut self) -> &mut super::commonmodule::LogicalNode {
        self.logical_node.get_or_insert(Default::default())
    }
}
pub trait IsBooleanControlGgio {
    fn _boolean_control_ggio(&self) -> &BooleanControlGgio;
    fn _boolean_control_ggio_mut(&mut self) -> &mut BooleanControlGgio;
    fn logical_node(&self) -> &super::commonmodule::LogicalNode {
        self._boolean_control_ggio().logical_node.as_ref().unwrap_or(&boolean_control_ggio::LOGICAL_NODE)
    }
    fn logical_node_mut(&mut self) -> &mut super::commonmodule::LogicalNode {
        self._boolean_control_ggio_mut().logical_node.get_or_insert(Default::default())
    }
    fn phase(&self) -> &super::commonmodule::OptionalPhaseCodeKind {
        self._boolean_control_ggio().phase.as_ref().unwrap_or(&boolean_control_ggio::PHASE)
    }
    fn phase_mut(&mut self) -> &mut super::commonmodule::OptionalPhaseCodeKind {
        self._boolean_control_ggio_mut().phase.get_or_insert(Default::default())
    }
    fn spcso(&self) -> &super::commonmodule::ControlSpc {
        self._boolean_control_ggio().spcso.as_ref().unwrap_or(&boolean_control_ggio::SPCSO)
    }
    fn spcso_mut(&mut self) -> &mut super::commonmodule::ControlSpc {
        self._boolean_control_ggio_mut().spcso.get_or_insert(Default::default())
    }
}
impl IsBooleanControlGgio for BooleanControlGgio {
    fn _boolean_control_ggio(&self) -> &BooleanControlGgio {
        self
    }
    fn _boolean_control_ggio_mut(&mut self) -> &mut BooleanControlGgio {
        self
    }
}
impl IsLogicalNode for BooleanControlGgio {
    fn _logical_node(&self) -> &super::commonmodule::LogicalNode {
        self.parent()
    }
    fn _logical_node_mut(&mut self) -> &mut LogicalNode {
        self.parent_mut()
    }
}
impl IsIdentifiedObject for BooleanControlGgio {
    fn _identified_object(&self) -> &super::commonmodule::IdentifiedObject {
        self.parent().parent()
    }
    fn _identified_object_mut(&mut self) -> &mut IdentifiedObject {
        self.parent_mut().parent_mut()
    }
}
/// Status expressed in integer based on IEC61850 GGIO.
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct IntegerControlGgio {
    /// UML inherited base object
    // parent_message: true
    // required_field: false
    // multiplicity_min: None
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, optional, tag="1")]
    pub logical_node: ::std::option::Option<super::commonmodule::LogicalNode>,
    /// (controllable) Generic integer controllable status output <i>n</i>.
    // parent_message: false
    // required_field: true
    // multiplicity_min: Some(1)
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, optional, tag="2")]
    pub iscso: ::std::option::Option<super::commonmodule::ControlInc>,
    /// Phase code
    #[prost(message, optional, tag="3")]
    pub phase: ::std::option::Option<super::commonmodule::OptionalPhaseCodeKind>,
}
mod integer_control_ggio {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref LOGICAL_NODE: crate::commonmodule::LogicalNode = Default::default();
        pub(super) static ref ISCSO: crate::commonmodule::ControlInc = Default::default();
        pub(super) static ref PHASE: crate::commonmodule::OptionalPhaseCodeKind = Default::default();
    }
}
impl IntegerControlGgio {
    pub(crate) fn parent(&self) -> &super::commonmodule::LogicalNode {
        self.logical_node.as_ref().unwrap_or(&integer_control_ggio::LOGICAL_NODE)
    }
    pub(crate) fn parent_mut(&mut self) -> &mut super::commonmodule::LogicalNode {
        self.logical_node.get_or_insert(Default::default())
    }
}
pub trait IsIntegerControlGgio {
    fn _integer_control_ggio(&self) -> &IntegerControlGgio;
    fn _integer_control_ggio_mut(&mut self) -> &mut IntegerControlGgio;
    fn logical_node(&self) -> &super::commonmodule::LogicalNode {
        self._integer_control_ggio().logical_node.as_ref().unwrap_or(&integer_control_ggio::LOGICAL_NODE)
    }
    fn logical_node_mut(&mut self) -> &mut super::commonmodule::LogicalNode {
        self._integer_control_ggio_mut().logical_node.get_or_insert(Default::default())
    }
    fn iscso(&self) -> &super::commonmodule::ControlInc {
        self._integer_control_ggio().iscso.as_ref().unwrap_or(&integer_control_ggio::ISCSO)
    }
    fn iscso_mut(&mut self) -> &mut super::commonmodule::ControlInc {
        self._integer_control_ggio_mut().iscso.get_or_insert(Default::default())
    }
    fn phase(&self) -> &super::commonmodule::OptionalPhaseCodeKind {
        self._integer_control_ggio().phase.as_ref().unwrap_or(&integer_control_ggio::PHASE)
    }
    fn phase_mut(&mut self) -> &mut super::commonmodule::OptionalPhaseCodeKind {
        self._integer_control_ggio_mut().phase.get_or_insert(Default::default())
    }
}
impl IsIntegerControlGgio for IntegerControlGgio {
    fn _integer_control_ggio(&self) -> &IntegerControlGgio {
        self
    }
    fn _integer_control_ggio_mut(&mut self) -> &mut IntegerControlGgio {
        self
    }
}
impl IsLogicalNode for IntegerControlGgio {
    fn _logical_node(&self) -> &super::commonmodule::LogicalNode {
        self.parent()
    }
    fn _logical_node_mut(&mut self) -> &mut LogicalNode {
        self.parent_mut()
    }
}
impl IsIdentifiedObject for IntegerControlGgio {
    fn _identified_object(&self) -> &super::commonmodule::IdentifiedObject {
        self.parent().parent()
    }
    fn _identified_object_mut(&mut self) -> &mut IdentifiedObject {
        self.parent_mut().parent_mut()
    }
}
/// LN: Generic process I/O   Name: GGIO
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct StringControlGgio {
    /// UML inherited base object
    // parent_message: true
    // required_field: false
    // multiplicity_min: None
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, optional, tag="1")]
    pub logical_node: ::std::option::Option<super::commonmodule::LogicalNode>,
    /// Phase code
    #[prost(message, optional, tag="2")]
    pub phase: ::std::option::Option<super::commonmodule::OptionalPhaseCodeKind>,
    /// String control
    // parent_message: false
    // required_field: true
    // multiplicity_min: Some(1)
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, optional, tag="3")]
    pub str_out: ::std::option::Option<super::commonmodule::Vsc>,
}
mod string_control_ggio {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref LOGICAL_NODE: crate::commonmodule::LogicalNode = Default::default();
        pub(super) static ref PHASE: crate::commonmodule::OptionalPhaseCodeKind = Default::default();
        pub(super) static ref STR_OUT: crate::commonmodule::Vsc = Default::default();
    }
}
impl StringControlGgio {
    pub(crate) fn parent(&self) -> &super::commonmodule::LogicalNode {
        self.logical_node.as_ref().unwrap_or(&string_control_ggio::LOGICAL_NODE)
    }
    pub(crate) fn parent_mut(&mut self) -> &mut super::commonmodule::LogicalNode {
        self.logical_node.get_or_insert(Default::default())
    }
}
pub trait IsStringControlGgio {
    fn _string_control_ggio(&self) -> &StringControlGgio;
    fn _string_control_ggio_mut(&mut self) -> &mut StringControlGgio;
    fn logical_node(&self) -> &super::commonmodule::LogicalNode {
        self._string_control_ggio().logical_node.as_ref().unwrap_or(&string_control_ggio::LOGICAL_NODE)
    }
    fn logical_node_mut(&mut self) -> &mut super::commonmodule::LogicalNode {
        self._string_control_ggio_mut().logical_node.get_or_insert(Default::default())
    }
    fn phase(&self) -> &super::commonmodule::OptionalPhaseCodeKind {
        self._string_control_ggio().phase.as_ref().unwrap_or(&string_control_ggio::PHASE)
    }
    fn phase_mut(&mut self) -> &mut super::commonmodule::OptionalPhaseCodeKind {
        self._string_control_ggio_mut().phase.get_or_insert(Default::default())
    }
    fn str_out(&self) -> &super::commonmodule::Vsc {
        self._string_control_ggio().str_out.as_ref().unwrap_or(&string_control_ggio::STR_OUT)
    }
    fn str_out_mut(&mut self) -> &mut super::commonmodule::Vsc {
        self._string_control_ggio_mut().str_out.get_or_insert(Default::default())
    }
}
impl IsStringControlGgio for StringControlGgio {
    fn _string_control_ggio(&self) -> &StringControlGgio {
        self
    }
    fn _string_control_ggio_mut(&mut self) -> &mut StringControlGgio {
        self
    }
}
impl IsLogicalNode for StringControlGgio {
    fn _logical_node(&self) -> &super::commonmodule::LogicalNode {
        self.parent()
    }
    fn _logical_node_mut(&mut self) -> &mut LogicalNode {
        self.parent_mut()
    }
}
impl IsIdentifiedObject for StringControlGgio {
    fn _identified_object(&self) -> &super::commonmodule::IdentifiedObject {
        self.parent().parent()
    }
    fn _identified_object_mut(&mut self) -> &mut IdentifiedObject {
        self.parent_mut().parent_mut()
    }
}
/// LN: Generic process I/O   Name: GGIO
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct AnalogControlGgio {
    /// UML inherited base object
    // parent_message: true
    // required_field: false
    // multiplicity_min: None
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, optional, tag="1")]
    pub logical_node: ::std::option::Option<super::commonmodule::LogicalNode>,
    /// (controllable) Value of the generic controllable analogue output setpoint <i>n</i>. Analog value
    /// (MX) feeds back the setpoint of the output.
    // parent_message: false
    // required_field: true
    // multiplicity_min: Some(1)
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, optional, tag="2")]
    pub an_out: ::std::option::Option<super::commonmodule::ControlApc>,
    /// Phase code
    #[prost(message, optional, tag="3")]
    pub phase: ::std::option::Option<super::commonmodule::OptionalPhaseCodeKind>,
}
mod analog_control_ggio {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref LOGICAL_NODE: crate::commonmodule::LogicalNode = Default::default();
        pub(super) static ref AN_OUT: crate::commonmodule::ControlApc = Default::default();
        pub(super) static ref PHASE: crate::commonmodule::OptionalPhaseCodeKind = Default::default();
    }
}
impl AnalogControlGgio {
    pub(crate) fn parent(&self) -> &super::commonmodule::LogicalNode {
        self.logical_node.as_ref().unwrap_or(&analog_control_ggio::LOGICAL_NODE)
    }
    pub(crate) fn parent_mut(&mut self) -> &mut super::commonmodule::LogicalNode {
        self.logical_node.get_or_insert(Default::default())
    }
}
pub trait IsAnalogControlGgio {
    fn _analog_control_ggio(&self) -> &AnalogControlGgio;
    fn _analog_control_ggio_mut(&mut self) -> &mut AnalogControlGgio;
    fn logical_node(&self) -> &super::commonmodule::LogicalNode {
        self._analog_control_ggio().logical_node.as_ref().unwrap_or(&analog_control_ggio::LOGICAL_NODE)
    }
    fn logical_node_mut(&mut self) -> &mut super::commonmodule::LogicalNode {
        self._analog_control_ggio_mut().logical_node.get_or_insert(Default::default())
    }
    fn an_out(&self) -> &super::commonmodule::ControlApc {
        self._analog_control_ggio().an_out.as_ref().unwrap_or(&analog_control_ggio::AN_OUT)
    }
    fn an_out_mut(&mut self) -> &mut super::commonmodule::ControlApc {
        self._analog_control_ggio_mut().an_out.get_or_insert(Default::default())
    }
    fn phase(&self) -> &super::commonmodule::OptionalPhaseCodeKind {
        self._analog_control_ggio().phase.as_ref().unwrap_or(&analog_control_ggio::PHASE)
    }
    fn phase_mut(&mut self) -> &mut super::commonmodule::OptionalPhaseCodeKind {
        self._analog_control_ggio_mut().phase.get_or_insert(Default::default())
    }
}
impl IsAnalogControlGgio for AnalogControlGgio {
    fn _analog_control_ggio(&self) -> &AnalogControlGgio {
        self
    }
    fn _analog_control_ggio_mut(&mut self) -> &mut AnalogControlGgio {
        self
    }
}
impl IsLogicalNode for AnalogControlGgio {
    fn _logical_node(&self) -> &super::commonmodule::LogicalNode {
        self.parent()
    }
    fn _logical_node_mut(&mut self) -> &mut LogicalNode {
        self.parent_mut()
    }
}
impl IsIdentifiedObject for AnalogControlGgio {
    fn _identified_object(&self) -> &super::commonmodule::IdentifiedObject {
        self.parent().parent()
    }
    fn _identified_object_mut(&mut self) -> &mut IdentifiedObject {
        self.parent_mut().parent_mut()
    }
}
/// Resource discrete control class
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResourceDiscreteControl {
    /// UML inherited base object
    // parent_message: true
    // required_field: false
    // multiplicity_min: None
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, optional, tag="1")]
    pub identified_object: ::std::option::Option<super::commonmodule::IdentifiedObject>,
    /// IEC61850 enhanced control parameters.
    #[prost(message, optional, tag="2")]
    pub check: ::std::option::Option<super::commonmodule::CheckConditions>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: false
    // multiplicity_min: Some(0)
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, repeated, tag="3")]
    pub analog_control_ggio: ::std::vec::Vec<AnalogControlGgio>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: false
    // multiplicity_min: Some(0)
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, repeated, tag="4")]
    pub boolean_control_ggio: ::std::vec::Vec<BooleanControlGgio>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: false
    // multiplicity_min: Some(0)
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, repeated, tag="5")]
    pub integer_control_ggio: ::std::vec::Vec<IntegerControlGgio>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: false
    // multiplicity_min: Some(0)
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, repeated, tag="6")]
    pub string_control_ggio: ::std::vec::Vec<StringControlGgio>,
}
mod resource_discrete_control {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref IDENTIFIED_OBJECT: crate::commonmodule::IdentifiedObject = Default::default();
        pub(super) static ref CHECK: crate::commonmodule::CheckConditions = Default::default();
    }
}
impl ResourceDiscreteControl {
    pub(crate) fn parent(&self) -> &super::commonmodule::IdentifiedObject {
        self.identified_object.as_ref().unwrap_or(&resource_discrete_control::IDENTIFIED_OBJECT)
    }
    pub(crate) fn parent_mut(&mut self) -> &mut super::commonmodule::IdentifiedObject {
        self.identified_object.get_or_insert(Default::default())
    }
}
pub trait IsResourceDiscreteControl {
    fn _resource_discrete_control(&self) -> &ResourceDiscreteControl;
    fn _resource_discrete_control_mut(&mut self) -> &mut ResourceDiscreteControl;
    fn identified_object(&self) -> &super::commonmodule::IdentifiedObject {
        self._resource_discrete_control().identified_object.as_ref().unwrap_or(&resource_discrete_control::IDENTIFIED_OBJECT)
    }
    fn identified_object_mut(&mut self) -> &mut super::commonmodule::IdentifiedObject {
        self._resource_discrete_control_mut().identified_object.get_or_insert(Default::default())
    }
    fn check(&self) -> &super::commonmodule::CheckConditions {
        self._resource_discrete_control().check.as_ref().unwrap_or(&resource_discrete_control::CHECK)
    }
    fn check_mut(&mut self) -> &mut super::commonmodule::CheckConditions {
        self._resource_discrete_control_mut().check.get_or_insert(Default::default())
    }
    fn analog_control_ggio(&self) -> &::std::vec::Vec<AnalogControlGgio> {
        &self._resource_discrete_control().analog_control_ggio
    }
    fn analog_control_ggio_mut(&mut self) -> &mut ::std::vec::Vec<AnalogControlGgio> {
        &mut self._resource_discrete_control_mut().analog_control_ggio
    }
    fn boolean_control_ggio(&self) -> &::std::vec::Vec<BooleanControlGgio> {
        &self._resource_discrete_control().boolean_control_ggio
    }
    fn boolean_control_ggio_mut(&mut self) -> &mut ::std::vec::Vec<BooleanControlGgio> {
        &mut self._resource_discrete_control_mut().boolean_control_ggio
    }
    fn integer_control_ggio(&self) -> &::std::vec::Vec<IntegerControlGgio> {
        &self._resource_discrete_control().integer_control_ggio
    }
    fn integer_control_ggio_mut(&mut self) -> &mut ::std::vec::Vec<IntegerControlGgio> {
        &mut self._resource_discrete_control_mut().integer_control_ggio
    }
    fn string_control_ggio(&self) -> &::std::vec::Vec<StringControlGgio> {
        &self._resource_discrete_control().string_control_ggio
    }
    fn string_control_ggio_mut(&mut self) -> &mut ::std::vec::Vec<StringControlGgio> {
        &mut self._resource_discrete_control_mut().string_control_ggio
    }
}
impl IsResourceDiscreteControl for ResourceDiscreteControl {
    fn _resource_discrete_control(&self) -> &ResourceDiscreteControl {
        self
    }
    fn _resource_discrete_control_mut(&mut self) -> &mut ResourceDiscreteControl {
        self
    }
}
impl IsIdentifiedObject for ResourceDiscreteControl {
    fn _identified_object(&self) -> &super::commonmodule::IdentifiedObject {
        self.parent()
    }
    fn _identified_object_mut(&mut self) -> &mut IdentifiedObject {
        self.parent_mut()
    }
}
/// Instructs a resource to perform a specified action.
/// OpenFMB Profile Message: true
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResourceDiscreteControlProfile {
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
    pub conducting_equipment: ::std::option::Option<super::commonmodule::ConductingEquipment>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: Some(1)
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, optional, tag="3")]
    pub resource_discrete_control: ::std::option::Option<ResourceDiscreteControl>,
}
mod resource_discrete_control_profile {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref CONTROL_MESSAGE_INFO: crate::commonmodule::ControlMessageInfo = Default::default();
        pub(super) static ref CONDUCTING_EQUIPMENT: crate::commonmodule::ConductingEquipment = Default::default();
        pub(super) static ref RESOURCE_DISCRETE_CONTROL: crate::resourcemodule::ResourceDiscreteControl = Default::default();
    }
}
impl ResourceDiscreteControlProfile {
    pub(crate) fn parent(&self) -> &super::commonmodule::ControlMessageInfo {
        self.control_message_info.as_ref().unwrap_or(&resource_discrete_control_profile::CONTROL_MESSAGE_INFO)
    }
    pub(crate) fn parent_mut(&mut self) -> &mut super::commonmodule::ControlMessageInfo {
        self.control_message_info.get_or_insert(Default::default())
    }
}
pub trait IsResourceDiscreteControlProfile {
    fn _resource_discrete_control_profile(&self) -> &ResourceDiscreteControlProfile;
    fn _resource_discrete_control_profile_mut(&mut self) -> &mut ResourceDiscreteControlProfile;
    fn control_message_info(&self) -> &super::commonmodule::ControlMessageInfo {
        self._resource_discrete_control_profile().control_message_info.as_ref().unwrap_or(&resource_discrete_control_profile::CONTROL_MESSAGE_INFO)
    }
    fn control_message_info_mut(&mut self) -> &mut super::commonmodule::ControlMessageInfo {
        self._resource_discrete_control_profile_mut().control_message_info.get_or_insert(Default::default())
    }
    fn conducting_equipment(&self) -> &super::commonmodule::ConductingEquipment {
        self._resource_discrete_control_profile().conducting_equipment.as_ref().unwrap_or(&resource_discrete_control_profile::CONDUCTING_EQUIPMENT)
    }
    fn conducting_equipment_mut(&mut self) -> &mut super::commonmodule::ConductingEquipment {
        self._resource_discrete_control_profile_mut().conducting_equipment.get_or_insert(Default::default())
    }
    fn resource_discrete_control(&self) -> &ResourceDiscreteControl {
        self._resource_discrete_control_profile().resource_discrete_control.as_ref().unwrap_or(&resource_discrete_control_profile::RESOURCE_DISCRETE_CONTROL)
    }
    fn resource_discrete_control_mut(&mut self) -> &mut ResourceDiscreteControl {
        self._resource_discrete_control_profile_mut().resource_discrete_control.get_or_insert(Default::default())
    }
}
impl IsResourceDiscreteControlProfile for ResourceDiscreteControlProfile {
    fn _resource_discrete_control_profile(&self) -> &ResourceDiscreteControlProfile {
        self
    }
    fn _resource_discrete_control_profile_mut(&mut self) -> &mut ResourceDiscreteControlProfile {
        self
    }
}
impl IsControlMessageInfo for ResourceDiscreteControlProfile {
    fn _control_message_info(&self) -> &super::commonmodule::ControlMessageInfo {
        self.parent()
    }
    fn _control_message_info_mut(&mut self) -> &mut ControlMessageInfo {
        self.parent_mut()
    }
}
impl IsMessageInfo for ResourceDiscreteControlProfile {
    fn _message_info(&self) -> &super::commonmodule::MessageInfo {
        self.parent().parent()
    }
    fn _message_info_mut(&mut self) -> &mut MessageInfo {
        self.parent_mut().parent_mut()
    }
}
impl IsIdentifiedObject for ResourceDiscreteControlProfile {
    fn _identified_object(&self) -> &super::commonmodule::IdentifiedObject {
        self.parent().parent().parent()
    }
    fn _identified_object_mut(&mut self) -> &mut IdentifiedObject {
        self.parent_mut().parent_mut().parent_mut()
    }
}
/// Resource reading value
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResourceReading {
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
mod resource_reading {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref CONDUCTING_EQUIPMENT_TERMINAL_READING: crate::commonmodule::ConductingEquipmentTerminalReading = Default::default();
        pub(super) static ref PHASE_MMTN: crate::commonmodule::PhaseMmtn = Default::default();
        pub(super) static ref READING_MMTR: crate::commonmodule::ReadingMmtr = Default::default();
        pub(super) static ref READING_MMXU: crate::commonmodule::ReadingMmxu = Default::default();
    }
}
impl ResourceReading {
    pub(crate) fn parent(&self) -> &super::commonmodule::ConductingEquipmentTerminalReading {
        self.conducting_equipment_terminal_reading.as_ref().unwrap_or(&resource_reading::CONDUCTING_EQUIPMENT_TERMINAL_READING)
    }
    pub(crate) fn parent_mut(&mut self) -> &mut super::commonmodule::ConductingEquipmentTerminalReading {
        self.conducting_equipment_terminal_reading.get_or_insert(Default::default())
    }
}
pub trait IsResourceReading {
    fn _resource_reading(&self) -> &ResourceReading;
    fn _resource_reading_mut(&mut self) -> &mut ResourceReading;
    fn conducting_equipment_terminal_reading(&self) -> &super::commonmodule::ConductingEquipmentTerminalReading {
        self._resource_reading().conducting_equipment_terminal_reading.as_ref().unwrap_or(&resource_reading::CONDUCTING_EQUIPMENT_TERMINAL_READING)
    }
    fn conducting_equipment_terminal_reading_mut(&mut self) -> &mut super::commonmodule::ConductingEquipmentTerminalReading {
        self._resource_reading_mut().conducting_equipment_terminal_reading.get_or_insert(Default::default())
    }
    fn phase_mmtn(&self) -> &super::commonmodule::PhaseMmtn {
        self._resource_reading().phase_mmtn.as_ref().unwrap_or(&resource_reading::PHASE_MMTN)
    }
    fn phase_mmtn_mut(&mut self) -> &mut super::commonmodule::PhaseMmtn {
        self._resource_reading_mut().phase_mmtn.get_or_insert(Default::default())
    }
    fn reading_mmtr(&self) -> &super::commonmodule::ReadingMmtr {
        self._resource_reading().reading_mmtr.as_ref().unwrap_or(&resource_reading::READING_MMTR)
    }
    fn reading_mmtr_mut(&mut self) -> &mut super::commonmodule::ReadingMmtr {
        self._resource_reading_mut().reading_mmtr.get_or_insert(Default::default())
    }
    fn reading_mmxu(&self) -> &super::commonmodule::ReadingMmxu {
        self._resource_reading().reading_mmxu.as_ref().unwrap_or(&resource_reading::READING_MMXU)
    }
    fn reading_mmxu_mut(&mut self) -> &mut super::commonmodule::ReadingMmxu {
        self._resource_reading_mut().reading_mmxu.get_or_insert(Default::default())
    }
}
impl IsResourceReading for ResourceReading {
    fn _resource_reading(&self) -> &ResourceReading {
        self
    }
    fn _resource_reading_mut(&mut self) -> &mut ResourceReading {
        self
    }
}
impl IsConductingEquipmentTerminalReading for ResourceReading {
    fn _conducting_equipment_terminal_reading(&self) -> &super::commonmodule::ConductingEquipmentTerminalReading {
        self.parent()
    }
    fn _conducting_equipment_terminal_reading_mut(&mut self) -> &mut ConductingEquipmentTerminalReading {
        self.parent_mut()
    }
}
/// Resource reading profile
/// OpenFMB Profile Message: true
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResourceReadingProfile {
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
    pub conducting_equipment: ::std::option::Option<super::commonmodule::ConductingEquipment>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: Some(1)
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, optional, tag="3")]
    pub resource_reading: ::std::option::Option<ResourceReading>,
}
mod resource_reading_profile {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref READING_MESSAGE_INFO: crate::commonmodule::ReadingMessageInfo = Default::default();
        pub(super) static ref CONDUCTING_EQUIPMENT: crate::commonmodule::ConductingEquipment = Default::default();
        pub(super) static ref RESOURCE_READING: crate::resourcemodule::ResourceReading = Default::default();
    }
}
impl ResourceReadingProfile {
    pub(crate) fn parent(&self) -> &super::commonmodule::ReadingMessageInfo {
        self.reading_message_info.as_ref().unwrap_or(&resource_reading_profile::READING_MESSAGE_INFO)
    }
    pub(crate) fn parent_mut(&mut self) -> &mut super::commonmodule::ReadingMessageInfo {
        self.reading_message_info.get_or_insert(Default::default())
    }
}
pub trait IsResourceReadingProfile {
    fn _resource_reading_profile(&self) -> &ResourceReadingProfile;
    fn _resource_reading_profile_mut(&mut self) -> &mut ResourceReadingProfile;
    fn reading_message_info(&self) -> &super::commonmodule::ReadingMessageInfo {
        self._resource_reading_profile().reading_message_info.as_ref().unwrap_or(&resource_reading_profile::READING_MESSAGE_INFO)
    }
    fn reading_message_info_mut(&mut self) -> &mut super::commonmodule::ReadingMessageInfo {
        self._resource_reading_profile_mut().reading_message_info.get_or_insert(Default::default())
    }
    fn conducting_equipment(&self) -> &super::commonmodule::ConductingEquipment {
        self._resource_reading_profile().conducting_equipment.as_ref().unwrap_or(&resource_reading_profile::CONDUCTING_EQUIPMENT)
    }
    fn conducting_equipment_mut(&mut self) -> &mut super::commonmodule::ConductingEquipment {
        self._resource_reading_profile_mut().conducting_equipment.get_or_insert(Default::default())
    }
    fn resource_reading(&self) -> &ResourceReading {
        self._resource_reading_profile().resource_reading.as_ref().unwrap_or(&resource_reading_profile::RESOURCE_READING)
    }
    fn resource_reading_mut(&mut self) -> &mut ResourceReading {
        self._resource_reading_profile_mut().resource_reading.get_or_insert(Default::default())
    }
}
impl IsResourceReadingProfile for ResourceReadingProfile {
    fn _resource_reading_profile(&self) -> &ResourceReadingProfile {
        self
    }
    fn _resource_reading_profile_mut(&mut self) -> &mut ResourceReadingProfile {
        self
    }
}
impl IsReadingMessageInfo for ResourceReadingProfile {
    fn _reading_message_info(&self) -> &super::commonmodule::ReadingMessageInfo {
        self.parent()
    }
    fn _reading_message_info_mut(&mut self) -> &mut ReadingMessageInfo {
        self.parent_mut()
    }
}
impl IsMessageInfo for ResourceReadingProfile {
    fn _message_info(&self) -> &super::commonmodule::MessageInfo {
        self.parent().parent()
    }
    fn _message_info_mut(&mut self) -> &mut MessageInfo {
        self.parent_mut().parent_mut()
    }
}
impl IsIdentifiedObject for ResourceReadingProfile {
    fn _identified_object(&self) -> &super::commonmodule::IdentifiedObject {
        self.parent().parent().parent()
    }
    fn _identified_object_mut(&mut self) -> &mut IdentifiedObject {
        self.parent_mut().parent_mut().parent_mut()
    }
}
/// Current event information relevant to an entity.
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResourceEvent {
    /// UML inherited base object
    // parent_message: true
    // required_field: false
    // multiplicity_min: None
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, optional, tag="1")]
    pub identified_object: ::std::option::Option<super::commonmodule::IdentifiedObject>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: false
    // multiplicity_min: Some(0)
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, repeated, tag="2")]
    pub analog_event_and_status_ggio: ::std::vec::Vec<super::commonmodule::AnalogEventAndStatusGgio>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: false
    // multiplicity_min: Some(0)
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, repeated, tag="3")]
    pub boolean_event_and_status_ggio: ::std::vec::Vec<super::commonmodule::BooleanEventAndStatusGgio>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: false
    // multiplicity_min: Some(0)
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, repeated, tag="4")]
    pub integer_event_and_status_ggio: ::std::vec::Vec<super::commonmodule::IntegerEventAndStatusGgio>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: false
    // multiplicity_min: Some(0)
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, repeated, tag="5")]
    pub string_event_and_status_ggio: ::std::vec::Vec<super::commonmodule::StringEventAndStatusGgio>,
}
mod resource_event {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref IDENTIFIED_OBJECT: crate::commonmodule::IdentifiedObject = Default::default();
    }
}
impl ResourceEvent {
    pub(crate) fn parent(&self) -> &super::commonmodule::IdentifiedObject {
        self.identified_object.as_ref().unwrap_or(&resource_event::IDENTIFIED_OBJECT)
    }
    pub(crate) fn parent_mut(&mut self) -> &mut super::commonmodule::IdentifiedObject {
        self.identified_object.get_or_insert(Default::default())
    }
}
pub trait IsResourceEvent {
    fn _resource_event(&self) -> &ResourceEvent;
    fn _resource_event_mut(&mut self) -> &mut ResourceEvent;
    fn identified_object(&self) -> &super::commonmodule::IdentifiedObject {
        self._resource_event().identified_object.as_ref().unwrap_or(&resource_event::IDENTIFIED_OBJECT)
    }
    fn identified_object_mut(&mut self) -> &mut super::commonmodule::IdentifiedObject {
        self._resource_event_mut().identified_object.get_or_insert(Default::default())
    }
    fn analog_event_and_status_ggio(&self) -> &::std::vec::Vec<super::commonmodule::AnalogEventAndStatusGgio> {
        &self._resource_event().analog_event_and_status_ggio
    }
    fn analog_event_and_status_ggio_mut(&mut self) -> &mut ::std::vec::Vec<super::commonmodule::AnalogEventAndStatusGgio> {
        &mut self._resource_event_mut().analog_event_and_status_ggio
    }
    fn boolean_event_and_status_ggio(&self) -> &::std::vec::Vec<super::commonmodule::BooleanEventAndStatusGgio> {
        &self._resource_event().boolean_event_and_status_ggio
    }
    fn boolean_event_and_status_ggio_mut(&mut self) -> &mut ::std::vec::Vec<super::commonmodule::BooleanEventAndStatusGgio> {
        &mut self._resource_event_mut().boolean_event_and_status_ggio
    }
    fn integer_event_and_status_ggio(&self) -> &::std::vec::Vec<super::commonmodule::IntegerEventAndStatusGgio> {
        &self._resource_event().integer_event_and_status_ggio
    }
    fn integer_event_and_status_ggio_mut(&mut self) -> &mut ::std::vec::Vec<super::commonmodule::IntegerEventAndStatusGgio> {
        &mut self._resource_event_mut().integer_event_and_status_ggio
    }
    fn string_event_and_status_ggio(&self) -> &::std::vec::Vec<super::commonmodule::StringEventAndStatusGgio> {
        &self._resource_event().string_event_and_status_ggio
    }
    fn string_event_and_status_ggio_mut(&mut self) -> &mut ::std::vec::Vec<super::commonmodule::StringEventAndStatusGgio> {
        &mut self._resource_event_mut().string_event_and_status_ggio
    }
}
impl IsResourceEvent for ResourceEvent {
    fn _resource_event(&self) -> &ResourceEvent {
        self
    }
    fn _resource_event_mut(&mut self) -> &mut ResourceEvent {
        self
    }
}
impl IsIdentifiedObject for ResourceEvent {
    fn _identified_object(&self) -> &super::commonmodule::IdentifiedObject {
        self.parent()
    }
    fn _identified_object_mut(&mut self) -> &mut IdentifiedObject {
        self.parent_mut()
    }
}
/// Resource event module
/// OpenFMB Profile Message: true
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResourceEventProfile {
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
    pub conducting_equipment: ::std::option::Option<super::commonmodule::ConductingEquipment>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: Some(1)
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, optional, tag="3")]
    pub resource_event: ::std::option::Option<ResourceEvent>,
}
mod resource_event_profile {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref EVENT_MESSAGE_INFO: crate::commonmodule::EventMessageInfo = Default::default();
        pub(super) static ref CONDUCTING_EQUIPMENT: crate::commonmodule::ConductingEquipment = Default::default();
        pub(super) static ref RESOURCE_EVENT: crate::resourcemodule::ResourceEvent = Default::default();
    }
}
impl ResourceEventProfile {
    pub(crate) fn parent(&self) -> &super::commonmodule::EventMessageInfo {
        self.event_message_info.as_ref().unwrap_or(&resource_event_profile::EVENT_MESSAGE_INFO)
    }
    pub(crate) fn parent_mut(&mut self) -> &mut super::commonmodule::EventMessageInfo {
        self.event_message_info.get_or_insert(Default::default())
    }
}
pub trait IsResourceEventProfile {
    fn _resource_event_profile(&self) -> &ResourceEventProfile;
    fn _resource_event_profile_mut(&mut self) -> &mut ResourceEventProfile;
    fn event_message_info(&self) -> &super::commonmodule::EventMessageInfo {
        self._resource_event_profile().event_message_info.as_ref().unwrap_or(&resource_event_profile::EVENT_MESSAGE_INFO)
    }
    fn event_message_info_mut(&mut self) -> &mut super::commonmodule::EventMessageInfo {
        self._resource_event_profile_mut().event_message_info.get_or_insert(Default::default())
    }
    fn conducting_equipment(&self) -> &super::commonmodule::ConductingEquipment {
        self._resource_event_profile().conducting_equipment.as_ref().unwrap_or(&resource_event_profile::CONDUCTING_EQUIPMENT)
    }
    fn conducting_equipment_mut(&mut self) -> &mut super::commonmodule::ConductingEquipment {
        self._resource_event_profile_mut().conducting_equipment.get_or_insert(Default::default())
    }
    fn resource_event(&self) -> &ResourceEvent {
        self._resource_event_profile().resource_event.as_ref().unwrap_or(&resource_event_profile::RESOURCE_EVENT)
    }
    fn resource_event_mut(&mut self) -> &mut ResourceEvent {
        self._resource_event_profile_mut().resource_event.get_or_insert(Default::default())
    }
}
impl IsResourceEventProfile for ResourceEventProfile {
    fn _resource_event_profile(&self) -> &ResourceEventProfile {
        self
    }
    fn _resource_event_profile_mut(&mut self) -> &mut ResourceEventProfile {
        self
    }
}
impl IsEventMessageInfo for ResourceEventProfile {
    fn _event_message_info(&self) -> &super::commonmodule::EventMessageInfo {
        self.parent()
    }
    fn _event_message_info_mut(&mut self) -> &mut EventMessageInfo {
        self.parent_mut()
    }
}
impl IsMessageInfo for ResourceEventProfile {
    fn _message_info(&self) -> &super::commonmodule::MessageInfo {
        self.parent().parent()
    }
    fn _message_info_mut(&mut self) -> &mut MessageInfo {
        self.parent_mut().parent_mut()
    }
}
impl IsIdentifiedObject for ResourceEventProfile {
    fn _identified_object(&self) -> &super::commonmodule::IdentifiedObject {
        self.parent().parent().parent()
    }
    fn _identified_object_mut(&mut self) -> &mut IdentifiedObject {
        self.parent_mut().parent_mut().parent_mut()
    }
}
/// Current status information relevant to an entity.
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResourceStatus {
    /// UML inherited base object
    // parent_message: true
    // required_field: false
    // multiplicity_min: None
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, optional, tag="1")]
    pub identified_object: ::std::option::Option<super::commonmodule::IdentifiedObject>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: false
    // multiplicity_min: Some(0)
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, repeated, tag="2")]
    pub analog_event_and_status_ggio: ::std::vec::Vec<super::commonmodule::AnalogEventAndStatusGgio>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: false
    // multiplicity_min: Some(0)
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, repeated, tag="3")]
    pub boolean_event_and_status_ggio: ::std::vec::Vec<super::commonmodule::BooleanEventAndStatusGgio>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: false
    // multiplicity_min: Some(0)
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, repeated, tag="4")]
    pub integer_event_and_status_ggio: ::std::vec::Vec<super::commonmodule::IntegerEventAndStatusGgio>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: false
    // multiplicity_min: Some(0)
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, repeated, tag="5")]
    pub string_event_and_status_ggio: ::std::vec::Vec<super::commonmodule::StringEventAndStatusGgio>,
}
mod resource_status {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref IDENTIFIED_OBJECT: crate::commonmodule::IdentifiedObject = Default::default();
    }
}
impl ResourceStatus {
    pub(crate) fn parent(&self) -> &super::commonmodule::IdentifiedObject {
        self.identified_object.as_ref().unwrap_or(&resource_status::IDENTIFIED_OBJECT)
    }
    pub(crate) fn parent_mut(&mut self) -> &mut super::commonmodule::IdentifiedObject {
        self.identified_object.get_or_insert(Default::default())
    }
}
pub trait IsResourceStatus {
    fn _resource_status(&self) -> &ResourceStatus;
    fn _resource_status_mut(&mut self) -> &mut ResourceStatus;
    fn identified_object(&self) -> &super::commonmodule::IdentifiedObject {
        self._resource_status().identified_object.as_ref().unwrap_or(&resource_status::IDENTIFIED_OBJECT)
    }
    fn identified_object_mut(&mut self) -> &mut super::commonmodule::IdentifiedObject {
        self._resource_status_mut().identified_object.get_or_insert(Default::default())
    }
    fn analog_event_and_status_ggio(&self) -> &::std::vec::Vec<super::commonmodule::AnalogEventAndStatusGgio> {
        &self._resource_status().analog_event_and_status_ggio
    }
    fn analog_event_and_status_ggio_mut(&mut self) -> &mut ::std::vec::Vec<super::commonmodule::AnalogEventAndStatusGgio> {
        &mut self._resource_status_mut().analog_event_and_status_ggio
    }
    fn boolean_event_and_status_ggio(&self) -> &::std::vec::Vec<super::commonmodule::BooleanEventAndStatusGgio> {
        &self._resource_status().boolean_event_and_status_ggio
    }
    fn boolean_event_and_status_ggio_mut(&mut self) -> &mut ::std::vec::Vec<super::commonmodule::BooleanEventAndStatusGgio> {
        &mut self._resource_status_mut().boolean_event_and_status_ggio
    }
    fn integer_event_and_status_ggio(&self) -> &::std::vec::Vec<super::commonmodule::IntegerEventAndStatusGgio> {
        &self._resource_status().integer_event_and_status_ggio
    }
    fn integer_event_and_status_ggio_mut(&mut self) -> &mut ::std::vec::Vec<super::commonmodule::IntegerEventAndStatusGgio> {
        &mut self._resource_status_mut().integer_event_and_status_ggio
    }
    fn string_event_and_status_ggio(&self) -> &::std::vec::Vec<super::commonmodule::StringEventAndStatusGgio> {
        &self._resource_status().string_event_and_status_ggio
    }
    fn string_event_and_status_ggio_mut(&mut self) -> &mut ::std::vec::Vec<super::commonmodule::StringEventAndStatusGgio> {
        &mut self._resource_status_mut().string_event_and_status_ggio
    }
}
impl IsResourceStatus for ResourceStatus {
    fn _resource_status(&self) -> &ResourceStatus {
        self
    }
    fn _resource_status_mut(&mut self) -> &mut ResourceStatus {
        self
    }
}
impl IsIdentifiedObject for ResourceStatus {
    fn _identified_object(&self) -> &super::commonmodule::IdentifiedObject {
        self.parent()
    }
    fn _identified_object_mut(&mut self) -> &mut IdentifiedObject {
        self.parent_mut()
    }
}
/// Resource status module
/// OpenFMB Profile Message: true
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResourceStatusProfile {
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
    pub conducting_equipment: ::std::option::Option<super::commonmodule::ConductingEquipment>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: Some(1)
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, optional, tag="3")]
    pub resource_status: ::std::option::Option<ResourceStatus>,
}
mod resource_status_profile {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref STATUS_MESSAGE_INFO: crate::commonmodule::StatusMessageInfo = Default::default();
        pub(super) static ref CONDUCTING_EQUIPMENT: crate::commonmodule::ConductingEquipment = Default::default();
        pub(super) static ref RESOURCE_STATUS: crate::resourcemodule::ResourceStatus = Default::default();
    }
}
impl ResourceStatusProfile {
    pub(crate) fn parent(&self) -> &super::commonmodule::StatusMessageInfo {
        self.status_message_info.as_ref().unwrap_or(&resource_status_profile::STATUS_MESSAGE_INFO)
    }
    pub(crate) fn parent_mut(&mut self) -> &mut super::commonmodule::StatusMessageInfo {
        self.status_message_info.get_or_insert(Default::default())
    }
}
pub trait IsResourceStatusProfile {
    fn _resource_status_profile(&self) -> &ResourceStatusProfile;
    fn _resource_status_profile_mut(&mut self) -> &mut ResourceStatusProfile;
    fn status_message_info(&self) -> &super::commonmodule::StatusMessageInfo {
        self._resource_status_profile().status_message_info.as_ref().unwrap_or(&resource_status_profile::STATUS_MESSAGE_INFO)
    }
    fn status_message_info_mut(&mut self) -> &mut super::commonmodule::StatusMessageInfo {
        self._resource_status_profile_mut().status_message_info.get_or_insert(Default::default())
    }
    fn conducting_equipment(&self) -> &super::commonmodule::ConductingEquipment {
        self._resource_status_profile().conducting_equipment.as_ref().unwrap_or(&resource_status_profile::CONDUCTING_EQUIPMENT)
    }
    fn conducting_equipment_mut(&mut self) -> &mut super::commonmodule::ConductingEquipment {
        self._resource_status_profile_mut().conducting_equipment.get_or_insert(Default::default())
    }
    fn resource_status(&self) -> &ResourceStatus {
        self._resource_status_profile().resource_status.as_ref().unwrap_or(&resource_status_profile::RESOURCE_STATUS)
    }
    fn resource_status_mut(&mut self) -> &mut ResourceStatus {
        self._resource_status_profile_mut().resource_status.get_or_insert(Default::default())
    }
}
impl IsResourceStatusProfile for ResourceStatusProfile {
    fn _resource_status_profile(&self) -> &ResourceStatusProfile {
        self
    }
    fn _resource_status_profile_mut(&mut self) -> &mut ResourceStatusProfile {
        self
    }
}
impl IsStatusMessageInfo for ResourceStatusProfile {
    fn _status_message_info(&self) -> &super::commonmodule::StatusMessageInfo {
        self.parent()
    }
    fn _status_message_info_mut(&mut self) -> &mut StatusMessageInfo {
        self.parent_mut()
    }
}
impl IsMessageInfo for ResourceStatusProfile {
    fn _message_info(&self) -> &super::commonmodule::MessageInfo {
        self.parent().parent()
    }
    fn _message_info_mut(&mut self) -> &mut MessageInfo {
        self.parent_mut().parent_mut()
    }
}
impl IsIdentifiedObject for ResourceStatusProfile {
    fn _identified_object(&self) -> &super::commonmodule::IdentifiedObject {
        self.parent().parent().parent()
    }
    fn _identified_object_mut(&mut self) -> &mut IdentifiedObject {
        self.parent_mut().parent_mut().parent_mut()
    }
}
