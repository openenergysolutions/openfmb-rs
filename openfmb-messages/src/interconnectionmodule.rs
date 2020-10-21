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
    // multiplicity_min: 1
    // multiplicity_max: 0
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
pub trait IsInterconnectionPoint {
    fn _interconnection_point(&self) -> &InterconnectionPoint;
    fn _mut_interconnection_point(&mut self) -> &mut InterconnectionPoint;
    fn black_start_enabled(&self) -> &super::commonmodule::ControlDpc {
        self._interconnection_point().black_start_enabled.as_ref().unwrap_or(&interconnection_point::BLACK_START_ENABLED)
    }
    fn mut_black_start_enabled(&mut self) -> &mut super::commonmodule::ControlDpc {
        self._mut_interconnection_point().black_start_enabled.get_or_insert(interconnection_point::BLACK_START_ENABLED.clone())
    }
    fn frequency_set_point_enabled(&self) -> &super::commonmodule::ControlDpc {
        self._interconnection_point().frequency_set_point_enabled.as_ref().unwrap_or(&interconnection_point::FREQUENCY_SET_POINT_ENABLED)
    }
    fn mut_frequency_set_point_enabled(&mut self) -> &mut super::commonmodule::ControlDpc {
        self._mut_interconnection_point().frequency_set_point_enabled.get_or_insert(interconnection_point::FREQUENCY_SET_POINT_ENABLED.clone())
    }
    fn island(&self) -> &super::commonmodule::ControlDpc {
        self._interconnection_point().island.as_ref().unwrap_or(&interconnection_point::ISLAND)
    }
    fn mut_island(&mut self) -> &mut super::commonmodule::ControlDpc {
        self._mut_interconnection_point().island.get_or_insert(interconnection_point::ISLAND.clone())
    }
    fn pct_hz_droop(&self) -> &f32 {
        self._interconnection_point().pct_hz_droop.as_ref().unwrap_or(&interconnection_point::PCT_HZ_DROOP)
    }
    fn mut_pct_hz_droop(&mut self) -> &mut f32 {
        self._mut_interconnection_point().pct_hz_droop.get_or_insert(interconnection_point::PCT_HZ_DROOP.clone())
    }
    fn pct_v_droop(&self) -> &f32 {
        self._interconnection_point().pct_v_droop.as_ref().unwrap_or(&interconnection_point::PCT_V_DROOP)
    }
    fn mut_pct_v_droop(&mut self) -> &mut f32 {
        self._mut_interconnection_point().pct_v_droop.get_or_insert(interconnection_point::PCT_V_DROOP.clone())
    }
    fn ramp_rates(&self) -> &super::commonmodule::RampRate {
        self._interconnection_point().ramp_rates.as_ref().unwrap_or(&interconnection_point::RAMP_RATES)
    }
    fn mut_ramp_rates(&mut self) -> &mut super::commonmodule::RampRate {
        self._mut_interconnection_point().ramp_rates.get_or_insert(interconnection_point::RAMP_RATES.clone())
    }
    fn reactive_pwr_set_point_enabled(&self) -> &super::commonmodule::ControlDpc {
        self._interconnection_point().reactive_pwr_set_point_enabled.as_ref().unwrap_or(&interconnection_point::REACTIVE_PWR_SET_POINT_ENABLED)
    }
    fn mut_reactive_pwr_set_point_enabled(&mut self) -> &mut super::commonmodule::ControlDpc {
        self._mut_interconnection_point().reactive_pwr_set_point_enabled.get_or_insert(interconnection_point::REACTIVE_PWR_SET_POINT_ENABLED.clone())
    }
    fn real_pwr_set_point_enabled(&self) -> &super::commonmodule::ControlDpc {
        self._interconnection_point().real_pwr_set_point_enabled.as_ref().unwrap_or(&interconnection_point::REAL_PWR_SET_POINT_ENABLED)
    }
    fn mut_real_pwr_set_point_enabled(&mut self) -> &mut super::commonmodule::ControlDpc {
        self._mut_interconnection_point().real_pwr_set_point_enabled.get_or_insert(interconnection_point::REAL_PWR_SET_POINT_ENABLED.clone())
    }
    fn voltage_set_point_enabled(&self) -> &super::commonmodule::ControlDpc {
        self._interconnection_point().voltage_set_point_enabled.as_ref().unwrap_or(&interconnection_point::VOLTAGE_SET_POINT_ENABLED)
    }
    fn mut_voltage_set_point_enabled(&mut self) -> &mut super::commonmodule::ControlDpc {
        self._mut_interconnection_point().voltage_set_point_enabled.get_or_insert(interconnection_point::VOLTAGE_SET_POINT_ENABLED.clone())
    }
    fn start_time(&self) -> &super::commonmodule::Timestamp {
        self._interconnection_point().start_time.as_ref().unwrap_or(&interconnection_point::START_TIME)
    }
    fn mut_start_time(&mut self) -> &mut super::commonmodule::Timestamp {
        self._mut_interconnection_point().start_time.get_or_insert(interconnection_point::START_TIME.clone())
    }
}
impl IsInterconnectionPoint for InterconnectionPoint {
    fn _interconnection_point(&self) -> &InterconnectionPoint {
        self
    }
    fn _mut_interconnection_point(&mut self) -> &mut InterconnectionPoint {
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
    // multiplicity_min: 1
    // multiplicity_max: 0
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
pub trait IsInterconnectionCsg {
    fn _interconnection_csg(&self) -> &InterconnectionCsg;
    fn _mut_interconnection_csg(&mut self) -> &mut InterconnectionCsg;
    fn crv_pts(&self) -> &::std::vec::Vec<InterconnectionPoint> {
        &self._interconnection_csg().crv_pts    }
    fn mut_crv_pts(&mut self) -> &mut ::std::vec::Vec<InterconnectionPoint> {
        &mut self._mut_interconnection_csg().crv_pts    }
}
impl IsInterconnectionCsg for InterconnectionCsg {
    fn _interconnection_csg(&self) -> &InterconnectionCsg {
        self
    }
    fn _mut_interconnection_csg(&mut self) -> &mut InterconnectionCsg {
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
    // multiplicity_min: 1
    // multiplicity_max: 0
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
pub trait IsInterconnectionControlScheduleFsch {
    fn _interconnection_control_schedule_fsch(&self) -> &InterconnectionControlScheduleFsch;
    fn _mut_interconnection_control_schedule_fsch(&mut self) -> &mut InterconnectionControlScheduleFsch;
    fn val_dcsg(&self) -> &InterconnectionCsg {
        self._interconnection_control_schedule_fsch().val_dcsg.as_ref().unwrap_or(&interconnection_control_schedule_fsch::VAL_DCSG)
    }
    fn mut_val_dcsg(&mut self) -> &mut InterconnectionCsg {
        self._mut_interconnection_control_schedule_fsch().val_dcsg.get_or_insert(interconnection_control_schedule_fsch::VAL_DCSG.clone())
    }
}
impl IsInterconnectionControlScheduleFsch for InterconnectionControlScheduleFsch {
    fn _interconnection_control_schedule_fsch(&self) -> &InterconnectionControlScheduleFsch {
        self
    }
    fn _mut_interconnection_control_schedule_fsch(&mut self) -> &mut InterconnectionControlScheduleFsch {
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
    // multiplicity_min: 0
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="1")]
    pub control_fscc: ::std::option::Option<super::commonmodule::ControlFscc>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: false
    // multiplicity_min: 0
    // multiplicity_max: 0
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
pub trait IsInterconnectionScheduleFscc {
    fn _interconnection_schedule_fscc(&self) -> &InterconnectionScheduleFscc;
    fn _mut_interconnection_schedule_fscc(&mut self) -> &mut InterconnectionScheduleFscc;
    fn control_fscc(&self) -> &super::commonmodule::ControlFscc {
        self._interconnection_schedule_fscc().control_fscc.as_ref().unwrap_or(&interconnection_schedule_fscc::CONTROL_FSCC)
    }
    fn mut_control_fscc(&mut self) -> &mut super::commonmodule::ControlFscc {
        self._mut_interconnection_schedule_fscc().control_fscc.get_or_insert(interconnection_schedule_fscc::CONTROL_FSCC.clone())
    }
    fn interconnection_control_schedule_fsch(&self) -> &::std::vec::Vec<InterconnectionControlScheduleFsch> {
        &self._interconnection_schedule_fscc().interconnection_control_schedule_fsch    }
    fn mut_interconnection_control_schedule_fsch(&mut self) -> &mut ::std::vec::Vec<InterconnectionControlScheduleFsch> {
        &mut self._mut_interconnection_schedule_fscc().interconnection_control_schedule_fsch    }
}
impl IsInterconnectionScheduleFscc for InterconnectionScheduleFscc {
    fn _interconnection_schedule_fscc(&self) -> &InterconnectionScheduleFscc {
        self
    }
    fn _mut_interconnection_schedule_fscc(&mut self) -> &mut InterconnectionScheduleFscc {
        self
    }
}
/// Interconnection schedule
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct InterconnectionSchedule {
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
    pub interconnection_schedule_fscc: ::std::option::Option<InterconnectionScheduleFscc>,
}
mod interconnection_schedule {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref CONTROL_VALUE: crate::commonmodule::ControlValue = Default::default();
        pub(super) static ref CHECK: crate::commonmodule::CheckConditions = Default::default();
        pub(super) static ref INTERCONNECTION_SCHEDULE_FSCC: crate::interconnectionmodule::InterconnectionScheduleFscc = Default::default();
    }
}
pub trait IsInterconnectionSchedule {
    fn _interconnection_schedule(&self) -> &InterconnectionSchedule;
    fn _mut_interconnection_schedule(&mut self) -> &mut InterconnectionSchedule;
    fn control_value(&self) -> &super::commonmodule::ControlValue {
        self._interconnection_schedule().control_value.as_ref().unwrap_or(&interconnection_schedule::CONTROL_VALUE)
    }
    fn mut_control_value(&mut self) -> &mut super::commonmodule::ControlValue {
        self._mut_interconnection_schedule().control_value.get_or_insert(interconnection_schedule::CONTROL_VALUE.clone())
    }
    fn check(&self) -> &super::commonmodule::CheckConditions {
        self._interconnection_schedule().check.as_ref().unwrap_or(&interconnection_schedule::CHECK)
    }
    fn mut_check(&mut self) -> &mut super::commonmodule::CheckConditions {
        self._mut_interconnection_schedule().check.get_or_insert(interconnection_schedule::CHECK.clone())
    }
    fn interconnection_schedule_fscc(&self) -> &InterconnectionScheduleFscc {
        self._interconnection_schedule().interconnection_schedule_fscc.as_ref().unwrap_or(&interconnection_schedule::INTERCONNECTION_SCHEDULE_FSCC)
    }
    fn mut_interconnection_schedule_fscc(&mut self) -> &mut InterconnectionScheduleFscc {
        self._mut_interconnection_schedule().interconnection_schedule_fscc.get_or_insert(interconnection_schedule::INTERCONNECTION_SCHEDULE_FSCC.clone())
    }
}
impl IsInterconnectionSchedule for InterconnectionSchedule {
    fn _interconnection_schedule(&self) -> &InterconnectionSchedule {
        self
    }
    fn _mut_interconnection_schedule(&mut self) -> &mut InterconnectionSchedule {
        self
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
    pub application_system: ::std::option::Option<super::commonmodule::ApplicationSystem>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="3")]
    pub interconnection_schedule: ::std::option::Option<InterconnectionSchedule>,
}
mod planned_interconnection_schedule_profile {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref CONTROL_MESSAGE_INFO: crate::commonmodule::ControlMessageInfo = Default::default();
        pub(super) static ref APPLICATION_SYSTEM: crate::commonmodule::ApplicationSystem = Default::default();
        pub(super) static ref INTERCONNECTION_SCHEDULE: crate::interconnectionmodule::InterconnectionSchedule = Default::default();
    }
}
pub trait IsPlannedInterconnectionScheduleProfile {
    fn _planned_interconnection_schedule_profile(&self) -> &PlannedInterconnectionScheduleProfile;
    fn _mut_planned_interconnection_schedule_profile(&mut self) -> &mut PlannedInterconnectionScheduleProfile;
    fn control_message_info(&self) -> &super::commonmodule::ControlMessageInfo {
        self._planned_interconnection_schedule_profile().control_message_info.as_ref().unwrap_or(&planned_interconnection_schedule_profile::CONTROL_MESSAGE_INFO)
    }
    fn mut_control_message_info(&mut self) -> &mut super::commonmodule::ControlMessageInfo {
        self._mut_planned_interconnection_schedule_profile().control_message_info.get_or_insert(planned_interconnection_schedule_profile::CONTROL_MESSAGE_INFO.clone())
    }
    fn application_system(&self) -> &super::commonmodule::ApplicationSystem {
        self._planned_interconnection_schedule_profile().application_system.as_ref().unwrap_or(&planned_interconnection_schedule_profile::APPLICATION_SYSTEM)
    }
    fn mut_application_system(&mut self) -> &mut super::commonmodule::ApplicationSystem {
        self._mut_planned_interconnection_schedule_profile().application_system.get_or_insert(planned_interconnection_schedule_profile::APPLICATION_SYSTEM.clone())
    }
    fn interconnection_schedule(&self) -> &InterconnectionSchedule {
        self._planned_interconnection_schedule_profile().interconnection_schedule.as_ref().unwrap_or(&planned_interconnection_schedule_profile::INTERCONNECTION_SCHEDULE)
    }
    fn mut_interconnection_schedule(&mut self) -> &mut InterconnectionSchedule {
        self._mut_planned_interconnection_schedule_profile().interconnection_schedule.get_or_insert(planned_interconnection_schedule_profile::INTERCONNECTION_SCHEDULE.clone())
    }
}
impl IsPlannedInterconnectionScheduleProfile for PlannedInterconnectionScheduleProfile {
    fn _planned_interconnection_schedule_profile(&self) -> &PlannedInterconnectionScheduleProfile {
        self
    }
    fn _mut_planned_interconnection_schedule_profile(&mut self) -> &mut PlannedInterconnectionScheduleProfile {
        self
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
    pub application_system: ::std::option::Option<super::commonmodule::ApplicationSystem>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="3")]
    pub interconnection_schedule: ::std::option::Option<InterconnectionSchedule>,
}
mod requested_interconnection_schedule_profile {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref CONTROL_MESSAGE_INFO: crate::commonmodule::ControlMessageInfo = Default::default();
        pub(super) static ref APPLICATION_SYSTEM: crate::commonmodule::ApplicationSystem = Default::default();
        pub(super) static ref INTERCONNECTION_SCHEDULE: crate::interconnectionmodule::InterconnectionSchedule = Default::default();
    }
}
pub trait IsRequestedInterconnectionScheduleProfile {
    fn _requested_interconnection_schedule_profile(&self) -> &RequestedInterconnectionScheduleProfile;
    fn _mut_requested_interconnection_schedule_profile(&mut self) -> &mut RequestedInterconnectionScheduleProfile;
    fn control_message_info(&self) -> &super::commonmodule::ControlMessageInfo {
        self._requested_interconnection_schedule_profile().control_message_info.as_ref().unwrap_or(&requested_interconnection_schedule_profile::CONTROL_MESSAGE_INFO)
    }
    fn mut_control_message_info(&mut self) -> &mut super::commonmodule::ControlMessageInfo {
        self._mut_requested_interconnection_schedule_profile().control_message_info.get_or_insert(requested_interconnection_schedule_profile::CONTROL_MESSAGE_INFO.clone())
    }
    fn application_system(&self) -> &super::commonmodule::ApplicationSystem {
        self._requested_interconnection_schedule_profile().application_system.as_ref().unwrap_or(&requested_interconnection_schedule_profile::APPLICATION_SYSTEM)
    }
    fn mut_application_system(&mut self) -> &mut super::commonmodule::ApplicationSystem {
        self._mut_requested_interconnection_schedule_profile().application_system.get_or_insert(requested_interconnection_schedule_profile::APPLICATION_SYSTEM.clone())
    }
    fn interconnection_schedule(&self) -> &InterconnectionSchedule {
        self._requested_interconnection_schedule_profile().interconnection_schedule.as_ref().unwrap_or(&requested_interconnection_schedule_profile::INTERCONNECTION_SCHEDULE)
    }
    fn mut_interconnection_schedule(&mut self) -> &mut InterconnectionSchedule {
        self._mut_requested_interconnection_schedule_profile().interconnection_schedule.get_or_insert(requested_interconnection_schedule_profile::INTERCONNECTION_SCHEDULE.clone())
    }
}
impl IsRequestedInterconnectionScheduleProfile for RequestedInterconnectionScheduleProfile {
    fn _requested_interconnection_schedule_profile(&self) -> &RequestedInterconnectionScheduleProfile {
        self
    }
    fn _mut_requested_interconnection_schedule_profile(&mut self) -> &mut RequestedInterconnectionScheduleProfile {
        self
    }
}
