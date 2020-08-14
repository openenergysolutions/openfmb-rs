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
    #[prost(message, optional, tag="13")]
    pub start_time: ::std::option::Option<super::commonmodule::ControlTimestamp>,
}
/// Curve shape setting (FC=SP) (CSG_SP)
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct GenerationCsg {
    /// The array with the points specifying a curve shape.
    #[prost(message, repeated, tag="1")]
    pub crv_pts: ::std::vec::Vec<GenerationPoint>,
}
/// OpenFMB specialization for control schedule using:  LN: Schedule   Name: FSCH
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct GenerationControlScheduleFsch {
    /// Discrete value in GenerationCSG type
    #[prost(message, optional, tag="1")]
    pub val_dcsg: ::std::option::Option<GenerationCsg>,
}
/// LN: Schedule controller   Name: FSCC
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct GenerationControlFscc {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub control_fscc: ::std::option::Option<super::commonmodule::ControlFscc>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub generation_control_schedule_fsch: ::std::option::Option<GenerationControlScheduleFsch>,
}
/// Generation control
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct GenerationControl {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub control_value: ::std::option::Option<super::commonmodule::ControlValue>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub check: ::std::option::Option<super::commonmodule::CheckConditions>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub generation_control_fscc: ::std::option::Option<GenerationControlFscc>,
}
/// A single or set of synchronous machines for converting mechanical power into alternating-current
/// power. For example, individual machines within a set may be defined for scheduling purposes while a
/// single control signal is derived for the set. In this case there would be a GeneratingUnit for each
/// member of the set and an additional GeneratingUnit corresponding to the set.
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct GeneratingUnit {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub conducting_equipment: ::std::option::Option<super::commonmodule::ConductingEquipment>,
    /// This is the maximum operating active power limit the dispatcher can enter for this unit.
    #[prost(message, optional, tag="2")]
    pub max_operating_p: ::std::option::Option<super::commonmodule::ActivePower>,
}
/// Generation control profile
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct GenerationControlProfile {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub control_message_info: ::std::option::Option<super::commonmodule::ControlMessageInfo>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub generating_unit: ::std::option::Option<GeneratingUnit>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub generation_control: ::std::option::Option<GenerationControl>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="4")]
    pub ied: ::std::option::Option<super::commonmodule::Ied>,
}
/// Generation reading value
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct GenerationReading {
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
/// Generation reading profile
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct GenerationReadingProfile {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub reading_message_info: ::std::option::Option<super::commonmodule::ReadingMessageInfo>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub generating_unit: ::std::option::Option<GeneratingUnit>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub generation_reading: ::std::option::Option<GenerationReading>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="4")]
    pub ied: ::std::option::Option<super::commonmodule::Ied>,
}
/// Point definition (Point)
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct GenerationPointStatus {
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
    /// Enable real power set point
    #[prost(message, optional, tag="7")]
    pub real_pwr_set_point_enabled: ::std::option::Option<super::commonmodule::ControlDpc>,
    /// ESS state
    #[prost(message, optional, tag="8")]
    pub state: ::std::option::Option<super::commonmodule::OptionalStateKind>,
    /// Synchronize back to grid
    #[prost(message, optional, tag="9")]
    pub sync_back_to_grid: ::std::option::Option<super::commonmodule::ControlDpc>,
    /// Transition to island on grid loss enable
    #[prost(message, optional, tag="10")]
    pub trans_to_islnd_on_grid_loss_enabled: ::std::option::Option<super::commonmodule::ControlDpc>,
    /// Enable voltage set point
    #[prost(message, optional, tag="11")]
    pub voltage_set_point_enabled: ::std::option::Option<super::commonmodule::ControlDpc>,
}
/// Specialized 61850 ZGEN class
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct GenerationEventAndStatusZgen {
    /// UML inherited base object
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
/// Specialized generation event ZGEN
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct GenerationEventZgen {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub generation_event_and_status_zgen: ::std::option::Option<GenerationEventAndStatusZgen>,
}
/// Generation event
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct GenerationEvent {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub event_value: ::std::option::Option<super::commonmodule::EventValue>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub generation_event_zgen: ::std::option::Option<GenerationEventZgen>,
}
/// Generation event profile
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct GenerationEventProfile {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub event_message_info: ::std::option::Option<super::commonmodule::EventMessageInfo>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub generating_unit: ::std::option::Option<GeneratingUnit>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub generation_event: ::std::option::Option<GenerationEvent>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="4")]
    pub ied: ::std::option::Option<super::commonmodule::Ied>,
}
/// Specialized 61850 ZGEN class
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct GenerationStatusZgen {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub generation_event_and_status_zgen: ::std::option::Option<GenerationEventAndStatusZgen>,
}
/// Generation status
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct GenerationStatus {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub status_value: ::std::option::Option<super::commonmodule::StatusValue>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub generation_status_zgen: ::std::option::Option<GenerationStatusZgen>,
}
/// Generation status profile
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct GenerationStatusProfile {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub status_message_info: ::std::option::Option<super::commonmodule::StatusMessageInfo>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub generating_unit: ::std::option::Option<GeneratingUnit>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub generation_status: ::std::option::Option<GenerationStatus>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="4")]
    pub ied: ::std::option::Option<super::commonmodule::Ied>,
}
