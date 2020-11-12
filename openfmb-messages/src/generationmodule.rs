use crate::commonmodule::*;
/// Point definition (Point)
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct GenerationPoint {
    /// Black start enable
    #[prost(message, optional, tag="1")]
    pub black_start_enabled: ::std::option::Option<super::commonmodule::ControlDpc>,
    /// Enable frequency set point
    #[prost(message, optional, tag="2")]
    pub frequency_set_point_enabled: ::std::option::Option<super::commonmodule::ControlDpc>,
    /// Black start enable
    #[prost(message, optional, tag="3")]
    pub pct_hz_droop: ::std::option::Option<f32>,
    /// Black start enable
    #[prost(message, optional, tag="4")]
    pub pct_v_droop: ::std::option::Option<f32>,
    /// Ramp rates
    #[prost(message, optional, tag="5")]
    pub ramp_rates: ::std::option::Option<super::commonmodule::RampRate>,
    /// Enable reactive power set point
    #[prost(message, optional, tag="6")]
    pub reactive_pwr_set_point_enabled: ::std::option::Option<super::commonmodule::ControlDpc>,
    /// Enable joint real power set point
    #[prost(message, optional, tag="7")]
    pub real_pwr_set_point_enabled: ::std::option::Option<super::commonmodule::ControlDpc>,
    /// Reset device
    #[prost(message, optional, tag="8")]
    pub reset: ::std::option::Option<super::commonmodule::ControlDpc>,
    /// ESS state
    #[prost(message, optional, tag="9")]
    pub state: ::std::option::Option<super::commonmodule::OptionalStateKind>,
    /// Synchronize back to grid
    #[prost(message, optional, tag="10")]
    pub sync_back_to_grid: ::std::option::Option<super::commonmodule::ControlDpc>,
    /// Transition to island on grid loss enable
    #[prost(message, optional, tag="11")]
    pub trans_to_islnd_on_grid_loss_enabled: ::std::option::Option<super::commonmodule::ControlDpc>,
    /// Enable voltage set point
    #[prost(message, optional, tag="12")]
    pub voltage_set_point_enabled: ::std::option::Option<super::commonmodule::ControlDpc>,
    /// Start time
    // parent_message: false
    // required_field: true
    // multiplicity_min: Some(1)
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, optional, tag="13")]
    pub start_time: ::std::option::Option<super::commonmodule::ControlTimestamp>,
}
mod generation_point {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref BLACK_START_ENABLED: crate::commonmodule::ControlDpc = Default::default();
        pub(super) static ref FREQUENCY_SET_POINT_ENABLED: crate::commonmodule::ControlDpc = Default::default();
        pub(super) static ref PCT_HZ_DROOP: f32 = Default::default();
        pub(super) static ref PCT_V_DROOP: f32 = Default::default();
        pub(super) static ref RAMP_RATES: crate::commonmodule::RampRate = Default::default();
        pub(super) static ref REACTIVE_PWR_SET_POINT_ENABLED: crate::commonmodule::ControlDpc = Default::default();
        pub(super) static ref REAL_PWR_SET_POINT_ENABLED: crate::commonmodule::ControlDpc = Default::default();
        pub(super) static ref RESET: crate::commonmodule::ControlDpc = Default::default();
        pub(super) static ref STATE: crate::commonmodule::OptionalStateKind = Default::default();
        pub(super) static ref SYNC_BACK_TO_GRID: crate::commonmodule::ControlDpc = Default::default();
        pub(super) static ref TRANS_TO_ISLND_ON_GRID_LOSS_ENABLED: crate::commonmodule::ControlDpc = Default::default();
        pub(super) static ref VOLTAGE_SET_POINT_ENABLED: crate::commonmodule::ControlDpc = Default::default();
        pub(super) static ref START_TIME: crate::commonmodule::ControlTimestamp = Default::default();
    }
}
impl GenerationPoint {
}
pub trait IsGenerationPoint {
    fn _generation_point(&self) -> &GenerationPoint;
    fn _generation_point_mut(&mut self) -> &mut GenerationPoint;
    fn black_start_enabled(&self) -> &super::commonmodule::ControlDpc {
        self._generation_point().black_start_enabled.as_ref().unwrap_or(&generation_point::BLACK_START_ENABLED)
    }
    fn black_start_enabled_mut(&mut self) -> &mut super::commonmodule::ControlDpc {
        self._generation_point_mut().black_start_enabled.get_or_insert(Default::default())
    }
    fn frequency_set_point_enabled(&self) -> &super::commonmodule::ControlDpc {
        self._generation_point().frequency_set_point_enabled.as_ref().unwrap_or(&generation_point::FREQUENCY_SET_POINT_ENABLED)
    }
    fn frequency_set_point_enabled_mut(&mut self) -> &mut super::commonmodule::ControlDpc {
        self._generation_point_mut().frequency_set_point_enabled.get_or_insert(Default::default())
    }
    fn pct_hz_droop(&self) -> &f32 {
        self._generation_point().pct_hz_droop.as_ref().unwrap_or(&generation_point::PCT_HZ_DROOP)
    }
    fn pct_hz_droop_mut(&mut self) -> &mut f32 {
        self._generation_point_mut().pct_hz_droop.get_or_insert(Default::default())
    }
    fn pct_v_droop(&self) -> &f32 {
        self._generation_point().pct_v_droop.as_ref().unwrap_or(&generation_point::PCT_V_DROOP)
    }
    fn pct_v_droop_mut(&mut self) -> &mut f32 {
        self._generation_point_mut().pct_v_droop.get_or_insert(Default::default())
    }
    fn ramp_rates(&self) -> &super::commonmodule::RampRate {
        self._generation_point().ramp_rates.as_ref().unwrap_or(&generation_point::RAMP_RATES)
    }
    fn ramp_rates_mut(&mut self) -> &mut super::commonmodule::RampRate {
        self._generation_point_mut().ramp_rates.get_or_insert(Default::default())
    }
    fn reactive_pwr_set_point_enabled(&self) -> &super::commonmodule::ControlDpc {
        self._generation_point().reactive_pwr_set_point_enabled.as_ref().unwrap_or(&generation_point::REACTIVE_PWR_SET_POINT_ENABLED)
    }
    fn reactive_pwr_set_point_enabled_mut(&mut self) -> &mut super::commonmodule::ControlDpc {
        self._generation_point_mut().reactive_pwr_set_point_enabled.get_or_insert(Default::default())
    }
    fn real_pwr_set_point_enabled(&self) -> &super::commonmodule::ControlDpc {
        self._generation_point().real_pwr_set_point_enabled.as_ref().unwrap_or(&generation_point::REAL_PWR_SET_POINT_ENABLED)
    }
    fn real_pwr_set_point_enabled_mut(&mut self) -> &mut super::commonmodule::ControlDpc {
        self._generation_point_mut().real_pwr_set_point_enabled.get_or_insert(Default::default())
    }
    fn reset(&self) -> &super::commonmodule::ControlDpc {
        self._generation_point().reset.as_ref().unwrap_or(&generation_point::RESET)
    }
    fn reset_mut(&mut self) -> &mut super::commonmodule::ControlDpc {
        self._generation_point_mut().reset.get_or_insert(Default::default())
    }
    fn state(&self) -> &super::commonmodule::OptionalStateKind {
        self._generation_point().state.as_ref().unwrap_or(&generation_point::STATE)
    }
    fn state_mut(&mut self) -> &mut super::commonmodule::OptionalStateKind {
        self._generation_point_mut().state.get_or_insert(Default::default())
    }
    fn sync_back_to_grid(&self) -> &super::commonmodule::ControlDpc {
        self._generation_point().sync_back_to_grid.as_ref().unwrap_or(&generation_point::SYNC_BACK_TO_GRID)
    }
    fn sync_back_to_grid_mut(&mut self) -> &mut super::commonmodule::ControlDpc {
        self._generation_point_mut().sync_back_to_grid.get_or_insert(Default::default())
    }
    fn trans_to_islnd_on_grid_loss_enabled(&self) -> &super::commonmodule::ControlDpc {
        self._generation_point().trans_to_islnd_on_grid_loss_enabled.as_ref().unwrap_or(&generation_point::TRANS_TO_ISLND_ON_GRID_LOSS_ENABLED)
    }
    fn trans_to_islnd_on_grid_loss_enabled_mut(&mut self) -> &mut super::commonmodule::ControlDpc {
        self._generation_point_mut().trans_to_islnd_on_grid_loss_enabled.get_or_insert(Default::default())
    }
    fn voltage_set_point_enabled(&self) -> &super::commonmodule::ControlDpc {
        self._generation_point().voltage_set_point_enabled.as_ref().unwrap_or(&generation_point::VOLTAGE_SET_POINT_ENABLED)
    }
    fn voltage_set_point_enabled_mut(&mut self) -> &mut super::commonmodule::ControlDpc {
        self._generation_point_mut().voltage_set_point_enabled.get_or_insert(Default::default())
    }
    fn start_time(&self) -> &super::commonmodule::ControlTimestamp {
        self._generation_point().start_time.as_ref().unwrap_or(&generation_point::START_TIME)
    }
    fn start_time_mut(&mut self) -> &mut super::commonmodule::ControlTimestamp {
        self._generation_point_mut().start_time.get_or_insert(Default::default())
    }
}
impl IsGenerationPoint for GenerationPoint {
    fn _generation_point(&self) -> &GenerationPoint {
        self
    }
    fn _generation_point_mut(&mut self) -> &mut GenerationPoint {
        self
    }
}
/// Curve shape setting (FC=SP) (CSG_SP)
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct GenerationCsg {
    /// The array with the points specifying a curve shape.
    // parent_message: false
    // required_field: true
    // multiplicity_min: Some(1)
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, repeated, tag="1")]
    pub crv_pts: ::std::vec::Vec<GenerationPoint>,
}
mod generation_csg {
    use lazy_static::lazy_static;
    lazy_static! {
    }
}
impl GenerationCsg {
}
pub trait IsGenerationCsg {
    fn _generation_csg(&self) -> &GenerationCsg;
    fn _generation_csg_mut(&mut self) -> &mut GenerationCsg;
    fn crv_pts(&self) -> &::std::vec::Vec<GenerationPoint> {
        &self._generation_csg().crv_pts
    }
    fn crv_pts_mut(&mut self) -> &mut ::std::vec::Vec<GenerationPoint> {
        &mut self._generation_csg_mut().crv_pts
    }
}
impl IsGenerationCsg for GenerationCsg {
    fn _generation_csg(&self) -> &GenerationCsg {
        self
    }
    fn _generation_csg_mut(&mut self) -> &mut GenerationCsg {
        self
    }
}
/// OpenFMB specialization for control schedule using:  LN: Schedule   Name: FSCH
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct GenerationControlScheduleFsch {
    /// Discrete value in GenerationCSG type
    // parent_message: false
    // required_field: true
    // multiplicity_min: Some(1)
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, optional, tag="1")]
    pub val_dcsg: ::std::option::Option<GenerationCsg>,
}
mod generation_control_schedule_fsch {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref VAL_DCSG: crate::generationmodule::GenerationCsg = Default::default();
    }
}
impl GenerationControlScheduleFsch {
}
pub trait IsGenerationControlScheduleFsch {
    fn _generation_control_schedule_fsch(&self) -> &GenerationControlScheduleFsch;
    fn _generation_control_schedule_fsch_mut(&mut self) -> &mut GenerationControlScheduleFsch;
    fn val_dcsg(&self) -> &GenerationCsg {
        self._generation_control_schedule_fsch().val_dcsg.as_ref().unwrap_or(&generation_control_schedule_fsch::VAL_DCSG)
    }
    fn val_dcsg_mut(&mut self) -> &mut GenerationCsg {
        self._generation_control_schedule_fsch_mut().val_dcsg.get_or_insert(Default::default())
    }
}
impl IsGenerationControlScheduleFsch for GenerationControlScheduleFsch {
    fn _generation_control_schedule_fsch(&self) -> &GenerationControlScheduleFsch {
        self
    }
    fn _generation_control_schedule_fsch_mut(&mut self) -> &mut GenerationControlScheduleFsch {
        self
    }
}
/// LN: Schedule controller   Name: FSCC
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct GenerationControlFscc {
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
    pub generation_control_schedule_fsch: ::std::option::Option<GenerationControlScheduleFsch>,
}
mod generation_control_fscc {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref CONTROL_FSCC: crate::commonmodule::ControlFscc = Default::default();
        pub(super) static ref GENERATION_CONTROL_SCHEDULE_FSCH: crate::generationmodule::GenerationControlScheduleFsch = Default::default();
    }
}
impl GenerationControlFscc {
    pub(crate) fn parent(&self) -> &super::commonmodule::ControlFscc {
        self.control_fscc.as_ref().unwrap_or(&generation_control_fscc::CONTROL_FSCC)
    }
    pub(crate) fn parent_mut(&mut self) -> &mut super::commonmodule::ControlFscc {
        self.control_fscc.get_or_insert(Default::default())
    }
}
pub trait IsGenerationControlFscc {
    fn _generation_control_fscc(&self) -> &GenerationControlFscc;
    fn _generation_control_fscc_mut(&mut self) -> &mut GenerationControlFscc;
    fn control_fscc(&self) -> &super::commonmodule::ControlFscc {
        self._generation_control_fscc().control_fscc.as_ref().unwrap_or(&generation_control_fscc::CONTROL_FSCC)
    }
    fn control_fscc_mut(&mut self) -> &mut super::commonmodule::ControlFscc {
        self._generation_control_fscc_mut().control_fscc.get_or_insert(Default::default())
    }
    fn generation_control_schedule_fsch(&self) -> &GenerationControlScheduleFsch {
        self._generation_control_fscc().generation_control_schedule_fsch.as_ref().unwrap_or(&generation_control_fscc::GENERATION_CONTROL_SCHEDULE_FSCH)
    }
    fn generation_control_schedule_fsch_mut(&mut self) -> &mut GenerationControlScheduleFsch {
        self._generation_control_fscc_mut().generation_control_schedule_fsch.get_or_insert(Default::default())
    }
}
impl IsGenerationControlFscc for GenerationControlFscc {
    fn _generation_control_fscc(&self) -> &GenerationControlFscc {
        self
    }
    fn _generation_control_fscc_mut(&mut self) -> &mut GenerationControlFscc {
        self
    }
}
impl IsControlFscc for GenerationControlFscc {
    fn _control_fscc(&self) -> &super::commonmodule::ControlFscc {
        self.parent()
    }
    fn _control_fscc_mut(&mut self) -> &mut ControlFscc {
        self.parent_mut()
    }
}
impl IsLogicalNodeForControl for GenerationControlFscc {
    fn _logical_node_for_control(&self) -> &super::commonmodule::LogicalNodeForControl {
        self.parent().parent()
    }
    fn _logical_node_for_control_mut(&mut self) -> &mut LogicalNodeForControl {
        self.parent_mut().parent_mut()
    }
}
impl IsLogicalNode for GenerationControlFscc {
    fn _logical_node(&self) -> &super::commonmodule::LogicalNode {
        self.parent().parent().parent()
    }
    fn _logical_node_mut(&mut self) -> &mut LogicalNode {
        self.parent_mut().parent_mut().parent_mut()
    }
}
impl IsIdentifiedObject for GenerationControlFscc {
    fn _identified_object(&self) -> &super::commonmodule::IdentifiedObject {
        self.parent().parent().parent().parent()
    }
    fn _identified_object_mut(&mut self) -> &mut IdentifiedObject {
        self.parent_mut().parent_mut().parent_mut().parent_mut()
    }
}
/// Generation control
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct GenerationControl {
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
    pub generation_control_fscc: ::std::option::Option<GenerationControlFscc>,
}
mod generation_control {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref CONTROL_VALUE: crate::commonmodule::ControlValue = Default::default();
        pub(super) static ref CHECK: crate::commonmodule::CheckConditions = Default::default();
        pub(super) static ref GENERATION_CONTROL_FSCC: crate::generationmodule::GenerationControlFscc = Default::default();
    }
}
impl GenerationControl {
    pub(crate) fn parent(&self) -> &super::commonmodule::ControlValue {
        self.control_value.as_ref().unwrap_or(&generation_control::CONTROL_VALUE)
    }
    pub(crate) fn parent_mut(&mut self) -> &mut super::commonmodule::ControlValue {
        self.control_value.get_or_insert(Default::default())
    }
}
pub trait IsGenerationControl {
    fn _generation_control(&self) -> &GenerationControl;
    fn _generation_control_mut(&mut self) -> &mut GenerationControl;
    fn control_value(&self) -> &super::commonmodule::ControlValue {
        self._generation_control().control_value.as_ref().unwrap_or(&generation_control::CONTROL_VALUE)
    }
    fn control_value_mut(&mut self) -> &mut super::commonmodule::ControlValue {
        self._generation_control_mut().control_value.get_or_insert(Default::default())
    }
    fn check(&self) -> &super::commonmodule::CheckConditions {
        self._generation_control().check.as_ref().unwrap_or(&generation_control::CHECK)
    }
    fn check_mut(&mut self) -> &mut super::commonmodule::CheckConditions {
        self._generation_control_mut().check.get_or_insert(Default::default())
    }
    fn generation_control_fscc(&self) -> &GenerationControlFscc {
        self._generation_control().generation_control_fscc.as_ref().unwrap_or(&generation_control::GENERATION_CONTROL_FSCC)
    }
    fn generation_control_fscc_mut(&mut self) -> &mut GenerationControlFscc {
        self._generation_control_mut().generation_control_fscc.get_or_insert(Default::default())
    }
}
impl IsGenerationControl for GenerationControl {
    fn _generation_control(&self) -> &GenerationControl {
        self
    }
    fn _generation_control_mut(&mut self) -> &mut GenerationControl {
        self
    }
}
impl IsControlValue for GenerationControl {
    fn _control_value(&self) -> &super::commonmodule::ControlValue {
        self.parent()
    }
    fn _control_value_mut(&mut self) -> &mut ControlValue {
        self.parent_mut()
    }
}
impl IsIdentifiedObject for GenerationControl {
    fn _identified_object(&self) -> &super::commonmodule::IdentifiedObject {
        self.parent().parent()
    }
    fn _identified_object_mut(&mut self) -> &mut IdentifiedObject {
        self.parent_mut().parent_mut()
    }
}
/// A single or set of synchronous machines for converting mechanical power into alternating-current
/// power. For example, individual machines within a set may be defined for scheduling purposes while a
/// single control signal is derived for the set. In this case there would be a GeneratingUnit for each
/// member of the set and an additional GeneratingUnit corresponding to the set.
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct GeneratingUnit {
    /// UML inherited base object
    // parent_message: true
    // required_field: false
    // multiplicity_min: None
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, optional, tag="1")]
    pub conducting_equipment: ::std::option::Option<super::commonmodule::ConductingEquipment>,
    /// This is the maximum operating active power limit the dispatcher can enter for this unit.
    #[prost(message, optional, tag="2")]
    pub max_operating_p: ::std::option::Option<super::commonmodule::ActivePower>,
}
mod generating_unit {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref CONDUCTING_EQUIPMENT: crate::commonmodule::ConductingEquipment = Default::default();
        pub(super) static ref MAX_OPERATING_P: crate::commonmodule::ActivePower = Default::default();
    }
}
impl GeneratingUnit {
    pub(crate) fn parent(&self) -> &super::commonmodule::ConductingEquipment {
        self.conducting_equipment.as_ref().unwrap_or(&generating_unit::CONDUCTING_EQUIPMENT)
    }
    pub(crate) fn parent_mut(&mut self) -> &mut super::commonmodule::ConductingEquipment {
        self.conducting_equipment.get_or_insert(Default::default())
    }
}
pub trait IsGeneratingUnit {
    fn _generating_unit(&self) -> &GeneratingUnit;
    fn _generating_unit_mut(&mut self) -> &mut GeneratingUnit;
    fn conducting_equipment(&self) -> &super::commonmodule::ConductingEquipment {
        self._generating_unit().conducting_equipment.as_ref().unwrap_or(&generating_unit::CONDUCTING_EQUIPMENT)
    }
    fn conducting_equipment_mut(&mut self) -> &mut super::commonmodule::ConductingEquipment {
        self._generating_unit_mut().conducting_equipment.get_or_insert(Default::default())
    }
    fn max_operating_p(&self) -> &super::commonmodule::ActivePower {
        self._generating_unit().max_operating_p.as_ref().unwrap_or(&generating_unit::MAX_OPERATING_P)
    }
    fn max_operating_p_mut(&mut self) -> &mut super::commonmodule::ActivePower {
        self._generating_unit_mut().max_operating_p.get_or_insert(Default::default())
    }
}
impl IsGeneratingUnit for GeneratingUnit {
    fn _generating_unit(&self) -> &GeneratingUnit {
        self
    }
    fn _generating_unit_mut(&mut self) -> &mut GeneratingUnit {
        self
    }
}
impl IsConductingEquipment for GeneratingUnit {
    fn _conducting_equipment(&self) -> &super::commonmodule::ConductingEquipment {
        self.parent()
    }
    fn _conducting_equipment_mut(&mut self) -> &mut ConductingEquipment {
        self.parent_mut()
    }
}
impl IsNamedObject for GeneratingUnit {
    fn _named_object(&self) -> &super::commonmodule::NamedObject {
        self.parent().parent()
    }
    fn _named_object_mut(&mut self) -> &mut NamedObject {
        self.parent_mut().parent_mut()
    }
}
/// Generation control profile
/// OpenFMB Profile Message: true
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct GenerationControlProfile {
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
    pub generating_unit: ::std::option::Option<GeneratingUnit>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: Some(1)
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, optional, tag="3")]
    pub generation_control: ::std::option::Option<GenerationControl>,
}
mod generation_control_profile {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref CONTROL_MESSAGE_INFO: crate::commonmodule::ControlMessageInfo = Default::default();
        pub(super) static ref GENERATING_UNIT: crate::generationmodule::GeneratingUnit = Default::default();
        pub(super) static ref GENERATION_CONTROL: crate::generationmodule::GenerationControl = Default::default();
    }
}
impl GenerationControlProfile {
    pub(crate) fn parent(&self) -> &super::commonmodule::ControlMessageInfo {
        self.control_message_info.as_ref().unwrap_or(&generation_control_profile::CONTROL_MESSAGE_INFO)
    }
    pub(crate) fn parent_mut(&mut self) -> &mut super::commonmodule::ControlMessageInfo {
        self.control_message_info.get_or_insert(Default::default())
    }
}
pub trait IsGenerationControlProfile {
    fn _generation_control_profile(&self) -> &GenerationControlProfile;
    fn _generation_control_profile_mut(&mut self) -> &mut GenerationControlProfile;
    fn control_message_info(&self) -> &super::commonmodule::ControlMessageInfo {
        self._generation_control_profile().control_message_info.as_ref().unwrap_or(&generation_control_profile::CONTROL_MESSAGE_INFO)
    }
    fn control_message_info_mut(&mut self) -> &mut super::commonmodule::ControlMessageInfo {
        self._generation_control_profile_mut().control_message_info.get_or_insert(Default::default())
    }
    fn generating_unit(&self) -> &GeneratingUnit {
        self._generation_control_profile().generating_unit.as_ref().unwrap_or(&generation_control_profile::GENERATING_UNIT)
    }
    fn generating_unit_mut(&mut self) -> &mut GeneratingUnit {
        self._generation_control_profile_mut().generating_unit.get_or_insert(Default::default())
    }
    fn generation_control(&self) -> &GenerationControl {
        self._generation_control_profile().generation_control.as_ref().unwrap_or(&generation_control_profile::GENERATION_CONTROL)
    }
    fn generation_control_mut(&mut self) -> &mut GenerationControl {
        self._generation_control_profile_mut().generation_control.get_or_insert(Default::default())
    }
}
impl IsGenerationControlProfile for GenerationControlProfile {
    fn _generation_control_profile(&self) -> &GenerationControlProfile {
        self
    }
    fn _generation_control_profile_mut(&mut self) -> &mut GenerationControlProfile {
        self
    }
}
impl IsControlMessageInfo for GenerationControlProfile {
    fn _control_message_info(&self) -> &super::commonmodule::ControlMessageInfo {
        self.parent()
    }
    fn _control_message_info_mut(&mut self) -> &mut ControlMessageInfo {
        self.parent_mut()
    }
}
impl IsMessageInfo for GenerationControlProfile {
    fn _message_info(&self) -> &super::commonmodule::MessageInfo {
        self.parent().parent()
    }
    fn _message_info_mut(&mut self) -> &mut MessageInfo {
        self.parent_mut().parent_mut()
    }
}
impl IsIdentifiedObject for GenerationControlProfile {
    fn _identified_object(&self) -> &super::commonmodule::IdentifiedObject {
        self.parent().parent().parent()
    }
    fn _identified_object_mut(&mut self) -> &mut IdentifiedObject {
        self.parent_mut().parent_mut().parent_mut()
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct OptionalRealPowerControlKind {
    #[prost(enumeration="RealPowerControlKind", tag="1")]
    pub value: i32,
}
mod optional_real_power_control_kind {
    use lazy_static::lazy_static;
    lazy_static! {
    }
}
impl OptionalRealPowerControlKind {
}
pub trait IsOptionalRealPowerControlKind {
    fn _optional_real_power_control_kind(&self) -> &OptionalRealPowerControlKind;
    fn _optional_real_power_control_kind_mut(&mut self) -> &mut OptionalRealPowerControlKind;
    fn value(&self) -> i32 {
        self._optional_real_power_control_kind().value
    }
    fn value_mut(&mut self) -> &mut i32 {
        &mut self._optional_real_power_control_kind_mut().value
    }
}
impl IsOptionalRealPowerControlKind for OptionalRealPowerControlKind {
    fn _optional_real_power_control_kind(&self) -> &OptionalRealPowerControlKind {
        self
    }
    fn _optional_real_power_control_kind_mut(&mut self) -> &mut OptionalRealPowerControlKind {
        self
    }
}
/// Generation discrete control
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct DroopParameter {
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="1")]
    pub slope: ::std::option::Option<f32>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub unloaded_offset: ::std::option::Option<f32>,
}
mod droop_parameter {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref SLOPE: f32 = Default::default();
        pub(super) static ref UNLOADED_OFFSET: f32 = Default::default();
    }
}
impl DroopParameter {
}
pub trait IsDroopParameter {
    fn _droop_parameter(&self) -> &DroopParameter;
    fn _droop_parameter_mut(&mut self) -> &mut DroopParameter;
    fn slope(&self) -> &f32 {
        self._droop_parameter().slope.as_ref().unwrap_or(&droop_parameter::SLOPE)
    }
    fn slope_mut(&mut self) -> &mut f32 {
        self._droop_parameter_mut().slope.get_or_insert(Default::default())
    }
    fn unloaded_offset(&self) -> &f32 {
        self._droop_parameter().unloaded_offset.as_ref().unwrap_or(&droop_parameter::UNLOADED_OFFSET)
    }
    fn unloaded_offset_mut(&mut self) -> &mut f32 {
        self._droop_parameter_mut().unloaded_offset.get_or_insert(Default::default())
    }
}
impl IsDroopParameter for DroopParameter {
    fn _droop_parameter(&self) -> &DroopParameter {
        self
    }
    fn _droop_parameter_mut(&mut self) -> &mut DroopParameter {
        self
    }
}
/// Generation real power control
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct RealPowerControl {
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="1")]
    pub droop_setpoint: ::std::option::Option<DroopParameter>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub isochronous_setpoint: ::std::option::Option<f32>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub real_power_control_mode: ::std::option::Option<OptionalRealPowerControlKind>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="4")]
    pub real_power_setpoint: ::std::option::Option<f32>,
}
mod real_power_control {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref DROOP_SETPOINT: crate::generationmodule::DroopParameter = Default::default();
        pub(super) static ref ISOCHRONOUS_SETPOINT: f32 = Default::default();
        pub(super) static ref REAL_POWER_CONTROL_MODE: crate::generationmodule::OptionalRealPowerControlKind = Default::default();
        pub(super) static ref REAL_POWER_SETPOINT: f32 = Default::default();
    }
}
impl RealPowerControl {
}
pub trait IsRealPowerControl {
    fn _real_power_control(&self) -> &RealPowerControl;
    fn _real_power_control_mut(&mut self) -> &mut RealPowerControl;
    fn droop_setpoint(&self) -> &DroopParameter {
        self._real_power_control().droop_setpoint.as_ref().unwrap_or(&real_power_control::DROOP_SETPOINT)
    }
    fn droop_setpoint_mut(&mut self) -> &mut DroopParameter {
        self._real_power_control_mut().droop_setpoint.get_or_insert(Default::default())
    }
    fn isochronous_setpoint(&self) -> &f32 {
        self._real_power_control().isochronous_setpoint.as_ref().unwrap_or(&real_power_control::ISOCHRONOUS_SETPOINT)
    }
    fn isochronous_setpoint_mut(&mut self) -> &mut f32 {
        self._real_power_control_mut().isochronous_setpoint.get_or_insert(Default::default())
    }
    fn real_power_control_mode(&self) -> &OptionalRealPowerControlKind {
        self._real_power_control().real_power_control_mode.as_ref().unwrap_or(&real_power_control::REAL_POWER_CONTROL_MODE)
    }
    fn real_power_control_mode_mut(&mut self) -> &mut OptionalRealPowerControlKind {
        self._real_power_control_mut().real_power_control_mode.get_or_insert(Default::default())
    }
    fn real_power_setpoint(&self) -> &f32 {
        self._real_power_control().real_power_setpoint.as_ref().unwrap_or(&real_power_control::REAL_POWER_SETPOINT)
    }
    fn real_power_setpoint_mut(&mut self) -> &mut f32 {
        self._real_power_control_mut().real_power_setpoint.get_or_insert(Default::default())
    }
}
impl IsRealPowerControl for RealPowerControl {
    fn _real_power_control(&self) -> &RealPowerControl {
        self
    }
    fn _real_power_control_mut(&mut self) -> &mut RealPowerControl {
        self
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct OptionalReactivePowerControlKind {
    #[prost(enumeration="ReactivePowerControlKind", tag="1")]
    pub value: i32,
}
mod optional_reactive_power_control_kind {
    use lazy_static::lazy_static;
    lazy_static! {
    }
}
impl OptionalReactivePowerControlKind {
}
pub trait IsOptionalReactivePowerControlKind {
    fn _optional_reactive_power_control_kind(&self) -> &OptionalReactivePowerControlKind;
    fn _optional_reactive_power_control_kind_mut(&mut self) -> &mut OptionalReactivePowerControlKind;
    fn value(&self) -> i32 {
        self._optional_reactive_power_control_kind().value
    }
    fn value_mut(&mut self) -> &mut i32 {
        &mut self._optional_reactive_power_control_kind_mut().value
    }
}
impl IsOptionalReactivePowerControlKind for OptionalReactivePowerControlKind {
    fn _optional_reactive_power_control_kind(&self) -> &OptionalReactivePowerControlKind {
        self
    }
    fn _optional_reactive_power_control_kind_mut(&mut self) -> &mut OptionalReactivePowerControlKind {
        self
    }
}
/// Generation real power control
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ReactivePowerControl {
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="1")]
    pub droop_setpoint: ::std::option::Option<DroopParameter>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub power_factor_setpoint: ::std::option::Option<f32>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub reactive_power_control_mode: ::std::option::Option<OptionalReactivePowerControlKind>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="4")]
    pub reactive_power_setpoint: ::std::option::Option<f32>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="5")]
    pub voltage_setpoint: ::std::option::Option<f32>,
}
mod reactive_power_control {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref DROOP_SETPOINT: crate::generationmodule::DroopParameter = Default::default();
        pub(super) static ref POWER_FACTOR_SETPOINT: f32 = Default::default();
        pub(super) static ref REACTIVE_POWER_CONTROL_MODE: crate::generationmodule::OptionalReactivePowerControlKind = Default::default();
        pub(super) static ref REACTIVE_POWER_SETPOINT: f32 = Default::default();
        pub(super) static ref VOLTAGE_SETPOINT: f32 = Default::default();
    }
}
impl ReactivePowerControl {
}
pub trait IsReactivePowerControl {
    fn _reactive_power_control(&self) -> &ReactivePowerControl;
    fn _reactive_power_control_mut(&mut self) -> &mut ReactivePowerControl;
    fn droop_setpoint(&self) -> &DroopParameter {
        self._reactive_power_control().droop_setpoint.as_ref().unwrap_or(&reactive_power_control::DROOP_SETPOINT)
    }
    fn droop_setpoint_mut(&mut self) -> &mut DroopParameter {
        self._reactive_power_control_mut().droop_setpoint.get_or_insert(Default::default())
    }
    fn power_factor_setpoint(&self) -> &f32 {
        self._reactive_power_control().power_factor_setpoint.as_ref().unwrap_or(&reactive_power_control::POWER_FACTOR_SETPOINT)
    }
    fn power_factor_setpoint_mut(&mut self) -> &mut f32 {
        self._reactive_power_control_mut().power_factor_setpoint.get_or_insert(Default::default())
    }
    fn reactive_power_control_mode(&self) -> &OptionalReactivePowerControlKind {
        self._reactive_power_control().reactive_power_control_mode.as_ref().unwrap_or(&reactive_power_control::REACTIVE_POWER_CONTROL_MODE)
    }
    fn reactive_power_control_mode_mut(&mut self) -> &mut OptionalReactivePowerControlKind {
        self._reactive_power_control_mut().reactive_power_control_mode.get_or_insert(Default::default())
    }
    fn reactive_power_setpoint(&self) -> &f32 {
        self._reactive_power_control().reactive_power_setpoint.as_ref().unwrap_or(&reactive_power_control::REACTIVE_POWER_SETPOINT)
    }
    fn reactive_power_setpoint_mut(&mut self) -> &mut f32 {
        self._reactive_power_control_mut().reactive_power_setpoint.get_or_insert(Default::default())
    }
    fn voltage_setpoint(&self) -> &f32 {
        self._reactive_power_control().voltage_setpoint.as_ref().unwrap_or(&reactive_power_control::VOLTAGE_SETPOINT)
    }
    fn voltage_setpoint_mut(&mut self) -> &mut f32 {
        self._reactive_power_control_mut().voltage_setpoint.get_or_insert(Default::default())
    }
}
impl IsReactivePowerControl for ReactivePowerControl {
    fn _reactive_power_control(&self) -> &ReactivePowerControl {
        self
    }
    fn _reactive_power_control_mut(&mut self) -> &mut ReactivePowerControl {
        self
    }
}
/// Generation discrete control
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct GenerationDiscreteControl {
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
    pub reactive_power_control: ::std::option::Option<ReactivePowerControl>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="4")]
    pub real_power_control: ::std::option::Option<RealPowerControl>,
}
mod generation_discrete_control {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref CONTROL_VALUE: crate::commonmodule::ControlValue = Default::default();
        pub(super) static ref CHECK: crate::commonmodule::CheckConditions = Default::default();
        pub(super) static ref REACTIVE_POWER_CONTROL: crate::generationmodule::ReactivePowerControl = Default::default();
        pub(super) static ref REAL_POWER_CONTROL: crate::generationmodule::RealPowerControl = Default::default();
    }
}
impl GenerationDiscreteControl {
    pub(crate) fn parent(&self) -> &super::commonmodule::ControlValue {
        self.control_value.as_ref().unwrap_or(&generation_discrete_control::CONTROL_VALUE)
    }
    pub(crate) fn parent_mut(&mut self) -> &mut super::commonmodule::ControlValue {
        self.control_value.get_or_insert(Default::default())
    }
}
pub trait IsGenerationDiscreteControl {
    fn _generation_discrete_control(&self) -> &GenerationDiscreteControl;
    fn _generation_discrete_control_mut(&mut self) -> &mut GenerationDiscreteControl;
    fn control_value(&self) -> &super::commonmodule::ControlValue {
        self._generation_discrete_control().control_value.as_ref().unwrap_or(&generation_discrete_control::CONTROL_VALUE)
    }
    fn control_value_mut(&mut self) -> &mut super::commonmodule::ControlValue {
        self._generation_discrete_control_mut().control_value.get_or_insert(Default::default())
    }
    fn check(&self) -> &super::commonmodule::CheckConditions {
        self._generation_discrete_control().check.as_ref().unwrap_or(&generation_discrete_control::CHECK)
    }
    fn check_mut(&mut self) -> &mut super::commonmodule::CheckConditions {
        self._generation_discrete_control_mut().check.get_or_insert(Default::default())
    }
    fn reactive_power_control(&self) -> &ReactivePowerControl {
        self._generation_discrete_control().reactive_power_control.as_ref().unwrap_or(&generation_discrete_control::REACTIVE_POWER_CONTROL)
    }
    fn reactive_power_control_mut(&mut self) -> &mut ReactivePowerControl {
        self._generation_discrete_control_mut().reactive_power_control.get_or_insert(Default::default())
    }
    fn real_power_control(&self) -> &RealPowerControl {
        self._generation_discrete_control().real_power_control.as_ref().unwrap_or(&generation_discrete_control::REAL_POWER_CONTROL)
    }
    fn real_power_control_mut(&mut self) -> &mut RealPowerControl {
        self._generation_discrete_control_mut().real_power_control.get_or_insert(Default::default())
    }
}
impl IsGenerationDiscreteControl for GenerationDiscreteControl {
    fn _generation_discrete_control(&self) -> &GenerationDiscreteControl {
        self
    }
    fn _generation_discrete_control_mut(&mut self) -> &mut GenerationDiscreteControl {
        self
    }
}
impl IsControlValue for GenerationDiscreteControl {
    fn _control_value(&self) -> &super::commonmodule::ControlValue {
        self.parent()
    }
    fn _control_value_mut(&mut self) -> &mut ControlValue {
        self.parent_mut()
    }
}
impl IsIdentifiedObject for GenerationDiscreteControl {
    fn _identified_object(&self) -> &super::commonmodule::IdentifiedObject {
        self.parent().parent()
    }
    fn _identified_object_mut(&mut self) -> &mut IdentifiedObject {
        self.parent_mut().parent_mut()
    }
}
/// Generation discrete control profile
/// OpenFMB Profile Message: true
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct GenerationDiscreteControlProfile {
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
    pub generating_unit: ::std::option::Option<GeneratingUnit>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: Some(1)
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, optional, tag="3")]
    pub generation_discrete_control: ::std::option::Option<GenerationDiscreteControl>,
}
mod generation_discrete_control_profile {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref CONTROL_MESSAGE_INFO: crate::commonmodule::ControlMessageInfo = Default::default();
        pub(super) static ref GENERATING_UNIT: crate::generationmodule::GeneratingUnit = Default::default();
        pub(super) static ref GENERATION_DISCRETE_CONTROL: crate::generationmodule::GenerationDiscreteControl = Default::default();
    }
}
impl GenerationDiscreteControlProfile {
    pub(crate) fn parent(&self) -> &super::commonmodule::ControlMessageInfo {
        self.control_message_info.as_ref().unwrap_or(&generation_discrete_control_profile::CONTROL_MESSAGE_INFO)
    }
    pub(crate) fn parent_mut(&mut self) -> &mut super::commonmodule::ControlMessageInfo {
        self.control_message_info.get_or_insert(Default::default())
    }
}
pub trait IsGenerationDiscreteControlProfile {
    fn _generation_discrete_control_profile(&self) -> &GenerationDiscreteControlProfile;
    fn _generation_discrete_control_profile_mut(&mut self) -> &mut GenerationDiscreteControlProfile;
    fn control_message_info(&self) -> &super::commonmodule::ControlMessageInfo {
        self._generation_discrete_control_profile().control_message_info.as_ref().unwrap_or(&generation_discrete_control_profile::CONTROL_MESSAGE_INFO)
    }
    fn control_message_info_mut(&mut self) -> &mut super::commonmodule::ControlMessageInfo {
        self._generation_discrete_control_profile_mut().control_message_info.get_or_insert(Default::default())
    }
    fn generating_unit(&self) -> &GeneratingUnit {
        self._generation_discrete_control_profile().generating_unit.as_ref().unwrap_or(&generation_discrete_control_profile::GENERATING_UNIT)
    }
    fn generating_unit_mut(&mut self) -> &mut GeneratingUnit {
        self._generation_discrete_control_profile_mut().generating_unit.get_or_insert(Default::default())
    }
    fn generation_discrete_control(&self) -> &GenerationDiscreteControl {
        self._generation_discrete_control_profile().generation_discrete_control.as_ref().unwrap_or(&generation_discrete_control_profile::GENERATION_DISCRETE_CONTROL)
    }
    fn generation_discrete_control_mut(&mut self) -> &mut GenerationDiscreteControl {
        self._generation_discrete_control_profile_mut().generation_discrete_control.get_or_insert(Default::default())
    }
}
impl IsGenerationDiscreteControlProfile for GenerationDiscreteControlProfile {
    fn _generation_discrete_control_profile(&self) -> &GenerationDiscreteControlProfile {
        self
    }
    fn _generation_discrete_control_profile_mut(&mut self) -> &mut GenerationDiscreteControlProfile {
        self
    }
}
impl IsControlMessageInfo for GenerationDiscreteControlProfile {
    fn _control_message_info(&self) -> &super::commonmodule::ControlMessageInfo {
        self.parent()
    }
    fn _control_message_info_mut(&mut self) -> &mut ControlMessageInfo {
        self.parent_mut()
    }
}
impl IsMessageInfo for GenerationDiscreteControlProfile {
    fn _message_info(&self) -> &super::commonmodule::MessageInfo {
        self.parent().parent()
    }
    fn _message_info_mut(&mut self) -> &mut MessageInfo {
        self.parent_mut().parent_mut()
    }
}
impl IsIdentifiedObject for GenerationDiscreteControlProfile {
    fn _identified_object(&self) -> &super::commonmodule::IdentifiedObject {
        self.parent().parent().parent()
    }
    fn _identified_object_mut(&mut self) -> &mut IdentifiedObject {
        self.parent_mut().parent_mut().parent_mut()
    }
}
/// Generation reading value
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct GenerationReading {
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
mod generation_reading {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref CONDUCTING_EQUIPMENT_TERMINAL_READING: crate::commonmodule::ConductingEquipmentTerminalReading = Default::default();
        pub(super) static ref PHASE_MMTN: crate::commonmodule::PhaseMmtn = Default::default();
        pub(super) static ref READING_MMTR: crate::commonmodule::ReadingMmtr = Default::default();
        pub(super) static ref READING_MMXU: crate::commonmodule::ReadingMmxu = Default::default();
    }
}
impl GenerationReading {
    pub(crate) fn parent(&self) -> &super::commonmodule::ConductingEquipmentTerminalReading {
        self.conducting_equipment_terminal_reading.as_ref().unwrap_or(&generation_reading::CONDUCTING_EQUIPMENT_TERMINAL_READING)
    }
    pub(crate) fn parent_mut(&mut self) -> &mut super::commonmodule::ConductingEquipmentTerminalReading {
        self.conducting_equipment_terminal_reading.get_or_insert(Default::default())
    }
}
pub trait IsGenerationReading {
    fn _generation_reading(&self) -> &GenerationReading;
    fn _generation_reading_mut(&mut self) -> &mut GenerationReading;
    fn conducting_equipment_terminal_reading(&self) -> &super::commonmodule::ConductingEquipmentTerminalReading {
        self._generation_reading().conducting_equipment_terminal_reading.as_ref().unwrap_or(&generation_reading::CONDUCTING_EQUIPMENT_TERMINAL_READING)
    }
    fn conducting_equipment_terminal_reading_mut(&mut self) -> &mut super::commonmodule::ConductingEquipmentTerminalReading {
        self._generation_reading_mut().conducting_equipment_terminal_reading.get_or_insert(Default::default())
    }
    fn phase_mmtn(&self) -> &super::commonmodule::PhaseMmtn {
        self._generation_reading().phase_mmtn.as_ref().unwrap_or(&generation_reading::PHASE_MMTN)
    }
    fn phase_mmtn_mut(&mut self) -> &mut super::commonmodule::PhaseMmtn {
        self._generation_reading_mut().phase_mmtn.get_or_insert(Default::default())
    }
    fn reading_mmtr(&self) -> &super::commonmodule::ReadingMmtr {
        self._generation_reading().reading_mmtr.as_ref().unwrap_or(&generation_reading::READING_MMTR)
    }
    fn reading_mmtr_mut(&mut self) -> &mut super::commonmodule::ReadingMmtr {
        self._generation_reading_mut().reading_mmtr.get_or_insert(Default::default())
    }
    fn reading_mmxu(&self) -> &super::commonmodule::ReadingMmxu {
        self._generation_reading().reading_mmxu.as_ref().unwrap_or(&generation_reading::READING_MMXU)
    }
    fn reading_mmxu_mut(&mut self) -> &mut super::commonmodule::ReadingMmxu {
        self._generation_reading_mut().reading_mmxu.get_or_insert(Default::default())
    }
}
impl IsGenerationReading for GenerationReading {
    fn _generation_reading(&self) -> &GenerationReading {
        self
    }
    fn _generation_reading_mut(&mut self) -> &mut GenerationReading {
        self
    }
}
impl IsConductingEquipmentTerminalReading for GenerationReading {
    fn _conducting_equipment_terminal_reading(&self) -> &super::commonmodule::ConductingEquipmentTerminalReading {
        self.parent()
    }
    fn _conducting_equipment_terminal_reading_mut(&mut self) -> &mut ConductingEquipmentTerminalReading {
        self.parent_mut()
    }
}
/// Generation reading profile
/// OpenFMB Profile Message: true
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct GenerationReadingProfile {
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
    pub generating_unit: ::std::option::Option<GeneratingUnit>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: Some(1)
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, optional, tag="3")]
    pub generation_reading: ::std::option::Option<GenerationReading>,
}
mod generation_reading_profile {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref READING_MESSAGE_INFO: crate::commonmodule::ReadingMessageInfo = Default::default();
        pub(super) static ref GENERATING_UNIT: crate::generationmodule::GeneratingUnit = Default::default();
        pub(super) static ref GENERATION_READING: crate::generationmodule::GenerationReading = Default::default();
    }
}
impl GenerationReadingProfile {
    pub(crate) fn parent(&self) -> &super::commonmodule::ReadingMessageInfo {
        self.reading_message_info.as_ref().unwrap_or(&generation_reading_profile::READING_MESSAGE_INFO)
    }
    pub(crate) fn parent_mut(&mut self) -> &mut super::commonmodule::ReadingMessageInfo {
        self.reading_message_info.get_or_insert(Default::default())
    }
}
pub trait IsGenerationReadingProfile {
    fn _generation_reading_profile(&self) -> &GenerationReadingProfile;
    fn _generation_reading_profile_mut(&mut self) -> &mut GenerationReadingProfile;
    fn reading_message_info(&self) -> &super::commonmodule::ReadingMessageInfo {
        self._generation_reading_profile().reading_message_info.as_ref().unwrap_or(&generation_reading_profile::READING_MESSAGE_INFO)
    }
    fn reading_message_info_mut(&mut self) -> &mut super::commonmodule::ReadingMessageInfo {
        self._generation_reading_profile_mut().reading_message_info.get_or_insert(Default::default())
    }
    fn generating_unit(&self) -> &GeneratingUnit {
        self._generation_reading_profile().generating_unit.as_ref().unwrap_or(&generation_reading_profile::GENERATING_UNIT)
    }
    fn generating_unit_mut(&mut self) -> &mut GeneratingUnit {
        self._generation_reading_profile_mut().generating_unit.get_or_insert(Default::default())
    }
    fn generation_reading(&self) -> &GenerationReading {
        self._generation_reading_profile().generation_reading.as_ref().unwrap_or(&generation_reading_profile::GENERATION_READING)
    }
    fn generation_reading_mut(&mut self) -> &mut GenerationReading {
        self._generation_reading_profile_mut().generation_reading.get_or_insert(Default::default())
    }
}
impl IsGenerationReadingProfile for GenerationReadingProfile {
    fn _generation_reading_profile(&self) -> &GenerationReadingProfile {
        self
    }
    fn _generation_reading_profile_mut(&mut self) -> &mut GenerationReadingProfile {
        self
    }
}
impl IsReadingMessageInfo for GenerationReadingProfile {
    fn _reading_message_info(&self) -> &super::commonmodule::ReadingMessageInfo {
        self.parent()
    }
    fn _reading_message_info_mut(&mut self) -> &mut ReadingMessageInfo {
        self.parent_mut()
    }
}
impl IsMessageInfo for GenerationReadingProfile {
    fn _message_info(&self) -> &super::commonmodule::MessageInfo {
        self.parent().parent()
    }
    fn _message_info_mut(&mut self) -> &mut MessageInfo {
        self.parent_mut().parent_mut()
    }
}
impl IsIdentifiedObject for GenerationReadingProfile {
    fn _identified_object(&self) -> &super::commonmodule::IdentifiedObject {
        self.parent().parent().parent()
    }
    fn _identified_object_mut(&mut self) -> &mut IdentifiedObject {
        self.parent_mut().parent_mut().parent_mut()
    }
}
/// Point definition (Point)
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct GenerationPointStatus {
    /// Black start enable
    #[prost(message, optional, tag="1")]
    pub black_start_enabled: ::std::option::Option<super::commonmodule::StatusDps>,
    /// Enable frequency set point
    #[prost(message, optional, tag="2")]
    pub frequency_set_point_enabled: ::std::option::Option<super::commonmodule::StatusDps>,
    /// Black start enable
    #[prost(message, optional, tag="3")]
    pub pct_hz_droop: ::std::option::Option<f32>,
    /// Black start enable
    #[prost(message, optional, tag="4")]
    pub pct_v_droop: ::std::option::Option<f32>,
    /// Ramp rates
    #[prost(message, optional, tag="5")]
    pub ramp_rates: ::std::option::Option<super::commonmodule::RampRate>,
    /// Enable reactive power set point
    #[prost(message, optional, tag="6")]
    pub reactive_pwr_set_point_enabled: ::std::option::Option<super::commonmodule::StatusDps>,
    /// Enable real power set point
    #[prost(message, optional, tag="7")]
    pub real_pwr_set_point_enabled: ::std::option::Option<super::commonmodule::StatusDps>,
    /// ESS state
    #[prost(message, optional, tag="8")]
    pub state: ::std::option::Option<super::commonmodule::OptionalStateKind>,
    /// Synchronize back to grid
    #[prost(message, optional, tag="9")]
    pub sync_back_to_grid: ::std::option::Option<super::commonmodule::StatusDps>,
    /// Transition to island on grid loss enable
    #[prost(message, optional, tag="10")]
    pub trans_to_islnd_on_grid_loss_enabled: ::std::option::Option<super::commonmodule::StatusDps>,
    /// Enable voltage set point
    #[prost(message, optional, tag="11")]
    pub voltage_set_point_enabled: ::std::option::Option<super::commonmodule::StatusDps>,
}
mod generation_point_status {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref BLACK_START_ENABLED: crate::commonmodule::StatusDps = Default::default();
        pub(super) static ref FREQUENCY_SET_POINT_ENABLED: crate::commonmodule::StatusDps = Default::default();
        pub(super) static ref PCT_HZ_DROOP: f32 = Default::default();
        pub(super) static ref PCT_V_DROOP: f32 = Default::default();
        pub(super) static ref RAMP_RATES: crate::commonmodule::RampRate = Default::default();
        pub(super) static ref REACTIVE_PWR_SET_POINT_ENABLED: crate::commonmodule::StatusDps = Default::default();
        pub(super) static ref REAL_PWR_SET_POINT_ENABLED: crate::commonmodule::StatusDps = Default::default();
        pub(super) static ref STATE: crate::commonmodule::OptionalStateKind = Default::default();
        pub(super) static ref SYNC_BACK_TO_GRID: crate::commonmodule::StatusDps = Default::default();
        pub(super) static ref TRANS_TO_ISLND_ON_GRID_LOSS_ENABLED: crate::commonmodule::StatusDps = Default::default();
        pub(super) static ref VOLTAGE_SET_POINT_ENABLED: crate::commonmodule::StatusDps = Default::default();
    }
}
impl GenerationPointStatus {
}
pub trait IsGenerationPointStatus {
    fn _generation_point_status(&self) -> &GenerationPointStatus;
    fn _generation_point_status_mut(&mut self) -> &mut GenerationPointStatus;
    fn black_start_enabled(&self) -> &super::commonmodule::StatusDps {
        self._generation_point_status().black_start_enabled.as_ref().unwrap_or(&generation_point_status::BLACK_START_ENABLED)
    }
    fn black_start_enabled_mut(&mut self) -> &mut super::commonmodule::StatusDps {
        self._generation_point_status_mut().black_start_enabled.get_or_insert(Default::default())
    }
    fn frequency_set_point_enabled(&self) -> &super::commonmodule::StatusDps {
        self._generation_point_status().frequency_set_point_enabled.as_ref().unwrap_or(&generation_point_status::FREQUENCY_SET_POINT_ENABLED)
    }
    fn frequency_set_point_enabled_mut(&mut self) -> &mut super::commonmodule::StatusDps {
        self._generation_point_status_mut().frequency_set_point_enabled.get_or_insert(Default::default())
    }
    fn pct_hz_droop(&self) -> &f32 {
        self._generation_point_status().pct_hz_droop.as_ref().unwrap_or(&generation_point_status::PCT_HZ_DROOP)
    }
    fn pct_hz_droop_mut(&mut self) -> &mut f32 {
        self._generation_point_status_mut().pct_hz_droop.get_or_insert(Default::default())
    }
    fn pct_v_droop(&self) -> &f32 {
        self._generation_point_status().pct_v_droop.as_ref().unwrap_or(&generation_point_status::PCT_V_DROOP)
    }
    fn pct_v_droop_mut(&mut self) -> &mut f32 {
        self._generation_point_status_mut().pct_v_droop.get_or_insert(Default::default())
    }
    fn ramp_rates(&self) -> &super::commonmodule::RampRate {
        self._generation_point_status().ramp_rates.as_ref().unwrap_or(&generation_point_status::RAMP_RATES)
    }
    fn ramp_rates_mut(&mut self) -> &mut super::commonmodule::RampRate {
        self._generation_point_status_mut().ramp_rates.get_or_insert(Default::default())
    }
    fn reactive_pwr_set_point_enabled(&self) -> &super::commonmodule::StatusDps {
        self._generation_point_status().reactive_pwr_set_point_enabled.as_ref().unwrap_or(&generation_point_status::REACTIVE_PWR_SET_POINT_ENABLED)
    }
    fn reactive_pwr_set_point_enabled_mut(&mut self) -> &mut super::commonmodule::StatusDps {
        self._generation_point_status_mut().reactive_pwr_set_point_enabled.get_or_insert(Default::default())
    }
    fn real_pwr_set_point_enabled(&self) -> &super::commonmodule::StatusDps {
        self._generation_point_status().real_pwr_set_point_enabled.as_ref().unwrap_or(&generation_point_status::REAL_PWR_SET_POINT_ENABLED)
    }
    fn real_pwr_set_point_enabled_mut(&mut self) -> &mut super::commonmodule::StatusDps {
        self._generation_point_status_mut().real_pwr_set_point_enabled.get_or_insert(Default::default())
    }
    fn state(&self) -> &super::commonmodule::OptionalStateKind {
        self._generation_point_status().state.as_ref().unwrap_or(&generation_point_status::STATE)
    }
    fn state_mut(&mut self) -> &mut super::commonmodule::OptionalStateKind {
        self._generation_point_status_mut().state.get_or_insert(Default::default())
    }
    fn sync_back_to_grid(&self) -> &super::commonmodule::StatusDps {
        self._generation_point_status().sync_back_to_grid.as_ref().unwrap_or(&generation_point_status::SYNC_BACK_TO_GRID)
    }
    fn sync_back_to_grid_mut(&mut self) -> &mut super::commonmodule::StatusDps {
        self._generation_point_status_mut().sync_back_to_grid.get_or_insert(Default::default())
    }
    fn trans_to_islnd_on_grid_loss_enabled(&self) -> &super::commonmodule::StatusDps {
        self._generation_point_status().trans_to_islnd_on_grid_loss_enabled.as_ref().unwrap_or(&generation_point_status::TRANS_TO_ISLND_ON_GRID_LOSS_ENABLED)
    }
    fn trans_to_islnd_on_grid_loss_enabled_mut(&mut self) -> &mut super::commonmodule::StatusDps {
        self._generation_point_status_mut().trans_to_islnd_on_grid_loss_enabled.get_or_insert(Default::default())
    }
    fn voltage_set_point_enabled(&self) -> &super::commonmodule::StatusDps {
        self._generation_point_status().voltage_set_point_enabled.as_ref().unwrap_or(&generation_point_status::VOLTAGE_SET_POINT_ENABLED)
    }
    fn voltage_set_point_enabled_mut(&mut self) -> &mut super::commonmodule::StatusDps {
        self._generation_point_status_mut().voltage_set_point_enabled.get_or_insert(Default::default())
    }
}
impl IsGenerationPointStatus for GenerationPointStatus {
    fn _generation_point_status(&self) -> &GenerationPointStatus {
        self
    }
    fn _generation_point_status_mut(&mut self) -> &mut GenerationPointStatus {
        self
    }
}
/// Specialized 61850 ZGEN class
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct GenerationEventAndStatusZgen {
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
    pub aux_pwr_st: ::std::option::Option<super::commonmodule::StatusSps>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub dynamic_test: ::std::option::Option<super::commonmodule::EnsDynamicTestKind>,
    /// Emergency stop
    #[prost(message, optional, tag="4")]
    pub emg_stop: ::std::option::Option<super::commonmodule::StatusSps>,
    /// Generator is synchronized to EPS, or not; True = Synchronized
    #[prost(message, optional, tag="5")]
    pub gn_syn_st: ::std::option::Option<super::commonmodule::StatusSps>,
    /// Point status
    #[prost(message, optional, tag="6")]
    pub point_status: ::std::option::Option<GenerationPointStatus>,
}
mod generation_event_and_status_zgen {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref LOGICAL_NODE_FOR_EVENT_AND_STATUS: crate::commonmodule::LogicalNodeForEventAndStatus = Default::default();
        pub(super) static ref AUX_PWR_ST: crate::commonmodule::StatusSps = Default::default();
        pub(super) static ref DYNAMIC_TEST: crate::commonmodule::EnsDynamicTestKind = Default::default();
        pub(super) static ref EMG_STOP: crate::commonmodule::StatusSps = Default::default();
        pub(super) static ref GN_SYN_ST: crate::commonmodule::StatusSps = Default::default();
        pub(super) static ref POINT_STATUS: crate::generationmodule::GenerationPointStatus = Default::default();
    }
}
impl GenerationEventAndStatusZgen {
    pub(crate) fn parent(&self) -> &super::commonmodule::LogicalNodeForEventAndStatus {
        self.logical_node_for_event_and_status.as_ref().unwrap_or(&generation_event_and_status_zgen::LOGICAL_NODE_FOR_EVENT_AND_STATUS)
    }
    pub(crate) fn parent_mut(&mut self) -> &mut super::commonmodule::LogicalNodeForEventAndStatus {
        self.logical_node_for_event_and_status.get_or_insert(Default::default())
    }
}
pub trait IsGenerationEventAndStatusZgen {
    fn _generation_event_and_status_zgen(&self) -> &GenerationEventAndStatusZgen;
    fn _generation_event_and_status_zgen_mut(&mut self) -> &mut GenerationEventAndStatusZgen;
    fn logical_node_for_event_and_status(&self) -> &super::commonmodule::LogicalNodeForEventAndStatus {
        self._generation_event_and_status_zgen().logical_node_for_event_and_status.as_ref().unwrap_or(&generation_event_and_status_zgen::LOGICAL_NODE_FOR_EVENT_AND_STATUS)
    }
    fn logical_node_for_event_and_status_mut(&mut self) -> &mut super::commonmodule::LogicalNodeForEventAndStatus {
        self._generation_event_and_status_zgen_mut().logical_node_for_event_and_status.get_or_insert(Default::default())
    }
    fn aux_pwr_st(&self) -> &super::commonmodule::StatusSps {
        self._generation_event_and_status_zgen().aux_pwr_st.as_ref().unwrap_or(&generation_event_and_status_zgen::AUX_PWR_ST)
    }
    fn aux_pwr_st_mut(&mut self) -> &mut super::commonmodule::StatusSps {
        self._generation_event_and_status_zgen_mut().aux_pwr_st.get_or_insert(Default::default())
    }
    fn dynamic_test(&self) -> &super::commonmodule::EnsDynamicTestKind {
        self._generation_event_and_status_zgen().dynamic_test.as_ref().unwrap_or(&generation_event_and_status_zgen::DYNAMIC_TEST)
    }
    fn dynamic_test_mut(&mut self) -> &mut super::commonmodule::EnsDynamicTestKind {
        self._generation_event_and_status_zgen_mut().dynamic_test.get_or_insert(Default::default())
    }
    fn emg_stop(&self) -> &super::commonmodule::StatusSps {
        self._generation_event_and_status_zgen().emg_stop.as_ref().unwrap_or(&generation_event_and_status_zgen::EMG_STOP)
    }
    fn emg_stop_mut(&mut self) -> &mut super::commonmodule::StatusSps {
        self._generation_event_and_status_zgen_mut().emg_stop.get_or_insert(Default::default())
    }
    fn gn_syn_st(&self) -> &super::commonmodule::StatusSps {
        self._generation_event_and_status_zgen().gn_syn_st.as_ref().unwrap_or(&generation_event_and_status_zgen::GN_SYN_ST)
    }
    fn gn_syn_st_mut(&mut self) -> &mut super::commonmodule::StatusSps {
        self._generation_event_and_status_zgen_mut().gn_syn_st.get_or_insert(Default::default())
    }
    fn point_status(&self) -> &GenerationPointStatus {
        self._generation_event_and_status_zgen().point_status.as_ref().unwrap_or(&generation_event_and_status_zgen::POINT_STATUS)
    }
    fn point_status_mut(&mut self) -> &mut GenerationPointStatus {
        self._generation_event_and_status_zgen_mut().point_status.get_or_insert(Default::default())
    }
}
impl IsGenerationEventAndStatusZgen for GenerationEventAndStatusZgen {
    fn _generation_event_and_status_zgen(&self) -> &GenerationEventAndStatusZgen {
        self
    }
    fn _generation_event_and_status_zgen_mut(&mut self) -> &mut GenerationEventAndStatusZgen {
        self
    }
}
impl IsLogicalNodeForEventAndStatus for GenerationEventAndStatusZgen {
    fn _logical_node_for_event_and_status(&self) -> &super::commonmodule::LogicalNodeForEventAndStatus {
        self.parent()
    }
    fn _logical_node_for_event_and_status_mut(&mut self) -> &mut LogicalNodeForEventAndStatus {
        self.parent_mut()
    }
}
impl IsLogicalNode for GenerationEventAndStatusZgen {
    fn _logical_node(&self) -> &super::commonmodule::LogicalNode {
        self.parent().parent()
    }
    fn _logical_node_mut(&mut self) -> &mut LogicalNode {
        self.parent_mut().parent_mut()
    }
}
impl IsIdentifiedObject for GenerationEventAndStatusZgen {
    fn _identified_object(&self) -> &super::commonmodule::IdentifiedObject {
        self.parent().parent().parent()
    }
    fn _identified_object_mut(&mut self) -> &mut IdentifiedObject {
        self.parent_mut().parent_mut().parent_mut()
    }
}
/// Specialized generation event ZGEN
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct GenerationEventZgen {
    /// UML inherited base object
    // parent_message: true
    // required_field: false
    // multiplicity_min: None
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, optional, tag="1")]
    pub generation_event_and_status_zgen: ::std::option::Option<GenerationEventAndStatusZgen>,
}
mod generation_event_zgen {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref GENERATION_EVENT_AND_STATUS_ZGEN: crate::generationmodule::GenerationEventAndStatusZgen = Default::default();
    }
}
impl GenerationEventZgen {
    pub(crate) fn parent(&self) -> &GenerationEventAndStatusZgen {
        self.generation_event_and_status_zgen.as_ref().unwrap_or(&generation_event_zgen::GENERATION_EVENT_AND_STATUS_ZGEN)
    }
    pub(crate) fn parent_mut(&mut self) -> &mut GenerationEventAndStatusZgen {
        self.generation_event_and_status_zgen.get_or_insert(Default::default())
    }
}
pub trait IsGenerationEventZgen {
    fn _generation_event_zgen(&self) -> &GenerationEventZgen;
    fn _generation_event_zgen_mut(&mut self) -> &mut GenerationEventZgen;
    fn generation_event_and_status_zgen(&self) -> &GenerationEventAndStatusZgen {
        self._generation_event_zgen().generation_event_and_status_zgen.as_ref().unwrap_or(&generation_event_zgen::GENERATION_EVENT_AND_STATUS_ZGEN)
    }
    fn generation_event_and_status_zgen_mut(&mut self) -> &mut GenerationEventAndStatusZgen {
        self._generation_event_zgen_mut().generation_event_and_status_zgen.get_or_insert(Default::default())
    }
}
impl IsGenerationEventZgen for GenerationEventZgen {
    fn _generation_event_zgen(&self) -> &GenerationEventZgen {
        self
    }
    fn _generation_event_zgen_mut(&mut self) -> &mut GenerationEventZgen {
        self
    }
}
impl IsGenerationEventAndStatusZgen for GenerationEventZgen {
    fn _generation_event_and_status_zgen(&self) -> &GenerationEventAndStatusZgen {
        self.parent()
    }
    fn _generation_event_and_status_zgen_mut(&mut self) -> &mut GenerationEventAndStatusZgen {
        self.parent_mut()
    }
}
impl IsLogicalNodeForEventAndStatus for GenerationEventZgen {
    fn _logical_node_for_event_and_status(&self) -> &super::commonmodule::LogicalNodeForEventAndStatus {
        self.parent().parent()
    }
    fn _logical_node_for_event_and_status_mut(&mut self) -> &mut LogicalNodeForEventAndStatus {
        self.parent_mut().parent_mut()
    }
}
impl IsLogicalNode for GenerationEventZgen {
    fn _logical_node(&self) -> &super::commonmodule::LogicalNode {
        self.parent().parent().parent()
    }
    fn _logical_node_mut(&mut self) -> &mut LogicalNode {
        self.parent_mut().parent_mut().parent_mut()
    }
}
impl IsIdentifiedObject for GenerationEventZgen {
    fn _identified_object(&self) -> &super::commonmodule::IdentifiedObject {
        self.parent().parent().parent().parent()
    }
    fn _identified_object_mut(&mut self) -> &mut IdentifiedObject {
        self.parent_mut().parent_mut().parent_mut().parent_mut()
    }
}
/// Generation event
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct GenerationEvent {
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
    pub generation_event_zgen: ::std::option::Option<GenerationEventZgen>,
}
mod generation_event {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref EVENT_VALUE: crate::commonmodule::EventValue = Default::default();
        pub(super) static ref GENERATION_EVENT_ZGEN: crate::generationmodule::GenerationEventZgen = Default::default();
    }
}
impl GenerationEvent {
    pub(crate) fn parent(&self) -> &super::commonmodule::EventValue {
        self.event_value.as_ref().unwrap_or(&generation_event::EVENT_VALUE)
    }
    pub(crate) fn parent_mut(&mut self) -> &mut super::commonmodule::EventValue {
        self.event_value.get_or_insert(Default::default())
    }
}
pub trait IsGenerationEvent {
    fn _generation_event(&self) -> &GenerationEvent;
    fn _generation_event_mut(&mut self) -> &mut GenerationEvent;
    fn event_value(&self) -> &super::commonmodule::EventValue {
        self._generation_event().event_value.as_ref().unwrap_or(&generation_event::EVENT_VALUE)
    }
    fn event_value_mut(&mut self) -> &mut super::commonmodule::EventValue {
        self._generation_event_mut().event_value.get_or_insert(Default::default())
    }
    fn generation_event_zgen(&self) -> &GenerationEventZgen {
        self._generation_event().generation_event_zgen.as_ref().unwrap_or(&generation_event::GENERATION_EVENT_ZGEN)
    }
    fn generation_event_zgen_mut(&mut self) -> &mut GenerationEventZgen {
        self._generation_event_mut().generation_event_zgen.get_or_insert(Default::default())
    }
}
impl IsGenerationEvent for GenerationEvent {
    fn _generation_event(&self) -> &GenerationEvent {
        self
    }
    fn _generation_event_mut(&mut self) -> &mut GenerationEvent {
        self
    }
}
impl IsEventValue for GenerationEvent {
    fn _event_value(&self) -> &super::commonmodule::EventValue {
        self.parent()
    }
    fn _event_value_mut(&mut self) -> &mut EventValue {
        self.parent_mut()
    }
}
impl IsIdentifiedObject for GenerationEvent {
    fn _identified_object(&self) -> &super::commonmodule::IdentifiedObject {
        self.parent().parent()
    }
    fn _identified_object_mut(&mut self) -> &mut IdentifiedObject {
        self.parent_mut().parent_mut()
    }
}
/// Generation event profile
/// OpenFMB Profile Message: true
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct GenerationEventProfile {
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
    pub generating_unit: ::std::option::Option<GeneratingUnit>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: Some(1)
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, optional, tag="3")]
    pub generation_event: ::std::option::Option<GenerationEvent>,
}
mod generation_event_profile {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref EVENT_MESSAGE_INFO: crate::commonmodule::EventMessageInfo = Default::default();
        pub(super) static ref GENERATING_UNIT: crate::generationmodule::GeneratingUnit = Default::default();
        pub(super) static ref GENERATION_EVENT: crate::generationmodule::GenerationEvent = Default::default();
    }
}
impl GenerationEventProfile {
    pub(crate) fn parent(&self) -> &super::commonmodule::EventMessageInfo {
        self.event_message_info.as_ref().unwrap_or(&generation_event_profile::EVENT_MESSAGE_INFO)
    }
    pub(crate) fn parent_mut(&mut self) -> &mut super::commonmodule::EventMessageInfo {
        self.event_message_info.get_or_insert(Default::default())
    }
}
pub trait IsGenerationEventProfile {
    fn _generation_event_profile(&self) -> &GenerationEventProfile;
    fn _generation_event_profile_mut(&mut self) -> &mut GenerationEventProfile;
    fn event_message_info(&self) -> &super::commonmodule::EventMessageInfo {
        self._generation_event_profile().event_message_info.as_ref().unwrap_or(&generation_event_profile::EVENT_MESSAGE_INFO)
    }
    fn event_message_info_mut(&mut self) -> &mut super::commonmodule::EventMessageInfo {
        self._generation_event_profile_mut().event_message_info.get_or_insert(Default::default())
    }
    fn generating_unit(&self) -> &GeneratingUnit {
        self._generation_event_profile().generating_unit.as_ref().unwrap_or(&generation_event_profile::GENERATING_UNIT)
    }
    fn generating_unit_mut(&mut self) -> &mut GeneratingUnit {
        self._generation_event_profile_mut().generating_unit.get_or_insert(Default::default())
    }
    fn generation_event(&self) -> &GenerationEvent {
        self._generation_event_profile().generation_event.as_ref().unwrap_or(&generation_event_profile::GENERATION_EVENT)
    }
    fn generation_event_mut(&mut self) -> &mut GenerationEvent {
        self._generation_event_profile_mut().generation_event.get_or_insert(Default::default())
    }
}
impl IsGenerationEventProfile for GenerationEventProfile {
    fn _generation_event_profile(&self) -> &GenerationEventProfile {
        self
    }
    fn _generation_event_profile_mut(&mut self) -> &mut GenerationEventProfile {
        self
    }
}
impl IsEventMessageInfo for GenerationEventProfile {
    fn _event_message_info(&self) -> &super::commonmodule::EventMessageInfo {
        self.parent()
    }
    fn _event_message_info_mut(&mut self) -> &mut EventMessageInfo {
        self.parent_mut()
    }
}
impl IsMessageInfo for GenerationEventProfile {
    fn _message_info(&self) -> &super::commonmodule::MessageInfo {
        self.parent().parent()
    }
    fn _message_info_mut(&mut self) -> &mut MessageInfo {
        self.parent_mut().parent_mut()
    }
}
impl IsIdentifiedObject for GenerationEventProfile {
    fn _identified_object(&self) -> &super::commonmodule::IdentifiedObject {
        self.parent().parent().parent()
    }
    fn _identified_object_mut(&mut self) -> &mut IdentifiedObject {
        self.parent_mut().parent_mut().parent_mut()
    }
}
/// Specialized 61850 ZGEN class
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct GenerationStatusZgen {
    /// UML inherited base object
    // parent_message: true
    // required_field: false
    // multiplicity_min: None
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, optional, tag="1")]
    pub generation_event_and_status_zgen: ::std::option::Option<GenerationEventAndStatusZgen>,
}
mod generation_status_zgen {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref GENERATION_EVENT_AND_STATUS_ZGEN: crate::generationmodule::GenerationEventAndStatusZgen = Default::default();
    }
}
impl GenerationStatusZgen {
    pub(crate) fn parent(&self) -> &GenerationEventAndStatusZgen {
        self.generation_event_and_status_zgen.as_ref().unwrap_or(&generation_status_zgen::GENERATION_EVENT_AND_STATUS_ZGEN)
    }
    pub(crate) fn parent_mut(&mut self) -> &mut GenerationEventAndStatusZgen {
        self.generation_event_and_status_zgen.get_or_insert(Default::default())
    }
}
pub trait IsGenerationStatusZgen {
    fn _generation_status_zgen(&self) -> &GenerationStatusZgen;
    fn _generation_status_zgen_mut(&mut self) -> &mut GenerationStatusZgen;
    fn generation_event_and_status_zgen(&self) -> &GenerationEventAndStatusZgen {
        self._generation_status_zgen().generation_event_and_status_zgen.as_ref().unwrap_or(&generation_status_zgen::GENERATION_EVENT_AND_STATUS_ZGEN)
    }
    fn generation_event_and_status_zgen_mut(&mut self) -> &mut GenerationEventAndStatusZgen {
        self._generation_status_zgen_mut().generation_event_and_status_zgen.get_or_insert(Default::default())
    }
}
impl IsGenerationStatusZgen for GenerationStatusZgen {
    fn _generation_status_zgen(&self) -> &GenerationStatusZgen {
        self
    }
    fn _generation_status_zgen_mut(&mut self) -> &mut GenerationStatusZgen {
        self
    }
}
impl IsGenerationEventAndStatusZgen for GenerationStatusZgen {
    fn _generation_event_and_status_zgen(&self) -> &GenerationEventAndStatusZgen {
        self.parent()
    }
    fn _generation_event_and_status_zgen_mut(&mut self) -> &mut GenerationEventAndStatusZgen {
        self.parent_mut()
    }
}
impl IsLogicalNodeForEventAndStatus for GenerationStatusZgen {
    fn _logical_node_for_event_and_status(&self) -> &super::commonmodule::LogicalNodeForEventAndStatus {
        self.parent().parent()
    }
    fn _logical_node_for_event_and_status_mut(&mut self) -> &mut LogicalNodeForEventAndStatus {
        self.parent_mut().parent_mut()
    }
}
impl IsLogicalNode for GenerationStatusZgen {
    fn _logical_node(&self) -> &super::commonmodule::LogicalNode {
        self.parent().parent().parent()
    }
    fn _logical_node_mut(&mut self) -> &mut LogicalNode {
        self.parent_mut().parent_mut().parent_mut()
    }
}
impl IsIdentifiedObject for GenerationStatusZgen {
    fn _identified_object(&self) -> &super::commonmodule::IdentifiedObject {
        self.parent().parent().parent().parent()
    }
    fn _identified_object_mut(&mut self) -> &mut IdentifiedObject {
        self.parent_mut().parent_mut().parent_mut().parent_mut()
    }
}
/// Generation status
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct GenerationStatus {
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
    pub generation_status_zgen: ::std::option::Option<GenerationStatusZgen>,
}
mod generation_status {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref STATUS_VALUE: crate::commonmodule::StatusValue = Default::default();
        pub(super) static ref GENERATION_STATUS_ZGEN: crate::generationmodule::GenerationStatusZgen = Default::default();
    }
}
impl GenerationStatus {
    pub(crate) fn parent(&self) -> &super::commonmodule::StatusValue {
        self.status_value.as_ref().unwrap_or(&generation_status::STATUS_VALUE)
    }
    pub(crate) fn parent_mut(&mut self) -> &mut super::commonmodule::StatusValue {
        self.status_value.get_or_insert(Default::default())
    }
}
pub trait IsGenerationStatus {
    fn _generation_status(&self) -> &GenerationStatus;
    fn _generation_status_mut(&mut self) -> &mut GenerationStatus;
    fn status_value(&self) -> &super::commonmodule::StatusValue {
        self._generation_status().status_value.as_ref().unwrap_or(&generation_status::STATUS_VALUE)
    }
    fn status_value_mut(&mut self) -> &mut super::commonmodule::StatusValue {
        self._generation_status_mut().status_value.get_or_insert(Default::default())
    }
    fn generation_status_zgen(&self) -> &GenerationStatusZgen {
        self._generation_status().generation_status_zgen.as_ref().unwrap_or(&generation_status::GENERATION_STATUS_ZGEN)
    }
    fn generation_status_zgen_mut(&mut self) -> &mut GenerationStatusZgen {
        self._generation_status_mut().generation_status_zgen.get_or_insert(Default::default())
    }
}
impl IsGenerationStatus for GenerationStatus {
    fn _generation_status(&self) -> &GenerationStatus {
        self
    }
    fn _generation_status_mut(&mut self) -> &mut GenerationStatus {
        self
    }
}
impl IsStatusValue for GenerationStatus {
    fn _status_value(&self) -> &super::commonmodule::StatusValue {
        self.parent()
    }
    fn _status_value_mut(&mut self) -> &mut StatusValue {
        self.parent_mut()
    }
}
impl IsIdentifiedObject for GenerationStatus {
    fn _identified_object(&self) -> &super::commonmodule::IdentifiedObject {
        self.parent().parent()
    }
    fn _identified_object_mut(&mut self) -> &mut IdentifiedObject {
        self.parent_mut().parent_mut()
    }
}
/// Generation status profile
/// OpenFMB Profile Message: true
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct GenerationStatusProfile {
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
    pub generating_unit: ::std::option::Option<GeneratingUnit>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: Some(1)
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, optional, tag="3")]
    pub generation_status: ::std::option::Option<GenerationStatus>,
}
mod generation_status_profile {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref STATUS_MESSAGE_INFO: crate::commonmodule::StatusMessageInfo = Default::default();
        pub(super) static ref GENERATING_UNIT: crate::generationmodule::GeneratingUnit = Default::default();
        pub(super) static ref GENERATION_STATUS: crate::generationmodule::GenerationStatus = Default::default();
    }
}
impl GenerationStatusProfile {
    pub(crate) fn parent(&self) -> &super::commonmodule::StatusMessageInfo {
        self.status_message_info.as_ref().unwrap_or(&generation_status_profile::STATUS_MESSAGE_INFO)
    }
    pub(crate) fn parent_mut(&mut self) -> &mut super::commonmodule::StatusMessageInfo {
        self.status_message_info.get_or_insert(Default::default())
    }
}
pub trait IsGenerationStatusProfile {
    fn _generation_status_profile(&self) -> &GenerationStatusProfile;
    fn _generation_status_profile_mut(&mut self) -> &mut GenerationStatusProfile;
    fn status_message_info(&self) -> &super::commonmodule::StatusMessageInfo {
        self._generation_status_profile().status_message_info.as_ref().unwrap_or(&generation_status_profile::STATUS_MESSAGE_INFO)
    }
    fn status_message_info_mut(&mut self) -> &mut super::commonmodule::StatusMessageInfo {
        self._generation_status_profile_mut().status_message_info.get_or_insert(Default::default())
    }
    fn generating_unit(&self) -> &GeneratingUnit {
        self._generation_status_profile().generating_unit.as_ref().unwrap_or(&generation_status_profile::GENERATING_UNIT)
    }
    fn generating_unit_mut(&mut self) -> &mut GeneratingUnit {
        self._generation_status_profile_mut().generating_unit.get_or_insert(Default::default())
    }
    fn generation_status(&self) -> &GenerationStatus {
        self._generation_status_profile().generation_status.as_ref().unwrap_or(&generation_status_profile::GENERATION_STATUS)
    }
    fn generation_status_mut(&mut self) -> &mut GenerationStatus {
        self._generation_status_profile_mut().generation_status.get_or_insert(Default::default())
    }
}
impl IsGenerationStatusProfile for GenerationStatusProfile {
    fn _generation_status_profile(&self) -> &GenerationStatusProfile {
        self
    }
    fn _generation_status_profile_mut(&mut self) -> &mut GenerationStatusProfile {
        self
    }
}
impl IsStatusMessageInfo for GenerationStatusProfile {
    fn _status_message_info(&self) -> &super::commonmodule::StatusMessageInfo {
        self.parent()
    }
    fn _status_message_info_mut(&mut self) -> &mut StatusMessageInfo {
        self.parent_mut()
    }
}
impl IsMessageInfo for GenerationStatusProfile {
    fn _message_info(&self) -> &super::commonmodule::MessageInfo {
        self.parent().parent()
    }
    fn _message_info_mut(&mut self) -> &mut MessageInfo {
        self.parent_mut().parent_mut()
    }
}
impl IsIdentifiedObject for GenerationStatusProfile {
    fn _identified_object(&self) -> &super::commonmodule::IdentifiedObject {
        self.parent().parent().parent()
    }
    fn _identified_object_mut(&mut self) -> &mut IdentifiedObject {
        self.parent_mut().parent_mut().parent_mut()
    }
}
/// Real power control kind
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum RealPowerControlKind {
    /// MISSING DOCUMENTATION!!!
    Undefined = 0,
    /// MISSING DOCUMENTATION!!!
    Advanced = 1,
    /// MISSING DOCUMENTATION!!!
    Droop = 2,
    /// MISSING DOCUMENTATION!!!
    Isochronous = 3,
    /// Real power setpoint
    RealPower = 4,
}
/// Real power control kind
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum ReactivePowerControlKind {
    /// MISSING DOCUMENTATION!!!
    Undefined = 0,
    /// MISSING DOCUMENTATION!!!
    Advanced = 1,
    /// MISSING DOCUMENTATION!!!
    Droop = 2,
    /// Voltage setpoint
    Voltage = 3,
    /// Reactive power setpoint
    ReactivePower = 4,
    /// MISSING DOCUMENTATION!!!
    PowerFactor = 5,
}
