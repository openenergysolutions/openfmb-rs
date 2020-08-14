/// Point definition (Point)
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct LoadPoint {
    /// Ramp rates
    #[prost(message, optional, tag="1")]
    pub ramp_rates: ::std::option::Option<super::commonmodule::RampRate>,
    /// Enable reactive power set point
    #[prost(message, optional, tag="2")]
    pub reactive_pwr_set_point_enabled: ::std::option::Option<super::commonmodule::ControlDpc>,
    /// Enable joint real power set point
    #[prost(message, optional, tag="3")]
    pub real_pwr_set_point_enabled: ::std::option::Option<super::commonmodule::ControlDpc>,
    /// Reset device
    #[prost(message, optional, tag="4")]
    pub reset: ::std::option::Option<super::commonmodule::ControlDpc>,
    /// ESS state
    #[prost(message, optional, tag="5")]
    pub state: ::std::option::Option<super::commonmodule::OptionalStateKind>,
    /// Start time
    #[prost(message, optional, tag="6")]
    pub start_time: ::std::option::Option<super::commonmodule::ControlTimestamp>,
}
/// Curve shape setting (FC=SP) (CSG_SP)
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct LoadCsg {
    /// The array with the points specifying a curve shape.
    #[prost(message, repeated, tag="1")]
    pub crv_pts: ::std::vec::Vec<LoadPoint>,
}
/// OpenFMB specialization for control schedule using:  LN: Schedule   Name: FSCH
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct LoadControlScheduleFsch {
    /// Discrete value in LoadCSG type
    #[prost(message, optional, tag="1")]
    pub val_dcsg: ::std::option::Option<LoadCsg>,
}
/// Specialized FSCC 61850 class.  LN: Schedule controller   Name: FSCC
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct LoadControlFscc {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub control_fscc: ::std::option::Option<super::commonmodule::ControlFscc>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub load_control_schedule_fsch: ::std::option::Option<LoadControlScheduleFsch>,
}
/// Load control
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct LoadControl {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub control_value: ::std::option::Option<super::commonmodule::ControlValue>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub check: ::std::option::Option<super::commonmodule::CheckConditions>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub load_control_fscc: ::std::option::Option<LoadControlFscc>,
}
/// Load control profile
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct LoadControlProfile {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub control_message_info: ::std::option::Option<super::commonmodule::ControlMessageInfo>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub energy_consumer: ::std::option::Option<super::commonmodule::EnergyConsumer>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub ied: ::std::option::Option<super::commonmodule::Ied>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="4")]
    pub load_control: ::std::option::Option<LoadControl>,
}
/// Point definition (Point)
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct LoadPointStatus {
    /// Ramp rates
    #[prost(message, optional, tag="1")]
    pub ramp_rates: ::std::option::Option<super::commonmodule::RampRate>,
    /// Enable reactive power set point
    #[prost(message, optional, tag="2")]
    pub reactive_pwr_set_point_enabled: ::std::option::Option<super::commonmodule::ControlDpc>,
    /// Enable joint real power set point
    #[prost(message, optional, tag="3")]
    pub real_pwr_set_point_enabled: ::std::option::Option<super::commonmodule::ControlDpc>,
    /// Reset device
    #[prost(message, optional, tag="4")]
    pub reset: ::std::option::Option<super::commonmodule::ControlDpc>,
    /// ESS state
    #[prost(message, optional, tag="5")]
    pub state: ::std::option::Option<super::commonmodule::OptionalStateKind>,
}
/// Specialized 61850 ZGLD class
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct LoadEventAndStatusZgld {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub logical_node_for_event_and_status: ::std::option::Option<super::commonmodule::LogicalNodeForEventAndStatus>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub dynamic_test: ::std::option::Option<super::commonmodule::EnsDynamicTestKind>,
    /// Emergency stop
    #[prost(message, optional, tag="3")]
    pub emg_stop: ::std::option::Option<super::commonmodule::StatusSps>,
    /// Point status
    #[prost(message, optional, tag="4")]
    pub point_status: ::std::option::Option<LoadPointStatus>,
}
/// Specialized 61850 ZGLD LN class
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct LoadEventZgld {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub load_event_and_status_zgld: ::std::option::Option<LoadEventAndStatusZgld>,
}
/// Load event
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct LoadEvent {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub event_value: ::std::option::Option<super::commonmodule::EventValue>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub load_event_zgld: ::std::option::Option<LoadEventZgld>,
}
/// Load event profile
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct LoadEventProfile {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub event_message_info: ::std::option::Option<super::commonmodule::EventMessageInfo>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub energy_consumer: ::std::option::Option<super::commonmodule::EnergyConsumer>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub ied: ::std::option::Option<super::commonmodule::Ied>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="4")]
    pub load_event: ::std::option::Option<LoadEvent>,
}
/// Load reading value
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct LoadReading {
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
/// Load reading profile
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct LoadReadingProfile {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub reading_message_info: ::std::option::Option<super::commonmodule::ReadingMessageInfo>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub energy_consumer: ::std::option::Option<super::commonmodule::EnergyConsumer>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub ied: ::std::option::Option<super::commonmodule::Ied>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="4")]
    pub load_reading: ::std::option::Option<LoadReading>,
}
/// Specialized 61850 ZGLD LN class
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct LoadStatusZgld {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub load_event_and_status_zgld: ::std::option::Option<LoadEventAndStatusZgld>,
}
/// Load status
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct LoadStatus {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub status_value: ::std::option::Option<super::commonmodule::StatusValue>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub load_status_zgld: ::std::option::Option<LoadStatusZgld>,
}
/// Load status profile
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct LoadStatusProfile {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub status_message_info: ::std::option::Option<super::commonmodule::StatusMessageInfo>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub energy_consumer: ::std::option::Option<super::commonmodule::EnergyConsumer>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub ied: ::std::option::Option<super::commonmodule::Ied>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="4")]
    pub load_status: ::std::option::Option<LoadStatus>,
}
