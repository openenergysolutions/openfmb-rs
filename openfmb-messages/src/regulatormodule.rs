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
    #[prost(message, optional, tag="8")]
    pub start_time: ::std::option::Option<super::commonmodule::Timestamp>,
}
/// Curve shape setting (FC=SP) (CSG_SP)
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct RegulatorCsg {
    /// The array with the points specifying a curve shape.
    #[prost(message, repeated, tag="1")]
    pub crv_pts: ::std::vec::Vec<RegulatorPoint>,
}
/// OpenFMB specialization for control schedule using:  LN: Schedule   Name: FSCH
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct RegulatorControlScheduleFsch {
    /// Discrete value in RegulatorCSG type
    #[prost(message, optional, tag="1")]
    pub val_dcsg: ::std::option::Option<RegulatorCsg>,
}
/// Using 61850 FSCC for regulator control
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct RegulatorControlFscc {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub control_fscc: ::std::option::Option<super::commonmodule::ControlFscc>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub regulator_control_schedule_fsch: ::std::option::Option<RegulatorControlScheduleFsch>,
}
/// Regulator control
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct RegulatorControl {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub control_value: ::std::option::Option<super::commonmodule::ControlValue>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub check: ::std::option::Option<super::commonmodule::CheckConditions>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub regulator_control_fscc: ::std::option::Option<RegulatorControlFscc>,
}
/// Pole-mounted fault interrupter with built-in phase and ground relays, current transformer (CT),
/// and supplemental controls.
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct RegulatorSystem {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub conducting_equipment: ::std::option::Option<super::commonmodule::ConductingEquipment>,
}
/// Regulator control profile.  Instructs an end device (or an end device group) to perform a
/// specified action.
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct RegulatorControlProfile {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub control_message_info: ::std::option::Option<super::commonmodule::ControlMessageInfo>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub ied: ::std::option::Option<super::commonmodule::Ied>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub regulator_control: ::std::option::Option<RegulatorControl>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="4")]
    pub regulator_system: ::std::option::Option<RegulatorSystem>,
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
/// OpenFMB 61850 specialization for both RegulatorEventProfile and RegulatorStatusProfile
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct RegulatorEventAndStatusAncr {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub logical_node_for_event_and_status: ::std::option::Option<super::commonmodule::LogicalNodeForEventAndStatus>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub dynamic_test: ::std::option::Option<super::commonmodule::EnsDynamicTestKind>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub point_status: ::std::option::Option<RegulatorEventAndStatusPoint>,
}
/// Regulator event
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct RegulatorEvent {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub event_value: ::std::option::Option<super::commonmodule::EventValue>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub regulator_event_and_status_ancr: ::std::option::Option<RegulatorEventAndStatusAncr>,
}
/// Regulator event profile
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct RegulatorEventProfile {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub event_message_info: ::std::option::Option<super::commonmodule::EventMessageInfo>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub ied: ::std::option::Option<super::commonmodule::Ied>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub regulator_event: ::std::option::Option<RegulatorEvent>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="4")]
    pub regulator_system: ::std::option::Option<RegulatorSystem>,
}
/// Regulator reading value
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct RegulatorReading {
    /// UML inherited base object
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
/// Regulator reading profile
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct RegulatorReadingProfile {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub reading_message_info: ::std::option::Option<super::commonmodule::ReadingMessageInfo>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub ied: ::std::option::Option<super::commonmodule::Ied>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, repeated, tag="3")]
    pub regulator_reading: ::std::vec::Vec<RegulatorReading>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="4")]
    pub regulator_system: ::std::option::Option<RegulatorSystem>,
}
/// Regulator status
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct RegulatorStatus {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub status_value: ::std::option::Option<super::commonmodule::StatusValue>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub regulator_event_and_status_ancr: ::std::option::Option<RegulatorEventAndStatusAncr>,
}
/// Regulator status profile
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct RegulatorStatusProfile {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub status_message_info: ::std::option::Option<super::commonmodule::StatusMessageInfo>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub ied: ::std::option::Option<super::commonmodule::Ied>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub regulator_status: ::std::option::Option<RegulatorStatus>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="4")]
    pub regulator_system: ::std::option::Option<RegulatorSystem>,
}
