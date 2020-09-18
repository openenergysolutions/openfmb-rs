/// LN: Automatic tap changer controller   Name: ATCC
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct RegulatorControlAtcc {
    /// Centre of voltage control bandwidth (forward power flow presumed).
    #[prost(message, optional, tag="1")]
    pub bnd_ctr: ::std::option::Option<super::commonmodule::Asg>,
    /// Control (secondary) voltage bandwidth (i.e., range), given either as voltage value or percentage
    /// of the nominal voltage (forward power flow presumed).
    #[prost(message, optional, tag="2")]
    pub bnd_wid: ::std::option::Option<super::commonmodule::Asg>,
    /// Time to wait before operating, after reaching the control point (forward power flow presumed).
    #[prost(message, optional, tag="3")]
    pub ctl_dl_tmms: ::std::option::Option<super::commonmodule::ControlIng>,
    /// Line drop voltage due to line resistance component (forward power flow presumed) at rated current.
    #[prost(message, optional, tag="4")]
    pub ldcr: ::std::option::Option<super::commonmodule::Asg>,
    /// Line drop voltage due to line reactance component (forward power flow presumed) at rated current.
    #[prost(message, optional, tag="5")]
    pub ldcx: ::std::option::Option<super::commonmodule::Asg>,
    /// Line drop voltage due to line total impedance (forward power flow presumed) at rated current.
    #[prost(message, optional, tag="6")]
    pub ldcz: ::std::option::Option<super::commonmodule::Asg>,
    /// (controllable) If true, transformers operate in parallel, otherwise they operate independently.
    #[prost(message, optional, tag="7")]
    pub par_op: ::std::option::Option<super::commonmodule::ControlSpc>,
    /// (controllable) Tap position change to the specified value.
    #[prost(message, optional, tag="8")]
    pub tap_pos: ::std::option::Option<super::commonmodule::ControlIsc>,
}
mod regulator_control_atcc {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref BND_CTR: crate::commonmodule::Asg = Default::default();
        pub(super) static ref BND_WID: crate::commonmodule::Asg = Default::default();
        pub(super) static ref CTL_DL_TMMS: crate::commonmodule::ControlIng = Default::default();
        pub(super) static ref LDCR: crate::commonmodule::Asg = Default::default();
        pub(super) static ref LDCX: crate::commonmodule::Asg = Default::default();
        pub(super) static ref LDCZ: crate::commonmodule::Asg = Default::default();
        pub(super) static ref PAR_OP: crate::commonmodule::ControlSpc = Default::default();
        pub(super) static ref TAP_POS: crate::commonmodule::ControlIsc = Default::default();
    }
}
trait IsRegulatorControlAtcc {
    fn _regulator_control_atcc(&self) -> &RegulatorControlAtcc;
    fn bnd_ctr(&self) -> &super::commonmodule::Asg {
        self._regulator_control_atcc().bnd_ctr.as_ref().unwrap_or(&regulator_control_atcc::BND_CTR)
    }
    fn bnd_wid(&self) -> &super::commonmodule::Asg {
        self._regulator_control_atcc().bnd_wid.as_ref().unwrap_or(&regulator_control_atcc::BND_WID)
    }
    fn ctl_dl_tmms(&self) -> &super::commonmodule::ControlIng {
        self._regulator_control_atcc().ctl_dl_tmms.as_ref().unwrap_or(&regulator_control_atcc::CTL_DL_TMMS)
    }
    fn ldcr(&self) -> &super::commonmodule::Asg {
        self._regulator_control_atcc().ldcr.as_ref().unwrap_or(&regulator_control_atcc::LDCR)
    }
    fn ldcx(&self) -> &super::commonmodule::Asg {
        self._regulator_control_atcc().ldcx.as_ref().unwrap_or(&regulator_control_atcc::LDCX)
    }
    fn ldcz(&self) -> &super::commonmodule::Asg {
        self._regulator_control_atcc().ldcz.as_ref().unwrap_or(&regulator_control_atcc::LDCZ)
    }
    fn par_op(&self) -> &super::commonmodule::ControlSpc {
        self._regulator_control_atcc().par_op.as_ref().unwrap_or(&regulator_control_atcc::PAR_OP)
    }
    fn tap_pos(&self) -> &super::commonmodule::ControlIsc {
        self._regulator_control_atcc().tap_pos.as_ref().unwrap_or(&regulator_control_atcc::TAP_POS)
    }
}
impl IsRegulatorControlAtcc for RegulatorControlAtcc {
    fn _regulator_control_atcc(&self) -> &RegulatorControlAtcc {
        self
    }
}
/// Point definition (Point)
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct RegulatorPoint {
    /// Regulator control
    #[prost(message, optional, tag="1")]
    pub control: ::std::option::Option<RegulatorControlAtcc>,
    /// Black start enable
    #[prost(message, optional, tag="2")]
    pub pct_v_droop: ::std::option::Option<f32>,
    /// Ramp rates
    #[prost(message, optional, tag="3")]
    pub ramp_rates: ::std::option::Option<super::commonmodule::RampRate>,
    /// Enable reactive power set point
    #[prost(message, optional, tag="4")]
    pub reactive_pwr_set_point_enabled: ::std::option::Option<super::commonmodule::ControlDpc>,
    /// Reset device
    #[prost(message, optional, tag="5")]
    pub reset: ::std::option::Option<super::commonmodule::ControlDpc>,
    /// State
    #[prost(message, optional, tag="6")]
    pub state: ::std::option::Option<super::commonmodule::OptionalStateKind>,
    /// Enable voltage set point
    #[prost(message, optional, tag="7")]
    pub voltage_set_point_enabled: ::std::option::Option<super::commonmodule::ControlDpc>,
    /// Start time
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="8")]
    pub start_time: ::std::option::Option<super::commonmodule::Timestamp>,
}
mod regulator_point {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref CONTROL: crate::regulatormodule::RegulatorControlAtcc = Default::default();
        pub(super) static ref PCT_V_DROOP: f32 = Default::default();
        pub(super) static ref RAMP_RATES: crate::commonmodule::RampRate = Default::default();
        pub(super) static ref REACTIVE_PWR_SET_POINT_ENABLED: crate::commonmodule::ControlDpc = Default::default();
        pub(super) static ref RESET: crate::commonmodule::ControlDpc = Default::default();
        pub(super) static ref STATE: crate::commonmodule::OptionalStateKind = Default::default();
        pub(super) static ref VOLTAGE_SET_POINT_ENABLED: crate::commonmodule::ControlDpc = Default::default();
        pub(super) static ref START_TIME: crate::commonmodule::Timestamp = Default::default();
    }
}
trait IsRegulatorPoint {
    fn _regulator_point(&self) -> &RegulatorPoint;
    fn control(&self) -> &RegulatorControlAtcc {
        self._regulator_point().control.as_ref().unwrap_or(&regulator_point::CONTROL)
    }
    fn pct_v_droop(&self) -> &f32 {
        self._regulator_point().pct_v_droop.as_ref().unwrap_or(&regulator_point::PCT_V_DROOP)
    }
    fn ramp_rates(&self) -> &super::commonmodule::RampRate {
        self._regulator_point().ramp_rates.as_ref().unwrap_or(&regulator_point::RAMP_RATES)
    }
    fn reactive_pwr_set_point_enabled(&self) -> &super::commonmodule::ControlDpc {
        self._regulator_point().reactive_pwr_set_point_enabled.as_ref().unwrap_or(&regulator_point::REACTIVE_PWR_SET_POINT_ENABLED)
    }
    fn reset(&self) -> &super::commonmodule::ControlDpc {
        self._regulator_point().reset.as_ref().unwrap_or(&regulator_point::RESET)
    }
    fn state(&self) -> &super::commonmodule::OptionalStateKind {
        self._regulator_point().state.as_ref().unwrap_or(&regulator_point::STATE)
    }
    fn voltage_set_point_enabled(&self) -> &super::commonmodule::ControlDpc {
        self._regulator_point().voltage_set_point_enabled.as_ref().unwrap_or(&regulator_point::VOLTAGE_SET_POINT_ENABLED)
    }
    fn start_time(&self) -> &super::commonmodule::Timestamp {
        self._regulator_point().start_time.as_ref().unwrap_or(&regulator_point::START_TIME)
    }
}
impl IsRegulatorPoint for RegulatorPoint {
    fn _regulator_point(&self) -> &RegulatorPoint {
        self
    }
}
/// Curve shape setting (FC=SP) (CSG_SP)
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct RegulatorCsg {
    /// The array with the points specifying a curve shape.
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, repeated, tag="1")]
    pub crv_pts: ::std::vec::Vec<RegulatorPoint>,
}
mod regulator_csg {
    use lazy_static::lazy_static;
    lazy_static! {
    }
}
trait IsRegulatorCsg {
    fn _regulator_csg(&self) -> &RegulatorCsg;
    fn crv_pts(&self) -> &::std::vec::Vec<RegulatorPoint> {
        &self._regulator_csg().crv_pts    }
}
impl IsRegulatorCsg for RegulatorCsg {
    fn _regulator_csg(&self) -> &RegulatorCsg {
        self
    }
}
/// OpenFMB specialization for control schedule using:  LN: Schedule   Name: FSCH
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct RegulatorControlScheduleFsch {
    /// Discrete value in RegulatorCSG type
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="1")]
    pub val_dcsg: ::std::option::Option<RegulatorCsg>,
}
mod regulator_control_schedule_fsch {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref VAL_DCSG: crate::regulatormodule::RegulatorCsg = Default::default();
    }
}
trait IsRegulatorControlScheduleFsch {
    fn _regulator_control_schedule_fsch(&self) -> &RegulatorControlScheduleFsch;
    fn val_dcsg(&self) -> &RegulatorCsg {
        self._regulator_control_schedule_fsch().val_dcsg.as_ref().unwrap_or(&regulator_control_schedule_fsch::VAL_DCSG)
    }
}
impl IsRegulatorControlScheduleFsch for RegulatorControlScheduleFsch {
    fn _regulator_control_schedule_fsch(&self) -> &RegulatorControlScheduleFsch {
        self
    }
}
/// Using 61850 FSCC for regulator control
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct RegulatorControlFscc {
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
    #[prost(message, optional, tag="2")]
    pub regulator_control_schedule_fsch: ::std::option::Option<RegulatorControlScheduleFsch>,
}
mod regulator_control_fscc {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref CONTROL_FSCC: crate::commonmodule::ControlFscc = Default::default();
        pub(super) static ref REGULATOR_CONTROL_SCHEDULE_FSCH: crate::regulatormodule::RegulatorControlScheduleFsch = Default::default();
    }
}
trait IsRegulatorControlFscc {
    fn _regulator_control_fscc(&self) -> &RegulatorControlFscc;
    fn control_fscc(&self) -> &super::commonmodule::ControlFscc {
        self._regulator_control_fscc().control_fscc.as_ref().unwrap_or(&regulator_control_fscc::CONTROL_FSCC)
    }
    fn regulator_control_schedule_fsch(&self) -> &RegulatorControlScheduleFsch {
        self._regulator_control_fscc().regulator_control_schedule_fsch.as_ref().unwrap_or(&regulator_control_fscc::REGULATOR_CONTROL_SCHEDULE_FSCH)
    }
}
impl IsRegulatorControlFscc for RegulatorControlFscc {
    fn _regulator_control_fscc(&self) -> &RegulatorControlFscc {
        self
    }
}
/// Regulator control
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct RegulatorControl {
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
    pub regulator_control_fscc: ::std::option::Option<RegulatorControlFscc>,
}
mod regulator_control {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref CONTROL_VALUE: crate::commonmodule::ControlValue = Default::default();
        pub(super) static ref CHECK: crate::commonmodule::CheckConditions = Default::default();
        pub(super) static ref REGULATOR_CONTROL_FSCC: crate::regulatormodule::RegulatorControlFscc = Default::default();
    }
}
trait IsRegulatorControl {
    fn _regulator_control(&self) -> &RegulatorControl;
    fn control_value(&self) -> &super::commonmodule::ControlValue {
        self._regulator_control().control_value.as_ref().unwrap_or(&regulator_control::CONTROL_VALUE)
    }
    fn check(&self) -> &super::commonmodule::CheckConditions {
        self._regulator_control().check.as_ref().unwrap_or(&regulator_control::CHECK)
    }
    fn regulator_control_fscc(&self) -> &RegulatorControlFscc {
        self._regulator_control().regulator_control_fscc.as_ref().unwrap_or(&regulator_control::REGULATOR_CONTROL_FSCC)
    }
}
impl IsRegulatorControl for RegulatorControl {
    fn _regulator_control(&self) -> &RegulatorControl {
        self
    }
}
/// Pole-mounted fault interrupter with built-in phase and ground relays, current transformer (CT),
/// and supplemental controls.
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct RegulatorSystem {
    /// UML inherited base object
    // parent_message: true
    // required_field: false
    // multiplicity_min: 0
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="1")]
    pub conducting_equipment: ::std::option::Option<super::commonmodule::ConductingEquipment>,
}
mod regulator_system {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref CONDUCTING_EQUIPMENT: crate::commonmodule::ConductingEquipment = Default::default();
    }
}
trait IsRegulatorSystem {
    fn _regulator_system(&self) -> &RegulatorSystem;
    fn conducting_equipment(&self) -> &super::commonmodule::ConductingEquipment {
        self._regulator_system().conducting_equipment.as_ref().unwrap_or(&regulator_system::CONDUCTING_EQUIPMENT)
    }
}
impl IsRegulatorSystem for RegulatorSystem {
    fn _regulator_system(&self) -> &RegulatorSystem {
        self
    }
}
/// Regulator control profile.  Instructs an end device (or an end device group) to perform a
/// specified action.
/// OpenFMB Profile Message: true
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct RegulatorControlProfile {
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
    pub regulator_control: ::std::option::Option<RegulatorControl>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="4")]
    pub regulator_system: ::std::option::Option<RegulatorSystem>,
}
mod regulator_control_profile {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref CONTROL_MESSAGE_INFO: crate::commonmodule::ControlMessageInfo = Default::default();
        pub(super) static ref IED: crate::commonmodule::Ied = Default::default();
        pub(super) static ref REGULATOR_CONTROL: crate::regulatormodule::RegulatorControl = Default::default();
        pub(super) static ref REGULATOR_SYSTEM: crate::regulatormodule::RegulatorSystem = Default::default();
    }
}
trait IsRegulatorControlProfile {
    fn _regulator_control_profile(&self) -> &RegulatorControlProfile;
    fn control_message_info(&self) -> &super::commonmodule::ControlMessageInfo {
        self._regulator_control_profile().control_message_info.as_ref().unwrap_or(&regulator_control_profile::CONTROL_MESSAGE_INFO)
    }
    fn ied(&self) -> &super::commonmodule::Ied {
        self._regulator_control_profile().ied.as_ref().unwrap_or(&regulator_control_profile::IED)
    }
    fn regulator_control(&self) -> &RegulatorControl {
        self._regulator_control_profile().regulator_control.as_ref().unwrap_or(&regulator_control_profile::REGULATOR_CONTROL)
    }
    fn regulator_system(&self) -> &RegulatorSystem {
        self._regulator_control_profile().regulator_system.as_ref().unwrap_or(&regulator_control_profile::REGULATOR_SYSTEM)
    }
}
impl IsRegulatorControlProfile for RegulatorControlProfile {
    fn _regulator_control_profile(&self) -> &RegulatorControlProfile {
        self
    }
}
/// LN: Automatic tap changer controller   Name: ATCC
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct RegulatorEventAndStatusAtcc {
    /// Centre of voltage control bandwidth (forward power flow presumed).
    #[prost(message, optional, tag="1")]
    pub bnd_ctr: ::std::option::Option<super::commonmodule::Asg>,
    /// Control (secondary) voltage bandwidth (i.e., range), given either as voltage value or percentage
    /// of the nominal voltage (forward power flow presumed).
    #[prost(message, optional, tag="2")]
    pub bnd_wid: ::std::option::Option<super::commonmodule::Asg>,
    /// Line drop voltage due to line resistance component (forward power flow presumed) at rated current.
    #[prost(message, optional, tag="3")]
    pub ldcr: ::std::option::Option<super::commonmodule::Asg>,
    /// Line drop voltage due to line reactance component (forward power flow presumed) at rated current.
    #[prost(message, optional, tag="4")]
    pub ldcx: ::std::option::Option<super::commonmodule::Asg>,
    /// Line drop voltage due to line total impedance (forward power flow presumed) at rated current.
    #[prost(message, optional, tag="5")]
    pub ldcz: ::std::option::Option<super::commonmodule::Asg>,
    /// (controllable) If true, transformers operate in parallel, otherwise they operate independently.
    #[prost(message, optional, tag="6")]
    pub par_op: ::std::option::Option<super::commonmodule::StatusSpc>,
    /// If true, there was an error in tap position change, or in tap indication (for instance, wrong
    /// Binary Coded Decimal (BCD) code).
    #[prost(message, optional, tag="7")]
    pub tap_op_err: ::std::option::Option<super::commonmodule::StatusSps>,
    /// (controllable) Tap position change to the specified value.
    #[prost(message, optional, tag="8")]
    pub tap_pos: ::std::option::Option<super::commonmodule::StatusIsc>,
}
mod regulator_event_and_status_atcc {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref BND_CTR: crate::commonmodule::Asg = Default::default();
        pub(super) static ref BND_WID: crate::commonmodule::Asg = Default::default();
        pub(super) static ref LDCR: crate::commonmodule::Asg = Default::default();
        pub(super) static ref LDCX: crate::commonmodule::Asg = Default::default();
        pub(super) static ref LDCZ: crate::commonmodule::Asg = Default::default();
        pub(super) static ref PAR_OP: crate::commonmodule::StatusSpc = Default::default();
        pub(super) static ref TAP_OP_ERR: crate::commonmodule::StatusSps = Default::default();
        pub(super) static ref TAP_POS: crate::commonmodule::StatusIsc = Default::default();
    }
}
trait IsRegulatorEventAndStatusAtcc {
    fn _regulator_event_and_status_atcc(&self) -> &RegulatorEventAndStatusAtcc;
    fn bnd_ctr(&self) -> &super::commonmodule::Asg {
        self._regulator_event_and_status_atcc().bnd_ctr.as_ref().unwrap_or(&regulator_event_and_status_atcc::BND_CTR)
    }
    fn bnd_wid(&self) -> &super::commonmodule::Asg {
        self._regulator_event_and_status_atcc().bnd_wid.as_ref().unwrap_or(&regulator_event_and_status_atcc::BND_WID)
    }
    fn ldcr(&self) -> &super::commonmodule::Asg {
        self._regulator_event_and_status_atcc().ldcr.as_ref().unwrap_or(&regulator_event_and_status_atcc::LDCR)
    }
    fn ldcx(&self) -> &super::commonmodule::Asg {
        self._regulator_event_and_status_atcc().ldcx.as_ref().unwrap_or(&regulator_event_and_status_atcc::LDCX)
    }
    fn ldcz(&self) -> &super::commonmodule::Asg {
        self._regulator_event_and_status_atcc().ldcz.as_ref().unwrap_or(&regulator_event_and_status_atcc::LDCZ)
    }
    fn par_op(&self) -> &super::commonmodule::StatusSpc {
        self._regulator_event_and_status_atcc().par_op.as_ref().unwrap_or(&regulator_event_and_status_atcc::PAR_OP)
    }
    fn tap_op_err(&self) -> &super::commonmodule::StatusSps {
        self._regulator_event_and_status_atcc().tap_op_err.as_ref().unwrap_or(&regulator_event_and_status_atcc::TAP_OP_ERR)
    }
    fn tap_pos(&self) -> &super::commonmodule::StatusIsc {
        self._regulator_event_and_status_atcc().tap_pos.as_ref().unwrap_or(&regulator_event_and_status_atcc::TAP_POS)
    }
}
impl IsRegulatorEventAndStatusAtcc for RegulatorEventAndStatusAtcc {
    fn _regulator_event_and_status_atcc(&self) -> &RegulatorEventAndStatusAtcc {
        self
    }
}
/// Point definition (Point)
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct RegulatorEventAndStatusPoint {
    /// Regulator event and status
    #[prost(message, optional, tag="1")]
    pub event_and_status: ::std::option::Option<RegulatorEventAndStatusAtcc>,
    /// Black start enable
    #[prost(message, optional, tag="2")]
    pub pct_v_droop: ::std::option::Option<f32>,
    /// Ramp rates
    #[prost(message, optional, tag="3")]
    pub ramp_rates: ::std::option::Option<super::commonmodule::RampRate>,
    /// Enable reactive power set point
    #[prost(message, optional, tag="4")]
    pub reactive_pwr_set_point_enabled: ::std::option::Option<super::commonmodule::StatusSpc>,
    /// State
    #[prost(message, optional, tag="5")]
    pub state: ::std::option::Option<super::commonmodule::OptionalStateKind>,
    /// Enable voltage set point
    #[prost(message, optional, tag="6")]
    pub voltage_set_point_enabled: ::std::option::Option<super::commonmodule::StatusSpc>,
}
mod regulator_event_and_status_point {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref EVENT_AND_STATUS: crate::regulatormodule::RegulatorEventAndStatusAtcc = Default::default();
        pub(super) static ref PCT_V_DROOP: f32 = Default::default();
        pub(super) static ref RAMP_RATES: crate::commonmodule::RampRate = Default::default();
        pub(super) static ref REACTIVE_PWR_SET_POINT_ENABLED: crate::commonmodule::StatusSpc = Default::default();
        pub(super) static ref STATE: crate::commonmodule::OptionalStateKind = Default::default();
        pub(super) static ref VOLTAGE_SET_POINT_ENABLED: crate::commonmodule::StatusSpc = Default::default();
    }
}
trait IsRegulatorEventAndStatusPoint {
    fn _regulator_event_and_status_point(&self) -> &RegulatorEventAndStatusPoint;
    fn event_and_status(&self) -> &RegulatorEventAndStatusAtcc {
        self._regulator_event_and_status_point().event_and_status.as_ref().unwrap_or(&regulator_event_and_status_point::EVENT_AND_STATUS)
    }
    fn pct_v_droop(&self) -> &f32 {
        self._regulator_event_and_status_point().pct_v_droop.as_ref().unwrap_or(&regulator_event_and_status_point::PCT_V_DROOP)
    }
    fn ramp_rates(&self) -> &super::commonmodule::RampRate {
        self._regulator_event_and_status_point().ramp_rates.as_ref().unwrap_or(&regulator_event_and_status_point::RAMP_RATES)
    }
    fn reactive_pwr_set_point_enabled(&self) -> &super::commonmodule::StatusSpc {
        self._regulator_event_and_status_point().reactive_pwr_set_point_enabled.as_ref().unwrap_or(&regulator_event_and_status_point::REACTIVE_PWR_SET_POINT_ENABLED)
    }
    fn state(&self) -> &super::commonmodule::OptionalStateKind {
        self._regulator_event_and_status_point().state.as_ref().unwrap_or(&regulator_event_and_status_point::STATE)
    }
    fn voltage_set_point_enabled(&self) -> &super::commonmodule::StatusSpc {
        self._regulator_event_and_status_point().voltage_set_point_enabled.as_ref().unwrap_or(&regulator_event_and_status_point::VOLTAGE_SET_POINT_ENABLED)
    }
}
impl IsRegulatorEventAndStatusPoint for RegulatorEventAndStatusPoint {
    fn _regulator_event_and_status_point(&self) -> &RegulatorEventAndStatusPoint {
        self
    }
}
/// OpenFMB 61850 specialization for both RegulatorEventProfile and RegulatorStatusProfile
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct RegulatorEventAndStatusAncr {
    /// UML inherited base object
    // parent_message: true
    // required_field: false
    // multiplicity_min: 0
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="1")]
    pub logical_node_for_event_and_status: ::std::option::Option<super::commonmodule::LogicalNodeForEventAndStatus>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub dynamic_test: ::std::option::Option<super::commonmodule::EnsDynamicTestKind>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="3")]
    pub point_status: ::std::option::Option<RegulatorEventAndStatusPoint>,
}
mod regulator_event_and_status_ancr {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref LOGICAL_NODE_FOR_EVENT_AND_STATUS: crate::commonmodule::LogicalNodeForEventAndStatus = Default::default();
        pub(super) static ref DYNAMIC_TEST: crate::commonmodule::EnsDynamicTestKind = Default::default();
        pub(super) static ref POINT_STATUS: crate::regulatormodule::RegulatorEventAndStatusPoint = Default::default();
    }
}
trait IsRegulatorEventAndStatusAncr {
    fn _regulator_event_and_status_ancr(&self) -> &RegulatorEventAndStatusAncr;
    fn logical_node_for_event_and_status(&self) -> &super::commonmodule::LogicalNodeForEventAndStatus {
        self._regulator_event_and_status_ancr().logical_node_for_event_and_status.as_ref().unwrap_or(&regulator_event_and_status_ancr::LOGICAL_NODE_FOR_EVENT_AND_STATUS)
    }
    fn dynamic_test(&self) -> &super::commonmodule::EnsDynamicTestKind {
        self._regulator_event_and_status_ancr().dynamic_test.as_ref().unwrap_or(&regulator_event_and_status_ancr::DYNAMIC_TEST)
    }
    fn point_status(&self) -> &RegulatorEventAndStatusPoint {
        self._regulator_event_and_status_ancr().point_status.as_ref().unwrap_or(&regulator_event_and_status_ancr::POINT_STATUS)
    }
}
impl IsRegulatorEventAndStatusAncr for RegulatorEventAndStatusAncr {
    fn _regulator_event_and_status_ancr(&self) -> &RegulatorEventAndStatusAncr {
        self
    }
}
/// Regulator event
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct RegulatorEvent {
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
    pub regulator_event_and_status_ancr: ::std::option::Option<RegulatorEventAndStatusAncr>,
}
mod regulator_event {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref EVENT_VALUE: crate::commonmodule::EventValue = Default::default();
        pub(super) static ref REGULATOR_EVENT_AND_STATUS_ANCR: crate::regulatormodule::RegulatorEventAndStatusAncr = Default::default();
    }
}
trait IsRegulatorEvent {
    fn _regulator_event(&self) -> &RegulatorEvent;
    fn event_value(&self) -> &super::commonmodule::EventValue {
        self._regulator_event().event_value.as_ref().unwrap_or(&regulator_event::EVENT_VALUE)
    }
    fn regulator_event_and_status_ancr(&self) -> &RegulatorEventAndStatusAncr {
        self._regulator_event().regulator_event_and_status_ancr.as_ref().unwrap_or(&regulator_event::REGULATOR_EVENT_AND_STATUS_ANCR)
    }
}
impl IsRegulatorEvent for RegulatorEvent {
    fn _regulator_event(&self) -> &RegulatorEvent {
        self
    }
}
/// Regulator event profile
/// OpenFMB Profile Message: true
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct RegulatorEventProfile {
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
    pub regulator_event: ::std::option::Option<RegulatorEvent>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="4")]
    pub regulator_system: ::std::option::Option<RegulatorSystem>,
}
mod regulator_event_profile {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref EVENT_MESSAGE_INFO: crate::commonmodule::EventMessageInfo = Default::default();
        pub(super) static ref IED: crate::commonmodule::Ied = Default::default();
        pub(super) static ref REGULATOR_EVENT: crate::regulatormodule::RegulatorEvent = Default::default();
        pub(super) static ref REGULATOR_SYSTEM: crate::regulatormodule::RegulatorSystem = Default::default();
    }
}
trait IsRegulatorEventProfile {
    fn _regulator_event_profile(&self) -> &RegulatorEventProfile;
    fn event_message_info(&self) -> &super::commonmodule::EventMessageInfo {
        self._regulator_event_profile().event_message_info.as_ref().unwrap_or(&regulator_event_profile::EVENT_MESSAGE_INFO)
    }
    fn ied(&self) -> &super::commonmodule::Ied {
        self._regulator_event_profile().ied.as_ref().unwrap_or(&regulator_event_profile::IED)
    }
    fn regulator_event(&self) -> &RegulatorEvent {
        self._regulator_event_profile().regulator_event.as_ref().unwrap_or(&regulator_event_profile::REGULATOR_EVENT)
    }
    fn regulator_system(&self) -> &RegulatorSystem {
        self._regulator_event_profile().regulator_system.as_ref().unwrap_or(&regulator_event_profile::REGULATOR_SYSTEM)
    }
}
impl IsRegulatorEventProfile for RegulatorEventProfile {
    fn _regulator_event_profile(&self) -> &RegulatorEventProfile {
        self
    }
}
/// Regulator reading value
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct RegulatorReading {
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
    pub phase_mmtn: ::std::option::Option<super::commonmodule::PhaseMmtn>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub reading_mmtr: ::std::option::Option<super::commonmodule::ReadingMmtr>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="4")]
    pub reading_mmxu: ::std::option::Option<super::commonmodule::ReadingMmxu>,
}
mod regulator_reading {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref CONDUCTING_EQUIPMENT_TERMINAL_READING: crate::commonmodule::ConductingEquipmentTerminalReading = Default::default();
        pub(super) static ref PHASE_MMTN: crate::commonmodule::PhaseMmtn = Default::default();
        pub(super) static ref READING_MMTR: crate::commonmodule::ReadingMmtr = Default::default();
        pub(super) static ref READING_MMXU: crate::commonmodule::ReadingMmxu = Default::default();
    }
}
trait IsRegulatorReading {
    fn _regulator_reading(&self) -> &RegulatorReading;
    fn conducting_equipment_terminal_reading(&self) -> &super::commonmodule::ConductingEquipmentTerminalReading {
        self._regulator_reading().conducting_equipment_terminal_reading.as_ref().unwrap_or(&regulator_reading::CONDUCTING_EQUIPMENT_TERMINAL_READING)
    }
    fn phase_mmtn(&self) -> &super::commonmodule::PhaseMmtn {
        self._regulator_reading().phase_mmtn.as_ref().unwrap_or(&regulator_reading::PHASE_MMTN)
    }
    fn reading_mmtr(&self) -> &super::commonmodule::ReadingMmtr {
        self._regulator_reading().reading_mmtr.as_ref().unwrap_or(&regulator_reading::READING_MMTR)
    }
    fn reading_mmxu(&self) -> &super::commonmodule::ReadingMmxu {
        self._regulator_reading().reading_mmxu.as_ref().unwrap_or(&regulator_reading::READING_MMXU)
    }
}
impl IsRegulatorReading for RegulatorReading {
    fn _regulator_reading(&self) -> &RegulatorReading {
        self
    }
}
/// Regulator reading profile
/// OpenFMB Profile Message: true
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct RegulatorReadingProfile {
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
    // multiplicity_max: 2
    // uuid: false
    // key: false
    #[prost(message, repeated, tag="3")]
    pub regulator_reading: ::std::vec::Vec<RegulatorReading>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="4")]
    pub regulator_system: ::std::option::Option<RegulatorSystem>,
}
mod regulator_reading_profile {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref READING_MESSAGE_INFO: crate::commonmodule::ReadingMessageInfo = Default::default();
        pub(super) static ref IED: crate::commonmodule::Ied = Default::default();
        pub(super) static ref REGULATOR_SYSTEM: crate::regulatormodule::RegulatorSystem = Default::default();
    }
}
trait IsRegulatorReadingProfile {
    fn _regulator_reading_profile(&self) -> &RegulatorReadingProfile;
    fn reading_message_info(&self) -> &super::commonmodule::ReadingMessageInfo {
        self._regulator_reading_profile().reading_message_info.as_ref().unwrap_or(&regulator_reading_profile::READING_MESSAGE_INFO)
    }
    fn ied(&self) -> &super::commonmodule::Ied {
        self._regulator_reading_profile().ied.as_ref().unwrap_or(&regulator_reading_profile::IED)
    }
    fn regulator_reading(&self) -> &::std::vec::Vec<RegulatorReading> {
        &self._regulator_reading_profile().regulator_reading    }
    fn regulator_system(&self) -> &RegulatorSystem {
        self._regulator_reading_profile().regulator_system.as_ref().unwrap_or(&regulator_reading_profile::REGULATOR_SYSTEM)
    }
}
impl IsRegulatorReadingProfile for RegulatorReadingProfile {
    fn _regulator_reading_profile(&self) -> &RegulatorReadingProfile {
        self
    }
}
/// Regulator status
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct RegulatorStatus {
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
    pub regulator_event_and_status_ancr: ::std::option::Option<RegulatorEventAndStatusAncr>,
}
mod regulator_status {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref STATUS_VALUE: crate::commonmodule::StatusValue = Default::default();
        pub(super) static ref REGULATOR_EVENT_AND_STATUS_ANCR: crate::regulatormodule::RegulatorEventAndStatusAncr = Default::default();
    }
}
trait IsRegulatorStatus {
    fn _regulator_status(&self) -> &RegulatorStatus;
    fn status_value(&self) -> &super::commonmodule::StatusValue {
        self._regulator_status().status_value.as_ref().unwrap_or(&regulator_status::STATUS_VALUE)
    }
    fn regulator_event_and_status_ancr(&self) -> &RegulatorEventAndStatusAncr {
        self._regulator_status().regulator_event_and_status_ancr.as_ref().unwrap_or(&regulator_status::REGULATOR_EVENT_AND_STATUS_ANCR)
    }
}
impl IsRegulatorStatus for RegulatorStatus {
    fn _regulator_status(&self) -> &RegulatorStatus {
        self
    }
}
/// Regulator status profile
/// OpenFMB Profile Message: true
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct RegulatorStatusProfile {
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
    pub regulator_status: ::std::option::Option<RegulatorStatus>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="4")]
    pub regulator_system: ::std::option::Option<RegulatorSystem>,
}
mod regulator_status_profile {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref STATUS_MESSAGE_INFO: crate::commonmodule::StatusMessageInfo = Default::default();
        pub(super) static ref IED: crate::commonmodule::Ied = Default::default();
        pub(super) static ref REGULATOR_STATUS: crate::regulatormodule::RegulatorStatus = Default::default();
        pub(super) static ref REGULATOR_SYSTEM: crate::regulatormodule::RegulatorSystem = Default::default();
    }
}
trait IsRegulatorStatusProfile {
    fn _regulator_status_profile(&self) -> &RegulatorStatusProfile;
    fn status_message_info(&self) -> &super::commonmodule::StatusMessageInfo {
        self._regulator_status_profile().status_message_info.as_ref().unwrap_or(&regulator_status_profile::STATUS_MESSAGE_INFO)
    }
    fn ied(&self) -> &super::commonmodule::Ied {
        self._regulator_status_profile().ied.as_ref().unwrap_or(&regulator_status_profile::IED)
    }
    fn regulator_status(&self) -> &RegulatorStatus {
        self._regulator_status_profile().regulator_status.as_ref().unwrap_or(&regulator_status_profile::REGULATOR_STATUS)
    }
    fn regulator_system(&self) -> &RegulatorSystem {
        self._regulator_status_profile().regulator_system.as_ref().unwrap_or(&regulator_status_profile::REGULATOR_SYSTEM)
    }
}
impl IsRegulatorStatusProfile for RegulatorStatusProfile {
    fn _regulator_status_profile(&self) -> &RegulatorStatusProfile {
        self
    }
}
