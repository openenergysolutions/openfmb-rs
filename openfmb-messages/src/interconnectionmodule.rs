use crate::commonmodule::*;
/// Point definition (Point)
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct InterconnectionPoint {
    /// Black start enable
    #[prost(message, optional, tag="1")]
    pub black_start_enabled: ::std::option::Option<super::commonmodule::ControlDpc>,
    /// Enable frequency set point
    #[prost(message, optional, tag="2")]
    pub frequency_set_point_enabled: ::std::option::Option<super::commonmodule::ControlDpc>,
    /// Island control
    #[prost(message, optional, tag="3")]
    pub island: ::std::option::Option<super::commonmodule::ControlDpc>,
    /// Black start enable
    #[prost(message, optional, tag="4")]
    pub pct_hz_droop: ::std::option::Option<f32>,
    /// Black start enable
    #[prost(message, optional, tag="5")]
    pub pct_v_droop: ::std::option::Option<f32>,
    /// Ramp rates
    #[prost(message, optional, tag="6")]
    pub ramp_rates: ::std::option::Option<super::commonmodule::RampRate>,
    /// Enable reactive power set point
    #[prost(message, optional, tag="7")]
    pub reactive_pwr_set_point_enabled: ::std::option::Option<super::commonmodule::ControlDpc>,
    /// Enable real power set point
    #[prost(message, optional, tag="8")]
    pub real_pwr_set_point_enabled: ::std::option::Option<super::commonmodule::ControlDpc>,
    /// Enable voltage set point
    #[prost(message, optional, tag="9")]
    pub voltage_set_point_enabled: ::std::option::Option<super::commonmodule::ControlDpc>,
    /// Start time
    // parent_message: false
    // required_field: true
    // multiplicity_min: Some(1)
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, optional, tag="10")]
    pub start_time: ::std::option::Option<super::commonmodule::Timestamp>,
}
mod interconnection_point {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref BLACK_START_ENABLED: crate::commonmodule::ControlDpc = Default::default();
        pub(super) static ref FREQUENCY_SET_POINT_ENABLED: crate::commonmodule::ControlDpc = Default::default();
        pub(super) static ref ISLAND: crate::commonmodule::ControlDpc = Default::default();
        pub(super) static ref PCT_HZ_DROOP: f32 = Default::default();
        pub(super) static ref PCT_V_DROOP: f32 = Default::default();
        pub(super) static ref RAMP_RATES: crate::commonmodule::RampRate = Default::default();
        pub(super) static ref REACTIVE_PWR_SET_POINT_ENABLED: crate::commonmodule::ControlDpc = Default::default();
        pub(super) static ref REAL_PWR_SET_POINT_ENABLED: crate::commonmodule::ControlDpc = Default::default();
        pub(super) static ref VOLTAGE_SET_POINT_ENABLED: crate::commonmodule::ControlDpc = Default::default();
        pub(super) static ref START_TIME: crate::commonmodule::Timestamp = Default::default();
    }
}
impl InterconnectionPoint {
}
pub trait IsInterconnectionPoint {
    fn _interconnection_point(&self) -> &InterconnectionPoint;
    fn _interconnection_point_mut(&mut self) -> &mut InterconnectionPoint;
    fn black_start_enabled(&self) -> &super::commonmodule::ControlDpc {
        self._interconnection_point().black_start_enabled.as_ref().unwrap_or(&interconnection_point::BLACK_START_ENABLED)
    }
    fn black_start_enabled_mut(&mut self) -> &mut super::commonmodule::ControlDpc {
        self._interconnection_point_mut().black_start_enabled.get_or_insert(Default::default())
    }
    fn frequency_set_point_enabled(&self) -> &super::commonmodule::ControlDpc {
        self._interconnection_point().frequency_set_point_enabled.as_ref().unwrap_or(&interconnection_point::FREQUENCY_SET_POINT_ENABLED)
    }
    fn frequency_set_point_enabled_mut(&mut self) -> &mut super::commonmodule::ControlDpc {
        self._interconnection_point_mut().frequency_set_point_enabled.get_or_insert(Default::default())
    }
    fn island(&self) -> &super::commonmodule::ControlDpc {
        self._interconnection_point().island.as_ref().unwrap_or(&interconnection_point::ISLAND)
    }
    fn island_mut(&mut self) -> &mut super::commonmodule::ControlDpc {
        self._interconnection_point_mut().island.get_or_insert(Default::default())
    }
    fn pct_hz_droop(&self) -> &f32 {
        self._interconnection_point().pct_hz_droop.as_ref().unwrap_or(&interconnection_point::PCT_HZ_DROOP)
    }
    fn pct_hz_droop_mut(&mut self) -> &mut f32 {
        self._interconnection_point_mut().pct_hz_droop.get_or_insert(Default::default())
    }
    fn pct_v_droop(&self) -> &f32 {
        self._interconnection_point().pct_v_droop.as_ref().unwrap_or(&interconnection_point::PCT_V_DROOP)
    }
    fn pct_v_droop_mut(&mut self) -> &mut f32 {
        self._interconnection_point_mut().pct_v_droop.get_or_insert(Default::default())
    }
    fn ramp_rates(&self) -> &super::commonmodule::RampRate {
        self._interconnection_point().ramp_rates.as_ref().unwrap_or(&interconnection_point::RAMP_RATES)
    }
    fn ramp_rates_mut(&mut self) -> &mut super::commonmodule::RampRate {
        self._interconnection_point_mut().ramp_rates.get_or_insert(Default::default())
    }
    fn reactive_pwr_set_point_enabled(&self) -> &super::commonmodule::ControlDpc {
        self._interconnection_point().reactive_pwr_set_point_enabled.as_ref().unwrap_or(&interconnection_point::REACTIVE_PWR_SET_POINT_ENABLED)
    }
    fn reactive_pwr_set_point_enabled_mut(&mut self) -> &mut super::commonmodule::ControlDpc {
        self._interconnection_point_mut().reactive_pwr_set_point_enabled.get_or_insert(Default::default())
    }
    fn real_pwr_set_point_enabled(&self) -> &super::commonmodule::ControlDpc {
        self._interconnection_point().real_pwr_set_point_enabled.as_ref().unwrap_or(&interconnection_point::REAL_PWR_SET_POINT_ENABLED)
    }
    fn real_pwr_set_point_enabled_mut(&mut self) -> &mut super::commonmodule::ControlDpc {
        self._interconnection_point_mut().real_pwr_set_point_enabled.get_or_insert(Default::default())
    }
    fn voltage_set_point_enabled(&self) -> &super::commonmodule::ControlDpc {
        self._interconnection_point().voltage_set_point_enabled.as_ref().unwrap_or(&interconnection_point::VOLTAGE_SET_POINT_ENABLED)
    }
    fn voltage_set_point_enabled_mut(&mut self) -> &mut super::commonmodule::ControlDpc {
        self._interconnection_point_mut().voltage_set_point_enabled.get_or_insert(Default::default())
    }
    fn start_time(&self) -> &super::commonmodule::Timestamp {
        self._interconnection_point().start_time.as_ref().unwrap_or(&interconnection_point::START_TIME)
    }
    fn start_time_mut(&mut self) -> &mut super::commonmodule::Timestamp {
        self._interconnection_point_mut().start_time.get_or_insert(Default::default())
    }
}
impl IsInterconnectionPoint for InterconnectionPoint {
    fn _interconnection_point(&self) -> &InterconnectionPoint {
        self
    }
    fn _interconnection_point_mut(&mut self) -> &mut InterconnectionPoint {
        self
    }
}
/// Curve shape setting (FC=SP) (CSG_SP)
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct InterconnectionCsg {
    /// The array with the points specifying a curve shape.
    // parent_message: false
    // required_field: true
    // multiplicity_min: Some(1)
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, repeated, tag="1")]
    pub crv_pts: ::std::vec::Vec<InterconnectionPoint>,
}
mod interconnection_csg {
    use lazy_static::lazy_static;
    lazy_static! {
    }
}
impl InterconnectionCsg {
}
pub trait IsInterconnectionCsg {
    fn _interconnection_csg(&self) -> &InterconnectionCsg;
    fn _interconnection_csg_mut(&mut self) -> &mut InterconnectionCsg;
    fn crv_pts(&self) -> &::std::vec::Vec<InterconnectionPoint> {
        &self._interconnection_csg().crv_pts
    }
    fn crv_pts_mut(&mut self) -> &mut ::std::vec::Vec<InterconnectionPoint> {
        &mut self._interconnection_csg_mut().crv_pts
    }
}
impl IsInterconnectionCsg for InterconnectionCsg {
    fn _interconnection_csg(&self) -> &InterconnectionCsg {
        self
    }
    fn _interconnection_csg_mut(&mut self) -> &mut InterconnectionCsg {
        self
    }
}
/// OpenFMB specialization for control schedule using:  LN: Schedule   Name: FSCH
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct InterconnectionControlScheduleFsch {
    /// Discrete value in InterconnectionCSG type
    // parent_message: false
    // required_field: true
    // multiplicity_min: Some(1)
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, optional, tag="1")]
    pub val_dcsg: ::std::option::Option<InterconnectionCsg>,
}
mod interconnection_control_schedule_fsch {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref VAL_DCSG: crate::interconnectionmodule::InterconnectionCsg = Default::default();
    }
}
impl InterconnectionControlScheduleFsch {
}
pub trait IsInterconnectionControlScheduleFsch {
    fn _interconnection_control_schedule_fsch(&self) -> &InterconnectionControlScheduleFsch;
    fn _interconnection_control_schedule_fsch_mut(&mut self) -> &mut InterconnectionControlScheduleFsch;
    fn val_dcsg(&self) -> &InterconnectionCsg {
        self._interconnection_control_schedule_fsch().val_dcsg.as_ref().unwrap_or(&interconnection_control_schedule_fsch::VAL_DCSG)
    }
    fn val_dcsg_mut(&mut self) -> &mut InterconnectionCsg {
        self._interconnection_control_schedule_fsch_mut().val_dcsg.get_or_insert(Default::default())
    }
}
impl IsInterconnectionControlScheduleFsch for InterconnectionControlScheduleFsch {
    fn _interconnection_control_schedule_fsch(&self) -> &InterconnectionControlScheduleFsch {
        self
    }
    fn _interconnection_control_schedule_fsch_mut(&mut self) -> &mut InterconnectionControlScheduleFsch {
        self
    }
}
/// Specialized 61850 FSCC class for interconnection schedule
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct InterconnectionScheduleFscc {
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
    // parent_message: false
    // required_field: false
    // multiplicity_min: Some(0)
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, repeated, tag="2")]
    pub interconnection_control_schedule_fsch: ::std::vec::Vec<InterconnectionControlScheduleFsch>,
}
mod interconnection_schedule_fscc {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref CONTROL_FSCC: crate::commonmodule::ControlFscc = Default::default();
    }
}
impl InterconnectionScheduleFscc {
    pub(crate) fn parent(&self) -> &super::commonmodule::ControlFscc {
        self.control_fscc.as_ref().unwrap_or(&interconnection_schedule_fscc::CONTROL_FSCC)
    }
    pub(crate) fn parent_mut(&mut self) -> &mut super::commonmodule::ControlFscc {
        self.control_fscc.get_or_insert(Default::default())
    }
}
pub trait IsInterconnectionScheduleFscc {
    fn _interconnection_schedule_fscc(&self) -> &InterconnectionScheduleFscc;
    fn _interconnection_schedule_fscc_mut(&mut self) -> &mut InterconnectionScheduleFscc;
    fn control_fscc(&self) -> &super::commonmodule::ControlFscc {
        self._interconnection_schedule_fscc().control_fscc.as_ref().unwrap_or(&interconnection_schedule_fscc::CONTROL_FSCC)
    }
    fn control_fscc_mut(&mut self) -> &mut super::commonmodule::ControlFscc {
        self._interconnection_schedule_fscc_mut().control_fscc.get_or_insert(Default::default())
    }
    fn interconnection_control_schedule_fsch(&self) -> &::std::vec::Vec<InterconnectionControlScheduleFsch> {
        &self._interconnection_schedule_fscc().interconnection_control_schedule_fsch
    }
    fn interconnection_control_schedule_fsch_mut(&mut self) -> &mut ::std::vec::Vec<InterconnectionControlScheduleFsch> {
        &mut self._interconnection_schedule_fscc_mut().interconnection_control_schedule_fsch
    }
}
impl IsInterconnectionScheduleFscc for InterconnectionScheduleFscc {
    fn _interconnection_schedule_fscc(&self) -> &InterconnectionScheduleFscc {
        self
    }
    fn _interconnection_schedule_fscc_mut(&mut self) -> &mut InterconnectionScheduleFscc {
        self
    }
}
impl IsControlFscc for InterconnectionScheduleFscc {
    fn _control_fscc(&self) -> &super::commonmodule::ControlFscc {
        self.parent()
    }
    fn _control_fscc_mut(&mut self) -> &mut ControlFscc {
        self.parent_mut()
    }
}
impl IsLogicalNodeForControl for InterconnectionScheduleFscc {
    fn _logical_node_for_control(&self) -> &super::commonmodule::LogicalNodeForControl {
        self.parent().parent()
    }
    fn _logical_node_for_control_mut(&mut self) -> &mut LogicalNodeForControl {
        self.parent_mut().parent_mut()
    }
}
impl IsLogicalNode for InterconnectionScheduleFscc {
    fn _logical_node(&self) -> &super::commonmodule::LogicalNode {
        self.parent().parent().parent()
    }
    fn _logical_node_mut(&mut self) -> &mut LogicalNode {
        self.parent_mut().parent_mut().parent_mut()
    }
}
impl IsIdentifiedObject for InterconnectionScheduleFscc {
    fn _identified_object(&self) -> &super::commonmodule::IdentifiedObject {
        self.parent().parent().parent().parent()
    }
    fn _identified_object_mut(&mut self) -> &mut IdentifiedObject {
        self.parent_mut().parent_mut().parent_mut().parent_mut()
    }
}
/// Interconnection schedule
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct InterconnectionSchedule {
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
    pub interconnection_schedule_fscc: ::std::option::Option<InterconnectionScheduleFscc>,
}
mod interconnection_schedule {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref IDENTIFIED_OBJECT: crate::commonmodule::IdentifiedObject = Default::default();
        pub(super) static ref CHECK: crate::commonmodule::CheckConditions = Default::default();
        pub(super) static ref INTERCONNECTION_SCHEDULE_FSCC: crate::interconnectionmodule::InterconnectionScheduleFscc = Default::default();
    }
}
impl InterconnectionSchedule {
    pub(crate) fn parent(&self) -> &super::commonmodule::IdentifiedObject {
        self.identified_object.as_ref().unwrap_or(&interconnection_schedule::IDENTIFIED_OBJECT)
    }
    pub(crate) fn parent_mut(&mut self) -> &mut super::commonmodule::IdentifiedObject {
        self.identified_object.get_or_insert(Default::default())
    }
}
pub trait IsInterconnectionSchedule {
    fn _interconnection_schedule(&self) -> &InterconnectionSchedule;
    fn _interconnection_schedule_mut(&mut self) -> &mut InterconnectionSchedule;
    fn identified_object(&self) -> &super::commonmodule::IdentifiedObject {
        self._interconnection_schedule().identified_object.as_ref().unwrap_or(&interconnection_schedule::IDENTIFIED_OBJECT)
    }
    fn identified_object_mut(&mut self) -> &mut super::commonmodule::IdentifiedObject {
        self._interconnection_schedule_mut().identified_object.get_or_insert(Default::default())
    }
    fn check(&self) -> &super::commonmodule::CheckConditions {
        self._interconnection_schedule().check.as_ref().unwrap_or(&interconnection_schedule::CHECK)
    }
    fn check_mut(&mut self) -> &mut super::commonmodule::CheckConditions {
        self._interconnection_schedule_mut().check.get_or_insert(Default::default())
    }
    fn interconnection_schedule_fscc(&self) -> &InterconnectionScheduleFscc {
        self._interconnection_schedule().interconnection_schedule_fscc.as_ref().unwrap_or(&interconnection_schedule::INTERCONNECTION_SCHEDULE_FSCC)
    }
    fn interconnection_schedule_fscc_mut(&mut self) -> &mut InterconnectionScheduleFscc {
        self._interconnection_schedule_mut().interconnection_schedule_fscc.get_or_insert(Default::default())
    }
}
impl IsInterconnectionSchedule for InterconnectionSchedule {
    fn _interconnection_schedule(&self) -> &InterconnectionSchedule {
        self
    }
    fn _interconnection_schedule_mut(&mut self) -> &mut InterconnectionSchedule {
        self
    }
}
impl IsIdentifiedObject for InterconnectionSchedule {
    fn _identified_object(&self) -> &super::commonmodule::IdentifiedObject {
        self.parent()
    }
    fn _identified_object_mut(&mut self) -> &mut IdentifiedObject {
        self.parent_mut()
    }
}
/// Planned interconnection schedule profile
/// OpenFMB Profile Message: true
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PlannedInterconnectionScheduleProfile {
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
    pub requester_circuit_segment_service: ::std::option::Option<super::commonmodule::ApplicationSystem>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: Some(1)
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, optional, tag="3")]
    pub interconnection_schedule: ::std::option::Option<InterconnectionSchedule>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: Some(1)
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, optional, tag="4")]
    pub tie_point: ::std::option::Option<super::commonmodule::ConductingEquipment>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: Some(1)
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, optional, tag="5")]
    pub responder_circuit_segment_service: ::std::option::Option<super::commonmodule::ApplicationSystem>,
}
mod planned_interconnection_schedule_profile {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref CONTROL_MESSAGE_INFO: crate::commonmodule::ControlMessageInfo = Default::default();
        pub(super) static ref REQUESTER_CIRCUIT_SEGMENT_SERVICE: crate::commonmodule::ApplicationSystem = Default::default();
        pub(super) static ref INTERCONNECTION_SCHEDULE: crate::interconnectionmodule::InterconnectionSchedule = Default::default();
        pub(super) static ref TIE_POINT: crate::commonmodule::ConductingEquipment = Default::default();
        pub(super) static ref RESPONDER_CIRCUIT_SEGMENT_SERVICE: crate::commonmodule::ApplicationSystem = Default::default();
    }
}
impl PlannedInterconnectionScheduleProfile {
    pub(crate) fn parent(&self) -> &super::commonmodule::ControlMessageInfo {
        self.control_message_info.as_ref().unwrap_or(&planned_interconnection_schedule_profile::CONTROL_MESSAGE_INFO)
    }
    pub(crate) fn parent_mut(&mut self) -> &mut super::commonmodule::ControlMessageInfo {
        self.control_message_info.get_or_insert(Default::default())
    }
}
pub trait IsPlannedInterconnectionScheduleProfile {
    fn _planned_interconnection_schedule_profile(&self) -> &PlannedInterconnectionScheduleProfile;
    fn _planned_interconnection_schedule_profile_mut(&mut self) -> &mut PlannedInterconnectionScheduleProfile;
    fn control_message_info(&self) -> &super::commonmodule::ControlMessageInfo {
        self._planned_interconnection_schedule_profile().control_message_info.as_ref().unwrap_or(&planned_interconnection_schedule_profile::CONTROL_MESSAGE_INFO)
    }
    fn control_message_info_mut(&mut self) -> &mut super::commonmodule::ControlMessageInfo {
        self._planned_interconnection_schedule_profile_mut().control_message_info.get_or_insert(Default::default())
    }
    fn requester_circuit_segment_service(&self) -> &super::commonmodule::ApplicationSystem {
        self._planned_interconnection_schedule_profile().requester_circuit_segment_service.as_ref().unwrap_or(&planned_interconnection_schedule_profile::REQUESTER_CIRCUIT_SEGMENT_SERVICE)
    }
    fn requester_circuit_segment_service_mut(&mut self) -> &mut super::commonmodule::ApplicationSystem {
        self._planned_interconnection_schedule_profile_mut().requester_circuit_segment_service.get_or_insert(Default::default())
    }
    fn interconnection_schedule(&self) -> &InterconnectionSchedule {
        self._planned_interconnection_schedule_profile().interconnection_schedule.as_ref().unwrap_or(&planned_interconnection_schedule_profile::INTERCONNECTION_SCHEDULE)
    }
    fn interconnection_schedule_mut(&mut self) -> &mut InterconnectionSchedule {
        self._planned_interconnection_schedule_profile_mut().interconnection_schedule.get_or_insert(Default::default())
    }
    fn tie_point(&self) -> &super::commonmodule::ConductingEquipment {
        self._planned_interconnection_schedule_profile().tie_point.as_ref().unwrap_or(&planned_interconnection_schedule_profile::TIE_POINT)
    }
    fn tie_point_mut(&mut self) -> &mut super::commonmodule::ConductingEquipment {
        self._planned_interconnection_schedule_profile_mut().tie_point.get_or_insert(Default::default())
    }
    fn responder_circuit_segment_service(&self) -> &super::commonmodule::ApplicationSystem {
        self._planned_interconnection_schedule_profile().responder_circuit_segment_service.as_ref().unwrap_or(&planned_interconnection_schedule_profile::RESPONDER_CIRCUIT_SEGMENT_SERVICE)
    }
    fn responder_circuit_segment_service_mut(&mut self) -> &mut super::commonmodule::ApplicationSystem {
        self._planned_interconnection_schedule_profile_mut().responder_circuit_segment_service.get_or_insert(Default::default())
    }
}
impl IsPlannedInterconnectionScheduleProfile for PlannedInterconnectionScheduleProfile {
    fn _planned_interconnection_schedule_profile(&self) -> &PlannedInterconnectionScheduleProfile {
        self
    }
    fn _planned_interconnection_schedule_profile_mut(&mut self) -> &mut PlannedInterconnectionScheduleProfile {
        self
    }
}
impl IsControlMessageInfo for PlannedInterconnectionScheduleProfile {
    fn _control_message_info(&self) -> &super::commonmodule::ControlMessageInfo {
        self.parent()
    }
    fn _control_message_info_mut(&mut self) -> &mut ControlMessageInfo {
        self.parent_mut()
    }
}
impl IsMessageInfo for PlannedInterconnectionScheduleProfile {
    fn _message_info(&self) -> &super::commonmodule::MessageInfo {
        self.parent().parent()
    }
    fn _message_info_mut(&mut self) -> &mut MessageInfo {
        self.parent_mut().parent_mut()
    }
}
impl IsIdentifiedObject for PlannedInterconnectionScheduleProfile {
    fn _identified_object(&self) -> &super::commonmodule::IdentifiedObject {
        self.parent().parent().parent()
    }
    fn _identified_object_mut(&mut self) -> &mut IdentifiedObject {
        self.parent_mut().parent_mut().parent_mut()
    }
}
/// Requested interconnection schedule profile
/// OpenFMB Profile Message: true
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct RequestedInterconnectionScheduleProfile {
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
    pub requester_circuit_segment_service: ::std::option::Option<super::commonmodule::ApplicationSystem>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: Some(1)
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, optional, tag="3")]
    pub interconnection_schedule: ::std::option::Option<InterconnectionSchedule>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: Some(1)
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, optional, tag="4")]
    pub tie_point: ::std::option::Option<super::commonmodule::ConductingEquipment>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: Some(1)
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, optional, tag="5")]
    pub responder_circuit_segment_service: ::std::option::Option<super::commonmodule::ApplicationSystem>,
}
mod requested_interconnection_schedule_profile {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref CONTROL_MESSAGE_INFO: crate::commonmodule::ControlMessageInfo = Default::default();
        pub(super) static ref REQUESTER_CIRCUIT_SEGMENT_SERVICE: crate::commonmodule::ApplicationSystem = Default::default();
        pub(super) static ref INTERCONNECTION_SCHEDULE: crate::interconnectionmodule::InterconnectionSchedule = Default::default();
        pub(super) static ref TIE_POINT: crate::commonmodule::ConductingEquipment = Default::default();
        pub(super) static ref RESPONDER_CIRCUIT_SEGMENT_SERVICE: crate::commonmodule::ApplicationSystem = Default::default();
    }
}
impl RequestedInterconnectionScheduleProfile {
    pub(crate) fn parent(&self) -> &super::commonmodule::ControlMessageInfo {
        self.control_message_info.as_ref().unwrap_or(&requested_interconnection_schedule_profile::CONTROL_MESSAGE_INFO)
    }
    pub(crate) fn parent_mut(&mut self) -> &mut super::commonmodule::ControlMessageInfo {
        self.control_message_info.get_or_insert(Default::default())
    }
}
pub trait IsRequestedInterconnectionScheduleProfile {
    fn _requested_interconnection_schedule_profile(&self) -> &RequestedInterconnectionScheduleProfile;
    fn _requested_interconnection_schedule_profile_mut(&mut self) -> &mut RequestedInterconnectionScheduleProfile;
    fn control_message_info(&self) -> &super::commonmodule::ControlMessageInfo {
        self._requested_interconnection_schedule_profile().control_message_info.as_ref().unwrap_or(&requested_interconnection_schedule_profile::CONTROL_MESSAGE_INFO)
    }
    fn control_message_info_mut(&mut self) -> &mut super::commonmodule::ControlMessageInfo {
        self._requested_interconnection_schedule_profile_mut().control_message_info.get_or_insert(Default::default())
    }
    fn requester_circuit_segment_service(&self) -> &super::commonmodule::ApplicationSystem {
        self._requested_interconnection_schedule_profile().requester_circuit_segment_service.as_ref().unwrap_or(&requested_interconnection_schedule_profile::REQUESTER_CIRCUIT_SEGMENT_SERVICE)
    }
    fn requester_circuit_segment_service_mut(&mut self) -> &mut super::commonmodule::ApplicationSystem {
        self._requested_interconnection_schedule_profile_mut().requester_circuit_segment_service.get_or_insert(Default::default())
    }
    fn interconnection_schedule(&self) -> &InterconnectionSchedule {
        self._requested_interconnection_schedule_profile().interconnection_schedule.as_ref().unwrap_or(&requested_interconnection_schedule_profile::INTERCONNECTION_SCHEDULE)
    }
    fn interconnection_schedule_mut(&mut self) -> &mut InterconnectionSchedule {
        self._requested_interconnection_schedule_profile_mut().interconnection_schedule.get_or_insert(Default::default())
    }
    fn tie_point(&self) -> &super::commonmodule::ConductingEquipment {
        self._requested_interconnection_schedule_profile().tie_point.as_ref().unwrap_or(&requested_interconnection_schedule_profile::TIE_POINT)
    }
    fn tie_point_mut(&mut self) -> &mut super::commonmodule::ConductingEquipment {
        self._requested_interconnection_schedule_profile_mut().tie_point.get_or_insert(Default::default())
    }
    fn responder_circuit_segment_service(&self) -> &super::commonmodule::ApplicationSystem {
        self._requested_interconnection_schedule_profile().responder_circuit_segment_service.as_ref().unwrap_or(&requested_interconnection_schedule_profile::RESPONDER_CIRCUIT_SEGMENT_SERVICE)
    }
    fn responder_circuit_segment_service_mut(&mut self) -> &mut super::commonmodule::ApplicationSystem {
        self._requested_interconnection_schedule_profile_mut().responder_circuit_segment_service.get_or_insert(Default::default())
    }
}
impl IsRequestedInterconnectionScheduleProfile for RequestedInterconnectionScheduleProfile {
    fn _requested_interconnection_schedule_profile(&self) -> &RequestedInterconnectionScheduleProfile {
        self
    }
    fn _requested_interconnection_schedule_profile_mut(&mut self) -> &mut RequestedInterconnectionScheduleProfile {
        self
    }
}
impl IsControlMessageInfo for RequestedInterconnectionScheduleProfile {
    fn _control_message_info(&self) -> &super::commonmodule::ControlMessageInfo {
        self.parent()
    }
    fn _control_message_info_mut(&mut self) -> &mut ControlMessageInfo {
        self.parent_mut()
    }
}
impl IsMessageInfo for RequestedInterconnectionScheduleProfile {
    fn _message_info(&self) -> &super::commonmodule::MessageInfo {
        self.parent().parent()
    }
    fn _message_info_mut(&mut self) -> &mut MessageInfo {
        self.parent_mut().parent_mut()
    }
}
impl IsIdentifiedObject for RequestedInterconnectionScheduleProfile {
    fn _identified_object(&self) -> &super::commonmodule::IdentifiedObject {
        self.parent().parent().parent()
    }
    fn _identified_object_mut(&mut self) -> &mut IdentifiedObject {
        self.parent_mut().parent_mut().parent_mut()
    }
}
