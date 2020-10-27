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
pub trait IsRegulatorControlAtcc {
    fn _regulator_control_atcc(&self) -> &RegulatorControlAtcc;
    fn _regulator_control_atcc_mut(&mut self) -> &mut RegulatorControlAtcc;
    fn bnd_ctr(&self) -> &super::commonmodule::Asg {
        self._regulator_control_atcc().bnd_ctr.as_ref().unwrap_or(&regulator_control_atcc::BND_CTR)
    }
    fn bnd_ctr_mut(&mut self) -> &mut super::commonmodule::Asg {
        self._regulator_control_atcc_mut().bnd_ctr.get_or_insert(Default::default())
    }
    fn bnd_wid(&self) -> &super::commonmodule::Asg {
        self._regulator_control_atcc().bnd_wid.as_ref().unwrap_or(&regulator_control_atcc::BND_WID)
    }
    fn bnd_wid_mut(&mut self) -> &mut super::commonmodule::Asg {
        self._regulator_control_atcc_mut().bnd_wid.get_or_insert(Default::default())
    }
    fn ctl_dl_tmms(&self) -> &super::commonmodule::ControlIng {
        self._regulator_control_atcc().ctl_dl_tmms.as_ref().unwrap_or(&regulator_control_atcc::CTL_DL_TMMS)
    }
    fn ctl_dl_tmms_mut(&mut self) -> &mut super::commonmodule::ControlIng {
        self._regulator_control_atcc_mut().ctl_dl_tmms.get_or_insert(Default::default())
    }
    fn ldcr(&self) -> &super::commonmodule::Asg {
        self._regulator_control_atcc().ldcr.as_ref().unwrap_or(&regulator_control_atcc::LDCR)
    }
    fn ldcr_mut(&mut self) -> &mut super::commonmodule::Asg {
        self._regulator_control_atcc_mut().ldcr.get_or_insert(Default::default())
    }
    fn ldcx(&self) -> &super::commonmodule::Asg {
        self._regulator_control_atcc().ldcx.as_ref().unwrap_or(&regulator_control_atcc::LDCX)
    }
    fn ldcx_mut(&mut self) -> &mut super::commonmodule::Asg {
        self._regulator_control_atcc_mut().ldcx.get_or_insert(Default::default())
    }
    fn ldcz(&self) -> &super::commonmodule::Asg {
        self._regulator_control_atcc().ldcz.as_ref().unwrap_or(&regulator_control_atcc::LDCZ)
    }
    fn ldcz_mut(&mut self) -> &mut super::commonmodule::Asg {
        self._regulator_control_atcc_mut().ldcz.get_or_insert(Default::default())
    }
    fn par_op(&self) -> &super::commonmodule::ControlSpc {
        self._regulator_control_atcc().par_op.as_ref().unwrap_or(&regulator_control_atcc::PAR_OP)
    }
    fn par_op_mut(&mut self) -> &mut super::commonmodule::ControlSpc {
        self._regulator_control_atcc_mut().par_op.get_or_insert(Default::default())
    }
    fn tap_pos(&self) -> &super::commonmodule::ControlIsc {
        self._regulator_control_atcc().tap_pos.as_ref().unwrap_or(&regulator_control_atcc::TAP_POS)
    }
    fn tap_pos_mut(&mut self) -> &mut super::commonmodule::ControlIsc {
        self._regulator_control_atcc_mut().tap_pos.get_or_insert(Default::default())
    }
}
impl IsRegulatorControlAtcc for RegulatorControlAtcc {
    fn _regulator_control_atcc(&self) -> &RegulatorControlAtcc {
        self
    }
    fn _regulator_control_atcc_mut(&mut self) -> &mut RegulatorControlAtcc {
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
    // multiplicity_min: Some(1)
    // multiplicity_max: None
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
pub trait IsRegulatorPoint {
    fn _regulator_point(&self) -> &RegulatorPoint;
    fn _regulator_point_mut(&mut self) -> &mut RegulatorPoint;
    fn control(&self) -> &RegulatorControlAtcc {
        self._regulator_point().control.as_ref().unwrap_or(&regulator_point::CONTROL)
    }
    fn control_mut(&mut self) -> &mut RegulatorControlAtcc {
        self._regulator_point_mut().control.get_or_insert(Default::default())
    }
    fn pct_v_droop(&self) -> &f32 {
        self._regulator_point().pct_v_droop.as_ref().unwrap_or(&regulator_point::PCT_V_DROOP)
    }
    fn pct_v_droop_mut(&mut self) -> &mut f32 {
        self._regulator_point_mut().pct_v_droop.get_or_insert(Default::default())
    }
    fn ramp_rates(&self) -> &super::commonmodule::RampRate {
        self._regulator_point().ramp_rates.as_ref().unwrap_or(&regulator_point::RAMP_RATES)
    }
    fn ramp_rates_mut(&mut self) -> &mut super::commonmodule::RampRate {
        self._regulator_point_mut().ramp_rates.get_or_insert(Default::default())
    }
    fn reactive_pwr_set_point_enabled(&self) -> &super::commonmodule::ControlDpc {
        self._regulator_point().reactive_pwr_set_point_enabled.as_ref().unwrap_or(&regulator_point::REACTIVE_PWR_SET_POINT_ENABLED)
    }
    fn reactive_pwr_set_point_enabled_mut(&mut self) -> &mut super::commonmodule::ControlDpc {
        self._regulator_point_mut().reactive_pwr_set_point_enabled.get_or_insert(Default::default())
    }
    fn reset(&self) -> &super::commonmodule::ControlDpc {
        self._regulator_point().reset.as_ref().unwrap_or(&regulator_point::RESET)
    }
    fn reset_mut(&mut self) -> &mut super::commonmodule::ControlDpc {
        self._regulator_point_mut().reset.get_or_insert(Default::default())
    }
    fn state(&self) -> &super::commonmodule::OptionalStateKind {
        self._regulator_point().state.as_ref().unwrap_or(&regulator_point::STATE)
    }
    fn state_mut(&mut self) -> &mut super::commonmodule::OptionalStateKind {
        self._regulator_point_mut().state.get_or_insert(Default::default())
    }
    fn voltage_set_point_enabled(&self) -> &super::commonmodule::ControlDpc {
        self._regulator_point().voltage_set_point_enabled.as_ref().unwrap_or(&regulator_point::VOLTAGE_SET_POINT_ENABLED)
    }
    fn voltage_set_point_enabled_mut(&mut self) -> &mut super::commonmodule::ControlDpc {
        self._regulator_point_mut().voltage_set_point_enabled.get_or_insert(Default::default())
    }
    fn start_time(&self) -> &super::commonmodule::Timestamp {
        self._regulator_point().start_time.as_ref().unwrap_or(&regulator_point::START_TIME)
    }
    fn start_time_mut(&mut self) -> &mut super::commonmodule::Timestamp {
        self._regulator_point_mut().start_time.get_or_insert(Default::default())
    }
}
impl IsRegulatorPoint for RegulatorPoint {
    fn _regulator_point(&self) -> &RegulatorPoint {
        self
    }
    fn _regulator_point_mut(&mut self) -> &mut RegulatorPoint {
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
    // multiplicity_min: Some(1)
    // multiplicity_max: None
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
pub trait IsRegulatorCsg {
    fn _regulator_csg(&self) -> &RegulatorCsg;
    fn _regulator_csg_mut(&mut self) -> &mut RegulatorCsg;
    fn crv_pts(&self) -> &::std::vec::Vec<RegulatorPoint> {
        &self._regulator_csg().crv_pts
    }
    fn crv_pts_mut(&mut self) -> &mut ::std::vec::Vec<RegulatorPoint> {
        &mut self._regulator_csg_mut().crv_pts
    }
}
impl IsRegulatorCsg for RegulatorCsg {
    fn _regulator_csg(&self) -> &RegulatorCsg {
        self
    }
    fn _regulator_csg_mut(&mut self) -> &mut RegulatorCsg {
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
    // multiplicity_min: Some(1)
    // multiplicity_max: None
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
pub trait IsRegulatorControlScheduleFsch {
    fn _regulator_control_schedule_fsch(&self) -> &RegulatorControlScheduleFsch;
    fn _regulator_control_schedule_fsch_mut(&mut self) -> &mut RegulatorControlScheduleFsch;
    fn val_dcsg(&self) -> &RegulatorCsg {
        self._regulator_control_schedule_fsch().val_dcsg.as_ref().unwrap_or(&regulator_control_schedule_fsch::VAL_DCSG)
    }
    fn val_dcsg_mut(&mut self) -> &mut RegulatorCsg {
        self._regulator_control_schedule_fsch_mut().val_dcsg.get_or_insert(Default::default())
    }
}
impl IsRegulatorControlScheduleFsch for RegulatorControlScheduleFsch {
    fn _regulator_control_schedule_fsch(&self) -> &RegulatorControlScheduleFsch {
        self
    }
    fn _regulator_control_schedule_fsch_mut(&mut self) -> &mut RegulatorControlScheduleFsch {
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
    // multiplicity_min: None
    // multiplicity_max: None
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
pub trait IsRegulatorControlFscc {
    fn _regulator_control_fscc(&self) -> &RegulatorControlFscc;
    fn _regulator_control_fscc_mut(&mut self) -> &mut RegulatorControlFscc;
    fn control_fscc(&self) -> &super::commonmodule::ControlFscc {
        self._regulator_control_fscc().control_fscc.as_ref().unwrap_or(&regulator_control_fscc::CONTROL_FSCC)
    }
    fn control_fscc_mut(&mut self) -> &mut super::commonmodule::ControlFscc {
        self._regulator_control_fscc_mut().control_fscc.get_or_insert(Default::default())
    }
    fn regulator_control_schedule_fsch(&self) -> &RegulatorControlScheduleFsch {
        self._regulator_control_fscc().regulator_control_schedule_fsch.as_ref().unwrap_or(&regulator_control_fscc::REGULATOR_CONTROL_SCHEDULE_FSCH)
    }
    fn regulator_control_schedule_fsch_mut(&mut self) -> &mut RegulatorControlScheduleFsch {
        self._regulator_control_fscc_mut().regulator_control_schedule_fsch.get_or_insert(Default::default())
    }
}
impl IsRegulatorControlFscc for RegulatorControlFscc {
    fn _regulator_control_fscc(&self) -> &RegulatorControlFscc {
        self
    }
    fn _regulator_control_fscc_mut(&mut self) -> &mut RegulatorControlFscc {
        self
    }
}
//impl IsControlFSCC for RegulatorControlFscc {
    //fn _control_fscc(&self) -> &ControlFscc {
        //
    //}
//fn _mut_control_fscc(&mut self) -> &mut ControlFscc {
        //
    //}
//}
//impl IsLogicalNodeForControl for RegulatorControlFscc {
    //fn _logical_node_for_control(&self) -> &LogicalNodeForControl {
        //
    //}
//fn _mut_logical_node_for_control(&mut self) -> &mut LogicalNodeForControl {
        //
    //}
//}
//impl IsLogicalNode for RegulatorControlFscc {
    //fn _logical_node(&self) -> &LogicalNode {
        //
    //}
//fn _mut_logical_node(&mut self) -> &mut LogicalNode {
        //
    //}
//}
//impl IsIdentifiedObject for RegulatorControlFscc {
    //fn _identified_object(&self) -> &IdentifiedObject {
        //
    //}
//fn _mut_identified_object(&mut self) -> &mut IdentifiedObject {
        //
    //}
//}
/// Regulator control
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct RegulatorControl {
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
pub trait IsRegulatorControl {
    fn _regulator_control(&self) -> &RegulatorControl;
    fn _regulator_control_mut(&mut self) -> &mut RegulatorControl;
    fn control_value(&self) -> &super::commonmodule::ControlValue {
        self._regulator_control().control_value.as_ref().unwrap_or(&regulator_control::CONTROL_VALUE)
    }
    fn control_value_mut(&mut self) -> &mut super::commonmodule::ControlValue {
        self._regulator_control_mut().control_value.get_or_insert(Default::default())
    }
    fn check(&self) -> &super::commonmodule::CheckConditions {
        self._regulator_control().check.as_ref().unwrap_or(&regulator_control::CHECK)
    }
    fn check_mut(&mut self) -> &mut super::commonmodule::CheckConditions {
        self._regulator_control_mut().check.get_or_insert(Default::default())
    }
    fn regulator_control_fscc(&self) -> &RegulatorControlFscc {
        self._regulator_control().regulator_control_fscc.as_ref().unwrap_or(&regulator_control::REGULATOR_CONTROL_FSCC)
    }
    fn regulator_control_fscc_mut(&mut self) -> &mut RegulatorControlFscc {
        self._regulator_control_mut().regulator_control_fscc.get_or_insert(Default::default())
    }
}
impl IsRegulatorControl for RegulatorControl {
    fn _regulator_control(&self) -> &RegulatorControl {
        self
    }
    fn _regulator_control_mut(&mut self) -> &mut RegulatorControl {
        self
    }
}
//impl IsControlValue for RegulatorControl {
    //fn _control_value(&self) -> &ControlValue {
        //
    //}
//fn _mut_control_value(&mut self) -> &mut ControlValue {
        //
    //}
//}
//impl IsIdentifiedObject for RegulatorControl {
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
pub struct RegulatorSystem {
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
mod regulator_system {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref CONDUCTING_EQUIPMENT: crate::commonmodule::ConductingEquipment = Default::default();
    }
}
pub trait IsRegulatorSystem {
    fn _regulator_system(&self) -> &RegulatorSystem;
    fn _regulator_system_mut(&mut self) -> &mut RegulatorSystem;
    fn conducting_equipment(&self) -> &super::commonmodule::ConductingEquipment {
        self._regulator_system().conducting_equipment.as_ref().unwrap_or(&regulator_system::CONDUCTING_EQUIPMENT)
    }
    fn conducting_equipment_mut(&mut self) -> &mut super::commonmodule::ConductingEquipment {
        self._regulator_system_mut().conducting_equipment.get_or_insert(Default::default())
    }
}
impl IsRegulatorSystem for RegulatorSystem {
    fn _regulator_system(&self) -> &RegulatorSystem {
        self
    }
    fn _regulator_system_mut(&mut self) -> &mut RegulatorSystem {
        self
    }
}
//impl IsConductingEquipment for RegulatorSystem {
    //fn _conducting_equipment(&self) -> &ConductingEquipment {
        //
    //}
//fn _mut_conducting_equipment(&mut self) -> &mut ConductingEquipment {
        //
    //}
//}
//impl IsNamedObject for RegulatorSystem {
    //fn _named_object(&self) -> &NamedObject {
        //
    //}
//fn _mut_named_object(&mut self) -> &mut NamedObject {
        //
    //}
//}
/// Regulator control profile.  Instructs an end device (or an end device group) to perform a
/// specified action.
/// OpenFMB Profile Message: true
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct RegulatorControlProfile {
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
    pub regulator_control: ::std::option::Option<RegulatorControl>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: Some(1)
    // multiplicity_max: None
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
pub trait IsRegulatorControlProfile {
    fn _regulator_control_profile(&self) -> &RegulatorControlProfile;
    fn _regulator_control_profile_mut(&mut self) -> &mut RegulatorControlProfile;
    fn control_message_info(&self) -> &super::commonmodule::ControlMessageInfo {
        self._regulator_control_profile().control_message_info.as_ref().unwrap_or(&regulator_control_profile::CONTROL_MESSAGE_INFO)
    }
    fn control_message_info_mut(&mut self) -> &mut super::commonmodule::ControlMessageInfo {
        self._regulator_control_profile_mut().control_message_info.get_or_insert(Default::default())
    }
    fn ied(&self) -> &super::commonmodule::Ied {
        self._regulator_control_profile().ied.as_ref().unwrap_or(&regulator_control_profile::IED)
    }
    fn ied_mut(&mut self) -> &mut super::commonmodule::Ied {
        self._regulator_control_profile_mut().ied.get_or_insert(Default::default())
    }
    fn regulator_control(&self) -> &RegulatorControl {
        self._regulator_control_profile().regulator_control.as_ref().unwrap_or(&regulator_control_profile::REGULATOR_CONTROL)
    }
    fn regulator_control_mut(&mut self) -> &mut RegulatorControl {
        self._regulator_control_profile_mut().regulator_control.get_or_insert(Default::default())
    }
    fn regulator_system(&self) -> &RegulatorSystem {
        self._regulator_control_profile().regulator_system.as_ref().unwrap_or(&regulator_control_profile::REGULATOR_SYSTEM)
    }
    fn regulator_system_mut(&mut self) -> &mut RegulatorSystem {
        self._regulator_control_profile_mut().regulator_system.get_or_insert(Default::default())
    }
}
impl IsRegulatorControlProfile for RegulatorControlProfile {
    fn _regulator_control_profile(&self) -> &RegulatorControlProfile {
        self
    }
    fn _regulator_control_profile_mut(&mut self) -> &mut RegulatorControlProfile {
        self
    }
}
//impl IsControlMessageInfo for RegulatorControlProfile {
    //fn _control_message_info(&self) -> &ControlMessageInfo {
        //
    //}
//fn _mut_control_message_info(&mut self) -> &mut ControlMessageInfo {
        //
    //}
//}
//impl IsMessageInfo for RegulatorControlProfile {
    //fn _message_info(&self) -> &MessageInfo {
        //
    //}
//fn _mut_message_info(&mut self) -> &mut MessageInfo {
        //
    //}
//}
//impl IsIdentifiedObject for RegulatorControlProfile {
    //fn _identified_object(&self) -> &IdentifiedObject {
        //
    //}
//fn _mut_identified_object(&mut self) -> &mut IdentifiedObject {
        //
    //}
//}
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
pub trait IsRegulatorEventAndStatusAtcc {
    fn _regulator_event_and_status_atcc(&self) -> &RegulatorEventAndStatusAtcc;
    fn _regulator_event_and_status_atcc_mut(&mut self) -> &mut RegulatorEventAndStatusAtcc;
    fn bnd_ctr(&self) -> &super::commonmodule::Asg {
        self._regulator_event_and_status_atcc().bnd_ctr.as_ref().unwrap_or(&regulator_event_and_status_atcc::BND_CTR)
    }
    fn bnd_ctr_mut(&mut self) -> &mut super::commonmodule::Asg {
        self._regulator_event_and_status_atcc_mut().bnd_ctr.get_or_insert(Default::default())
    }
    fn bnd_wid(&self) -> &super::commonmodule::Asg {
        self._regulator_event_and_status_atcc().bnd_wid.as_ref().unwrap_or(&regulator_event_and_status_atcc::BND_WID)
    }
    fn bnd_wid_mut(&mut self) -> &mut super::commonmodule::Asg {
        self._regulator_event_and_status_atcc_mut().bnd_wid.get_or_insert(Default::default())
    }
    fn ldcr(&self) -> &super::commonmodule::Asg {
        self._regulator_event_and_status_atcc().ldcr.as_ref().unwrap_or(&regulator_event_and_status_atcc::LDCR)
    }
    fn ldcr_mut(&mut self) -> &mut super::commonmodule::Asg {
        self._regulator_event_and_status_atcc_mut().ldcr.get_or_insert(Default::default())
    }
    fn ldcx(&self) -> &super::commonmodule::Asg {
        self._regulator_event_and_status_atcc().ldcx.as_ref().unwrap_or(&regulator_event_and_status_atcc::LDCX)
    }
    fn ldcx_mut(&mut self) -> &mut super::commonmodule::Asg {
        self._regulator_event_and_status_atcc_mut().ldcx.get_or_insert(Default::default())
    }
    fn ldcz(&self) -> &super::commonmodule::Asg {
        self._regulator_event_and_status_atcc().ldcz.as_ref().unwrap_or(&regulator_event_and_status_atcc::LDCZ)
    }
    fn ldcz_mut(&mut self) -> &mut super::commonmodule::Asg {
        self._regulator_event_and_status_atcc_mut().ldcz.get_or_insert(Default::default())
    }
    fn par_op(&self) -> &super::commonmodule::StatusSpc {
        self._regulator_event_and_status_atcc().par_op.as_ref().unwrap_or(&regulator_event_and_status_atcc::PAR_OP)
    }
    fn par_op_mut(&mut self) -> &mut super::commonmodule::StatusSpc {
        self._regulator_event_and_status_atcc_mut().par_op.get_or_insert(Default::default())
    }
    fn tap_op_err(&self) -> &super::commonmodule::StatusSps {
        self._regulator_event_and_status_atcc().tap_op_err.as_ref().unwrap_or(&regulator_event_and_status_atcc::TAP_OP_ERR)
    }
    fn tap_op_err_mut(&mut self) -> &mut super::commonmodule::StatusSps {
        self._regulator_event_and_status_atcc_mut().tap_op_err.get_or_insert(Default::default())
    }
    fn tap_pos(&self) -> &super::commonmodule::StatusIsc {
        self._regulator_event_and_status_atcc().tap_pos.as_ref().unwrap_or(&regulator_event_and_status_atcc::TAP_POS)
    }
    fn tap_pos_mut(&mut self) -> &mut super::commonmodule::StatusIsc {
        self._regulator_event_and_status_atcc_mut().tap_pos.get_or_insert(Default::default())
    }
}
impl IsRegulatorEventAndStatusAtcc for RegulatorEventAndStatusAtcc {
    fn _regulator_event_and_status_atcc(&self) -> &RegulatorEventAndStatusAtcc {
        self
    }
    fn _regulator_event_and_status_atcc_mut(&mut self) -> &mut RegulatorEventAndStatusAtcc {
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
pub trait IsRegulatorEventAndStatusPoint {
    fn _regulator_event_and_status_point(&self) -> &RegulatorEventAndStatusPoint;
    fn _regulator_event_and_status_point_mut(&mut self) -> &mut RegulatorEventAndStatusPoint;
    fn event_and_status(&self) -> &RegulatorEventAndStatusAtcc {
        self._regulator_event_and_status_point().event_and_status.as_ref().unwrap_or(&regulator_event_and_status_point::EVENT_AND_STATUS)
    }
    fn event_and_status_mut(&mut self) -> &mut RegulatorEventAndStatusAtcc {
        self._regulator_event_and_status_point_mut().event_and_status.get_or_insert(Default::default())
    }
    fn pct_v_droop(&self) -> &f32 {
        self._regulator_event_and_status_point().pct_v_droop.as_ref().unwrap_or(&regulator_event_and_status_point::PCT_V_DROOP)
    }
    fn pct_v_droop_mut(&mut self) -> &mut f32 {
        self._regulator_event_and_status_point_mut().pct_v_droop.get_or_insert(Default::default())
    }
    fn ramp_rates(&self) -> &super::commonmodule::RampRate {
        self._regulator_event_and_status_point().ramp_rates.as_ref().unwrap_or(&regulator_event_and_status_point::RAMP_RATES)
    }
    fn ramp_rates_mut(&mut self) -> &mut super::commonmodule::RampRate {
        self._regulator_event_and_status_point_mut().ramp_rates.get_or_insert(Default::default())
    }
    fn reactive_pwr_set_point_enabled(&self) -> &super::commonmodule::StatusSpc {
        self._regulator_event_and_status_point().reactive_pwr_set_point_enabled.as_ref().unwrap_or(&regulator_event_and_status_point::REACTIVE_PWR_SET_POINT_ENABLED)
    }
    fn reactive_pwr_set_point_enabled_mut(&mut self) -> &mut super::commonmodule::StatusSpc {
        self._regulator_event_and_status_point_mut().reactive_pwr_set_point_enabled.get_or_insert(Default::default())
    }
    fn state(&self) -> &super::commonmodule::OptionalStateKind {
        self._regulator_event_and_status_point().state.as_ref().unwrap_or(&regulator_event_and_status_point::STATE)
    }
    fn state_mut(&mut self) -> &mut super::commonmodule::OptionalStateKind {
        self._regulator_event_and_status_point_mut().state.get_or_insert(Default::default())
    }
    fn voltage_set_point_enabled(&self) -> &super::commonmodule::StatusSpc {
        self._regulator_event_and_status_point().voltage_set_point_enabled.as_ref().unwrap_or(&regulator_event_and_status_point::VOLTAGE_SET_POINT_ENABLED)
    }
    fn voltage_set_point_enabled_mut(&mut self) -> &mut super::commonmodule::StatusSpc {
        self._regulator_event_and_status_point_mut().voltage_set_point_enabled.get_or_insert(Default::default())
    }
}
impl IsRegulatorEventAndStatusPoint for RegulatorEventAndStatusPoint {
    fn _regulator_event_and_status_point(&self) -> &RegulatorEventAndStatusPoint {
        self
    }
    fn _regulator_event_and_status_point_mut(&mut self) -> &mut RegulatorEventAndStatusPoint {
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
    // multiplicity_min: None
    // multiplicity_max: None
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
    // multiplicity_min: Some(1)
    // multiplicity_max: None
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
pub trait IsRegulatorEventAndStatusAncr {
    fn _regulator_event_and_status_ancr(&self) -> &RegulatorEventAndStatusAncr;
    fn _regulator_event_and_status_ancr_mut(&mut self) -> &mut RegulatorEventAndStatusAncr;
    fn logical_node_for_event_and_status(&self) -> &super::commonmodule::LogicalNodeForEventAndStatus {
        self._regulator_event_and_status_ancr().logical_node_for_event_and_status.as_ref().unwrap_or(&regulator_event_and_status_ancr::LOGICAL_NODE_FOR_EVENT_AND_STATUS)
    }
    fn logical_node_for_event_and_status_mut(&mut self) -> &mut super::commonmodule::LogicalNodeForEventAndStatus {
        self._regulator_event_and_status_ancr_mut().logical_node_for_event_and_status.get_or_insert(Default::default())
    }
    fn dynamic_test(&self) -> &super::commonmodule::EnsDynamicTestKind {
        self._regulator_event_and_status_ancr().dynamic_test.as_ref().unwrap_or(&regulator_event_and_status_ancr::DYNAMIC_TEST)
    }
    fn dynamic_test_mut(&mut self) -> &mut super::commonmodule::EnsDynamicTestKind {
        self._regulator_event_and_status_ancr_mut().dynamic_test.get_or_insert(Default::default())
    }
    fn point_status(&self) -> &RegulatorEventAndStatusPoint {
        self._regulator_event_and_status_ancr().point_status.as_ref().unwrap_or(&regulator_event_and_status_ancr::POINT_STATUS)
    }
    fn point_status_mut(&mut self) -> &mut RegulatorEventAndStatusPoint {
        self._regulator_event_and_status_ancr_mut().point_status.get_or_insert(Default::default())
    }
}
impl IsRegulatorEventAndStatusAncr for RegulatorEventAndStatusAncr {
    fn _regulator_event_and_status_ancr(&self) -> &RegulatorEventAndStatusAncr {
        self
    }
    fn _regulator_event_and_status_ancr_mut(&mut self) -> &mut RegulatorEventAndStatusAncr {
        self
    }
}
//impl IsLogicalNodeForEventAndStatus for RegulatorEventAndStatusAncr {
    //fn _logical_node_for_event_and_status(&self) -> &LogicalNodeForEventAndStatus {
        //
    //}
//fn _mut_logical_node_for_event_and_status(&mut self) -> &mut LogicalNodeForEventAndStatus {
        //
    //}
//}
//impl IsLogicalNode for RegulatorEventAndStatusAncr {
    //fn _logical_node(&self) -> &LogicalNode {
        //
    //}
//fn _mut_logical_node(&mut self) -> &mut LogicalNode {
        //
    //}
//}
//impl IsIdentifiedObject for RegulatorEventAndStatusAncr {
    //fn _identified_object(&self) -> &IdentifiedObject {
        //
    //}
//fn _mut_identified_object(&mut self) -> &mut IdentifiedObject {
        //
    //}
//}
/// Regulator event
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct RegulatorEvent {
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
    pub regulator_event_and_status_ancr: ::std::option::Option<RegulatorEventAndStatusAncr>,
}
mod regulator_event {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref EVENT_VALUE: crate::commonmodule::EventValue = Default::default();
        pub(super) static ref REGULATOR_EVENT_AND_STATUS_ANCR: crate::regulatormodule::RegulatorEventAndStatusAncr = Default::default();
    }
}
pub trait IsRegulatorEvent {
    fn _regulator_event(&self) -> &RegulatorEvent;
    fn _regulator_event_mut(&mut self) -> &mut RegulatorEvent;
    fn event_value(&self) -> &super::commonmodule::EventValue {
        self._regulator_event().event_value.as_ref().unwrap_or(&regulator_event::EVENT_VALUE)
    }
    fn event_value_mut(&mut self) -> &mut super::commonmodule::EventValue {
        self._regulator_event_mut().event_value.get_or_insert(Default::default())
    }
    fn regulator_event_and_status_ancr(&self) -> &RegulatorEventAndStatusAncr {
        self._regulator_event().regulator_event_and_status_ancr.as_ref().unwrap_or(&regulator_event::REGULATOR_EVENT_AND_STATUS_ANCR)
    }
    fn regulator_event_and_status_ancr_mut(&mut self) -> &mut RegulatorEventAndStatusAncr {
        self._regulator_event_mut().regulator_event_and_status_ancr.get_or_insert(Default::default())
    }
}
impl IsRegulatorEvent for RegulatorEvent {
    fn _regulator_event(&self) -> &RegulatorEvent {
        self
    }
    fn _regulator_event_mut(&mut self) -> &mut RegulatorEvent {
        self
    }
}
//impl IsEventValue for RegulatorEvent {
    //fn _event_value(&self) -> &EventValue {
        //
    //}
//fn _mut_event_value(&mut self) -> &mut EventValue {
        //
    //}
//}
//impl IsIdentifiedObject for RegulatorEvent {
    //fn _identified_object(&self) -> &IdentifiedObject {
        //
    //}
//fn _mut_identified_object(&mut self) -> &mut IdentifiedObject {
        //
    //}
//}
/// Regulator event profile
/// OpenFMB Profile Message: true
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct RegulatorEventProfile {
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
    pub regulator_event: ::std::option::Option<RegulatorEvent>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: Some(1)
    // multiplicity_max: None
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
pub trait IsRegulatorEventProfile {
    fn _regulator_event_profile(&self) -> &RegulatorEventProfile;
    fn _regulator_event_profile_mut(&mut self) -> &mut RegulatorEventProfile;
    fn event_message_info(&self) -> &super::commonmodule::EventMessageInfo {
        self._regulator_event_profile().event_message_info.as_ref().unwrap_or(&regulator_event_profile::EVENT_MESSAGE_INFO)
    }
    fn event_message_info_mut(&mut self) -> &mut super::commonmodule::EventMessageInfo {
        self._regulator_event_profile_mut().event_message_info.get_or_insert(Default::default())
    }
    fn ied(&self) -> &super::commonmodule::Ied {
        self._regulator_event_profile().ied.as_ref().unwrap_or(&regulator_event_profile::IED)
    }
    fn ied_mut(&mut self) -> &mut super::commonmodule::Ied {
        self._regulator_event_profile_mut().ied.get_or_insert(Default::default())
    }
    fn regulator_event(&self) -> &RegulatorEvent {
        self._regulator_event_profile().regulator_event.as_ref().unwrap_or(&regulator_event_profile::REGULATOR_EVENT)
    }
    fn regulator_event_mut(&mut self) -> &mut RegulatorEvent {
        self._regulator_event_profile_mut().regulator_event.get_or_insert(Default::default())
    }
    fn regulator_system(&self) -> &RegulatorSystem {
        self._regulator_event_profile().regulator_system.as_ref().unwrap_or(&regulator_event_profile::REGULATOR_SYSTEM)
    }
    fn regulator_system_mut(&mut self) -> &mut RegulatorSystem {
        self._regulator_event_profile_mut().regulator_system.get_or_insert(Default::default())
    }
}
impl IsRegulatorEventProfile for RegulatorEventProfile {
    fn _regulator_event_profile(&self) -> &RegulatorEventProfile {
        self
    }
    fn _regulator_event_profile_mut(&mut self) -> &mut RegulatorEventProfile {
        self
    }
}
//impl IsEventMessageInfo for RegulatorEventProfile {
    //fn _event_message_info(&self) -> &EventMessageInfo {
        //
    //}
//fn _mut_event_message_info(&mut self) -> &mut EventMessageInfo {
        //
    //}
//}
//impl IsMessageInfo for RegulatorEventProfile {
    //fn _message_info(&self) -> &MessageInfo {
        //
    //}
//fn _mut_message_info(&mut self) -> &mut MessageInfo {
        //
    //}
//}
//impl IsIdentifiedObject for RegulatorEventProfile {
    //fn _identified_object(&self) -> &IdentifiedObject {
        //
    //}
//fn _mut_identified_object(&mut self) -> &mut IdentifiedObject {
        //
    //}
//}
/// Regulator reading value
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct RegulatorReading {
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
mod regulator_reading {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref CONDUCTING_EQUIPMENT_TERMINAL_READING: crate::commonmodule::ConductingEquipmentTerminalReading = Default::default();
        pub(super) static ref PHASE_MMTN: crate::commonmodule::PhaseMmtn = Default::default();
        pub(super) static ref READING_MMTR: crate::commonmodule::ReadingMmtr = Default::default();
        pub(super) static ref READING_MMXU: crate::commonmodule::ReadingMmxu = Default::default();
    }
}
pub trait IsRegulatorReading {
    fn _regulator_reading(&self) -> &RegulatorReading;
    fn _regulator_reading_mut(&mut self) -> &mut RegulatorReading;
    fn conducting_equipment_terminal_reading(&self) -> &super::commonmodule::ConductingEquipmentTerminalReading {
        self._regulator_reading().conducting_equipment_terminal_reading.as_ref().unwrap_or(&regulator_reading::CONDUCTING_EQUIPMENT_TERMINAL_READING)
    }
    fn conducting_equipment_terminal_reading_mut(&mut self) -> &mut super::commonmodule::ConductingEquipmentTerminalReading {
        self._regulator_reading_mut().conducting_equipment_terminal_reading.get_or_insert(Default::default())
    }
    fn phase_mmtn(&self) -> &super::commonmodule::PhaseMmtn {
        self._regulator_reading().phase_mmtn.as_ref().unwrap_or(&regulator_reading::PHASE_MMTN)
    }
    fn phase_mmtn_mut(&mut self) -> &mut super::commonmodule::PhaseMmtn {
        self._regulator_reading_mut().phase_mmtn.get_or_insert(Default::default())
    }
    fn reading_mmtr(&self) -> &super::commonmodule::ReadingMmtr {
        self._regulator_reading().reading_mmtr.as_ref().unwrap_or(&regulator_reading::READING_MMTR)
    }
    fn reading_mmtr_mut(&mut self) -> &mut super::commonmodule::ReadingMmtr {
        self._regulator_reading_mut().reading_mmtr.get_or_insert(Default::default())
    }
    fn reading_mmxu(&self) -> &super::commonmodule::ReadingMmxu {
        self._regulator_reading().reading_mmxu.as_ref().unwrap_or(&regulator_reading::READING_MMXU)
    }
    fn reading_mmxu_mut(&mut self) -> &mut super::commonmodule::ReadingMmxu {
        self._regulator_reading_mut().reading_mmxu.get_or_insert(Default::default())
    }
}
impl IsRegulatorReading for RegulatorReading {
    fn _regulator_reading(&self) -> &RegulatorReading {
        self
    }
    fn _regulator_reading_mut(&mut self) -> &mut RegulatorReading {
        self
    }
}
//impl IsConductingEquipmentTerminalReading for RegulatorReading {
    //fn _conducting_equipment_terminal_reading(&self) -> &ConductingEquipmentTerminalReading {
        //
    //}
//fn _mut_conducting_equipment_terminal_reading(&mut self) -> &mut ConductingEquipmentTerminalReading {
        //
    //}
//}
/// Regulator reading profile
/// OpenFMB Profile Message: true
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct RegulatorReadingProfile {
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
    // multiplicity_max: Some(2)
    // uuid: false
    // key: false
    #[prost(message, repeated, tag="3")]
    pub regulator_reading: ::std::vec::Vec<RegulatorReading>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: Some(1)
    // multiplicity_max: None
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
pub trait IsRegulatorReadingProfile {
    fn _regulator_reading_profile(&self) -> &RegulatorReadingProfile;
    fn _regulator_reading_profile_mut(&mut self) -> &mut RegulatorReadingProfile;
    fn reading_message_info(&self) -> &super::commonmodule::ReadingMessageInfo {
        self._regulator_reading_profile().reading_message_info.as_ref().unwrap_or(&regulator_reading_profile::READING_MESSAGE_INFO)
    }
    fn reading_message_info_mut(&mut self) -> &mut super::commonmodule::ReadingMessageInfo {
        self._regulator_reading_profile_mut().reading_message_info.get_or_insert(Default::default())
    }
    fn ied(&self) -> &super::commonmodule::Ied {
        self._regulator_reading_profile().ied.as_ref().unwrap_or(&regulator_reading_profile::IED)
    }
    fn ied_mut(&mut self) -> &mut super::commonmodule::Ied {
        self._regulator_reading_profile_mut().ied.get_or_insert(Default::default())
    }
    fn regulator_reading(&self) -> &::std::vec::Vec<RegulatorReading> {
        &self._regulator_reading_profile().regulator_reading
    }
    fn regulator_reading_mut(&mut self) -> &mut ::std::vec::Vec<RegulatorReading> {
        &mut self._regulator_reading_profile_mut().regulator_reading
    }
    fn regulator_system(&self) -> &RegulatorSystem {
        self._regulator_reading_profile().regulator_system.as_ref().unwrap_or(&regulator_reading_profile::REGULATOR_SYSTEM)
    }
    fn regulator_system_mut(&mut self) -> &mut RegulatorSystem {
        self._regulator_reading_profile_mut().regulator_system.get_or_insert(Default::default())
    }
}
impl IsRegulatorReadingProfile for RegulatorReadingProfile {
    fn _regulator_reading_profile(&self) -> &RegulatorReadingProfile {
        self
    }
    fn _regulator_reading_profile_mut(&mut self) -> &mut RegulatorReadingProfile {
        self
    }
}
//impl IsReadingMessageInfo for RegulatorReadingProfile {
    //fn _reading_message_info(&self) -> &ReadingMessageInfo {
        //
    //}
//fn _mut_reading_message_info(&mut self) -> &mut ReadingMessageInfo {
        //
    //}
//}
//impl IsMessageInfo for RegulatorReadingProfile {
    //fn _message_info(&self) -> &MessageInfo {
        //
    //}
//fn _mut_message_info(&mut self) -> &mut MessageInfo {
        //
    //}
//}
//impl IsIdentifiedObject for RegulatorReadingProfile {
    //fn _identified_object(&self) -> &IdentifiedObject {
        //
    //}
//fn _mut_identified_object(&mut self) -> &mut IdentifiedObject {
        //
    //}
//}
/// Regulator status
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct RegulatorStatus {
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
    pub regulator_event_and_status_ancr: ::std::option::Option<RegulatorEventAndStatusAncr>,
}
mod regulator_status {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref STATUS_VALUE: crate::commonmodule::StatusValue = Default::default();
        pub(super) static ref REGULATOR_EVENT_AND_STATUS_ANCR: crate::regulatormodule::RegulatorEventAndStatusAncr = Default::default();
    }
}
pub trait IsRegulatorStatus {
    fn _regulator_status(&self) -> &RegulatorStatus;
    fn _regulator_status_mut(&mut self) -> &mut RegulatorStatus;
    fn status_value(&self) -> &super::commonmodule::StatusValue {
        self._regulator_status().status_value.as_ref().unwrap_or(&regulator_status::STATUS_VALUE)
    }
    fn status_value_mut(&mut self) -> &mut super::commonmodule::StatusValue {
        self._regulator_status_mut().status_value.get_or_insert(Default::default())
    }
    fn regulator_event_and_status_ancr(&self) -> &RegulatorEventAndStatusAncr {
        self._regulator_status().regulator_event_and_status_ancr.as_ref().unwrap_or(&regulator_status::REGULATOR_EVENT_AND_STATUS_ANCR)
    }
    fn regulator_event_and_status_ancr_mut(&mut self) -> &mut RegulatorEventAndStatusAncr {
        self._regulator_status_mut().regulator_event_and_status_ancr.get_or_insert(Default::default())
    }
}
impl IsRegulatorStatus for RegulatorStatus {
    fn _regulator_status(&self) -> &RegulatorStatus {
        self
    }
    fn _regulator_status_mut(&mut self) -> &mut RegulatorStatus {
        self
    }
}
//impl IsStatusValue for RegulatorStatus {
    //fn _status_value(&self) -> &StatusValue {
        //
    //}
//fn _mut_status_value(&mut self) -> &mut StatusValue {
        //
    //}
//}
//impl IsIdentifiedObject for RegulatorStatus {
    //fn _identified_object(&self) -> &IdentifiedObject {
        //
    //}
//fn _mut_identified_object(&mut self) -> &mut IdentifiedObject {
        //
    //}
//}
/// Regulator status profile
/// OpenFMB Profile Message: true
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct RegulatorStatusProfile {
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
    pub regulator_status: ::std::option::Option<RegulatorStatus>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: Some(1)
    // multiplicity_max: None
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
pub trait IsRegulatorStatusProfile {
    fn _regulator_status_profile(&self) -> &RegulatorStatusProfile;
    fn _regulator_status_profile_mut(&mut self) -> &mut RegulatorStatusProfile;
    fn status_message_info(&self) -> &super::commonmodule::StatusMessageInfo {
        self._regulator_status_profile().status_message_info.as_ref().unwrap_or(&regulator_status_profile::STATUS_MESSAGE_INFO)
    }
    fn status_message_info_mut(&mut self) -> &mut super::commonmodule::StatusMessageInfo {
        self._regulator_status_profile_mut().status_message_info.get_or_insert(Default::default())
    }
    fn ied(&self) -> &super::commonmodule::Ied {
        self._regulator_status_profile().ied.as_ref().unwrap_or(&regulator_status_profile::IED)
    }
    fn ied_mut(&mut self) -> &mut super::commonmodule::Ied {
        self._regulator_status_profile_mut().ied.get_or_insert(Default::default())
    }
    fn regulator_status(&self) -> &RegulatorStatus {
        self._regulator_status_profile().regulator_status.as_ref().unwrap_or(&regulator_status_profile::REGULATOR_STATUS)
    }
    fn regulator_status_mut(&mut self) -> &mut RegulatorStatus {
        self._regulator_status_profile_mut().regulator_status.get_or_insert(Default::default())
    }
    fn regulator_system(&self) -> &RegulatorSystem {
        self._regulator_status_profile().regulator_system.as_ref().unwrap_or(&regulator_status_profile::REGULATOR_SYSTEM)
    }
    fn regulator_system_mut(&mut self) -> &mut RegulatorSystem {
        self._regulator_status_profile_mut().regulator_system.get_or_insert(Default::default())
    }
}
impl IsRegulatorStatusProfile for RegulatorStatusProfile {
    fn _regulator_status_profile(&self) -> &RegulatorStatusProfile {
        self
    }
    fn _regulator_status_profile_mut(&mut self) -> &mut RegulatorStatusProfile {
        self
    }
}
//impl IsStatusMessageInfo for RegulatorStatusProfile {
    //fn _status_message_info(&self) -> &StatusMessageInfo {
        //
    //}
//fn _mut_status_message_info(&mut self) -> &mut StatusMessageInfo {
        //
    //}
//}
//impl IsMessageInfo for RegulatorStatusProfile {
    //fn _message_info(&self) -> &MessageInfo {
        //
    //}
//fn _mut_message_info(&mut self) -> &mut MessageInfo {
        //
    //}
//}
//impl IsIdentifiedObject for RegulatorStatusProfile {
    //fn _identified_object(&self) -> &IdentifiedObject {
        //
    //}
//fn _mut_identified_object(&mut self) -> &mut IdentifiedObject {
        //
    //}
//}
