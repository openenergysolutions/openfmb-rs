/// MISSING DOCUMENTATION!!!
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct RegulatorControlScheduleProfileList {
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, repeated, tag="1")]
    pub regulator_control_profile: ::std::vec::Vec<super::regulatormodule::RegulatorControlProfile>,
}
mod regulator_control_schedule_profile_list {
    use lazy_static::lazy_static;
    lazy_static! {
    }
}
trait IsRegulatorControlScheduleProfileList {
    fn _regulator_control_schedule_profile_list(&self) -> &RegulatorControlScheduleProfileList;
    fn regulator_control_profile(&self) -> &::std::vec::Vec<super::regulatormodule::RegulatorControlProfile> {
        &self._regulator_control_schedule_profile_list().regulator_control_profile    }
}
impl IsRegulatorControlScheduleProfileList for RegulatorControlScheduleProfileList {
    fn _regulator_control_schedule_profile_list(&self) -> &RegulatorControlScheduleProfileList {
        self
    }
}
/// MISSING DOCUMENTATION!!!
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SolarControlScheduleProfileList {
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, repeated, tag="1")]
    pub solar_control_profile: ::std::vec::Vec<super::solarmodule::SolarControlProfile>,
}
mod solar_control_schedule_profile_list {
    use lazy_static::lazy_static;
    lazy_static! {
    }
}
trait IsSolarControlScheduleProfileList {
    fn _solar_control_schedule_profile_list(&self) -> &SolarControlScheduleProfileList;
    fn solar_control_profile(&self) -> &::std::vec::Vec<super::solarmodule::SolarControlProfile> {
        &self._solar_control_schedule_profile_list().solar_control_profile    }
}
impl IsSolarControlScheduleProfileList for SolarControlScheduleProfileList {
    fn _solar_control_schedule_profile_list(&self) -> &SolarControlScheduleProfileList {
        self
    }
}
/// Planned interconnection schedule profile
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PlannedInterconnectionScheduleProfileList {
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, repeated, tag="1")]
    pub planned_interconnection_schedule_profile: ::std::vec::Vec<super::interconnectionmodule::PlannedInterconnectionScheduleProfile>,
}
mod planned_interconnection_schedule_profile_list {
    use lazy_static::lazy_static;
    lazy_static! {
    }
}
trait IsPlannedInterconnectionScheduleProfileList {
    fn _planned_interconnection_schedule_profile_list(&self) -> &PlannedInterconnectionScheduleProfileList;
    fn planned_interconnection_schedule_profile(&self) -> &::std::vec::Vec<super::interconnectionmodule::PlannedInterconnectionScheduleProfile> {
        &self._planned_interconnection_schedule_profile_list().planned_interconnection_schedule_profile    }
}
impl IsPlannedInterconnectionScheduleProfileList for PlannedInterconnectionScheduleProfileList {
    fn _planned_interconnection_schedule_profile_list(&self) -> &PlannedInterconnectionScheduleProfileList {
        self
    }
}
/// MISSING DOCUMENTATION!!!
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct LoadControlScheduleProfileList {
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, repeated, tag="1")]
    pub load_control_profile: ::std::vec::Vec<super::loadmodule::LoadControlProfile>,
}
mod load_control_schedule_profile_list {
    use lazy_static::lazy_static;
    lazy_static! {
    }
}
trait IsLoadControlScheduleProfileList {
    fn _load_control_schedule_profile_list(&self) -> &LoadControlScheduleProfileList;
    fn load_control_profile(&self) -> &::std::vec::Vec<super::loadmodule::LoadControlProfile> {
        &self._load_control_schedule_profile_list().load_control_profile    }
}
impl IsLoadControlScheduleProfileList for LoadControlScheduleProfileList {
    fn _load_control_schedule_profile_list(&self) -> &LoadControlScheduleProfileList {
        self
    }
}
/// MISSING DOCUMENTATION!!!
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct EssControlScheduleProfileList {
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, repeated, tag="1")]
    pub ess_control_profile: ::std::vec::Vec<super::essmodule::EssControlProfile>,
}
mod ess_control_schedule_profile_list {
    use lazy_static::lazy_static;
    lazy_static! {
    }
}
trait IsEssControlScheduleProfileList {
    fn _ess_control_schedule_profile_list(&self) -> &EssControlScheduleProfileList;
    fn ess_control_profile(&self) -> &::std::vec::Vec<super::essmodule::EssControlProfile> {
        &self._ess_control_schedule_profile_list().ess_control_profile    }
}
impl IsEssControlScheduleProfileList for EssControlScheduleProfileList {
    fn _ess_control_schedule_profile_list(&self) -> &EssControlScheduleProfileList {
        self
    }
}
/// MISSING DOCUMENTATION!!!
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SwitchControlScheduleProfileList {
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, repeated, tag="1")]
    pub switch_control_profile: ::std::vec::Vec<super::switchmodule::SwitchControlProfile>,
}
mod switch_control_schedule_profile_list {
    use lazy_static::lazy_static;
    lazy_static! {
    }
}
trait IsSwitchControlScheduleProfileList {
    fn _switch_control_schedule_profile_list(&self) -> &SwitchControlScheduleProfileList;
    fn switch_control_profile(&self) -> &::std::vec::Vec<super::switchmodule::SwitchControlProfile> {
        &self._switch_control_schedule_profile_list().switch_control_profile    }
}
impl IsSwitchControlScheduleProfileList for SwitchControlScheduleProfileList {
    fn _switch_control_schedule_profile_list(&self) -> &SwitchControlScheduleProfileList {
        self
    }
}
/// MISSING DOCUMENTATION!!!
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct GenerationControlScheduleProfileList {
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, repeated, tag="1")]
    pub generation_control_profile: ::std::vec::Vec<super::generationmodule::GenerationControlProfile>,
}
mod generation_control_schedule_profile_list {
    use lazy_static::lazy_static;
    lazy_static! {
    }
}
trait IsGenerationControlScheduleProfileList {
    fn _generation_control_schedule_profile_list(&self) -> &GenerationControlScheduleProfileList;
    fn generation_control_profile(&self) -> &::std::vec::Vec<super::generationmodule::GenerationControlProfile> {
        &self._generation_control_schedule_profile_list().generation_control_profile    }
}
impl IsGenerationControlScheduleProfileList for GenerationControlScheduleProfileList {
    fn _generation_control_schedule_profile_list(&self) -> &GenerationControlScheduleProfileList {
        self
    }
}
/// MISSING DOCUMENTATION!!!
/// OpenFMB Profile Message: true
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PlannedOptimizerScheduleProfile {
    /// UML inherited base object
    // parent_message: true
    // required_field: false
    // multiplicity_min: 0
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="1")]
    pub optimization_message_info: ::std::option::Option<super::commonmodule::OptimizationMessageInfo>,
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
mod planned_optimizer_schedule_profile {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref OPTIMIZATION_MESSAGE_INFO: crate::commonmodule::OptimizationMessageInfo = Default::default();
        pub(super) static ref APPLICATION_SYSTEM: crate::commonmodule::ApplicationSystem = Default::default();
        pub(super) static ref ESS_CONTROL_SCHEDULE_PROFILE_LIST: crate::optimizermodule::EssControlScheduleProfileList = Default::default();
        pub(super) static ref GENERATION_CONTROL_SCHEDULE_PROFILE_LIST: crate::optimizermodule::GenerationControlScheduleProfileList = Default::default();
        pub(super) static ref LOAD_CONTROL_SCHEDULE_PROFILE_LIST: crate::optimizermodule::LoadControlScheduleProfileList = Default::default();
        pub(super) static ref PLANNED_INTERCONNECTION_SCHEDULE_PROFILE_LIST: crate::optimizermodule::PlannedInterconnectionScheduleProfileList = Default::default();
        pub(super) static ref REGULATOR_CONTROL_SCHEDULE_PROFILE_LIST: crate::optimizermodule::RegulatorControlScheduleProfileList = Default::default();
        pub(super) static ref SOLAR_CONTROL_SCHEDULE_PROFILE_LIST: crate::optimizermodule::SolarControlScheduleProfileList = Default::default();
        pub(super) static ref SWITCH_CONTROL_SCHEDULE_PROFILE_LIST: crate::optimizermodule::SwitchControlScheduleProfileList = Default::default();
    }
}
trait IsPlannedOptimizerScheduleProfile {
    fn _planned_optimizer_schedule_profile(&self) -> &PlannedOptimizerScheduleProfile;
    fn optimization_message_info(&self) -> &super::commonmodule::OptimizationMessageInfo {
        self._planned_optimizer_schedule_profile().optimization_message_info.as_ref().unwrap_or(&planned_optimizer_schedule_profile::OPTIMIZATION_MESSAGE_INFO)
    }
    fn application_system(&self) -> &super::commonmodule::ApplicationSystem {
        self._planned_optimizer_schedule_profile().application_system.as_ref().unwrap_or(&planned_optimizer_schedule_profile::APPLICATION_SYSTEM)
    }
    fn ess_control_schedule_profile_list(&self) -> &EssControlScheduleProfileList {
        self._planned_optimizer_schedule_profile().ess_control_schedule_profile_list.as_ref().unwrap_or(&planned_optimizer_schedule_profile::ESS_CONTROL_SCHEDULE_PROFILE_LIST)
    }
    fn generation_control_schedule_profile_list(&self) -> &GenerationControlScheduleProfileList {
        self._planned_optimizer_schedule_profile().generation_control_schedule_profile_list.as_ref().unwrap_or(&planned_optimizer_schedule_profile::GENERATION_CONTROL_SCHEDULE_PROFILE_LIST)
    }
    fn load_control_schedule_profile_list(&self) -> &LoadControlScheduleProfileList {
        self._planned_optimizer_schedule_profile().load_control_schedule_profile_list.as_ref().unwrap_or(&planned_optimizer_schedule_profile::LOAD_CONTROL_SCHEDULE_PROFILE_LIST)
    }
    fn planned_interconnection_schedule_profile_list(&self) -> &PlannedInterconnectionScheduleProfileList {
        self._planned_optimizer_schedule_profile().planned_interconnection_schedule_profile_list.as_ref().unwrap_or(&planned_optimizer_schedule_profile::PLANNED_INTERCONNECTION_SCHEDULE_PROFILE_LIST)
    }
    fn regulator_control_schedule_profile_list(&self) -> &RegulatorControlScheduleProfileList {
        self._planned_optimizer_schedule_profile().regulator_control_schedule_profile_list.as_ref().unwrap_or(&planned_optimizer_schedule_profile::REGULATOR_CONTROL_SCHEDULE_PROFILE_LIST)
    }
    fn solar_control_schedule_profile_list(&self) -> &SolarControlScheduleProfileList {
        self._planned_optimizer_schedule_profile().solar_control_schedule_profile_list.as_ref().unwrap_or(&planned_optimizer_schedule_profile::SOLAR_CONTROL_SCHEDULE_PROFILE_LIST)
    }
    fn switch_control_schedule_profile_list(&self) -> &SwitchControlScheduleProfileList {
        self._planned_optimizer_schedule_profile().switch_control_schedule_profile_list.as_ref().unwrap_or(&planned_optimizer_schedule_profile::SWITCH_CONTROL_SCHEDULE_PROFILE_LIST)
    }
}
impl IsPlannedOptimizerScheduleProfile for PlannedOptimizerScheduleProfile {
    fn _planned_optimizer_schedule_profile(&self) -> &PlannedOptimizerScheduleProfile {
        self
    }
}
/// MISSING DOCUMENTATION!!!
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct RequestedInterconnectionScheduleProfileList {
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, repeated, tag="1")]
    pub requested_interconnection_schedule_profile: ::std::vec::Vec<super::interconnectionmodule::RequestedInterconnectionScheduleProfile>,
}
mod requested_interconnection_schedule_profile_list {
    use lazy_static::lazy_static;
    lazy_static! {
    }
}
trait IsRequestedInterconnectionScheduleProfileList {
    fn _requested_interconnection_schedule_profile_list(&self) -> &RequestedInterconnectionScheduleProfileList;
    fn requested_interconnection_schedule_profile(&self) -> &::std::vec::Vec<super::interconnectionmodule::RequestedInterconnectionScheduleProfile> {
        &self._requested_interconnection_schedule_profile_list().requested_interconnection_schedule_profile    }
}
impl IsRequestedInterconnectionScheduleProfileList for RequestedInterconnectionScheduleProfileList {
    fn _requested_interconnection_schedule_profile_list(&self) -> &RequestedInterconnectionScheduleProfileList {
        self
    }
}
/// MISSING DOCUMENTATION!!!
/// OpenFMB Profile Message: true
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct RequestedOptimizerScheduleProfile {
    /// UML inherited base object
    // parent_message: true
    // required_field: false
    // multiplicity_min: 0
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="1")]
    pub optimization_message_info: ::std::option::Option<super::commonmodule::OptimizationMessageInfo>,
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
mod requested_optimizer_schedule_profile {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref OPTIMIZATION_MESSAGE_INFO: crate::commonmodule::OptimizationMessageInfo = Default::default();
        pub(super) static ref APPLICATION_SYSTEM: crate::commonmodule::ApplicationSystem = Default::default();
        pub(super) static ref ESS_CONTROL_SCHEDULE_PROFILE_LIST: crate::optimizermodule::EssControlScheduleProfileList = Default::default();
        pub(super) static ref GENERATION_CONTROL_SCHEDULE_PROFILE_LIST: crate::optimizermodule::GenerationControlScheduleProfileList = Default::default();
        pub(super) static ref LOAD_CONTROL_SCHEDULE_PROFILE_LIST: crate::optimizermodule::LoadControlScheduleProfileList = Default::default();
        pub(super) static ref REGULATOR_CONTROL_SCHEDULE_PROFILE_LIST: crate::optimizermodule::RegulatorControlScheduleProfileList = Default::default();
        pub(super) static ref REQUESTED_INTERCONNECTION_SCHEDULE_PROFILE_LIST: crate::optimizermodule::RequestedInterconnectionScheduleProfileList = Default::default();
        pub(super) static ref SOLAR_CONTROL_SCHEDULE_PROFILE_LIST: crate::optimizermodule::SolarControlScheduleProfileList = Default::default();
        pub(super) static ref SWITCH_CONTROL_SCHEDULE_PROFILE_LIST: crate::optimizermodule::SwitchControlScheduleProfileList = Default::default();
    }
}
trait IsRequestedOptimizerScheduleProfile {
    fn _requested_optimizer_schedule_profile(&self) -> &RequestedOptimizerScheduleProfile;
    fn optimization_message_info(&self) -> &super::commonmodule::OptimizationMessageInfo {
        self._requested_optimizer_schedule_profile().optimization_message_info.as_ref().unwrap_or(&requested_optimizer_schedule_profile::OPTIMIZATION_MESSAGE_INFO)
    }
    fn application_system(&self) -> &super::commonmodule::ApplicationSystem {
        self._requested_optimizer_schedule_profile().application_system.as_ref().unwrap_or(&requested_optimizer_schedule_profile::APPLICATION_SYSTEM)
    }
    fn ess_control_schedule_profile_list(&self) -> &EssControlScheduleProfileList {
        self._requested_optimizer_schedule_profile().ess_control_schedule_profile_list.as_ref().unwrap_or(&requested_optimizer_schedule_profile::ESS_CONTROL_SCHEDULE_PROFILE_LIST)
    }
    fn generation_control_schedule_profile_list(&self) -> &GenerationControlScheduleProfileList {
        self._requested_optimizer_schedule_profile().generation_control_schedule_profile_list.as_ref().unwrap_or(&requested_optimizer_schedule_profile::GENERATION_CONTROL_SCHEDULE_PROFILE_LIST)
    }
    fn load_control_schedule_profile_list(&self) -> &LoadControlScheduleProfileList {
        self._requested_optimizer_schedule_profile().load_control_schedule_profile_list.as_ref().unwrap_or(&requested_optimizer_schedule_profile::LOAD_CONTROL_SCHEDULE_PROFILE_LIST)
    }
    fn regulator_control_schedule_profile_list(&self) -> &RegulatorControlScheduleProfileList {
        self._requested_optimizer_schedule_profile().regulator_control_schedule_profile_list.as_ref().unwrap_or(&requested_optimizer_schedule_profile::REGULATOR_CONTROL_SCHEDULE_PROFILE_LIST)
    }
    fn requested_interconnection_schedule_profile_list(&self) -> &RequestedInterconnectionScheduleProfileList {
        self._requested_optimizer_schedule_profile().requested_interconnection_schedule_profile_list.as_ref().unwrap_or(&requested_optimizer_schedule_profile::REQUESTED_INTERCONNECTION_SCHEDULE_PROFILE_LIST)
    }
    fn solar_control_schedule_profile_list(&self) -> &SolarControlScheduleProfileList {
        self._requested_optimizer_schedule_profile().solar_control_schedule_profile_list.as_ref().unwrap_or(&requested_optimizer_schedule_profile::SOLAR_CONTROL_SCHEDULE_PROFILE_LIST)
    }
    fn switch_control_schedule_profile_list(&self) -> &SwitchControlScheduleProfileList {
        self._requested_optimizer_schedule_profile().switch_control_schedule_profile_list.as_ref().unwrap_or(&requested_optimizer_schedule_profile::SWITCH_CONTROL_SCHEDULE_PROFILE_LIST)
    }
}
impl IsRequestedOptimizerScheduleProfile for RequestedOptimizerScheduleProfile {
    fn _requested_optimizer_schedule_profile(&self) -> &RequestedOptimizerScheduleProfile {
        self
    }
}
