use crate::commonmodule::*;
/// Point definition (Point)
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize)]
pub struct SolarPoint {
    /// Enable frequency set point
    #[prost(message, optional, tag="1")]
    #[serde(default, rename = "frequencySetPointEnabled")]
    pub frequency_set_point_enabled: ::std::option::Option<super::commonmodule::ControlDpc>,
    /// Grid connect mode
    #[prost(message, optional, tag="2")]
    #[serde(default)]
    pub mode: ::std::option::Option<super::commonmodule::EngGridConnectModeKind>,
    /// Black start enable
    #[prost(message, optional, tag="3")]
    #[serde(default, rename = "pctHzDroop")]
    pub pct_hz_droop: ::std::option::Option<f32>,
    /// Black start enable
    #[prost(message, optional, tag="4")]
    #[serde(default, rename = "pctVDroop")]
    pub pct_v_droop: ::std::option::Option<f32>,
    /// Ramp rates
    #[prost(message, optional, tag="5")]
    #[serde(default, rename = "rampRates")]
    pub ramp_rates: ::std::option::Option<super::commonmodule::RampRate>,
    /// Enable reactive power set point
    #[prost(message, optional, tag="6")]
    #[serde(default, rename = "reactivePwrSetPointEnabled")]
    pub reactive_pwr_set_point_enabled: ::std::option::Option<super::commonmodule::ControlDpc>,
    /// Enable real power set point
    #[prost(message, optional, tag="7")]
    #[serde(default, rename = "realPwrSetPointEnabled")]
    pub real_pwr_set_point_enabled: ::std::option::Option<super::commonmodule::ControlDpc>,
    /// Reset device
    #[prost(message, optional, tag="8")]
    #[serde(default)]
    pub reset: ::std::option::Option<super::commonmodule::ControlDpc>,
    /// ESS state
    #[prost(message, optional, tag="9")]
    #[serde(default)]
    pub state: ::std::option::Option<super::commonmodule::OptionalStateKind>,
    /// Enable voltage set point
    #[prost(message, optional, tag="10")]
    #[serde(default, rename = "voltageSetPointEnabled")]
    pub voltage_set_point_enabled: ::std::option::Option<super::commonmodule::ControlDpc>,
    /// X-axis value (Unix time).
    // parent_message: false
    // required_field: true
    // multiplicity_min: Some(1)
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, optional, tag="11")]
    #[serde(default, rename = "startTime")]
    pub start_time: ::std::option::Option<super::commonmodule::ControlTimestamp>,
}
mod solar_point {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref FREQUENCY_SET_POINT_ENABLED: crate::commonmodule::ControlDpc = Default::default();
        pub(super) static ref MODE: crate::commonmodule::EngGridConnectModeKind = Default::default();
        pub(super) static ref PCT_HZ_DROOP: f32 = Default::default();
        pub(super) static ref PCT_V_DROOP: f32 = Default::default();
        pub(super) static ref RAMP_RATES: crate::commonmodule::RampRate = Default::default();
        pub(super) static ref REACTIVE_PWR_SET_POINT_ENABLED: crate::commonmodule::ControlDpc = Default::default();
        pub(super) static ref REAL_PWR_SET_POINT_ENABLED: crate::commonmodule::ControlDpc = Default::default();
        pub(super) static ref RESET: crate::commonmodule::ControlDpc = Default::default();
        pub(super) static ref STATE: crate::commonmodule::OptionalStateKind = Default::default();
        pub(super) static ref VOLTAGE_SET_POINT_ENABLED: crate::commonmodule::ControlDpc = Default::default();
        pub(super) static ref START_TIME: crate::commonmodule::ControlTimestamp = Default::default();
    }
}
impl SolarPoint {
}
pub trait IsSolarPoint {
    fn _solar_point(&self) -> &SolarPoint;
    fn _solar_point_mut(&mut self) -> &mut SolarPoint;
    fn frequency_set_point_enabled(&self) -> &super::commonmodule::ControlDpc {
        self._solar_point().frequency_set_point_enabled.as_ref().unwrap_or(&solar_point::FREQUENCY_SET_POINT_ENABLED)
    }
    fn frequency_set_point_enabled_mut(&mut self) -> &mut super::commonmodule::ControlDpc {
        self._solar_point_mut().frequency_set_point_enabled.get_or_insert(Default::default())
    }
    fn mode(&self) -> &super::commonmodule::EngGridConnectModeKind {
        self._solar_point().mode.as_ref().unwrap_or(&solar_point::MODE)
    }
    fn mode_mut(&mut self) -> &mut super::commonmodule::EngGridConnectModeKind {
        self._solar_point_mut().mode.get_or_insert(Default::default())
    }
    fn pct_hz_droop(&self) -> &f32 {
        self._solar_point().pct_hz_droop.as_ref().unwrap_or(&solar_point::PCT_HZ_DROOP)
    }
    fn pct_hz_droop_mut(&mut self) -> &mut f32 {
        self._solar_point_mut().pct_hz_droop.get_or_insert(Default::default())
    }
    fn pct_v_droop(&self) -> &f32 {
        self._solar_point().pct_v_droop.as_ref().unwrap_or(&solar_point::PCT_V_DROOP)
    }
    fn pct_v_droop_mut(&mut self) -> &mut f32 {
        self._solar_point_mut().pct_v_droop.get_or_insert(Default::default())
    }
    fn ramp_rates(&self) -> &super::commonmodule::RampRate {
        self._solar_point().ramp_rates.as_ref().unwrap_or(&solar_point::RAMP_RATES)
    }
    fn ramp_rates_mut(&mut self) -> &mut super::commonmodule::RampRate {
        self._solar_point_mut().ramp_rates.get_or_insert(Default::default())
    }
    fn reactive_pwr_set_point_enabled(&self) -> &super::commonmodule::ControlDpc {
        self._solar_point().reactive_pwr_set_point_enabled.as_ref().unwrap_or(&solar_point::REACTIVE_PWR_SET_POINT_ENABLED)
    }
    fn reactive_pwr_set_point_enabled_mut(&mut self) -> &mut super::commonmodule::ControlDpc {
        self._solar_point_mut().reactive_pwr_set_point_enabled.get_or_insert(Default::default())
    }
    fn real_pwr_set_point_enabled(&self) -> &super::commonmodule::ControlDpc {
        self._solar_point().real_pwr_set_point_enabled.as_ref().unwrap_or(&solar_point::REAL_PWR_SET_POINT_ENABLED)
    }
    fn real_pwr_set_point_enabled_mut(&mut self) -> &mut super::commonmodule::ControlDpc {
        self._solar_point_mut().real_pwr_set_point_enabled.get_or_insert(Default::default())
    }
    fn reset(&self) -> &super::commonmodule::ControlDpc {
        self._solar_point().reset.as_ref().unwrap_or(&solar_point::RESET)
    }
    fn reset_mut(&mut self) -> &mut super::commonmodule::ControlDpc {
        self._solar_point_mut().reset.get_or_insert(Default::default())
    }
    fn state(&self) -> &super::commonmodule::OptionalStateKind {
        self._solar_point().state.as_ref().unwrap_or(&solar_point::STATE)
    }
    fn state_mut(&mut self) -> &mut super::commonmodule::OptionalStateKind {
        self._solar_point_mut().state.get_or_insert(Default::default())
    }
    fn voltage_set_point_enabled(&self) -> &super::commonmodule::ControlDpc {
        self._solar_point().voltage_set_point_enabled.as_ref().unwrap_or(&solar_point::VOLTAGE_SET_POINT_ENABLED)
    }
    fn voltage_set_point_enabled_mut(&mut self) -> &mut super::commonmodule::ControlDpc {
        self._solar_point_mut().voltage_set_point_enabled.get_or_insert(Default::default())
    }
    fn start_time(&self) -> &super::commonmodule::ControlTimestamp {
        self._solar_point().start_time.as_ref().unwrap_or(&solar_point::START_TIME)
    }
    fn start_time_mut(&mut self) -> &mut super::commonmodule::ControlTimestamp {
        self._solar_point_mut().start_time.get_or_insert(Default::default())
    }
}
impl IsSolarPoint for SolarPoint {
    fn _solar_point(&self) -> &SolarPoint {
        self
    }
    fn _solar_point_mut(&mut self) -> &mut SolarPoint {
        self
    }
}
/// Curve shape setting (FC=SP) (CSG_SP)
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize)]
pub struct SolarCsg {
    /// The array with the points specifying a curve shape.
    // parent_message: false
    // required_field: true
    // multiplicity_min: Some(1)
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, repeated, tag="1")]
    #[serde(default, rename = "crvPts")]
    pub crv_pts: ::std::vec::Vec<SolarPoint>,
}
mod solar_csg {
    use lazy_static::lazy_static;
    lazy_static! {
    }
}
impl SolarCsg {
}
pub trait IsSolarCsg {
    fn _solar_csg(&self) -> &SolarCsg;
    fn _solar_csg_mut(&mut self) -> &mut SolarCsg;
    fn crv_pts(&self) -> &::std::vec::Vec<SolarPoint> {
        &self._solar_csg().crv_pts
    }
    fn crv_pts_mut(&mut self) -> &mut ::std::vec::Vec<SolarPoint> {
        &mut self._solar_csg_mut().crv_pts
    }
}
impl IsSolarCsg for SolarCsg {
    fn _solar_csg(&self) -> &SolarCsg {
        self
    }
    fn _solar_csg_mut(&mut self) -> &mut SolarCsg {
        self
    }
}
/// OpenFMB specialization for control schedule using:  LN: Schedule   Name: FSCH
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize)]
pub struct SolarControlScheduleFsch {
    /// Discrete value in SolarCSG type
    // parent_message: false
    // required_field: true
    // multiplicity_min: Some(1)
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, optional, tag="1")]
    #[serde(default, rename = "ValDCSG")]
    pub val_dcsg: ::std::option::Option<SolarCsg>,
}
mod solar_control_schedule_fsch {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref VAL_DCSG: crate::solarmodule::SolarCsg = Default::default();
    }
}
impl SolarControlScheduleFsch {
}
pub trait IsSolarControlScheduleFsch {
    fn _solar_control_schedule_fsch(&self) -> &SolarControlScheduleFsch;
    fn _solar_control_schedule_fsch_mut(&mut self) -> &mut SolarControlScheduleFsch;
    fn val_dcsg(&self) -> &SolarCsg {
        self._solar_control_schedule_fsch().val_dcsg.as_ref().unwrap_or(&solar_control_schedule_fsch::VAL_DCSG)
    }
    fn val_dcsg_mut(&mut self) -> &mut SolarCsg {
        self._solar_control_schedule_fsch_mut().val_dcsg.get_or_insert(Default::default())
    }
}
impl IsSolarControlScheduleFsch for SolarControlScheduleFsch {
    fn _solar_control_schedule_fsch(&self) -> &SolarControlScheduleFsch {
        self
    }
    fn _solar_control_schedule_fsch_mut(&mut self) -> &mut SolarControlScheduleFsch {
        self
    }
}
/// Specialized 61850 FSCC class.  LN: Schedule controller   Name: FSCC
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize)]
pub struct SolarControlFscc {
    /// UML inherited base object
    // parent_message: true
    // required_field: false
    // multiplicity_min: None
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, optional, tag="1")]
    #[serde(default, rename = "controlFSCC")]
    pub control_fscc: ::std::option::Option<super::commonmodule::ControlFscc>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    #[serde(default, rename = "SolarControlScheduleFSCH")]
    pub solar_control_schedule_fsch: ::std::option::Option<SolarControlScheduleFsch>,
}
mod solar_control_fscc {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref CONTROL_FSCC: crate::commonmodule::ControlFscc = Default::default();
        pub(super) static ref SOLAR_CONTROL_SCHEDULE_FSCH: crate::solarmodule::SolarControlScheduleFsch = Default::default();
    }
}
impl SolarControlFscc {
    pub(crate) fn parent(&self) -> &super::commonmodule::ControlFscc {
        self.control_fscc.as_ref().unwrap_or(&solar_control_fscc::CONTROL_FSCC)
    }
    pub(crate) fn parent_mut(&mut self) -> &mut super::commonmodule::ControlFscc {
        self.control_fscc.get_or_insert(Default::default())
    }
}
pub trait IsSolarControlFscc {
    fn _solar_control_fscc(&self) -> &SolarControlFscc;
    fn _solar_control_fscc_mut(&mut self) -> &mut SolarControlFscc;
    fn control_fscc(&self) -> &super::commonmodule::ControlFscc {
        self._solar_control_fscc().control_fscc.as_ref().unwrap_or(&solar_control_fscc::CONTROL_FSCC)
    }
    fn control_fscc_mut(&mut self) -> &mut super::commonmodule::ControlFscc {
        self._solar_control_fscc_mut().control_fscc.get_or_insert(Default::default())
    }
    fn solar_control_schedule_fsch(&self) -> &SolarControlScheduleFsch {
        self._solar_control_fscc().solar_control_schedule_fsch.as_ref().unwrap_or(&solar_control_fscc::SOLAR_CONTROL_SCHEDULE_FSCH)
    }
    fn solar_control_schedule_fsch_mut(&mut self) -> &mut SolarControlScheduleFsch {
        self._solar_control_fscc_mut().solar_control_schedule_fsch.get_or_insert(Default::default())
    }
}
impl IsSolarControlFscc for SolarControlFscc {
    fn _solar_control_fscc(&self) -> &SolarControlFscc {
        self
    }
    fn _solar_control_fscc_mut(&mut self) -> &mut SolarControlFscc {
        self
    }
}
impl IsControlFscc for SolarControlFscc {
    fn _control_fscc(&self) -> &super::commonmodule::ControlFscc {
        self.parent()
    }
    fn _control_fscc_mut(&mut self) -> &mut ControlFscc {
        self.parent_mut()
    }
}
impl IsLogicalNodeForControl for SolarControlFscc {
    fn _logical_node_for_control(&self) -> &super::commonmodule::LogicalNodeForControl {
        self.parent().parent()
    }
    fn _logical_node_for_control_mut(&mut self) -> &mut LogicalNodeForControl {
        self.parent_mut().parent_mut()
    }
}
impl IsLogicalNode for SolarControlFscc {
    fn _logical_node(&self) -> &super::commonmodule::LogicalNode {
        self.parent().parent().parent()
    }
    fn _logical_node_mut(&mut self) -> &mut LogicalNode {
        self.parent_mut().parent_mut().parent_mut()
    }
}
impl IsIdentifiedObject for SolarControlFscc {
    fn _identified_object(&self) -> &super::commonmodule::IdentifiedObject {
        self.parent().parent().parent().parent()
    }
    fn _identified_object_mut(&mut self) -> &mut IdentifiedObject {
        self.parent_mut().parent_mut().parent_mut().parent_mut()
    }
}
/// Solar control
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize)]
pub struct SolarControl {
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
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    #[serde(default)]
    pub check: ::std::option::Option<super::commonmodule::CheckConditions>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    #[serde(default, rename = "solarControlFSCC")]
    pub solar_control_fscc: ::std::option::Option<SolarControlFscc>,
}
mod solar_control {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref CONTROL_VALUE: crate::commonmodule::ControlValue = Default::default();
        pub(super) static ref CHECK: crate::commonmodule::CheckConditions = Default::default();
        pub(super) static ref SOLAR_CONTROL_FSCC: crate::solarmodule::SolarControlFscc = Default::default();
    }
}
impl SolarControl {
    pub(crate) fn parent(&self) -> &super::commonmodule::ControlValue {
        self.control_value.as_ref().unwrap_or(&solar_control::CONTROL_VALUE)
    }
    pub(crate) fn parent_mut(&mut self) -> &mut super::commonmodule::ControlValue {
        self.control_value.get_or_insert(Default::default())
    }
}
pub trait IsSolarControl {
    fn _solar_control(&self) -> &SolarControl;
    fn _solar_control_mut(&mut self) -> &mut SolarControl;
    fn control_value(&self) -> &super::commonmodule::ControlValue {
        self._solar_control().control_value.as_ref().unwrap_or(&solar_control::CONTROL_VALUE)
    }
    fn control_value_mut(&mut self) -> &mut super::commonmodule::ControlValue {
        self._solar_control_mut().control_value.get_or_insert(Default::default())
    }
    fn check(&self) -> &super::commonmodule::CheckConditions {
        self._solar_control().check.as_ref().unwrap_or(&solar_control::CHECK)
    }
    fn check_mut(&mut self) -> &mut super::commonmodule::CheckConditions {
        self._solar_control_mut().check.get_or_insert(Default::default())
    }
    fn solar_control_fscc(&self) -> &SolarControlFscc {
        self._solar_control().solar_control_fscc.as_ref().unwrap_or(&solar_control::SOLAR_CONTROL_FSCC)
    }
    fn solar_control_fscc_mut(&mut self) -> &mut SolarControlFscc {
        self._solar_control_mut().solar_control_fscc.get_or_insert(Default::default())
    }
}
impl IsSolarControl for SolarControl {
    fn _solar_control(&self) -> &SolarControl {
        self
    }
    fn _solar_control_mut(&mut self) -> &mut SolarControl {
        self
    }
}
impl IsControlValue for SolarControl {
    fn _control_value(&self) -> &super::commonmodule::ControlValue {
        self.parent()
    }
    fn _control_value_mut(&mut self) -> &mut ControlValue {
        self.parent_mut()
    }
}
impl IsIdentifiedObject for SolarControl {
    fn _identified_object(&self) -> &super::commonmodule::IdentifiedObject {
        self.parent().parent()
    }
    fn _identified_object_mut(&mut self) -> &mut IdentifiedObject {
        self.parent_mut().parent_mut()
    }
}
/// MISSING DOCUMENTATION!!!
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize)]
pub struct SolarInverter {
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
mod solar_inverter {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref CONDUCTING_EQUIPMENT: crate::commonmodule::ConductingEquipment = Default::default();
    }
}
impl SolarInverter {
    pub(crate) fn parent(&self) -> &super::commonmodule::ConductingEquipment {
        self.conducting_equipment.as_ref().unwrap_or(&solar_inverter::CONDUCTING_EQUIPMENT)
    }
    pub(crate) fn parent_mut(&mut self) -> &mut super::commonmodule::ConductingEquipment {
        self.conducting_equipment.get_or_insert(Default::default())
    }
}
pub trait IsSolarInverter {
    fn _solar_inverter(&self) -> &SolarInverter;
    fn _solar_inverter_mut(&mut self) -> &mut SolarInverter;
    fn conducting_equipment(&self) -> &super::commonmodule::ConductingEquipment {
        self._solar_inverter().conducting_equipment.as_ref().unwrap_or(&solar_inverter::CONDUCTING_EQUIPMENT)
    }
    fn conducting_equipment_mut(&mut self) -> &mut super::commonmodule::ConductingEquipment {
        self._solar_inverter_mut().conducting_equipment.get_or_insert(Default::default())
    }
}
impl IsSolarInverter for SolarInverter {
    fn _solar_inverter(&self) -> &SolarInverter {
        self
    }
    fn _solar_inverter_mut(&mut self) -> &mut SolarInverter {
        self
    }
}
impl IsConductingEquipment for SolarInverter {
    fn _conducting_equipment(&self) -> &super::commonmodule::ConductingEquipment {
        self.parent()
    }
    fn _conducting_equipment_mut(&mut self) -> &mut ConductingEquipment {
        self.parent_mut()
    }
}
impl IsNamedObject for SolarInverter {
    fn _named_object(&self) -> &super::commonmodule::NamedObject {
        self.parent().parent()
    }
    fn _named_object_mut(&mut self) -> &mut NamedObject {
        self.parent_mut().parent_mut()
    }
}
/// Solar control profile
/// OpenFMB Profile Message: true
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize)]
pub struct SolarControlProfile {
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
    #[serde(default, rename = "solarControl")]
    pub solar_control: ::std::option::Option<SolarControl>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: Some(1)
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, optional, tag="3")]
    #[serde(default, rename = "solarInverter")]
    pub solar_inverter: ::std::option::Option<SolarInverter>,
}
mod solar_control_profile {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref CONTROL_MESSAGE_INFO: crate::commonmodule::ControlMessageInfo = Default::default();
        pub(super) static ref SOLAR_CONTROL: crate::solarmodule::SolarControl = Default::default();
        pub(super) static ref SOLAR_INVERTER: crate::solarmodule::SolarInverter = Default::default();
    }
}
impl SolarControlProfile {
    pub(crate) fn parent(&self) -> &super::commonmodule::ControlMessageInfo {
        self.control_message_info.as_ref().unwrap_or(&solar_control_profile::CONTROL_MESSAGE_INFO)
    }
    pub(crate) fn parent_mut(&mut self) -> &mut super::commonmodule::ControlMessageInfo {
        self.control_message_info.get_or_insert(Default::default())
    }
}
pub trait IsSolarControlProfile {
    fn _solar_control_profile(&self) -> &SolarControlProfile;
    fn _solar_control_profile_mut(&mut self) -> &mut SolarControlProfile;
    fn control_message_info(&self) -> &super::commonmodule::ControlMessageInfo {
        self._solar_control_profile().control_message_info.as_ref().unwrap_or(&solar_control_profile::CONTROL_MESSAGE_INFO)
    }
    fn control_message_info_mut(&mut self) -> &mut super::commonmodule::ControlMessageInfo {
        self._solar_control_profile_mut().control_message_info.get_or_insert(Default::default())
    }
    fn solar_control(&self) -> &SolarControl {
        self._solar_control_profile().solar_control.as_ref().unwrap_or(&solar_control_profile::SOLAR_CONTROL)
    }
    fn solar_control_mut(&mut self) -> &mut SolarControl {
        self._solar_control_profile_mut().solar_control.get_or_insert(Default::default())
    }
    fn solar_inverter(&self) -> &SolarInverter {
        self._solar_control_profile().solar_inverter.as_ref().unwrap_or(&solar_control_profile::SOLAR_INVERTER)
    }
    fn solar_inverter_mut(&mut self) -> &mut SolarInverter {
        self._solar_control_profile_mut().solar_inverter.get_or_insert(Default::default())
    }
}
impl IsSolarControlProfile for SolarControlProfile {
    fn _solar_control_profile(&self) -> &SolarControlProfile {
        self
    }
    fn _solar_control_profile_mut(&mut self) -> &mut SolarControlProfile {
        self
    }
}
impl IsControlMessageInfo for SolarControlProfile {
    fn _control_message_info(&self) -> &super::commonmodule::ControlMessageInfo {
        self.parent()
    }
    fn _control_message_info_mut(&mut self) -> &mut ControlMessageInfo {
        self.parent_mut()
    }
}
impl IsMessageInfo for SolarControlProfile {
    fn _message_info(&self) -> &super::commonmodule::MessageInfo {
        self.parent().parent()
    }
    fn _message_info_mut(&mut self) -> &mut MessageInfo {
        self.parent_mut().parent_mut()
    }
}
impl IsIdentifiedObject for SolarControlProfile {
    fn _identified_object(&self) -> &super::commonmodule::IdentifiedObject {
        self.parent().parent().parent()
    }
    fn _identified_object_mut(&mut self) -> &mut IdentifiedObject {
        self.parent_mut().parent_mut().parent_mut()
    }
}
/// Point definition (Point)
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize)]
pub struct SolarPointStatus {
    /// Enable frequency set point
    #[prost(message, optional, tag="1")]
    #[serde(default, rename = "frequencySetPointEnabled")]
    pub frequency_set_point_enabled: ::std::option::Option<super::commonmodule::StatusDps>,
    /// Grid connect mode
    #[prost(message, optional, tag="2")]
    #[serde(default)]
    pub mode: ::std::option::Option<super::commonmodule::EngGridConnectModeKind>,
    /// Black start enable
    #[prost(message, optional, tag="3")]
    #[serde(default, rename = "pctHzDroop")]
    pub pct_hz_droop: ::std::option::Option<f32>,
    /// Black start enable
    #[prost(message, optional, tag="4")]
    #[serde(default, rename = "pctVDroop")]
    pub pct_v_droop: ::std::option::Option<f32>,
    /// Ramp rates
    #[prost(message, optional, tag="5")]
    #[serde(default, rename = "rampRates")]
    pub ramp_rates: ::std::option::Option<super::commonmodule::RampRate>,
    /// Enable reactive power set point
    #[prost(message, optional, tag="6")]
    #[serde(default, rename = "reactivePwrSetPointEnabled")]
    pub reactive_pwr_set_point_enabled: ::std::option::Option<super::commonmodule::StatusDps>,
    /// Enable real power set point
    #[prost(message, optional, tag="7")]
    #[serde(default, rename = "realPwrSetPointEnabled")]
    pub real_pwr_set_point_enabled: ::std::option::Option<super::commonmodule::StatusDps>,
    /// ESS state
    #[prost(message, optional, tag="8")]
    #[serde(default)]
    pub state: ::std::option::Option<super::commonmodule::OptionalStateKind>,
    /// Enable voltage set point
    #[prost(message, optional, tag="9")]
    #[serde(default, rename = "voltageSetPointEnabled")]
    pub voltage_set_point_enabled: ::std::option::Option<super::commonmodule::StatusDps>,
}
mod solar_point_status {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref FREQUENCY_SET_POINT_ENABLED: crate::commonmodule::StatusDps = Default::default();
        pub(super) static ref MODE: crate::commonmodule::EngGridConnectModeKind = Default::default();
        pub(super) static ref PCT_HZ_DROOP: f32 = Default::default();
        pub(super) static ref PCT_V_DROOP: f32 = Default::default();
        pub(super) static ref RAMP_RATES: crate::commonmodule::RampRate = Default::default();
        pub(super) static ref REACTIVE_PWR_SET_POINT_ENABLED: crate::commonmodule::StatusDps = Default::default();
        pub(super) static ref REAL_PWR_SET_POINT_ENABLED: crate::commonmodule::StatusDps = Default::default();
        pub(super) static ref STATE: crate::commonmodule::OptionalStateKind = Default::default();
        pub(super) static ref VOLTAGE_SET_POINT_ENABLED: crate::commonmodule::StatusDps = Default::default();
    }
}
impl SolarPointStatus {
}
pub trait IsSolarPointStatus {
    fn _solar_point_status(&self) -> &SolarPointStatus;
    fn _solar_point_status_mut(&mut self) -> &mut SolarPointStatus;
    fn frequency_set_point_enabled(&self) -> &super::commonmodule::StatusDps {
        self._solar_point_status().frequency_set_point_enabled.as_ref().unwrap_or(&solar_point_status::FREQUENCY_SET_POINT_ENABLED)
    }
    fn frequency_set_point_enabled_mut(&mut self) -> &mut super::commonmodule::StatusDps {
        self._solar_point_status_mut().frequency_set_point_enabled.get_or_insert(Default::default())
    }
    fn mode(&self) -> &super::commonmodule::EngGridConnectModeKind {
        self._solar_point_status().mode.as_ref().unwrap_or(&solar_point_status::MODE)
    }
    fn mode_mut(&mut self) -> &mut super::commonmodule::EngGridConnectModeKind {
        self._solar_point_status_mut().mode.get_or_insert(Default::default())
    }
    fn pct_hz_droop(&self) -> &f32 {
        self._solar_point_status().pct_hz_droop.as_ref().unwrap_or(&solar_point_status::PCT_HZ_DROOP)
    }
    fn pct_hz_droop_mut(&mut self) -> &mut f32 {
        self._solar_point_status_mut().pct_hz_droop.get_or_insert(Default::default())
    }
    fn pct_v_droop(&self) -> &f32 {
        self._solar_point_status().pct_v_droop.as_ref().unwrap_or(&solar_point_status::PCT_V_DROOP)
    }
    fn pct_v_droop_mut(&mut self) -> &mut f32 {
        self._solar_point_status_mut().pct_v_droop.get_or_insert(Default::default())
    }
    fn ramp_rates(&self) -> &super::commonmodule::RampRate {
        self._solar_point_status().ramp_rates.as_ref().unwrap_or(&solar_point_status::RAMP_RATES)
    }
    fn ramp_rates_mut(&mut self) -> &mut super::commonmodule::RampRate {
        self._solar_point_status_mut().ramp_rates.get_or_insert(Default::default())
    }
    fn reactive_pwr_set_point_enabled(&self) -> &super::commonmodule::StatusDps {
        self._solar_point_status().reactive_pwr_set_point_enabled.as_ref().unwrap_or(&solar_point_status::REACTIVE_PWR_SET_POINT_ENABLED)
    }
    fn reactive_pwr_set_point_enabled_mut(&mut self) -> &mut super::commonmodule::StatusDps {
        self._solar_point_status_mut().reactive_pwr_set_point_enabled.get_or_insert(Default::default())
    }
    fn real_pwr_set_point_enabled(&self) -> &super::commonmodule::StatusDps {
        self._solar_point_status().real_pwr_set_point_enabled.as_ref().unwrap_or(&solar_point_status::REAL_PWR_SET_POINT_ENABLED)
    }
    fn real_pwr_set_point_enabled_mut(&mut self) -> &mut super::commonmodule::StatusDps {
        self._solar_point_status_mut().real_pwr_set_point_enabled.get_or_insert(Default::default())
    }
    fn state(&self) -> &super::commonmodule::OptionalStateKind {
        self._solar_point_status().state.as_ref().unwrap_or(&solar_point_status::STATE)
    }
    fn state_mut(&mut self) -> &mut super::commonmodule::OptionalStateKind {
        self._solar_point_status_mut().state.get_or_insert(Default::default())
    }
    fn voltage_set_point_enabled(&self) -> &super::commonmodule::StatusDps {
        self._solar_point_status().voltage_set_point_enabled.as_ref().unwrap_or(&solar_point_status::VOLTAGE_SET_POINT_ENABLED)
    }
    fn voltage_set_point_enabled_mut(&mut self) -> &mut super::commonmodule::StatusDps {
        self._solar_point_status_mut().voltage_set_point_enabled.get_or_insert(Default::default())
    }
}
impl IsSolarPointStatus for SolarPointStatus {
    fn _solar_point_status(&self) -> &SolarPointStatus {
        self
    }
    fn _solar_point_status_mut(&mut self) -> &mut SolarPointStatus {
        self
    }
}
/// Specialized 61850 ZGEN class
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize)]
pub struct SolarEventAndStatusZgen {
    /// UML inherited base object
    // parent_message: true
    // required_field: false
    // multiplicity_min: None
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, optional, tag="1")]
    #[serde(default, rename = "logicalNodeForEventAndStatus")]
    pub logical_node_for_event_and_status: ::std::option::Option<super::commonmodule::LogicalNodeForEventAndStatus>,
    /// DC Power On/Off Status; True = DC power on
    #[prost(message, optional, tag="2")]
    #[serde(default, rename = "AuxPwrSt")]
    pub aux_pwr_st: ::std::option::Option<super::commonmodule::StatusSps>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    #[serde(default, rename = "DynamicTest")]
    pub dynamic_test: ::std::option::Option<super::commonmodule::EnsDynamicTestKind>,
    /// Emergency stop
    #[prost(message, optional, tag="4")]
    #[serde(default, rename = "EmgStop")]
    pub emg_stop: ::std::option::Option<super::commonmodule::StatusSps>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="5")]
    #[serde(default, rename = "PointStatus")]
    pub point_status: ::std::option::Option<SolarPointStatus>,
}
mod solar_event_and_status_zgen {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref LOGICAL_NODE_FOR_EVENT_AND_STATUS: crate::commonmodule::LogicalNodeForEventAndStatus = Default::default();
        pub(super) static ref AUX_PWR_ST: crate::commonmodule::StatusSps = Default::default();
        pub(super) static ref DYNAMIC_TEST: crate::commonmodule::EnsDynamicTestKind = Default::default();
        pub(super) static ref EMG_STOP: crate::commonmodule::StatusSps = Default::default();
        pub(super) static ref POINT_STATUS: crate::solarmodule::SolarPointStatus = Default::default();
    }
}
impl SolarEventAndStatusZgen {
    pub(crate) fn parent(&self) -> &super::commonmodule::LogicalNodeForEventAndStatus {
        self.logical_node_for_event_and_status.as_ref().unwrap_or(&solar_event_and_status_zgen::LOGICAL_NODE_FOR_EVENT_AND_STATUS)
    }
    pub(crate) fn parent_mut(&mut self) -> &mut super::commonmodule::LogicalNodeForEventAndStatus {
        self.logical_node_for_event_and_status.get_or_insert(Default::default())
    }
}
pub trait IsSolarEventAndStatusZgen {
    fn _solar_event_and_status_zgen(&self) -> &SolarEventAndStatusZgen;
    fn _solar_event_and_status_zgen_mut(&mut self) -> &mut SolarEventAndStatusZgen;
    fn logical_node_for_event_and_status(&self) -> &super::commonmodule::LogicalNodeForEventAndStatus {
        self._solar_event_and_status_zgen().logical_node_for_event_and_status.as_ref().unwrap_or(&solar_event_and_status_zgen::LOGICAL_NODE_FOR_EVENT_AND_STATUS)
    }
    fn logical_node_for_event_and_status_mut(&mut self) -> &mut super::commonmodule::LogicalNodeForEventAndStatus {
        self._solar_event_and_status_zgen_mut().logical_node_for_event_and_status.get_or_insert(Default::default())
    }
    fn aux_pwr_st(&self) -> &super::commonmodule::StatusSps {
        self._solar_event_and_status_zgen().aux_pwr_st.as_ref().unwrap_or(&solar_event_and_status_zgen::AUX_PWR_ST)
    }
    fn aux_pwr_st_mut(&mut self) -> &mut super::commonmodule::StatusSps {
        self._solar_event_and_status_zgen_mut().aux_pwr_st.get_or_insert(Default::default())
    }
    fn dynamic_test(&self) -> &super::commonmodule::EnsDynamicTestKind {
        self._solar_event_and_status_zgen().dynamic_test.as_ref().unwrap_or(&solar_event_and_status_zgen::DYNAMIC_TEST)
    }
    fn dynamic_test_mut(&mut self) -> &mut super::commonmodule::EnsDynamicTestKind {
        self._solar_event_and_status_zgen_mut().dynamic_test.get_or_insert(Default::default())
    }
    fn emg_stop(&self) -> &super::commonmodule::StatusSps {
        self._solar_event_and_status_zgen().emg_stop.as_ref().unwrap_or(&solar_event_and_status_zgen::EMG_STOP)
    }
    fn emg_stop_mut(&mut self) -> &mut super::commonmodule::StatusSps {
        self._solar_event_and_status_zgen_mut().emg_stop.get_or_insert(Default::default())
    }
    fn point_status(&self) -> &SolarPointStatus {
        self._solar_event_and_status_zgen().point_status.as_ref().unwrap_or(&solar_event_and_status_zgen::POINT_STATUS)
    }
    fn point_status_mut(&mut self) -> &mut SolarPointStatus {
        self._solar_event_and_status_zgen_mut().point_status.get_or_insert(Default::default())
    }
}
impl IsSolarEventAndStatusZgen for SolarEventAndStatusZgen {
    fn _solar_event_and_status_zgen(&self) -> &SolarEventAndStatusZgen {
        self
    }
    fn _solar_event_and_status_zgen_mut(&mut self) -> &mut SolarEventAndStatusZgen {
        self
    }
}
impl IsLogicalNodeForEventAndStatus for SolarEventAndStatusZgen {
    fn _logical_node_for_event_and_status(&self) -> &super::commonmodule::LogicalNodeForEventAndStatus {
        self.parent()
    }
    fn _logical_node_for_event_and_status_mut(&mut self) -> &mut LogicalNodeForEventAndStatus {
        self.parent_mut()
    }
}
impl IsLogicalNode for SolarEventAndStatusZgen {
    fn _logical_node(&self) -> &super::commonmodule::LogicalNode {
        self.parent().parent()
    }
    fn _logical_node_mut(&mut self) -> &mut LogicalNode {
        self.parent_mut().parent_mut()
    }
}
impl IsIdentifiedObject for SolarEventAndStatusZgen {
    fn _identified_object(&self) -> &super::commonmodule::IdentifiedObject {
        self.parent().parent().parent()
    }
    fn _identified_object_mut(&mut self) -> &mut IdentifiedObject {
        self.parent_mut().parent_mut().parent_mut()
    }
}
/// Specialized 61850 ZGEN class
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize)]
pub struct SolarEventZgen {
    /// UML inherited base object
    // parent_message: true
    // required_field: false
    // multiplicity_min: None
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, optional, tag="1")]
    #[serde(default, rename = "solarEventAndStatusZGEN")]
    pub solar_event_and_status_zgen: ::std::option::Option<SolarEventAndStatusZgen>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    #[serde(default, rename = "GriMod")]
    pub gri_mod: ::std::option::Option<super::commonmodule::EngGridConnectModeKind>,
}
mod solar_event_zgen {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref SOLAR_EVENT_AND_STATUS_ZGEN: crate::solarmodule::SolarEventAndStatusZgen = Default::default();
        pub(super) static ref GRI_MOD: crate::commonmodule::EngGridConnectModeKind = Default::default();
    }
}
impl SolarEventZgen {
    pub(crate) fn parent(&self) -> &SolarEventAndStatusZgen {
        self.solar_event_and_status_zgen.as_ref().unwrap_or(&solar_event_zgen::SOLAR_EVENT_AND_STATUS_ZGEN)
    }
    pub(crate) fn parent_mut(&mut self) -> &mut SolarEventAndStatusZgen {
        self.solar_event_and_status_zgen.get_or_insert(Default::default())
    }
}
pub trait IsSolarEventZgen {
    fn _solar_event_zgen(&self) -> &SolarEventZgen;
    fn _solar_event_zgen_mut(&mut self) -> &mut SolarEventZgen;
    fn solar_event_and_status_zgen(&self) -> &SolarEventAndStatusZgen {
        self._solar_event_zgen().solar_event_and_status_zgen.as_ref().unwrap_or(&solar_event_zgen::SOLAR_EVENT_AND_STATUS_ZGEN)
    }
    fn solar_event_and_status_zgen_mut(&mut self) -> &mut SolarEventAndStatusZgen {
        self._solar_event_zgen_mut().solar_event_and_status_zgen.get_or_insert(Default::default())
    }
    fn gri_mod(&self) -> &super::commonmodule::EngGridConnectModeKind {
        self._solar_event_zgen().gri_mod.as_ref().unwrap_or(&solar_event_zgen::GRI_MOD)
    }
    fn gri_mod_mut(&mut self) -> &mut super::commonmodule::EngGridConnectModeKind {
        self._solar_event_zgen_mut().gri_mod.get_or_insert(Default::default())
    }
}
impl IsSolarEventZgen for SolarEventZgen {
    fn _solar_event_zgen(&self) -> &SolarEventZgen {
        self
    }
    fn _solar_event_zgen_mut(&mut self) -> &mut SolarEventZgen {
        self
    }
}
impl IsSolarEventAndStatusZgen for SolarEventZgen {
    fn _solar_event_and_status_zgen(&self) -> &SolarEventAndStatusZgen {
        self.parent()
    }
    fn _solar_event_and_status_zgen_mut(&mut self) -> &mut SolarEventAndStatusZgen {
        self.parent_mut()
    }
}
impl IsLogicalNodeForEventAndStatus for SolarEventZgen {
    fn _logical_node_for_event_and_status(&self) -> &super::commonmodule::LogicalNodeForEventAndStatus {
        self.parent().parent()
    }
    fn _logical_node_for_event_and_status_mut(&mut self) -> &mut LogicalNodeForEventAndStatus {
        self.parent_mut().parent_mut()
    }
}
impl IsLogicalNode for SolarEventZgen {
    fn _logical_node(&self) -> &super::commonmodule::LogicalNode {
        self.parent().parent().parent()
    }
    fn _logical_node_mut(&mut self) -> &mut LogicalNode {
        self.parent_mut().parent_mut().parent_mut()
    }
}
impl IsIdentifiedObject for SolarEventZgen {
    fn _identified_object(&self) -> &super::commonmodule::IdentifiedObject {
        self.parent().parent().parent().parent()
    }
    fn _identified_object_mut(&mut self) -> &mut IdentifiedObject {
        self.parent_mut().parent_mut().parent_mut().parent_mut()
    }
}
/// Solar event
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize)]
pub struct SolarEvent {
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
    #[serde(default, rename = "solarEventZGEN")]
    pub solar_event_zgen: ::std::option::Option<SolarEventZgen>,
}
mod solar_event {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref EVENT_VALUE: crate::commonmodule::EventValue = Default::default();
        pub(super) static ref SOLAR_EVENT_ZGEN: crate::solarmodule::SolarEventZgen = Default::default();
    }
}
impl SolarEvent {
    pub(crate) fn parent(&self) -> &super::commonmodule::EventValue {
        self.event_value.as_ref().unwrap_or(&solar_event::EVENT_VALUE)
    }
    pub(crate) fn parent_mut(&mut self) -> &mut super::commonmodule::EventValue {
        self.event_value.get_or_insert(Default::default())
    }
}
pub trait IsSolarEvent {
    fn _solar_event(&self) -> &SolarEvent;
    fn _solar_event_mut(&mut self) -> &mut SolarEvent;
    fn event_value(&self) -> &super::commonmodule::EventValue {
        self._solar_event().event_value.as_ref().unwrap_or(&solar_event::EVENT_VALUE)
    }
    fn event_value_mut(&mut self) -> &mut super::commonmodule::EventValue {
        self._solar_event_mut().event_value.get_or_insert(Default::default())
    }
    fn solar_event_zgen(&self) -> &SolarEventZgen {
        self._solar_event().solar_event_zgen.as_ref().unwrap_or(&solar_event::SOLAR_EVENT_ZGEN)
    }
    fn solar_event_zgen_mut(&mut self) -> &mut SolarEventZgen {
        self._solar_event_mut().solar_event_zgen.get_or_insert(Default::default())
    }
}
impl IsSolarEvent for SolarEvent {
    fn _solar_event(&self) -> &SolarEvent {
        self
    }
    fn _solar_event_mut(&mut self) -> &mut SolarEvent {
        self
    }
}
impl IsEventValue for SolarEvent {
    fn _event_value(&self) -> &super::commonmodule::EventValue {
        self.parent()
    }
    fn _event_value_mut(&mut self) -> &mut EventValue {
        self.parent_mut()
    }
}
impl IsIdentifiedObject for SolarEvent {
    fn _identified_object(&self) -> &super::commonmodule::IdentifiedObject {
        self.parent().parent()
    }
    fn _identified_object_mut(&mut self) -> &mut IdentifiedObject {
        self.parent_mut().parent_mut()
    }
}
/// Solar event profile
/// OpenFMB Profile Message: true
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize)]
pub struct SolarEventProfile {
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
    #[serde(default, rename = "solarEvent")]
    pub solar_event: ::std::option::Option<SolarEvent>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: Some(1)
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, optional, tag="3")]
    #[serde(default, rename = "solarInverter")]
    pub solar_inverter: ::std::option::Option<SolarInverter>,
}
mod solar_event_profile {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref EVENT_MESSAGE_INFO: crate::commonmodule::EventMessageInfo = Default::default();
        pub(super) static ref SOLAR_EVENT: crate::solarmodule::SolarEvent = Default::default();
        pub(super) static ref SOLAR_INVERTER: crate::solarmodule::SolarInverter = Default::default();
    }
}
impl SolarEventProfile {
    pub(crate) fn parent(&self) -> &super::commonmodule::EventMessageInfo {
        self.event_message_info.as_ref().unwrap_or(&solar_event_profile::EVENT_MESSAGE_INFO)
    }
    pub(crate) fn parent_mut(&mut self) -> &mut super::commonmodule::EventMessageInfo {
        self.event_message_info.get_or_insert(Default::default())
    }
}
pub trait IsSolarEventProfile {
    fn _solar_event_profile(&self) -> &SolarEventProfile;
    fn _solar_event_profile_mut(&mut self) -> &mut SolarEventProfile;
    fn event_message_info(&self) -> &super::commonmodule::EventMessageInfo {
        self._solar_event_profile().event_message_info.as_ref().unwrap_or(&solar_event_profile::EVENT_MESSAGE_INFO)
    }
    fn event_message_info_mut(&mut self) -> &mut super::commonmodule::EventMessageInfo {
        self._solar_event_profile_mut().event_message_info.get_or_insert(Default::default())
    }
    fn solar_event(&self) -> &SolarEvent {
        self._solar_event_profile().solar_event.as_ref().unwrap_or(&solar_event_profile::SOLAR_EVENT)
    }
    fn solar_event_mut(&mut self) -> &mut SolarEvent {
        self._solar_event_profile_mut().solar_event.get_or_insert(Default::default())
    }
    fn solar_inverter(&self) -> &SolarInverter {
        self._solar_event_profile().solar_inverter.as_ref().unwrap_or(&solar_event_profile::SOLAR_INVERTER)
    }
    fn solar_inverter_mut(&mut self) -> &mut SolarInverter {
        self._solar_event_profile_mut().solar_inverter.get_or_insert(Default::default())
    }
}
impl IsSolarEventProfile for SolarEventProfile {
    fn _solar_event_profile(&self) -> &SolarEventProfile {
        self
    }
    fn _solar_event_profile_mut(&mut self) -> &mut SolarEventProfile {
        self
    }
}
impl IsEventMessageInfo for SolarEventProfile {
    fn _event_message_info(&self) -> &super::commonmodule::EventMessageInfo {
        self.parent()
    }
    fn _event_message_info_mut(&mut self) -> &mut EventMessageInfo {
        self.parent_mut()
    }
}
impl IsMessageInfo for SolarEventProfile {
    fn _message_info(&self) -> &super::commonmodule::MessageInfo {
        self.parent().parent()
    }
    fn _message_info_mut(&mut self) -> &mut MessageInfo {
        self.parent_mut().parent_mut()
    }
}
impl IsIdentifiedObject for SolarEventProfile {
    fn _identified_object(&self) -> &super::commonmodule::IdentifiedObject {
        self.parent().parent().parent()
    }
    fn _identified_object_mut(&mut self) -> &mut IdentifiedObject {
        self.parent_mut().parent_mut().parent_mut()
    }
}
/// Solar reading value
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize)]
pub struct SolarReading {
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
    #[serde(default, rename = "phaseMMTN")]
    pub phase_mmtn: ::std::option::Option<super::commonmodule::PhaseMmtn>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    #[serde(default, rename = "readingMMTR")]
    pub reading_mmtr: ::std::option::Option<super::commonmodule::ReadingMmtr>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="4")]
    #[serde(default, rename = "readingMMXU")]
    pub reading_mmxu: ::std::option::Option<super::commonmodule::ReadingMmxu>,
}
mod solar_reading {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref CONDUCTING_EQUIPMENT_TERMINAL_READING: crate::commonmodule::ConductingEquipmentTerminalReading = Default::default();
        pub(super) static ref PHASE_MMTN: crate::commonmodule::PhaseMmtn = Default::default();
        pub(super) static ref READING_MMTR: crate::commonmodule::ReadingMmtr = Default::default();
        pub(super) static ref READING_MMXU: crate::commonmodule::ReadingMmxu = Default::default();
    }
}
impl SolarReading {
    pub(crate) fn parent(&self) -> &super::commonmodule::ConductingEquipmentTerminalReading {
        self.conducting_equipment_terminal_reading.as_ref().unwrap_or(&solar_reading::CONDUCTING_EQUIPMENT_TERMINAL_READING)
    }
    pub(crate) fn parent_mut(&mut self) -> &mut super::commonmodule::ConductingEquipmentTerminalReading {
        self.conducting_equipment_terminal_reading.get_or_insert(Default::default())
    }
}
pub trait IsSolarReading {
    fn _solar_reading(&self) -> &SolarReading;
    fn _solar_reading_mut(&mut self) -> &mut SolarReading;
    fn conducting_equipment_terminal_reading(&self) -> &super::commonmodule::ConductingEquipmentTerminalReading {
        self._solar_reading().conducting_equipment_terminal_reading.as_ref().unwrap_or(&solar_reading::CONDUCTING_EQUIPMENT_TERMINAL_READING)
    }
    fn conducting_equipment_terminal_reading_mut(&mut self) -> &mut super::commonmodule::ConductingEquipmentTerminalReading {
        self._solar_reading_mut().conducting_equipment_terminal_reading.get_or_insert(Default::default())
    }
    fn phase_mmtn(&self) -> &super::commonmodule::PhaseMmtn {
        self._solar_reading().phase_mmtn.as_ref().unwrap_or(&solar_reading::PHASE_MMTN)
    }
    fn phase_mmtn_mut(&mut self) -> &mut super::commonmodule::PhaseMmtn {
        self._solar_reading_mut().phase_mmtn.get_or_insert(Default::default())
    }
    fn reading_mmtr(&self) -> &super::commonmodule::ReadingMmtr {
        self._solar_reading().reading_mmtr.as_ref().unwrap_or(&solar_reading::READING_MMTR)
    }
    fn reading_mmtr_mut(&mut self) -> &mut super::commonmodule::ReadingMmtr {
        self._solar_reading_mut().reading_mmtr.get_or_insert(Default::default())
    }
    fn reading_mmxu(&self) -> &super::commonmodule::ReadingMmxu {
        self._solar_reading().reading_mmxu.as_ref().unwrap_or(&solar_reading::READING_MMXU)
    }
    fn reading_mmxu_mut(&mut self) -> &mut super::commonmodule::ReadingMmxu {
        self._solar_reading_mut().reading_mmxu.get_or_insert(Default::default())
    }
}
impl IsSolarReading for SolarReading {
    fn _solar_reading(&self) -> &SolarReading {
        self
    }
    fn _solar_reading_mut(&mut self) -> &mut SolarReading {
        self
    }
}
impl IsConductingEquipmentTerminalReading for SolarReading {
    fn _conducting_equipment_terminal_reading(&self) -> &super::commonmodule::ConductingEquipmentTerminalReading {
        self.parent()
    }
    fn _conducting_equipment_terminal_reading_mut(&mut self) -> &mut ConductingEquipmentTerminalReading {
        self.parent_mut()
    }
}
/// Solar reading profile
/// OpenFMB Profile Message: true
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize)]
pub struct SolarReadingProfile {
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
    #[serde(default, rename = "solarInverter")]
    pub solar_inverter: ::std::option::Option<SolarInverter>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: Some(1)
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, optional, tag="3")]
    #[serde(default, rename = "solarReading")]
    pub solar_reading: ::std::option::Option<SolarReading>,
}
mod solar_reading_profile {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref READING_MESSAGE_INFO: crate::commonmodule::ReadingMessageInfo = Default::default();
        pub(super) static ref SOLAR_INVERTER: crate::solarmodule::SolarInverter = Default::default();
        pub(super) static ref SOLAR_READING: crate::solarmodule::SolarReading = Default::default();
    }
}
impl SolarReadingProfile {
    pub(crate) fn parent(&self) -> &super::commonmodule::ReadingMessageInfo {
        self.reading_message_info.as_ref().unwrap_or(&solar_reading_profile::READING_MESSAGE_INFO)
    }
    pub(crate) fn parent_mut(&mut self) -> &mut super::commonmodule::ReadingMessageInfo {
        self.reading_message_info.get_or_insert(Default::default())
    }
}
pub trait IsSolarReadingProfile {
    fn _solar_reading_profile(&self) -> &SolarReadingProfile;
    fn _solar_reading_profile_mut(&mut self) -> &mut SolarReadingProfile;
    fn reading_message_info(&self) -> &super::commonmodule::ReadingMessageInfo {
        self._solar_reading_profile().reading_message_info.as_ref().unwrap_or(&solar_reading_profile::READING_MESSAGE_INFO)
    }
    fn reading_message_info_mut(&mut self) -> &mut super::commonmodule::ReadingMessageInfo {
        self._solar_reading_profile_mut().reading_message_info.get_or_insert(Default::default())
    }
    fn solar_inverter(&self) -> &SolarInverter {
        self._solar_reading_profile().solar_inverter.as_ref().unwrap_or(&solar_reading_profile::SOLAR_INVERTER)
    }
    fn solar_inverter_mut(&mut self) -> &mut SolarInverter {
        self._solar_reading_profile_mut().solar_inverter.get_or_insert(Default::default())
    }
    fn solar_reading(&self) -> &SolarReading {
        self._solar_reading_profile().solar_reading.as_ref().unwrap_or(&solar_reading_profile::SOLAR_READING)
    }
    fn solar_reading_mut(&mut self) -> &mut SolarReading {
        self._solar_reading_profile_mut().solar_reading.get_or_insert(Default::default())
    }
}
impl IsSolarReadingProfile for SolarReadingProfile {
    fn _solar_reading_profile(&self) -> &SolarReadingProfile {
        self
    }
    fn _solar_reading_profile_mut(&mut self) -> &mut SolarReadingProfile {
        self
    }
}
impl IsReadingMessageInfo for SolarReadingProfile {
    fn _reading_message_info(&self) -> &super::commonmodule::ReadingMessageInfo {
        self.parent()
    }
    fn _reading_message_info_mut(&mut self) -> &mut ReadingMessageInfo {
        self.parent_mut()
    }
}
impl IsMessageInfo for SolarReadingProfile {
    fn _message_info(&self) -> &super::commonmodule::MessageInfo {
        self.parent().parent()
    }
    fn _message_info_mut(&mut self) -> &mut MessageInfo {
        self.parent_mut().parent_mut()
    }
}
impl IsIdentifiedObject for SolarReadingProfile {
    fn _identified_object(&self) -> &super::commonmodule::IdentifiedObject {
        self.parent().parent().parent()
    }
    fn _identified_object_mut(&mut self) -> &mut IdentifiedObject {
        self.parent_mut().parent_mut().parent_mut()
    }
}
/// Specialized 61850 ZGEN LN class
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize)]
pub struct SolarStatusZgen {
    /// UML inherited base object
    // parent_message: true
    // required_field: false
    // multiplicity_min: None
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, optional, tag="1")]
    #[serde(default, rename = "solarEventAndStatusZGEN")]
    pub solar_event_and_status_zgen: ::std::option::Option<SolarEventAndStatusZgen>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    #[serde(default, rename = "GriMod")]
    pub gri_mod: ::std::option::Option<super::commonmodule::EngGridConnectModeKind>,
}
mod solar_status_zgen {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref SOLAR_EVENT_AND_STATUS_ZGEN: crate::solarmodule::SolarEventAndStatusZgen = Default::default();
        pub(super) static ref GRI_MOD: crate::commonmodule::EngGridConnectModeKind = Default::default();
    }
}
impl SolarStatusZgen {
    pub(crate) fn parent(&self) -> &SolarEventAndStatusZgen {
        self.solar_event_and_status_zgen.as_ref().unwrap_or(&solar_status_zgen::SOLAR_EVENT_AND_STATUS_ZGEN)
    }
    pub(crate) fn parent_mut(&mut self) -> &mut SolarEventAndStatusZgen {
        self.solar_event_and_status_zgen.get_or_insert(Default::default())
    }
}
pub trait IsSolarStatusZgen {
    fn _solar_status_zgen(&self) -> &SolarStatusZgen;
    fn _solar_status_zgen_mut(&mut self) -> &mut SolarStatusZgen;
    fn solar_event_and_status_zgen(&self) -> &SolarEventAndStatusZgen {
        self._solar_status_zgen().solar_event_and_status_zgen.as_ref().unwrap_or(&solar_status_zgen::SOLAR_EVENT_AND_STATUS_ZGEN)
    }
    fn solar_event_and_status_zgen_mut(&mut self) -> &mut SolarEventAndStatusZgen {
        self._solar_status_zgen_mut().solar_event_and_status_zgen.get_or_insert(Default::default())
    }
    fn gri_mod(&self) -> &super::commonmodule::EngGridConnectModeKind {
        self._solar_status_zgen().gri_mod.as_ref().unwrap_or(&solar_status_zgen::GRI_MOD)
    }
    fn gri_mod_mut(&mut self) -> &mut super::commonmodule::EngGridConnectModeKind {
        self._solar_status_zgen_mut().gri_mod.get_or_insert(Default::default())
    }
}
impl IsSolarStatusZgen for SolarStatusZgen {
    fn _solar_status_zgen(&self) -> &SolarStatusZgen {
        self
    }
    fn _solar_status_zgen_mut(&mut self) -> &mut SolarStatusZgen {
        self
    }
}
impl IsSolarEventAndStatusZgen for SolarStatusZgen {
    fn _solar_event_and_status_zgen(&self) -> &SolarEventAndStatusZgen {
        self.parent()
    }
    fn _solar_event_and_status_zgen_mut(&mut self) -> &mut SolarEventAndStatusZgen {
        self.parent_mut()
    }
}
impl IsLogicalNodeForEventAndStatus for SolarStatusZgen {
    fn _logical_node_for_event_and_status(&self) -> &super::commonmodule::LogicalNodeForEventAndStatus {
        self.parent().parent()
    }
    fn _logical_node_for_event_and_status_mut(&mut self) -> &mut LogicalNodeForEventAndStatus {
        self.parent_mut().parent_mut()
    }
}
impl IsLogicalNode for SolarStatusZgen {
    fn _logical_node(&self) -> &super::commonmodule::LogicalNode {
        self.parent().parent().parent()
    }
    fn _logical_node_mut(&mut self) -> &mut LogicalNode {
        self.parent_mut().parent_mut().parent_mut()
    }
}
impl IsIdentifiedObject for SolarStatusZgen {
    fn _identified_object(&self) -> &super::commonmodule::IdentifiedObject {
        self.parent().parent().parent().parent()
    }
    fn _identified_object_mut(&mut self) -> &mut IdentifiedObject {
        self.parent_mut().parent_mut().parent_mut().parent_mut()
    }
}
/// Solar status
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize)]
pub struct SolarStatus {
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
    #[serde(default, rename = "solarStatusZGEN")]
    pub solar_status_zgen: ::std::option::Option<SolarStatusZgen>,
}
mod solar_status {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref STATUS_VALUE: crate::commonmodule::StatusValue = Default::default();
        pub(super) static ref SOLAR_STATUS_ZGEN: crate::solarmodule::SolarStatusZgen = Default::default();
    }
}
impl SolarStatus {
    pub(crate) fn parent(&self) -> &super::commonmodule::StatusValue {
        self.status_value.as_ref().unwrap_or(&solar_status::STATUS_VALUE)
    }
    pub(crate) fn parent_mut(&mut self) -> &mut super::commonmodule::StatusValue {
        self.status_value.get_or_insert(Default::default())
    }
}
pub trait IsSolarStatus {
    fn _solar_status(&self) -> &SolarStatus;
    fn _solar_status_mut(&mut self) -> &mut SolarStatus;
    fn status_value(&self) -> &super::commonmodule::StatusValue {
        self._solar_status().status_value.as_ref().unwrap_or(&solar_status::STATUS_VALUE)
    }
    fn status_value_mut(&mut self) -> &mut super::commonmodule::StatusValue {
        self._solar_status_mut().status_value.get_or_insert(Default::default())
    }
    fn solar_status_zgen(&self) -> &SolarStatusZgen {
        self._solar_status().solar_status_zgen.as_ref().unwrap_or(&solar_status::SOLAR_STATUS_ZGEN)
    }
    fn solar_status_zgen_mut(&mut self) -> &mut SolarStatusZgen {
        self._solar_status_mut().solar_status_zgen.get_or_insert(Default::default())
    }
}
impl IsSolarStatus for SolarStatus {
    fn _solar_status(&self) -> &SolarStatus {
        self
    }
    fn _solar_status_mut(&mut self) -> &mut SolarStatus {
        self
    }
}
impl IsStatusValue for SolarStatus {
    fn _status_value(&self) -> &super::commonmodule::StatusValue {
        self.parent()
    }
    fn _status_value_mut(&mut self) -> &mut StatusValue {
        self.parent_mut()
    }
}
impl IsIdentifiedObject for SolarStatus {
    fn _identified_object(&self) -> &super::commonmodule::IdentifiedObject {
        self.parent().parent()
    }
    fn _identified_object_mut(&mut self) -> &mut IdentifiedObject {
        self.parent_mut().parent_mut()
    }
}
/// Solar status profile
/// OpenFMB Profile Message: true
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize)]
pub struct SolarStatusProfile {
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
    #[serde(default, rename = "solarInverter")]
    pub solar_inverter: ::std::option::Option<SolarInverter>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: Some(1)
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, optional, tag="3")]
    #[serde(default, rename = "solarStatus")]
    pub solar_status: ::std::option::Option<SolarStatus>,
}
mod solar_status_profile {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref STATUS_MESSAGE_INFO: crate::commonmodule::StatusMessageInfo = Default::default();
        pub(super) static ref SOLAR_INVERTER: crate::solarmodule::SolarInverter = Default::default();
        pub(super) static ref SOLAR_STATUS: crate::solarmodule::SolarStatus = Default::default();
    }
}
impl SolarStatusProfile {
    pub(crate) fn parent(&self) -> &super::commonmodule::StatusMessageInfo {
        self.status_message_info.as_ref().unwrap_or(&solar_status_profile::STATUS_MESSAGE_INFO)
    }
    pub(crate) fn parent_mut(&mut self) -> &mut super::commonmodule::StatusMessageInfo {
        self.status_message_info.get_or_insert(Default::default())
    }
}
pub trait IsSolarStatusProfile {
    fn _solar_status_profile(&self) -> &SolarStatusProfile;
    fn _solar_status_profile_mut(&mut self) -> &mut SolarStatusProfile;
    fn status_message_info(&self) -> &super::commonmodule::StatusMessageInfo {
        self._solar_status_profile().status_message_info.as_ref().unwrap_or(&solar_status_profile::STATUS_MESSAGE_INFO)
    }
    fn status_message_info_mut(&mut self) -> &mut super::commonmodule::StatusMessageInfo {
        self._solar_status_profile_mut().status_message_info.get_or_insert(Default::default())
    }
    fn solar_inverter(&self) -> &SolarInverter {
        self._solar_status_profile().solar_inverter.as_ref().unwrap_or(&solar_status_profile::SOLAR_INVERTER)
    }
    fn solar_inverter_mut(&mut self) -> &mut SolarInverter {
        self._solar_status_profile_mut().solar_inverter.get_or_insert(Default::default())
    }
    fn solar_status(&self) -> &SolarStatus {
        self._solar_status_profile().solar_status.as_ref().unwrap_or(&solar_status_profile::SOLAR_STATUS)
    }
    fn solar_status_mut(&mut self) -> &mut SolarStatus {
        self._solar_status_profile_mut().solar_status.get_or_insert(Default::default())
    }
}
impl IsSolarStatusProfile for SolarStatusProfile {
    fn _solar_status_profile(&self) -> &SolarStatusProfile {
        self
    }
    fn _solar_status_profile_mut(&mut self) -> &mut SolarStatusProfile {
        self
    }
}
impl IsStatusMessageInfo for SolarStatusProfile {
    fn _status_message_info(&self) -> &super::commonmodule::StatusMessageInfo {
        self.parent()
    }
    fn _status_message_info_mut(&mut self) -> &mut StatusMessageInfo {
        self.parent_mut()
    }
}
impl IsMessageInfo for SolarStatusProfile {
    fn _message_info(&self) -> &super::commonmodule::MessageInfo {
        self.parent().parent()
    }
    fn _message_info_mut(&mut self) -> &mut MessageInfo {
        self.parent_mut().parent_mut()
    }
}
impl IsIdentifiedObject for SolarStatusProfile {
    fn _identified_object(&self) -> &super::commonmodule::IdentifiedObject {
        self.parent().parent().parent()
    }
    fn _identified_object_mut(&mut self) -> &mut IdentifiedObject {
        self.parent_mut().parent_mut().parent_mut()
    }
}