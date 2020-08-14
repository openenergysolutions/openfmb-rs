/// Point definition (Point)
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SolarPoint {
    /// Enable frequency set point
    #[prost(message, optional, tag="1")]
    pub frequency_set_point_enabled: ::std::option::Option<super::commonmodule::ControlDpc>,
    /// Grid connect mode
    #[prost(message, optional, tag="2")]
    pub mode: ::std::option::Option<super::commonmodule::EngGridConnectModeKind>,
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
    /// Enable real power set point
    #[prost(message, optional, tag="7")]
    pub real_pwr_set_point_enabled: ::std::option::Option<super::commonmodule::ControlDpc>,
    /// Reset device
    #[prost(message, optional, tag="8")]
    pub reset: ::std::option::Option<super::commonmodule::ControlDpc>,
    /// ESS state
    #[prost(message, optional, tag="9")]
    pub state: ::std::option::Option<super::commonmodule::OptionalStateKind>,
    /// Enable voltage set point
    #[prost(message, optional, tag="10")]
    pub voltage_set_point_enabled: ::std::option::Option<super::commonmodule::ControlDpc>,
    /// X-axis value (Unix time).
    #[prost(message, optional, tag="11")]
    pub start_time: ::std::option::Option<super::commonmodule::ControlTimestamp>,
}
/// Curve shape setting (FC=SP) (CSG_SP)
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SolarCsg {
    /// The array with the points specifying a curve shape.
    #[prost(message, repeated, tag="1")]
    pub crv_pts: ::std::vec::Vec<SolarPoint>,
}
/// OpenFMB specialization for control schedule using:  LN: Schedule   Name: FSCH
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SolarControlScheduleFsch {
    /// Discrete value in SolarCSG type
    #[prost(message, optional, tag="1")]
    pub val_dcsg: ::std::option::Option<SolarCsg>,
}
/// Specialized 61850 FSCC class.  LN: Schedule controller   Name: FSCC
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SolarControlFscc {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub control_fscc: ::std::option::Option<super::commonmodule::ControlFscc>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub solar_control_schedule_fsch: ::std::option::Option<SolarControlScheduleFsch>,
}
/// Solar control
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SolarControl {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub control_value: ::std::option::Option<super::commonmodule::ControlValue>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub check: ::std::option::Option<super::commonmodule::CheckConditions>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub solar_control_fscc: ::std::option::Option<SolarControlFscc>,
}
/// MISSING DOCUMENTATION!!!
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SolarInverter {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub conducting_equipment: ::std::option::Option<super::commonmodule::ConductingEquipment>,
}
/// Solar control profile
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SolarControlProfile {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub control_message_info: ::std::option::Option<super::commonmodule::ControlMessageInfo>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub ied: ::std::option::Option<super::commonmodule::Ied>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub solar_control: ::std::option::Option<SolarControl>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="4")]
    pub solar_inverter: ::std::option::Option<SolarInverter>,
}
/// Point definition (Point)
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SolarPointStatus {
    /// Enable frequency set point
    #[prost(message, optional, tag="1")]
    pub frequency_set_point_enabled: ::std::option::Option<super::commonmodule::ControlDpc>,
    /// Grid connect mode
    #[prost(message, optional, tag="2")]
    pub mode: ::std::option::Option<super::commonmodule::EngGridConnectModeKind>,
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
    /// Enable real power set point
    #[prost(message, optional, tag="7")]
    pub real_pwr_set_point_enabled: ::std::option::Option<super::commonmodule::ControlDpc>,
    /// ESS state
    #[prost(message, optional, tag="8")]
    pub state: ::std::option::Option<super::commonmodule::OptionalStateKind>,
    /// Enable voltage set point
    #[prost(message, optional, tag="9")]
    pub voltage_set_point_enabled: ::std::option::Option<super::commonmodule::ControlDpc>,
}
/// Specialized 61850 ZGEN class
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SolarEventAndStatusZgen {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub logical_node_for_event_and_status: ::std::option::Option<super::commonmodule::LogicalNodeForEventAndStatus>,
    /// DC Power On/Off Status; True = DC power on
    #[prost(message, optional, tag="2")]
    pub aux_pwr_st: ::std::option::Option<super::commonmodule::StatusSps>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub dynamic_test: ::std::option::Option<super::commonmodule::EnsDynamicTestKind>,
    /// Emergency stop
    #[prost(message, optional, tag="4")]
    pub emg_stop: ::std::option::Option<super::commonmodule::StatusSps>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="5")]
    pub point_status: ::std::option::Option<SolarPointStatus>,
}
/// Specialized 61850 ZGEN class
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SolarEventZgen {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub solar_event_and_status_zgen: ::std::option::Option<SolarEventAndStatusZgen>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub gri_mod: ::std::option::Option<super::commonmodule::EngGridConnectModeKind>,
}
/// Solar event
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SolarEvent {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub event_value: ::std::option::Option<super::commonmodule::EventValue>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub solar_event_zgen: ::std::option::Option<SolarEventZgen>,
}
/// Solar event profile
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SolarEventProfile {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub event_message_info: ::std::option::Option<super::commonmodule::EventMessageInfo>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub ied: ::std::option::Option<super::commonmodule::Ied>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub solar_event: ::std::option::Option<SolarEvent>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="4")]
    pub solar_inverter: ::std::option::Option<SolarInverter>,
}
/// Solar reading value
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SolarReading {
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
/// Solar reading profile
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SolarReadingProfile {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub reading_message_info: ::std::option::Option<super::commonmodule::ReadingMessageInfo>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub ied: ::std::option::Option<super::commonmodule::Ied>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub solar_inverter: ::std::option::Option<SolarInverter>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="4")]
    pub solar_reading: ::std::option::Option<SolarReading>,
}
/// Specialized 61850 ZGEN LN class
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SolarStatusZgen {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub solar_event_and_status_zgen: ::std::option::Option<SolarEventAndStatusZgen>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub gri_mod: ::std::option::Option<super::commonmodule::EngGridConnectModeKind>,
}
/// Solar status
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SolarStatus {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub status_value: ::std::option::Option<super::commonmodule::StatusValue>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub solar_status_zgen: ::std::option::Option<SolarStatusZgen>,
}
/// Solar status profile
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SolarStatusProfile {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub status_message_info: ::std::option::Option<super::commonmodule::StatusMessageInfo>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub ied: ::std::option::Option<super::commonmodule::Ied>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub solar_inverter: ::std::option::Option<SolarInverter>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="4")]
    pub solar_status: ::std::option::Option<SolarStatus>,
}
