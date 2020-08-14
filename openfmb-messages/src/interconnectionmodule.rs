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
    #[prost(message, optional, tag="10")]
    pub start_time: ::std::option::Option<super::commonmodule::Timestamp>,
}
/// Curve shape setting (FC=SP) (CSG_SP)
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct InterconnectionCsg {
    /// The array with the points specifying a curve shape.
    #[prost(message, repeated, tag="1")]
    pub crv_pts: ::std::vec::Vec<InterconnectionPoint>,
}
/// OpenFMB specialization for control schedule using:  LN: Schedule   Name: FSCH
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct InterconnectionControlScheduleFsch {
    /// Discrete value in InterconnectionCSG type
    #[prost(message, optional, tag="1")]
    pub val_dcsg: ::std::option::Option<InterconnectionCsg>,
}
/// Specialized 61850 FSCC class for interconnection schedule
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct InterconnectionScheduleFscc {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub control_fscc: ::std::option::Option<super::commonmodule::ControlFscc>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, repeated, tag="2")]
    pub interconnection_control_schedule_fsch: ::std::vec::Vec<InterconnectionControlScheduleFsch>,
}
/// Interconnection schedule
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct InterconnectionSchedule {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub control_value: ::std::option::Option<super::commonmodule::ControlValue>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub check: ::std::option::Option<super::commonmodule::CheckConditions>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub interconnection_schedule_fscc: ::std::option::Option<InterconnectionScheduleFscc>,
}
/// Planned interconnection schedule profile
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PlannedInterconnectionScheduleProfile {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub control_message_info: ::std::option::Option<super::commonmodule::ControlMessageInfo>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub application_system: ::std::option::Option<super::commonmodule::ApplicationSystem>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub interconnection_schedule: ::std::option::Option<InterconnectionSchedule>,
}
/// Requested interconnection schedule profile
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct RequestedInterconnectionScheduleProfile {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub control_message_info: ::std::option::Option<super::commonmodule::ControlMessageInfo>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub application_system: ::std::option::Option<super::commonmodule::ApplicationSystem>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub interconnection_schedule: ::std::option::Option<InterconnectionSchedule>,
}
