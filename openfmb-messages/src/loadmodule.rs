use crate::commonmodule::*;
/// Point definition (Point)
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize)]
pub struct LoadPoint {
    /// Ramp rates
    #[prost(message, optional, tag="1")]
    #[serde(default, rename = "rampRates")]
    pub ramp_rates: ::std::option::Option<super::commonmodule::RampRate>,
    /// Enable reactive power set point
    #[prost(message, optional, tag="2")]
    #[serde(default, rename = "reactivePwrSetPointEnabled")]
    pub reactive_pwr_set_point_enabled: ::std::option::Option<super::commonmodule::ControlDpc>,
    /// Enable joint real power set point
    #[prost(message, optional, tag="3")]
    #[serde(default, rename = "realPwrSetPointEnabled")]
    pub real_pwr_set_point_enabled: ::std::option::Option<super::commonmodule::ControlDpc>,
    /// Reset device
    #[prost(message, optional, tag="4")]
    #[serde(default)]
    pub reset: ::std::option::Option<super::commonmodule::ControlDpc>,
    /// ESS state
    #[prost(message, optional, tag="5")]
    #[serde(default)]
    pub state: ::std::option::Option<super::commonmodule::OptionalStateKind>,
    /// Start time
    // parent_message: false
    // required_field: true
    // multiplicity_min: Some(1)
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, optional, tag="6")]
    #[serde(default, rename = "startTime")]
    pub start_time: ::std::option::Option<super::commonmodule::ControlTimestamp>,
}
mod load_point {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref RAMP_RATES: crate::commonmodule::RampRate = Default::default();
        pub(super) static ref REACTIVE_PWR_SET_POINT_ENABLED: crate::commonmodule::ControlDpc = Default::default();
        pub(super) static ref REAL_PWR_SET_POINT_ENABLED: crate::commonmodule::ControlDpc = Default::default();
        pub(super) static ref RESET: crate::commonmodule::ControlDpc = Default::default();
        pub(super) static ref STATE: crate::commonmodule::OptionalStateKind = Default::default();
        pub(super) static ref START_TIME: crate::commonmodule::ControlTimestamp = Default::default();
    }
}
impl LoadPoint {
}
pub trait IsLoadPoint {
    fn _load_point(&self) -> &LoadPoint;
    fn _load_point_mut(&mut self) -> &mut LoadPoint;
    fn ramp_rates(&self) -> &super::commonmodule::RampRate {
        self._load_point().ramp_rates.as_ref().unwrap_or(&load_point::RAMP_RATES)
    }
    fn ramp_rates_mut(&mut self) -> &mut super::commonmodule::RampRate {
        self._load_point_mut().ramp_rates.get_or_insert(Default::default())
    }
    fn reactive_pwr_set_point_enabled(&self) -> &super::commonmodule::ControlDpc {
        self._load_point().reactive_pwr_set_point_enabled.as_ref().unwrap_or(&load_point::REACTIVE_PWR_SET_POINT_ENABLED)
    }
    fn reactive_pwr_set_point_enabled_mut(&mut self) -> &mut super::commonmodule::ControlDpc {
        self._load_point_mut().reactive_pwr_set_point_enabled.get_or_insert(Default::default())
    }
    fn real_pwr_set_point_enabled(&self) -> &super::commonmodule::ControlDpc {
        self._load_point().real_pwr_set_point_enabled.as_ref().unwrap_or(&load_point::REAL_PWR_SET_POINT_ENABLED)
    }
    fn real_pwr_set_point_enabled_mut(&mut self) -> &mut super::commonmodule::ControlDpc {
        self._load_point_mut().real_pwr_set_point_enabled.get_or_insert(Default::default())
    }
    fn reset(&self) -> &super::commonmodule::ControlDpc {
        self._load_point().reset.as_ref().unwrap_or(&load_point::RESET)
    }
    fn reset_mut(&mut self) -> &mut super::commonmodule::ControlDpc {
        self._load_point_mut().reset.get_or_insert(Default::default())
    }
    fn state(&self) -> &super::commonmodule::OptionalStateKind {
        self._load_point().state.as_ref().unwrap_or(&load_point::STATE)
    }
    fn state_mut(&mut self) -> &mut super::commonmodule::OptionalStateKind {
        self._load_point_mut().state.get_or_insert(Default::default())
    }
    fn start_time(&self) -> &super::commonmodule::ControlTimestamp {
        self._load_point().start_time.as_ref().unwrap_or(&load_point::START_TIME)
    }
    fn start_time_mut(&mut self) -> &mut super::commonmodule::ControlTimestamp {
        self._load_point_mut().start_time.get_or_insert(Default::default())
    }
}
impl IsLoadPoint for LoadPoint {
    fn _load_point(&self) -> &LoadPoint {
        self
    }
    fn _load_point_mut(&mut self) -> &mut LoadPoint {
        self
    }
}
/// Curve shape setting (FC=SP) (CSG_SP)
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize)]
pub struct LoadCsg {
    /// The array with the points specifying a curve shape.
    // parent_message: false
    // required_field: true
    // multiplicity_min: Some(1)
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, repeated, tag="1")]
    #[serde(default, rename = "crvPts")]
    pub crv_pts: ::std::vec::Vec<LoadPoint>,
}
mod load_csg {
    use lazy_static::lazy_static;
    lazy_static! {
    }
}
impl LoadCsg {
}
pub trait IsLoadCsg {
    fn _load_csg(&self) -> &LoadCsg;
    fn _load_csg_mut(&mut self) -> &mut LoadCsg;
    fn crv_pts(&self) -> &::std::vec::Vec<LoadPoint> {
        &self._load_csg().crv_pts
    }
    fn crv_pts_mut(&mut self) -> &mut ::std::vec::Vec<LoadPoint> {
        &mut self._load_csg_mut().crv_pts
    }
}
impl IsLoadCsg for LoadCsg {
    fn _load_csg(&self) -> &LoadCsg {
        self
    }
    fn _load_csg_mut(&mut self) -> &mut LoadCsg {
        self
    }
}
/// OpenFMB specialization for control schedule using:  LN: Schedule   Name: FSCH
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize)]
pub struct LoadControlScheduleFsch {
    /// Discrete value in LoadCSG type
    // parent_message: false
    // required_field: true
    // multiplicity_min: Some(1)
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, optional, tag="1")]
    #[serde(default, rename = "ValDCSG")]
    pub val_dcsg: ::std::option::Option<LoadCsg>,
}
mod load_control_schedule_fsch {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref VAL_DCSG: crate::loadmodule::LoadCsg = Default::default();
    }
}
impl LoadControlScheduleFsch {
}
pub trait IsLoadControlScheduleFsch {
    fn _load_control_schedule_fsch(&self) -> &LoadControlScheduleFsch;
    fn _load_control_schedule_fsch_mut(&mut self) -> &mut LoadControlScheduleFsch;
    fn val_dcsg(&self) -> &LoadCsg {
        self._load_control_schedule_fsch().val_dcsg.as_ref().unwrap_or(&load_control_schedule_fsch::VAL_DCSG)
    }
    fn val_dcsg_mut(&mut self) -> &mut LoadCsg {
        self._load_control_schedule_fsch_mut().val_dcsg.get_or_insert(Default::default())
    }
}
impl IsLoadControlScheduleFsch for LoadControlScheduleFsch {
    fn _load_control_schedule_fsch(&self) -> &LoadControlScheduleFsch {
        self
    }
    fn _load_control_schedule_fsch_mut(&mut self) -> &mut LoadControlScheduleFsch {
        self
    }
}
/// Specialized FSCC 61850 class.  LN: Schedule controller   Name: FSCC
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize)]
pub struct LoadControlFscc {
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
    #[serde(default, rename = "loadControlScheduleFSCH")]
    pub load_control_schedule_fsch: ::std::option::Option<LoadControlScheduleFsch>,
}
mod load_control_fscc {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref CONTROL_FSCC: crate::commonmodule::ControlFscc = Default::default();
        pub(super) static ref LOAD_CONTROL_SCHEDULE_FSCH: crate::loadmodule::LoadControlScheduleFsch = Default::default();
    }
}
impl LoadControlFscc {
    pub(crate) fn parent(&self) -> &super::commonmodule::ControlFscc {
        self.control_fscc.as_ref().unwrap_or(&load_control_fscc::CONTROL_FSCC)
    }
    pub(crate) fn parent_mut(&mut self) -> &mut super::commonmodule::ControlFscc {
        self.control_fscc.get_or_insert(Default::default())
    }
}
pub trait IsLoadControlFscc {
    fn _load_control_fscc(&self) -> &LoadControlFscc;
    fn _load_control_fscc_mut(&mut self) -> &mut LoadControlFscc;
    fn control_fscc(&self) -> &super::commonmodule::ControlFscc {
        self._load_control_fscc().control_fscc.as_ref().unwrap_or(&load_control_fscc::CONTROL_FSCC)
    }
    fn control_fscc_mut(&mut self) -> &mut super::commonmodule::ControlFscc {
        self._load_control_fscc_mut().control_fscc.get_or_insert(Default::default())
    }
    fn load_control_schedule_fsch(&self) -> &LoadControlScheduleFsch {
        self._load_control_fscc().load_control_schedule_fsch.as_ref().unwrap_or(&load_control_fscc::LOAD_CONTROL_SCHEDULE_FSCH)
    }
    fn load_control_schedule_fsch_mut(&mut self) -> &mut LoadControlScheduleFsch {
        self._load_control_fscc_mut().load_control_schedule_fsch.get_or_insert(Default::default())
    }
}
impl IsLoadControlFscc for LoadControlFscc {
    fn _load_control_fscc(&self) -> &LoadControlFscc {
        self
    }
    fn _load_control_fscc_mut(&mut self) -> &mut LoadControlFscc {
        self
    }
}
impl IsControlFscc for LoadControlFscc {
    fn _control_fscc(&self) -> &super::commonmodule::ControlFscc {
        self.parent()
    }
    fn _control_fscc_mut(&mut self) -> &mut ControlFscc {
        self.parent_mut()
    }
}
impl IsLogicalNodeForControl for LoadControlFscc {
    fn _logical_node_for_control(&self) -> &super::commonmodule::LogicalNodeForControl {
        self.parent().parent()
    }
    fn _logical_node_for_control_mut(&mut self) -> &mut LogicalNodeForControl {
        self.parent_mut().parent_mut()
    }
}
impl IsLogicalNode for LoadControlFscc {
    fn _logical_node(&self) -> &super::commonmodule::LogicalNode {
        self.parent().parent().parent()
    }
    fn _logical_node_mut(&mut self) -> &mut LogicalNode {
        self.parent_mut().parent_mut().parent_mut()
    }
}
impl IsIdentifiedObject for LoadControlFscc {
    fn _identified_object(&self) -> &super::commonmodule::IdentifiedObject {
        self.parent().parent().parent().parent()
    }
    fn _identified_object_mut(&mut self) -> &mut IdentifiedObject {
        self.parent_mut().parent_mut().parent_mut().parent_mut()
    }
}
/// Load control
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize)]
pub struct LoadControl {
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
    #[serde(default, rename = "loadControlFSCC")]
    pub load_control_fscc: ::std::option::Option<LoadControlFscc>,
}
mod load_control {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref CONTROL_VALUE: crate::commonmodule::ControlValue = Default::default();
        pub(super) static ref CHECK: crate::commonmodule::CheckConditions = Default::default();
        pub(super) static ref LOAD_CONTROL_FSCC: crate::loadmodule::LoadControlFscc = Default::default();
    }
}
impl LoadControl {
    pub(crate) fn parent(&self) -> &super::commonmodule::ControlValue {
        self.control_value.as_ref().unwrap_or(&load_control::CONTROL_VALUE)
    }
    pub(crate) fn parent_mut(&mut self) -> &mut super::commonmodule::ControlValue {
        self.control_value.get_or_insert(Default::default())
    }
}
pub trait IsLoadControl {
    fn _load_control(&self) -> &LoadControl;
    fn _load_control_mut(&mut self) -> &mut LoadControl;
    fn control_value(&self) -> &super::commonmodule::ControlValue {
        self._load_control().control_value.as_ref().unwrap_or(&load_control::CONTROL_VALUE)
    }
    fn control_value_mut(&mut self) -> &mut super::commonmodule::ControlValue {
        self._load_control_mut().control_value.get_or_insert(Default::default())
    }
    fn check(&self) -> &super::commonmodule::CheckConditions {
        self._load_control().check.as_ref().unwrap_or(&load_control::CHECK)
    }
    fn check_mut(&mut self) -> &mut super::commonmodule::CheckConditions {
        self._load_control_mut().check.get_or_insert(Default::default())
    }
    fn load_control_fscc(&self) -> &LoadControlFscc {
        self._load_control().load_control_fscc.as_ref().unwrap_or(&load_control::LOAD_CONTROL_FSCC)
    }
    fn load_control_fscc_mut(&mut self) -> &mut LoadControlFscc {
        self._load_control_mut().load_control_fscc.get_or_insert(Default::default())
    }
}
impl IsLoadControl for LoadControl {
    fn _load_control(&self) -> &LoadControl {
        self
    }
    fn _load_control_mut(&mut self) -> &mut LoadControl {
        self
    }
}
impl IsControlValue for LoadControl {
    fn _control_value(&self) -> &super::commonmodule::ControlValue {
        self.parent()
    }
    fn _control_value_mut(&mut self) -> &mut ControlValue {
        self.parent_mut()
    }
}
impl IsIdentifiedObject for LoadControl {
    fn _identified_object(&self) -> &super::commonmodule::IdentifiedObject {
        self.parent().parent()
    }
    fn _identified_object_mut(&mut self) -> &mut IdentifiedObject {
        self.parent_mut().parent_mut()
    }
}
/// Load control profile
/// OpenFMB Profile Message: true
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize)]
pub struct LoadControlProfile {
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
    #[serde(default, rename = "energyConsumer")]
    pub energy_consumer: ::std::option::Option<super::commonmodule::EnergyConsumer>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: Some(1)
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, optional, tag="3")]
    #[serde(default, rename = "loadControl")]
    pub load_control: ::std::option::Option<LoadControl>,
}
mod load_control_profile {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref CONTROL_MESSAGE_INFO: crate::commonmodule::ControlMessageInfo = Default::default();
        pub(super) static ref ENERGY_CONSUMER: crate::commonmodule::EnergyConsumer = Default::default();
        pub(super) static ref LOAD_CONTROL: crate::loadmodule::LoadControl = Default::default();
    }
}
impl LoadControlProfile {
    pub(crate) fn parent(&self) -> &super::commonmodule::ControlMessageInfo {
        self.control_message_info.as_ref().unwrap_or(&load_control_profile::CONTROL_MESSAGE_INFO)
    }
    pub(crate) fn parent_mut(&mut self) -> &mut super::commonmodule::ControlMessageInfo {
        self.control_message_info.get_or_insert(Default::default())
    }
}
pub trait IsLoadControlProfile {
    fn _load_control_profile(&self) -> &LoadControlProfile;
    fn _load_control_profile_mut(&mut self) -> &mut LoadControlProfile;
    fn control_message_info(&self) -> &super::commonmodule::ControlMessageInfo {
        self._load_control_profile().control_message_info.as_ref().unwrap_or(&load_control_profile::CONTROL_MESSAGE_INFO)
    }
    fn control_message_info_mut(&mut self) -> &mut super::commonmodule::ControlMessageInfo {
        self._load_control_profile_mut().control_message_info.get_or_insert(Default::default())
    }
    fn energy_consumer(&self) -> &super::commonmodule::EnergyConsumer {
        self._load_control_profile().energy_consumer.as_ref().unwrap_or(&load_control_profile::ENERGY_CONSUMER)
    }
    fn energy_consumer_mut(&mut self) -> &mut super::commonmodule::EnergyConsumer {
        self._load_control_profile_mut().energy_consumer.get_or_insert(Default::default())
    }
    fn load_control(&self) -> &LoadControl {
        self._load_control_profile().load_control.as_ref().unwrap_or(&load_control_profile::LOAD_CONTROL)
    }
    fn load_control_mut(&mut self) -> &mut LoadControl {
        self._load_control_profile_mut().load_control.get_or_insert(Default::default())
    }
}
impl IsLoadControlProfile for LoadControlProfile {
    fn _load_control_profile(&self) -> &LoadControlProfile {
        self
    }
    fn _load_control_profile_mut(&mut self) -> &mut LoadControlProfile {
        self
    }
}
impl IsControlMessageInfo for LoadControlProfile {
    fn _control_message_info(&self) -> &super::commonmodule::ControlMessageInfo {
        self.parent()
    }
    fn _control_message_info_mut(&mut self) -> &mut ControlMessageInfo {
        self.parent_mut()
    }
}
impl IsMessageInfo for LoadControlProfile {
    fn _message_info(&self) -> &super::commonmodule::MessageInfo {
        self.parent().parent()
    }
    fn _message_info_mut(&mut self) -> &mut MessageInfo {
        self.parent_mut().parent_mut()
    }
}
impl IsIdentifiedObject for LoadControlProfile {
    fn _identified_object(&self) -> &super::commonmodule::IdentifiedObject {
        self.parent().parent().parent()
    }
    fn _identified_object_mut(&mut self) -> &mut IdentifiedObject {
        self.parent_mut().parent_mut().parent_mut()
    }
}
/// Point definition (Point)
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize)]
pub struct LoadPointStatus {
    /// Ramp rates
    #[prost(message, optional, tag="1")]
    #[serde(default, rename = "rampRates")]
    pub ramp_rates: ::std::option::Option<super::commonmodule::RampRate>,
    /// Enable reactive power set point
    #[prost(message, optional, tag="2")]
    #[serde(default, rename = "reactivePwrSetPointEnabled")]
    pub reactive_pwr_set_point_enabled: ::std::option::Option<super::commonmodule::StatusDps>,
    /// Enable joint real power set point
    #[prost(message, optional, tag="3")]
    #[serde(default, rename = "realPwrSetPointEnabled")]
    pub real_pwr_set_point_enabled: ::std::option::Option<super::commonmodule::StatusDps>,
    /// Reset device
    #[prost(message, optional, tag="4")]
    #[serde(default)]
    pub reset: ::std::option::Option<super::commonmodule::StatusDps>,
    /// ESS state
    #[prost(message, optional, tag="5")]
    #[serde(default)]
    pub state: ::std::option::Option<super::commonmodule::OptionalStateKind>,
}
mod load_point_status {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref RAMP_RATES: crate::commonmodule::RampRate = Default::default();
        pub(super) static ref REACTIVE_PWR_SET_POINT_ENABLED: crate::commonmodule::StatusDps = Default::default();
        pub(super) static ref REAL_PWR_SET_POINT_ENABLED: crate::commonmodule::StatusDps = Default::default();
        pub(super) static ref RESET: crate::commonmodule::StatusDps = Default::default();
        pub(super) static ref STATE: crate::commonmodule::OptionalStateKind = Default::default();
    }
}
impl LoadPointStatus {
}
pub trait IsLoadPointStatus {
    fn _load_point_status(&self) -> &LoadPointStatus;
    fn _load_point_status_mut(&mut self) -> &mut LoadPointStatus;
    fn ramp_rates(&self) -> &super::commonmodule::RampRate {
        self._load_point_status().ramp_rates.as_ref().unwrap_or(&load_point_status::RAMP_RATES)
    }
    fn ramp_rates_mut(&mut self) -> &mut super::commonmodule::RampRate {
        self._load_point_status_mut().ramp_rates.get_or_insert(Default::default())
    }
    fn reactive_pwr_set_point_enabled(&self) -> &super::commonmodule::StatusDps {
        self._load_point_status().reactive_pwr_set_point_enabled.as_ref().unwrap_or(&load_point_status::REACTIVE_PWR_SET_POINT_ENABLED)
    }
    fn reactive_pwr_set_point_enabled_mut(&mut self) -> &mut super::commonmodule::StatusDps {
        self._load_point_status_mut().reactive_pwr_set_point_enabled.get_or_insert(Default::default())
    }
    fn real_pwr_set_point_enabled(&self) -> &super::commonmodule::StatusDps {
        self._load_point_status().real_pwr_set_point_enabled.as_ref().unwrap_or(&load_point_status::REAL_PWR_SET_POINT_ENABLED)
    }
    fn real_pwr_set_point_enabled_mut(&mut self) -> &mut super::commonmodule::StatusDps {
        self._load_point_status_mut().real_pwr_set_point_enabled.get_or_insert(Default::default())
    }
    fn reset(&self) -> &super::commonmodule::StatusDps {
        self._load_point_status().reset.as_ref().unwrap_or(&load_point_status::RESET)
    }
    fn reset_mut(&mut self) -> &mut super::commonmodule::StatusDps {
        self._load_point_status_mut().reset.get_or_insert(Default::default())
    }
    fn state(&self) -> &super::commonmodule::OptionalStateKind {
        self._load_point_status().state.as_ref().unwrap_or(&load_point_status::STATE)
    }
    fn state_mut(&mut self) -> &mut super::commonmodule::OptionalStateKind {
        self._load_point_status_mut().state.get_or_insert(Default::default())
    }
}
impl IsLoadPointStatus for LoadPointStatus {
    fn _load_point_status(&self) -> &LoadPointStatus {
        self
    }
    fn _load_point_status_mut(&mut self) -> &mut LoadPointStatus {
        self
    }
}
/// Specialized 61850 ZGLD class
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize)]
pub struct LoadEventAndStatusZgld {
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
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    #[serde(default, rename = "DynamicTest")]
    pub dynamic_test: ::std::option::Option<super::commonmodule::EnsDynamicTestKind>,
    /// Emergency stop
    #[prost(message, optional, tag="3")]
    #[serde(default, rename = "EmgStop")]
    pub emg_stop: ::std::option::Option<super::commonmodule::StatusSps>,
    /// Point status
    #[prost(message, optional, tag="4")]
    #[serde(default, rename = "PointStatus")]
    pub point_status: ::std::option::Option<LoadPointStatus>,
}
mod load_event_and_status_zgld {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref LOGICAL_NODE_FOR_EVENT_AND_STATUS: crate::commonmodule::LogicalNodeForEventAndStatus = Default::default();
        pub(super) static ref DYNAMIC_TEST: crate::commonmodule::EnsDynamicTestKind = Default::default();
        pub(super) static ref EMG_STOP: crate::commonmodule::StatusSps = Default::default();
        pub(super) static ref POINT_STATUS: crate::loadmodule::LoadPointStatus = Default::default();
    }
}
impl LoadEventAndStatusZgld {
    pub(crate) fn parent(&self) -> &super::commonmodule::LogicalNodeForEventAndStatus {
        self.logical_node_for_event_and_status.as_ref().unwrap_or(&load_event_and_status_zgld::LOGICAL_NODE_FOR_EVENT_AND_STATUS)
    }
    pub(crate) fn parent_mut(&mut self) -> &mut super::commonmodule::LogicalNodeForEventAndStatus {
        self.logical_node_for_event_and_status.get_or_insert(Default::default())
    }
}
pub trait IsLoadEventAndStatusZgld {
    fn _load_event_and_status_zgld(&self) -> &LoadEventAndStatusZgld;
    fn _load_event_and_status_zgld_mut(&mut self) -> &mut LoadEventAndStatusZgld;
    fn logical_node_for_event_and_status(&self) -> &super::commonmodule::LogicalNodeForEventAndStatus {
        self._load_event_and_status_zgld().logical_node_for_event_and_status.as_ref().unwrap_or(&load_event_and_status_zgld::LOGICAL_NODE_FOR_EVENT_AND_STATUS)
    }
    fn logical_node_for_event_and_status_mut(&mut self) -> &mut super::commonmodule::LogicalNodeForEventAndStatus {
        self._load_event_and_status_zgld_mut().logical_node_for_event_and_status.get_or_insert(Default::default())
    }
    fn dynamic_test(&self) -> &super::commonmodule::EnsDynamicTestKind {
        self._load_event_and_status_zgld().dynamic_test.as_ref().unwrap_or(&load_event_and_status_zgld::DYNAMIC_TEST)
    }
    fn dynamic_test_mut(&mut self) -> &mut super::commonmodule::EnsDynamicTestKind {
        self._load_event_and_status_zgld_mut().dynamic_test.get_or_insert(Default::default())
    }
    fn emg_stop(&self) -> &super::commonmodule::StatusSps {
        self._load_event_and_status_zgld().emg_stop.as_ref().unwrap_or(&load_event_and_status_zgld::EMG_STOP)
    }
    fn emg_stop_mut(&mut self) -> &mut super::commonmodule::StatusSps {
        self._load_event_and_status_zgld_mut().emg_stop.get_or_insert(Default::default())
    }
    fn point_status(&self) -> &LoadPointStatus {
        self._load_event_and_status_zgld().point_status.as_ref().unwrap_or(&load_event_and_status_zgld::POINT_STATUS)
    }
    fn point_status_mut(&mut self) -> &mut LoadPointStatus {
        self._load_event_and_status_zgld_mut().point_status.get_or_insert(Default::default())
    }
}
impl IsLoadEventAndStatusZgld for LoadEventAndStatusZgld {
    fn _load_event_and_status_zgld(&self) -> &LoadEventAndStatusZgld {
        self
    }
    fn _load_event_and_status_zgld_mut(&mut self) -> &mut LoadEventAndStatusZgld {
        self
    }
}
impl IsLogicalNodeForEventAndStatus for LoadEventAndStatusZgld {
    fn _logical_node_for_event_and_status(&self) -> &super::commonmodule::LogicalNodeForEventAndStatus {
        self.parent()
    }
    fn _logical_node_for_event_and_status_mut(&mut self) -> &mut LogicalNodeForEventAndStatus {
        self.parent_mut()
    }
}
impl IsLogicalNode for LoadEventAndStatusZgld {
    fn _logical_node(&self) -> &super::commonmodule::LogicalNode {
        self.parent().parent()
    }
    fn _logical_node_mut(&mut self) -> &mut LogicalNode {
        self.parent_mut().parent_mut()
    }
}
impl IsIdentifiedObject for LoadEventAndStatusZgld {
    fn _identified_object(&self) -> &super::commonmodule::IdentifiedObject {
        self.parent().parent().parent()
    }
    fn _identified_object_mut(&mut self) -> &mut IdentifiedObject {
        self.parent_mut().parent_mut().parent_mut()
    }
}
/// Specialized 61850 ZGLD LN class
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize)]
pub struct LoadEventZgld {
    /// UML inherited base object
    // parent_message: true
    // required_field: false
    // multiplicity_min: None
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, optional, tag="1")]
    #[serde(default, rename = "loadEventAndStatusZGLD")]
    pub load_event_and_status_zgld: ::std::option::Option<LoadEventAndStatusZgld>,
}
mod load_event_zgld {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref LOAD_EVENT_AND_STATUS_ZGLD: crate::loadmodule::LoadEventAndStatusZgld = Default::default();
    }
}
impl LoadEventZgld {
    pub(crate) fn parent(&self) -> &LoadEventAndStatusZgld {
        self.load_event_and_status_zgld.as_ref().unwrap_or(&load_event_zgld::LOAD_EVENT_AND_STATUS_ZGLD)
    }
    pub(crate) fn parent_mut(&mut self) -> &mut LoadEventAndStatusZgld {
        self.load_event_and_status_zgld.get_or_insert(Default::default())
    }
}
pub trait IsLoadEventZgld {
    fn _load_event_zgld(&self) -> &LoadEventZgld;
    fn _load_event_zgld_mut(&mut self) -> &mut LoadEventZgld;
    fn load_event_and_status_zgld(&self) -> &LoadEventAndStatusZgld {
        self._load_event_zgld().load_event_and_status_zgld.as_ref().unwrap_or(&load_event_zgld::LOAD_EVENT_AND_STATUS_ZGLD)
    }
    fn load_event_and_status_zgld_mut(&mut self) -> &mut LoadEventAndStatusZgld {
        self._load_event_zgld_mut().load_event_and_status_zgld.get_or_insert(Default::default())
    }
}
impl IsLoadEventZgld for LoadEventZgld {
    fn _load_event_zgld(&self) -> &LoadEventZgld {
        self
    }
    fn _load_event_zgld_mut(&mut self) -> &mut LoadEventZgld {
        self
    }
}
impl IsLoadEventAndStatusZgld for LoadEventZgld {
    fn _load_event_and_status_zgld(&self) -> &LoadEventAndStatusZgld {
        self.parent()
    }
    fn _load_event_and_status_zgld_mut(&mut self) -> &mut LoadEventAndStatusZgld {
        self.parent_mut()
    }
}
impl IsLogicalNodeForEventAndStatus for LoadEventZgld {
    fn _logical_node_for_event_and_status(&self) -> &super::commonmodule::LogicalNodeForEventAndStatus {
        self.parent().parent()
    }
    fn _logical_node_for_event_and_status_mut(&mut self) -> &mut LogicalNodeForEventAndStatus {
        self.parent_mut().parent_mut()
    }
}
impl IsLogicalNode for LoadEventZgld {
    fn _logical_node(&self) -> &super::commonmodule::LogicalNode {
        self.parent().parent().parent()
    }
    fn _logical_node_mut(&mut self) -> &mut LogicalNode {
        self.parent_mut().parent_mut().parent_mut()
    }
}
impl IsIdentifiedObject for LoadEventZgld {
    fn _identified_object(&self) -> &super::commonmodule::IdentifiedObject {
        self.parent().parent().parent().parent()
    }
    fn _identified_object_mut(&mut self) -> &mut IdentifiedObject {
        self.parent_mut().parent_mut().parent_mut().parent_mut()
    }
}
/// Load event
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize)]
pub struct LoadEvent {
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
    #[serde(default, rename = "loadEventZGLD")]
    pub load_event_zgld: ::std::option::Option<LoadEventZgld>,
}
mod load_event {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref EVENT_VALUE: crate::commonmodule::EventValue = Default::default();
        pub(super) static ref LOAD_EVENT_ZGLD: crate::loadmodule::LoadEventZgld = Default::default();
    }
}
impl LoadEvent {
    pub(crate) fn parent(&self) -> &super::commonmodule::EventValue {
        self.event_value.as_ref().unwrap_or(&load_event::EVENT_VALUE)
    }
    pub(crate) fn parent_mut(&mut self) -> &mut super::commonmodule::EventValue {
        self.event_value.get_or_insert(Default::default())
    }
}
pub trait IsLoadEvent {
    fn _load_event(&self) -> &LoadEvent;
    fn _load_event_mut(&mut self) -> &mut LoadEvent;
    fn event_value(&self) -> &super::commonmodule::EventValue {
        self._load_event().event_value.as_ref().unwrap_or(&load_event::EVENT_VALUE)
    }
    fn event_value_mut(&mut self) -> &mut super::commonmodule::EventValue {
        self._load_event_mut().event_value.get_or_insert(Default::default())
    }
    fn load_event_zgld(&self) -> &LoadEventZgld {
        self._load_event().load_event_zgld.as_ref().unwrap_or(&load_event::LOAD_EVENT_ZGLD)
    }
    fn load_event_zgld_mut(&mut self) -> &mut LoadEventZgld {
        self._load_event_mut().load_event_zgld.get_or_insert(Default::default())
    }
}
impl IsLoadEvent for LoadEvent {
    fn _load_event(&self) -> &LoadEvent {
        self
    }
    fn _load_event_mut(&mut self) -> &mut LoadEvent {
        self
    }
}
impl IsEventValue for LoadEvent {
    fn _event_value(&self) -> &super::commonmodule::EventValue {
        self.parent()
    }
    fn _event_value_mut(&mut self) -> &mut EventValue {
        self.parent_mut()
    }
}
impl IsIdentifiedObject for LoadEvent {
    fn _identified_object(&self) -> &super::commonmodule::IdentifiedObject {
        self.parent().parent()
    }
    fn _identified_object_mut(&mut self) -> &mut IdentifiedObject {
        self.parent_mut().parent_mut()
    }
}
/// Load event profile
/// OpenFMB Profile Message: true
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize)]
pub struct LoadEventProfile {
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
    #[serde(default, rename = "energyConsumer")]
    pub energy_consumer: ::std::option::Option<super::commonmodule::EnergyConsumer>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: Some(1)
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, optional, tag="3")]
    #[serde(default, rename = "loadEvent")]
    pub load_event: ::std::option::Option<LoadEvent>,
}
mod load_event_profile {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref EVENT_MESSAGE_INFO: crate::commonmodule::EventMessageInfo = Default::default();
        pub(super) static ref ENERGY_CONSUMER: crate::commonmodule::EnergyConsumer = Default::default();
        pub(super) static ref LOAD_EVENT: crate::loadmodule::LoadEvent = Default::default();
    }
}
impl LoadEventProfile {
    pub(crate) fn parent(&self) -> &super::commonmodule::EventMessageInfo {
        self.event_message_info.as_ref().unwrap_or(&load_event_profile::EVENT_MESSAGE_INFO)
    }
    pub(crate) fn parent_mut(&mut self) -> &mut super::commonmodule::EventMessageInfo {
        self.event_message_info.get_or_insert(Default::default())
    }
}
pub trait IsLoadEventProfile {
    fn _load_event_profile(&self) -> &LoadEventProfile;
    fn _load_event_profile_mut(&mut self) -> &mut LoadEventProfile;
    fn event_message_info(&self) -> &super::commonmodule::EventMessageInfo {
        self._load_event_profile().event_message_info.as_ref().unwrap_or(&load_event_profile::EVENT_MESSAGE_INFO)
    }
    fn event_message_info_mut(&mut self) -> &mut super::commonmodule::EventMessageInfo {
        self._load_event_profile_mut().event_message_info.get_or_insert(Default::default())
    }
    fn energy_consumer(&self) -> &super::commonmodule::EnergyConsumer {
        self._load_event_profile().energy_consumer.as_ref().unwrap_or(&load_event_profile::ENERGY_CONSUMER)
    }
    fn energy_consumer_mut(&mut self) -> &mut super::commonmodule::EnergyConsumer {
        self._load_event_profile_mut().energy_consumer.get_or_insert(Default::default())
    }
    fn load_event(&self) -> &LoadEvent {
        self._load_event_profile().load_event.as_ref().unwrap_or(&load_event_profile::LOAD_EVENT)
    }
    fn load_event_mut(&mut self) -> &mut LoadEvent {
        self._load_event_profile_mut().load_event.get_or_insert(Default::default())
    }
}
impl IsLoadEventProfile for LoadEventProfile {
    fn _load_event_profile(&self) -> &LoadEventProfile {
        self
    }
    fn _load_event_profile_mut(&mut self) -> &mut LoadEventProfile {
        self
    }
}
impl IsEventMessageInfo for LoadEventProfile {
    fn _event_message_info(&self) -> &super::commonmodule::EventMessageInfo {
        self.parent()
    }
    fn _event_message_info_mut(&mut self) -> &mut EventMessageInfo {
        self.parent_mut()
    }
}
impl IsMessageInfo for LoadEventProfile {
    fn _message_info(&self) -> &super::commonmodule::MessageInfo {
        self.parent().parent()
    }
    fn _message_info_mut(&mut self) -> &mut MessageInfo {
        self.parent_mut().parent_mut()
    }
}
impl IsIdentifiedObject for LoadEventProfile {
    fn _identified_object(&self) -> &super::commonmodule::IdentifiedObject {
        self.parent().parent().parent()
    }
    fn _identified_object_mut(&mut self) -> &mut IdentifiedObject {
        self.parent_mut().parent_mut().parent_mut()
    }
}
/// Load reading value
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize)]
pub struct LoadReading {
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
mod load_reading {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref CONDUCTING_EQUIPMENT_TERMINAL_READING: crate::commonmodule::ConductingEquipmentTerminalReading = Default::default();
        pub(super) static ref PHASE_MMTN: crate::commonmodule::PhaseMmtn = Default::default();
        pub(super) static ref READING_MMTR: crate::commonmodule::ReadingMmtr = Default::default();
        pub(super) static ref READING_MMXU: crate::commonmodule::ReadingMmxu = Default::default();
    }
}
impl LoadReading {
    pub(crate) fn parent(&self) -> &super::commonmodule::ConductingEquipmentTerminalReading {
        self.conducting_equipment_terminal_reading.as_ref().unwrap_or(&load_reading::CONDUCTING_EQUIPMENT_TERMINAL_READING)
    }
    pub(crate) fn parent_mut(&mut self) -> &mut super::commonmodule::ConductingEquipmentTerminalReading {
        self.conducting_equipment_terminal_reading.get_or_insert(Default::default())
    }
}
pub trait IsLoadReading {
    fn _load_reading(&self) -> &LoadReading;
    fn _load_reading_mut(&mut self) -> &mut LoadReading;
    fn conducting_equipment_terminal_reading(&self) -> &super::commonmodule::ConductingEquipmentTerminalReading {
        self._load_reading().conducting_equipment_terminal_reading.as_ref().unwrap_or(&load_reading::CONDUCTING_EQUIPMENT_TERMINAL_READING)
    }
    fn conducting_equipment_terminal_reading_mut(&mut self) -> &mut super::commonmodule::ConductingEquipmentTerminalReading {
        self._load_reading_mut().conducting_equipment_terminal_reading.get_or_insert(Default::default())
    }
    fn phase_mmtn(&self) -> &super::commonmodule::PhaseMmtn {
        self._load_reading().phase_mmtn.as_ref().unwrap_or(&load_reading::PHASE_MMTN)
    }
    fn phase_mmtn_mut(&mut self) -> &mut super::commonmodule::PhaseMmtn {
        self._load_reading_mut().phase_mmtn.get_or_insert(Default::default())
    }
    fn reading_mmtr(&self) -> &super::commonmodule::ReadingMmtr {
        self._load_reading().reading_mmtr.as_ref().unwrap_or(&load_reading::READING_MMTR)
    }
    fn reading_mmtr_mut(&mut self) -> &mut super::commonmodule::ReadingMmtr {
        self._load_reading_mut().reading_mmtr.get_or_insert(Default::default())
    }
    fn reading_mmxu(&self) -> &super::commonmodule::ReadingMmxu {
        self._load_reading().reading_mmxu.as_ref().unwrap_or(&load_reading::READING_MMXU)
    }
    fn reading_mmxu_mut(&mut self) -> &mut super::commonmodule::ReadingMmxu {
        self._load_reading_mut().reading_mmxu.get_or_insert(Default::default())
    }
}
impl IsLoadReading for LoadReading {
    fn _load_reading(&self) -> &LoadReading {
        self
    }
    fn _load_reading_mut(&mut self) -> &mut LoadReading {
        self
    }
}
impl IsConductingEquipmentTerminalReading for LoadReading {
    fn _conducting_equipment_terminal_reading(&self) -> &super::commonmodule::ConductingEquipmentTerminalReading {
        self.parent()
    }
    fn _conducting_equipment_terminal_reading_mut(&mut self) -> &mut ConductingEquipmentTerminalReading {
        self.parent_mut()
    }
}
/// Load reading profile
/// OpenFMB Profile Message: true
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize)]
pub struct LoadReadingProfile {
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
    #[serde(default, rename = "energyConsumer")]
    pub energy_consumer: ::std::option::Option<super::commonmodule::EnergyConsumer>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: Some(1)
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, optional, tag="3")]
    #[serde(default, rename = "loadReading")]
    pub load_reading: ::std::option::Option<LoadReading>,
}
mod load_reading_profile {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref READING_MESSAGE_INFO: crate::commonmodule::ReadingMessageInfo = Default::default();
        pub(super) static ref ENERGY_CONSUMER: crate::commonmodule::EnergyConsumer = Default::default();
        pub(super) static ref LOAD_READING: crate::loadmodule::LoadReading = Default::default();
    }
}
impl LoadReadingProfile {
    pub(crate) fn parent(&self) -> &super::commonmodule::ReadingMessageInfo {
        self.reading_message_info.as_ref().unwrap_or(&load_reading_profile::READING_MESSAGE_INFO)
    }
    pub(crate) fn parent_mut(&mut self) -> &mut super::commonmodule::ReadingMessageInfo {
        self.reading_message_info.get_or_insert(Default::default())
    }
}
pub trait IsLoadReadingProfile {
    fn _load_reading_profile(&self) -> &LoadReadingProfile;
    fn _load_reading_profile_mut(&mut self) -> &mut LoadReadingProfile;
    fn reading_message_info(&self) -> &super::commonmodule::ReadingMessageInfo {
        self._load_reading_profile().reading_message_info.as_ref().unwrap_or(&load_reading_profile::READING_MESSAGE_INFO)
    }
    fn reading_message_info_mut(&mut self) -> &mut super::commonmodule::ReadingMessageInfo {
        self._load_reading_profile_mut().reading_message_info.get_or_insert(Default::default())
    }
    fn energy_consumer(&self) -> &super::commonmodule::EnergyConsumer {
        self._load_reading_profile().energy_consumer.as_ref().unwrap_or(&load_reading_profile::ENERGY_CONSUMER)
    }
    fn energy_consumer_mut(&mut self) -> &mut super::commonmodule::EnergyConsumer {
        self._load_reading_profile_mut().energy_consumer.get_or_insert(Default::default())
    }
    fn load_reading(&self) -> &LoadReading {
        self._load_reading_profile().load_reading.as_ref().unwrap_or(&load_reading_profile::LOAD_READING)
    }
    fn load_reading_mut(&mut self) -> &mut LoadReading {
        self._load_reading_profile_mut().load_reading.get_or_insert(Default::default())
    }
}
impl IsLoadReadingProfile for LoadReadingProfile {
    fn _load_reading_profile(&self) -> &LoadReadingProfile {
        self
    }
    fn _load_reading_profile_mut(&mut self) -> &mut LoadReadingProfile {
        self
    }
}
impl IsReadingMessageInfo for LoadReadingProfile {
    fn _reading_message_info(&self) -> &super::commonmodule::ReadingMessageInfo {
        self.parent()
    }
    fn _reading_message_info_mut(&mut self) -> &mut ReadingMessageInfo {
        self.parent_mut()
    }
}
impl IsMessageInfo for LoadReadingProfile {
    fn _message_info(&self) -> &super::commonmodule::MessageInfo {
        self.parent().parent()
    }
    fn _message_info_mut(&mut self) -> &mut MessageInfo {
        self.parent_mut().parent_mut()
    }
}
impl IsIdentifiedObject for LoadReadingProfile {
    fn _identified_object(&self) -> &super::commonmodule::IdentifiedObject {
        self.parent().parent().parent()
    }
    fn _identified_object_mut(&mut self) -> &mut IdentifiedObject {
        self.parent_mut().parent_mut().parent_mut()
    }
}
/// Specialized 61850 ZGLD LN class
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize)]
pub struct LoadStatusZgld {
    /// UML inherited base object
    // parent_message: true
    // required_field: false
    // multiplicity_min: None
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, optional, tag="1")]
    #[serde(default, rename = "loadEventAndStatusZGLD")]
    pub load_event_and_status_zgld: ::std::option::Option<LoadEventAndStatusZgld>,
}
mod load_status_zgld {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref LOAD_EVENT_AND_STATUS_ZGLD: crate::loadmodule::LoadEventAndStatusZgld = Default::default();
    }
}
impl LoadStatusZgld {
    pub(crate) fn parent(&self) -> &LoadEventAndStatusZgld {
        self.load_event_and_status_zgld.as_ref().unwrap_or(&load_status_zgld::LOAD_EVENT_AND_STATUS_ZGLD)
    }
    pub(crate) fn parent_mut(&mut self) -> &mut LoadEventAndStatusZgld {
        self.load_event_and_status_zgld.get_or_insert(Default::default())
    }
}
pub trait IsLoadStatusZgld {
    fn _load_status_zgld(&self) -> &LoadStatusZgld;
    fn _load_status_zgld_mut(&mut self) -> &mut LoadStatusZgld;
    fn load_event_and_status_zgld(&self) -> &LoadEventAndStatusZgld {
        self._load_status_zgld().load_event_and_status_zgld.as_ref().unwrap_or(&load_status_zgld::LOAD_EVENT_AND_STATUS_ZGLD)
    }
    fn load_event_and_status_zgld_mut(&mut self) -> &mut LoadEventAndStatusZgld {
        self._load_status_zgld_mut().load_event_and_status_zgld.get_or_insert(Default::default())
    }
}
impl IsLoadStatusZgld for LoadStatusZgld {
    fn _load_status_zgld(&self) -> &LoadStatusZgld {
        self
    }
    fn _load_status_zgld_mut(&mut self) -> &mut LoadStatusZgld {
        self
    }
}
impl IsLoadEventAndStatusZgld for LoadStatusZgld {
    fn _load_event_and_status_zgld(&self) -> &LoadEventAndStatusZgld {
        self.parent()
    }
    fn _load_event_and_status_zgld_mut(&mut self) -> &mut LoadEventAndStatusZgld {
        self.parent_mut()
    }
}
impl IsLogicalNodeForEventAndStatus for LoadStatusZgld {
    fn _logical_node_for_event_and_status(&self) -> &super::commonmodule::LogicalNodeForEventAndStatus {
        self.parent().parent()
    }
    fn _logical_node_for_event_and_status_mut(&mut self) -> &mut LogicalNodeForEventAndStatus {
        self.parent_mut().parent_mut()
    }
}
impl IsLogicalNode for LoadStatusZgld {
    fn _logical_node(&self) -> &super::commonmodule::LogicalNode {
        self.parent().parent().parent()
    }
    fn _logical_node_mut(&mut self) -> &mut LogicalNode {
        self.parent_mut().parent_mut().parent_mut()
    }
}
impl IsIdentifiedObject for LoadStatusZgld {
    fn _identified_object(&self) -> &super::commonmodule::IdentifiedObject {
        self.parent().parent().parent().parent()
    }
    fn _identified_object_mut(&mut self) -> &mut IdentifiedObject {
        self.parent_mut().parent_mut().parent_mut().parent_mut()
    }
}
/// Load status
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize)]
pub struct LoadStatus {
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
    /// True if the load is uncontrollable.
    #[prost(message, optional, tag="2")]
    #[serde(default, rename = "isUncontrollable")]
    pub is_uncontrollable: ::std::option::Option<bool>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    #[serde(default, rename = "loadStatusZGLD")]
    pub load_status_zgld: ::std::option::Option<LoadStatusZgld>,
}
mod load_status {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref STATUS_VALUE: crate::commonmodule::StatusValue = Default::default();
        pub(super) static ref IS_UNCONTROLLABLE: bool = Default::default();
        pub(super) static ref LOAD_STATUS_ZGLD: crate::loadmodule::LoadStatusZgld = Default::default();
    }
}
impl LoadStatus {
    pub(crate) fn parent(&self) -> &super::commonmodule::StatusValue {
        self.status_value.as_ref().unwrap_or(&load_status::STATUS_VALUE)
    }
    pub(crate) fn parent_mut(&mut self) -> &mut super::commonmodule::StatusValue {
        self.status_value.get_or_insert(Default::default())
    }
}
pub trait IsLoadStatus {
    fn _load_status(&self) -> &LoadStatus;
    fn _load_status_mut(&mut self) -> &mut LoadStatus;
    fn status_value(&self) -> &super::commonmodule::StatusValue {
        self._load_status().status_value.as_ref().unwrap_or(&load_status::STATUS_VALUE)
    }
    fn status_value_mut(&mut self) -> &mut super::commonmodule::StatusValue {
        self._load_status_mut().status_value.get_or_insert(Default::default())
    }
    fn is_uncontrollable(&self) -> &bool {
        self._load_status().is_uncontrollable.as_ref().unwrap_or(&load_status::IS_UNCONTROLLABLE)
    }
    fn is_uncontrollable_mut(&mut self) -> &mut bool {
        self._load_status_mut().is_uncontrollable.get_or_insert(Default::default())
    }
    fn load_status_zgld(&self) -> &LoadStatusZgld {
        self._load_status().load_status_zgld.as_ref().unwrap_or(&load_status::LOAD_STATUS_ZGLD)
    }
    fn load_status_zgld_mut(&mut self) -> &mut LoadStatusZgld {
        self._load_status_mut().load_status_zgld.get_or_insert(Default::default())
    }
}
impl IsLoadStatus for LoadStatus {
    fn _load_status(&self) -> &LoadStatus {
        self
    }
    fn _load_status_mut(&mut self) -> &mut LoadStatus {
        self
    }
}
impl IsStatusValue for LoadStatus {
    fn _status_value(&self) -> &super::commonmodule::StatusValue {
        self.parent()
    }
    fn _status_value_mut(&mut self) -> &mut StatusValue {
        self.parent_mut()
    }
}
impl IsIdentifiedObject for LoadStatus {
    fn _identified_object(&self) -> &super::commonmodule::IdentifiedObject {
        self.parent().parent()
    }
    fn _identified_object_mut(&mut self) -> &mut IdentifiedObject {
        self.parent_mut().parent_mut()
    }
}
/// Load status profile
/// OpenFMB Profile Message: true
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize)]
pub struct LoadStatusProfile {
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
    #[serde(default, rename = "energyConsumer")]
    pub energy_consumer: ::std::option::Option<super::commonmodule::EnergyConsumer>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: Some(1)
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, optional, tag="3")]
    #[serde(default, rename = "loadStatus")]
    pub load_status: ::std::option::Option<LoadStatus>,
}
mod load_status_profile {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref STATUS_MESSAGE_INFO: crate::commonmodule::StatusMessageInfo = Default::default();
        pub(super) static ref ENERGY_CONSUMER: crate::commonmodule::EnergyConsumer = Default::default();
        pub(super) static ref LOAD_STATUS: crate::loadmodule::LoadStatus = Default::default();
    }
}
impl LoadStatusProfile {
    pub(crate) fn parent(&self) -> &super::commonmodule::StatusMessageInfo {
        self.status_message_info.as_ref().unwrap_or(&load_status_profile::STATUS_MESSAGE_INFO)
    }
    pub(crate) fn parent_mut(&mut self) -> &mut super::commonmodule::StatusMessageInfo {
        self.status_message_info.get_or_insert(Default::default())
    }
}
pub trait IsLoadStatusProfile {
    fn _load_status_profile(&self) -> &LoadStatusProfile;
    fn _load_status_profile_mut(&mut self) -> &mut LoadStatusProfile;
    fn status_message_info(&self) -> &super::commonmodule::StatusMessageInfo {
        self._load_status_profile().status_message_info.as_ref().unwrap_or(&load_status_profile::STATUS_MESSAGE_INFO)
    }
    fn status_message_info_mut(&mut self) -> &mut super::commonmodule::StatusMessageInfo {
        self._load_status_profile_mut().status_message_info.get_or_insert(Default::default())
    }
    fn energy_consumer(&self) -> &super::commonmodule::EnergyConsumer {
        self._load_status_profile().energy_consumer.as_ref().unwrap_or(&load_status_profile::ENERGY_CONSUMER)
    }
    fn energy_consumer_mut(&mut self) -> &mut super::commonmodule::EnergyConsumer {
        self._load_status_profile_mut().energy_consumer.get_or_insert(Default::default())
    }
    fn load_status(&self) -> &LoadStatus {
        self._load_status_profile().load_status.as_ref().unwrap_or(&load_status_profile::LOAD_STATUS)
    }
    fn load_status_mut(&mut self) -> &mut LoadStatus {
        self._load_status_profile_mut().load_status.get_or_insert(Default::default())
    }
}
impl IsLoadStatusProfile for LoadStatusProfile {
    fn _load_status_profile(&self) -> &LoadStatusProfile {
        self
    }
    fn _load_status_profile_mut(&mut self) -> &mut LoadStatusProfile {
        self
    }
}
impl IsStatusMessageInfo for LoadStatusProfile {
    fn _status_message_info(&self) -> &super::commonmodule::StatusMessageInfo {
        self.parent()
    }
    fn _status_message_info_mut(&mut self) -> &mut StatusMessageInfo {
        self.parent_mut()
    }
}
impl IsMessageInfo for LoadStatusProfile {
    fn _message_info(&self) -> &super::commonmodule::MessageInfo {
        self.parent().parent()
    }
    fn _message_info_mut(&mut self) -> &mut MessageInfo {
        self.parent_mut().parent_mut()
    }
}
impl IsIdentifiedObject for LoadStatusProfile {
    fn _identified_object(&self) -> &super::commonmodule::IdentifiedObject {
        self.parent().parent().parent()
    }
    fn _identified_object_mut(&mut self) -> &mut IdentifiedObject {
        self.parent_mut().parent_mut().parent_mut()
    }
}