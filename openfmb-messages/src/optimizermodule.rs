/// MISSING DOCUMENTATION!!!
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct RegulatorControlScheduleProfileList {
    /// MISSING DOCUMENTATION!!!
    #[prost(message, repeated, tag="1")]
    pub regulator_control_profile: ::std::vec::Vec<super::regulatormodule::RegulatorControlProfile>,
}
/// MISSING DOCUMENTATION!!!
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SolarControlScheduleProfileList {
    /// MISSING DOCUMENTATION!!!
    #[prost(message, repeated, tag="1")]
    pub solar_control_profile: ::std::vec::Vec<super::solarmodule::SolarControlProfile>,
}
/// Planned interconnection schedule profile
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PlannedInterconnectionScheduleProfileList {
    /// MISSING DOCUMENTATION!!!
    #[prost(message, repeated, tag="1")]
    pub planned_interconnection_schedule_profile: ::std::vec::Vec<super::interconnectionmodule::PlannedInterconnectionScheduleProfile>,
}
/// MISSING DOCUMENTATION!!!
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct LoadControlScheduleProfileList {
    /// MISSING DOCUMENTATION!!!
    #[prost(message, repeated, tag="1")]
    pub load_control_profile: ::std::vec::Vec<super::loadmodule::LoadControlProfile>,
}
/// MISSING DOCUMENTATION!!!
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct EssControlScheduleProfileList {
    /// MISSING DOCUMENTATION!!!
    #[prost(message, repeated, tag="1")]
    pub ess_control_profile: ::std::vec::Vec<super::essmodule::EssControlProfile>,
}
/// MISSING DOCUMENTATION!!!
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SwitchControlScheduleProfileList {
    /// MISSING DOCUMENTATION!!!
    #[prost(message, repeated, tag="1")]
    pub switch_control_profile: ::std::vec::Vec<super::switchmodule::SwitchControlProfile>,
}
/// MISSING DOCUMENTATION!!!
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct GenerationControlScheduleProfileList {
    /// MISSING DOCUMENTATION!!!
    #[prost(message, repeated, tag="1")]
    pub generation_control_profile: ::std::vec::Vec<super::generationmodule::GenerationControlProfile>,
}
/// MISSING DOCUMENTATION!!!
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PlannedOptimizerScheduleProfile {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub optimization_message_info: ::std::option::Option<super::commonmodule::OptimizationMessageInfo>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub application_system: ::std::option::Option<super::commonmodule::ApplicationSystem>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub ess_control_schedule_profile_list: ::std::option::Option<EssControlScheduleProfileList>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="4")]
    pub generation_control_schedule_profile_list: ::std::option::Option<GenerationControlScheduleProfileList>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="5")]
    pub load_control_schedule_profile_list: ::std::option::Option<LoadControlScheduleProfileList>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="6")]
    pub planned_interconnection_schedule_profile_list: ::std::option::Option<PlannedInterconnectionScheduleProfileList>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="7")]
    pub regulator_control_schedule_profile_list: ::std::option::Option<RegulatorControlScheduleProfileList>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="8")]
    pub solar_control_schedule_profile_list: ::std::option::Option<SolarControlScheduleProfileList>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="9")]
    pub switch_control_schedule_profile_list: ::std::option::Option<SwitchControlScheduleProfileList>,
}
/// MISSING DOCUMENTATION!!!
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct RequestedInterconnectionScheduleProfileList {
    /// MISSING DOCUMENTATION!!!
    #[prost(message, repeated, tag="1")]
    pub requested_interconnection_schedule_profile: ::std::vec::Vec<super::interconnectionmodule::RequestedInterconnectionScheduleProfile>,
}
/// MISSING DOCUMENTATION!!!
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct RequestedOptimizerScheduleProfile {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub optimization_message_info: ::std::option::Option<super::commonmodule::OptimizationMessageInfo>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub application_system: ::std::option::Option<super::commonmodule::ApplicationSystem>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub ess_control_schedule_profile_list: ::std::option::Option<EssControlScheduleProfileList>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="4")]
    pub generation_control_schedule_profile_list: ::std::option::Option<GenerationControlScheduleProfileList>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="5")]
    pub load_control_schedule_profile_list: ::std::option::Option<LoadControlScheduleProfileList>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="6")]
    pub regulator_control_schedule_profile_list: ::std::option::Option<RegulatorControlScheduleProfileList>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="7")]
    pub requested_interconnection_schedule_profile_list: ::std::option::Option<RequestedInterconnectionScheduleProfileList>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="8")]
    pub solar_control_schedule_profile_list: ::std::option::Option<SolarControlScheduleProfileList>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="9")]
    pub switch_control_schedule_profile_list: ::std::option::Option<SwitchControlScheduleProfileList>,
}
