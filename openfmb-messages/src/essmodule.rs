use crate::commonmodule::*;
/// Specialized 61850 ZBAT class  LN: Battery   Name: ZBAT
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize)]
pub struct EssEventZbat {
    /// UML inherited base object
    // parent_message: true
    // required_field: false
    // multiplicity_min: None
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, optional, tag="1")]
    #[serde(default, rename = "logicalNodeForEventAndStatus")]
    pub logical_node_for_event_and_status: ::std::option::Option<super::commonmodule::LogicalNodeForEventAndStatus>,
    /// If true, the battery is in overcharge (voltage or current) condition.
    #[prost(message, optional, tag="2")]
    #[serde(default, rename = "BatHi")]
    pub bat_hi: ::std::option::Option<super::commonmodule::StatusSps>,
    /// If true, the battery voltage or charge has dropped below a pre-set level.
    #[prost(message, optional, tag="3")]
    #[serde(default, rename = "BatLo")]
    pub bat_lo: ::std::option::Option<super::commonmodule::StatusSps>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="4")]
    #[serde(default, rename = "BatSt")]
    pub bat_st: ::std::option::Option<super::commonmodule::StatusSps>,
    /// State of charge (in percentage)
    #[prost(message, optional, tag="5")]
    #[serde(default, rename = "Soc")]
    pub soc: ::std::option::Option<super::commonmodule::Mv>,
    /// If stVal TRUE, the device is in standby.
    #[prost(message, optional, tag="6")]
    #[serde(default, rename = "Stdby")]
    pub stdby: ::std::option::Option<super::commonmodule::StatusSps>,
}
mod ess_event_zbat {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref LOGICAL_NODE_FOR_EVENT_AND_STATUS: crate::commonmodule::LogicalNodeForEventAndStatus = Default::default();
        pub(super) static ref BAT_HI: crate::commonmodule::StatusSps = Default::default();
        pub(super) static ref BAT_LO: crate::commonmodule::StatusSps = Default::default();
        pub(super) static ref BAT_ST: crate::commonmodule::StatusSps = Default::default();
        pub(super) static ref SOC: crate::commonmodule::Mv = Default::default();
        pub(super) static ref STDBY: crate::commonmodule::StatusSps = Default::default();
    }
}
impl EssEventZbat {
    pub(crate) fn parent(&self) -> &super::commonmodule::LogicalNodeForEventAndStatus {
        self.logical_node_for_event_and_status.as_ref().unwrap_or(&ess_event_zbat::LOGICAL_NODE_FOR_EVENT_AND_STATUS)
    }
    pub(crate) fn parent_mut(&mut self) -> &mut super::commonmodule::LogicalNodeForEventAndStatus {
        self.logical_node_for_event_and_status.get_or_insert(Default::default())
    }
}
pub trait IsEssEventZbat {
    fn _ess_event_zbat(&self) -> &EssEventZbat;
    fn _ess_event_zbat_mut(&mut self) -> &mut EssEventZbat;
    fn logical_node_for_event_and_status(&self) -> &super::commonmodule::LogicalNodeForEventAndStatus {
        self._ess_event_zbat().logical_node_for_event_and_status.as_ref().unwrap_or(&ess_event_zbat::LOGICAL_NODE_FOR_EVENT_AND_STATUS)
    }
    fn logical_node_for_event_and_status_mut(&mut self) -> &mut super::commonmodule::LogicalNodeForEventAndStatus {
        self._ess_event_zbat_mut().logical_node_for_event_and_status.get_or_insert(Default::default())
    }
    fn bat_hi(&self) -> &super::commonmodule::StatusSps {
        self._ess_event_zbat().bat_hi.as_ref().unwrap_or(&ess_event_zbat::BAT_HI)
    }
    fn bat_hi_mut(&mut self) -> &mut super::commonmodule::StatusSps {
        self._ess_event_zbat_mut().bat_hi.get_or_insert(Default::default())
    }
    fn bat_lo(&self) -> &super::commonmodule::StatusSps {
        self._ess_event_zbat().bat_lo.as_ref().unwrap_or(&ess_event_zbat::BAT_LO)
    }
    fn bat_lo_mut(&mut self) -> &mut super::commonmodule::StatusSps {
        self._ess_event_zbat_mut().bat_lo.get_or_insert(Default::default())
    }
    fn bat_st(&self) -> &super::commonmodule::StatusSps {
        self._ess_event_zbat().bat_st.as_ref().unwrap_or(&ess_event_zbat::BAT_ST)
    }
    fn bat_st_mut(&mut self) -> &mut super::commonmodule::StatusSps {
        self._ess_event_zbat_mut().bat_st.get_or_insert(Default::default())
    }
    fn soc(&self) -> &super::commonmodule::Mv {
        self._ess_event_zbat().soc.as_ref().unwrap_or(&ess_event_zbat::SOC)
    }
    fn soc_mut(&mut self) -> &mut super::commonmodule::Mv {
        self._ess_event_zbat_mut().soc.get_or_insert(Default::default())
    }
    fn stdby(&self) -> &super::commonmodule::StatusSps {
        self._ess_event_zbat().stdby.as_ref().unwrap_or(&ess_event_zbat::STDBY)
    }
    fn stdby_mut(&mut self) -> &mut super::commonmodule::StatusSps {
        self._ess_event_zbat_mut().stdby.get_or_insert(Default::default())
    }
}
impl IsEssEventZbat for EssEventZbat {
    fn _ess_event_zbat(&self) -> &EssEventZbat {
        self
    }
    fn _ess_event_zbat_mut(&mut self) -> &mut EssEventZbat {
        self
    }
}
impl IsLogicalNodeForEventAndStatus for EssEventZbat {
    fn _logical_node_for_event_and_status(&self) -> &super::commonmodule::LogicalNodeForEventAndStatus {
        self.parent()
    }
    fn _logical_node_for_event_and_status_mut(&mut self) -> &mut LogicalNodeForEventAndStatus {
        self.parent_mut()
    }
}
impl IsLogicalNode for EssEventZbat {
    fn _logical_node(&self) -> &super::commonmodule::LogicalNode {
        self.parent().parent()
    }
    fn _logical_node_mut(&mut self) -> &mut LogicalNode {
        self.parent_mut().parent_mut()
    }
}
impl IsIdentifiedObject for EssEventZbat {
    fn _identified_object(&self) -> &super::commonmodule::IdentifiedObject {
        self.parent().parent().parent()
    }
    fn _identified_object_mut(&mut self) -> &mut IdentifiedObject {
        self.parent_mut().parent_mut().parent_mut()
    }
}
/// ESS inverter high level function to maintain frequency within dead bands.
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize)]
pub struct FrequencyRegulation {
    /// uint/0.01Hz  Frequency regulation is performed when the grid frequency goes beyond the dead
    /// bands. The dead bands are defined as follows: Upper DB = frequency set point + dead band plus Lower
    /// DB = frequency set point – dead band minus
    #[prost(message, optional, tag="1")]
    #[serde(default, rename = "frequencyDeadBandMinus")]
    pub frequency_dead_band_minus: ::std::option::Option<f32>,
    /// uint/0.01Hz  Frequency regulation is performed when the grid frequency goes beyond the dead
    /// bands. The dead bands are defined as follows: Upper DB = frequency set point + dead band plus Lower
    /// DB = frequency set point – dead band minus
    #[prost(message, optional, tag="2")]
    #[serde(default, rename = "frequencyDeadBandPlus")]
    pub frequency_dead_band_plus: ::std::option::Option<f32>,
    /// Control value (TRUE or FALSE)
    #[prost(message, optional, tag="3")]
    #[serde(default, rename = "frequencyRegulationCtl")]
    pub frequency_regulation_ctl: ::std::option::Option<bool>,
    /// uint/0.01Hz  Target frequency
    #[prost(message, optional, tag="4")]
    #[serde(default, rename = "frequencySetPoint")]
    pub frequency_set_point: ::std::option::Option<f32>,
    /// uint/0.01Hz  Other modes of operation, such as peak shaving, smoothing or SOC management may
    /// operate if the grid frequency is within the stable band. Upper stable band = frequency set point +
    /// band plus Lower stable band = frequency set point – band minus
    #[prost(message, optional, tag="5")]
    #[serde(default, rename = "gridFrequencyStableBandMinus")]
    pub grid_frequency_stable_band_minus: ::std::option::Option<f32>,
    /// uint/0.01Hz  Other modes of operation, such as peak shaving, smoothing or SOC management may
    /// operate if the grid frequency is within the stable band. Upper stable band = frequency set point +
    /// band plus Lower stable band = frequency set point – band minus
    #[prost(message, optional, tag="6")]
    #[serde(default, rename = "gridFrequencyStableBandPlus")]
    pub grid_frequency_stable_band_plus: ::std::option::Option<f32>,
    /// uint/0.1%  The droops define the reaction of the PCS to under/over frequency events. A droop of
    /// 1% means that the PCS will output 100% power if the frequency is 1% of the nominal frequency away
    /// from the upper or lower dead band. The minimum droop value possible is 0.8%.
    #[prost(message, optional, tag="7")]
    #[serde(default, rename = "overFrequencyDroop")]
    pub over_frequency_droop: ::std::option::Option<f32>,
    /// uint/0.1%  The droops define the reaction of the PCS to under/over voltage events. A droop of 1%
    /// means that the PCS will output 100% power if the voltage is 1% of the nominal voltage away from the
    /// upper or lower dead band. The minimum droop value possible is 0.8%.
    #[prost(message, optional, tag="8")]
    #[serde(default, rename = "underFrequencyDroop")]
    pub under_frequency_droop: ::std::option::Option<f32>,
}
mod frequency_regulation {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref FREQUENCY_DEAD_BAND_MINUS: f32 = Default::default();
        pub(super) static ref FREQUENCY_DEAD_BAND_PLUS: f32 = Default::default();
        pub(super) static ref FREQUENCY_REGULATION_CTL: bool = Default::default();
        pub(super) static ref FREQUENCY_SET_POINT: f32 = Default::default();
        pub(super) static ref GRID_FREQUENCY_STABLE_BAND_MINUS: f32 = Default::default();
        pub(super) static ref GRID_FREQUENCY_STABLE_BAND_PLUS: f32 = Default::default();
        pub(super) static ref OVER_FREQUENCY_DROOP: f32 = Default::default();
        pub(super) static ref UNDER_FREQUENCY_DROOP: f32 = Default::default();
    }
}
impl FrequencyRegulation {
}
pub trait IsFrequencyRegulation {
    fn _frequency_regulation(&self) -> &FrequencyRegulation;
    fn _frequency_regulation_mut(&mut self) -> &mut FrequencyRegulation;
    fn frequency_dead_band_minus(&self) -> &f32 {
        self._frequency_regulation().frequency_dead_band_minus.as_ref().unwrap_or(&frequency_regulation::FREQUENCY_DEAD_BAND_MINUS)
    }
    fn frequency_dead_band_minus_mut(&mut self) -> &mut f32 {
        self._frequency_regulation_mut().frequency_dead_band_minus.get_or_insert(Default::default())
    }
    fn frequency_dead_band_plus(&self) -> &f32 {
        self._frequency_regulation().frequency_dead_band_plus.as_ref().unwrap_or(&frequency_regulation::FREQUENCY_DEAD_BAND_PLUS)
    }
    fn frequency_dead_band_plus_mut(&mut self) -> &mut f32 {
        self._frequency_regulation_mut().frequency_dead_band_plus.get_or_insert(Default::default())
    }
    fn frequency_regulation_ctl(&self) -> &bool {
        self._frequency_regulation().frequency_regulation_ctl.as_ref().unwrap_or(&frequency_regulation::FREQUENCY_REGULATION_CTL)
    }
    fn frequency_regulation_ctl_mut(&mut self) -> &mut bool {
        self._frequency_regulation_mut().frequency_regulation_ctl.get_or_insert(Default::default())
    }
    fn frequency_set_point(&self) -> &f32 {
        self._frequency_regulation().frequency_set_point.as_ref().unwrap_or(&frequency_regulation::FREQUENCY_SET_POINT)
    }
    fn frequency_set_point_mut(&mut self) -> &mut f32 {
        self._frequency_regulation_mut().frequency_set_point.get_or_insert(Default::default())
    }
    fn grid_frequency_stable_band_minus(&self) -> &f32 {
        self._frequency_regulation().grid_frequency_stable_band_minus.as_ref().unwrap_or(&frequency_regulation::GRID_FREQUENCY_STABLE_BAND_MINUS)
    }
    fn grid_frequency_stable_band_minus_mut(&mut self) -> &mut f32 {
        self._frequency_regulation_mut().grid_frequency_stable_band_minus.get_or_insert(Default::default())
    }
    fn grid_frequency_stable_band_plus(&self) -> &f32 {
        self._frequency_regulation().grid_frequency_stable_band_plus.as_ref().unwrap_or(&frequency_regulation::GRID_FREQUENCY_STABLE_BAND_PLUS)
    }
    fn grid_frequency_stable_band_plus_mut(&mut self) -> &mut f32 {
        self._frequency_regulation_mut().grid_frequency_stable_band_plus.get_or_insert(Default::default())
    }
    fn over_frequency_droop(&self) -> &f32 {
        self._frequency_regulation().over_frequency_droop.as_ref().unwrap_or(&frequency_regulation::OVER_FREQUENCY_DROOP)
    }
    fn over_frequency_droop_mut(&mut self) -> &mut f32 {
        self._frequency_regulation_mut().over_frequency_droop.get_or_insert(Default::default())
    }
    fn under_frequency_droop(&self) -> &f32 {
        self._frequency_regulation().under_frequency_droop.as_ref().unwrap_or(&frequency_regulation::UNDER_FREQUENCY_DROOP)
    }
    fn under_frequency_droop_mut(&mut self) -> &mut f32 {
        self._frequency_regulation_mut().under_frequency_droop.get_or_insert(Default::default())
    }
}
impl IsFrequencyRegulation for FrequencyRegulation {
    fn _frequency_regulation(&self) -> &FrequencyRegulation {
        self
    }
    fn _frequency_regulation_mut(&mut self) -> &mut FrequencyRegulation {
        self
    }
}
/// ESS inverter high level function to maintain power level by charging or discharging
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize)]
pub struct PeakShaving {
    /// uint/1kW  If the supervised power goes below this limit, the ESS will charge to maintain this limit.
    #[prost(message, optional, tag="1")]
    #[serde(default, rename = "baseShavingLimit")]
    pub base_shaving_limit: ::std::option::Option<f32>,
    /// Control value (TRUE or FALSE)
    #[prost(message, optional, tag="2")]
    #[serde(default, rename = "peakShavingCtl")]
    pub peak_shaving_ctl: ::std::option::Option<bool>,
    /// uint/1kW  If the supervised power goes above this limit, the ESS will discharge to maintain this
    /// limit.
    #[prost(message, optional, tag="3")]
    #[serde(default, rename = "peakShavingLimit")]
    pub peak_shaving_limit: ::std::option::Option<f32>,
    /// uint/1kW  If the supervised power is between the band defined by these two limits then SOC
    /// management is allowed.
    #[prost(message, optional, tag="4")]
    #[serde(default, rename = "socManagementAllowedHighLimit")]
    pub soc_management_allowed_high_limit: ::std::option::Option<f32>,
    /// uint/1kW  If the supervised power is between the band defined by these two limits then SOC
    /// management is allowed.
    #[prost(message, optional, tag="5")]
    #[serde(default, rename = "socManagementAllowedLowLimit")]
    pub soc_management_allowed_low_limit: ::std::option::Option<f32>,
}
mod peak_shaving {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref BASE_SHAVING_LIMIT: f32 = Default::default();
        pub(super) static ref PEAK_SHAVING_CTL: bool = Default::default();
        pub(super) static ref PEAK_SHAVING_LIMIT: f32 = Default::default();
        pub(super) static ref SOC_MANAGEMENT_ALLOWED_HIGH_LIMIT: f32 = Default::default();
        pub(super) static ref SOC_MANAGEMENT_ALLOWED_LOW_LIMIT: f32 = Default::default();
    }
}
impl PeakShaving {
}
pub trait IsPeakShaving {
    fn _peak_shaving(&self) -> &PeakShaving;
    fn _peak_shaving_mut(&mut self) -> &mut PeakShaving;
    fn base_shaving_limit(&self) -> &f32 {
        self._peak_shaving().base_shaving_limit.as_ref().unwrap_or(&peak_shaving::BASE_SHAVING_LIMIT)
    }
    fn base_shaving_limit_mut(&mut self) -> &mut f32 {
        self._peak_shaving_mut().base_shaving_limit.get_or_insert(Default::default())
    }
    fn peak_shaving_ctl(&self) -> &bool {
        self._peak_shaving().peak_shaving_ctl.as_ref().unwrap_or(&peak_shaving::PEAK_SHAVING_CTL)
    }
    fn peak_shaving_ctl_mut(&mut self) -> &mut bool {
        self._peak_shaving_mut().peak_shaving_ctl.get_or_insert(Default::default())
    }
    fn peak_shaving_limit(&self) -> &f32 {
        self._peak_shaving().peak_shaving_limit.as_ref().unwrap_or(&peak_shaving::PEAK_SHAVING_LIMIT)
    }
    fn peak_shaving_limit_mut(&mut self) -> &mut f32 {
        self._peak_shaving_mut().peak_shaving_limit.get_or_insert(Default::default())
    }
    fn soc_management_allowed_high_limit(&self) -> &f32 {
        self._peak_shaving().soc_management_allowed_high_limit.as_ref().unwrap_or(&peak_shaving::SOC_MANAGEMENT_ALLOWED_HIGH_LIMIT)
    }
    fn soc_management_allowed_high_limit_mut(&mut self) -> &mut f32 {
        self._peak_shaving_mut().soc_management_allowed_high_limit.get_or_insert(Default::default())
    }
    fn soc_management_allowed_low_limit(&self) -> &f32 {
        self._peak_shaving().soc_management_allowed_low_limit.as_ref().unwrap_or(&peak_shaving::SOC_MANAGEMENT_ALLOWED_LOW_LIMIT)
    }
    fn soc_management_allowed_low_limit_mut(&mut self) -> &mut f32 {
        self._peak_shaving_mut().soc_management_allowed_low_limit.get_or_insert(Default::default())
    }
}
impl IsPeakShaving for PeakShaving {
    fn _peak_shaving(&self) -> &PeakShaving {
        self
    }
    fn _peak_shaving_mut(&mut self) -> &mut PeakShaving {
        self
    }
}
/// ESS inverter high level function to shut down ESS if SOC exceeds high or low limits.
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize)]
pub struct SocLimit {
    /// uint/1%  These limits define the operational range of the battery. If a lineup reaches the SOC
    /// high limit, the inverter’s output is reduced to 0. Charging is then blocked until the hysteresis is
    /// overcome. The same logic applies to the SOC low limit, except that after the ramp down is complete,
    /// discharging is blocked until the hysteresis is overcome.
    #[prost(message, optional, tag="1")]
    #[serde(default, rename = "socHighLimit")]
    pub soc_high_limit: ::std::option::Option<f32>,
    /// uint/1%  These limits define the operational range of the battery. If a lineup reaches the SOC
    /// high limit, the inverter’s output is reduced to 0. Charging is then blocked until the hysteresis is
    /// overcome. The same logic applies to the SOC low limit, except that after the ramp down is complete,
    /// discharging is blocked until the hysteresis is overcome.
    #[prost(message, optional, tag="2")]
    #[serde(default, rename = "socHighLimitHysteresis")]
    pub soc_high_limit_hysteresis: ::std::option::Option<f32>,
    /// Control value (TRUE or FALSE)
    #[prost(message, optional, tag="3")]
    #[serde(default, rename = "socLimitCtl")]
    pub soc_limit_ctl: ::std::option::Option<bool>,
    /// uint/1%  These limits define the operational range of the battery. If a lineup reaches the SOC
    /// high limit, the inverter’s output is reduced to 0. Charging is then blocked until the hysteresis is
    /// overcome. The same logic applies to the SOC low limit, except that after the ramp down is complete,
    /// discharging is blocked until the hysteresis is overcome.
    #[prost(message, optional, tag="4")]
    #[serde(default, rename = "socLowLimit")]
    pub soc_low_limit: ::std::option::Option<f32>,
    /// uint/1%  These hysteresis define the release conditions for the block charge or discharge
    /// initiated by the SOC limits.For example, assume a SOC low limit of 10% and a SOC low limit
    /// hysteresis of 2% and that discharging is blocked because the batteries SOC reached the SOC low
    /// limit, discharging will only be allowed again after the battery’s SOC reaches 13%.
    #[prost(message, optional, tag="5")]
    #[serde(default, rename = "socLowLimitHysteresis")]
    pub soc_low_limit_hysteresis: ::std::option::Option<f32>,
}
mod soc_limit {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref SOC_HIGH_LIMIT: f32 = Default::default();
        pub(super) static ref SOC_HIGH_LIMIT_HYSTERESIS: f32 = Default::default();
        pub(super) static ref SOC_LIMIT_CTL: bool = Default::default();
        pub(super) static ref SOC_LOW_LIMIT: f32 = Default::default();
        pub(super) static ref SOC_LOW_LIMIT_HYSTERESIS: f32 = Default::default();
    }
}
impl SocLimit {
}
pub trait IsSocLimit {
    fn _soc_limit(&self) -> &SocLimit;
    fn _soc_limit_mut(&mut self) -> &mut SocLimit;
    fn soc_high_limit(&self) -> &f32 {
        self._soc_limit().soc_high_limit.as_ref().unwrap_or(&soc_limit::SOC_HIGH_LIMIT)
    }
    fn soc_high_limit_mut(&mut self) -> &mut f32 {
        self._soc_limit_mut().soc_high_limit.get_or_insert(Default::default())
    }
    fn soc_high_limit_hysteresis(&self) -> &f32 {
        self._soc_limit().soc_high_limit_hysteresis.as_ref().unwrap_or(&soc_limit::SOC_HIGH_LIMIT_HYSTERESIS)
    }
    fn soc_high_limit_hysteresis_mut(&mut self) -> &mut f32 {
        self._soc_limit_mut().soc_high_limit_hysteresis.get_or_insert(Default::default())
    }
    fn soc_limit_ctl(&self) -> &bool {
        self._soc_limit().soc_limit_ctl.as_ref().unwrap_or(&soc_limit::SOC_LIMIT_CTL)
    }
    fn soc_limit_ctl_mut(&mut self) -> &mut bool {
        self._soc_limit_mut().soc_limit_ctl.get_or_insert(Default::default())
    }
    fn soc_low_limit(&self) -> &f32 {
        self._soc_limit().soc_low_limit.as_ref().unwrap_or(&soc_limit::SOC_LOW_LIMIT)
    }
    fn soc_low_limit_mut(&mut self) -> &mut f32 {
        self._soc_limit_mut().soc_low_limit.get_or_insert(Default::default())
    }
    fn soc_low_limit_hysteresis(&self) -> &f32 {
        self._soc_limit().soc_low_limit_hysteresis.as_ref().unwrap_or(&soc_limit::SOC_LOW_LIMIT_HYSTERESIS)
    }
    fn soc_low_limit_hysteresis_mut(&mut self) -> &mut f32 {
        self._soc_limit_mut().soc_low_limit_hysteresis.get_or_insert(Default::default())
    }
}
impl IsSocLimit for SocLimit {
    fn _soc_limit(&self) -> &SocLimit {
        self
    }
    fn _soc_limit_mut(&mut self) -> &mut SocLimit {
        self
    }
}
/// ESS inverter high level function to maintain SOC within dead bands
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize)]
pub struct SocManagement {
    /// uint/1%  Define a dead band (DB) around the SOC set point. When the battery SOC goes outside the
    /// dead band, the SOC management executes and bring the SOC back to the set point. Upper DB = set point
    /// + dead band plus Lower DB = set point – dead band minus
    #[prost(message, optional, tag="1")]
    #[serde(default, rename = "socDeadBandMinus")]
    pub soc_dead_band_minus: ::std::option::Option<f32>,
    /// uint/1%  Define a dead band (DB) around the SOC set point. When the battery SOC goes outside the
    /// dead band, the SOC management executes and bring the SOC back to the set point. Upper DB = set point
    /// + dead band plus Lower DB = set point – dead band minus
    #[prost(message, optional, tag="2")]
    #[serde(default, rename = "socDeadBandPlus")]
    pub soc_dead_band_plus: ::std::option::Option<f32>,
    /// Control value (TRUE or FALSE)
    #[prost(message, optional, tag="3")]
    #[serde(default, rename = "socManagementCtl")]
    pub soc_management_ctl: ::std::option::Option<bool>,
    /// uint/1kW  Set point used for SOC maintenance
    #[prost(message, optional, tag="4")]
    #[serde(default, rename = "socPowerSetPoint")]
    pub soc_power_set_point: ::std::option::Option<f32>,
    /// uint/1%  SOC Target in percentage (%).
    #[prost(message, optional, tag="5")]
    #[serde(default, rename = "socSetPoint")]
    pub soc_set_point: ::std::option::Option<f32>,
}
mod soc_management {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref SOC_DEAD_BAND_MINUS: f32 = Default::default();
        pub(super) static ref SOC_DEAD_BAND_PLUS: f32 = Default::default();
        pub(super) static ref SOC_MANAGEMENT_CTL: bool = Default::default();
        pub(super) static ref SOC_POWER_SET_POINT: f32 = Default::default();
        pub(super) static ref SOC_SET_POINT: f32 = Default::default();
    }
}
impl SocManagement {
}
pub trait IsSocManagement {
    fn _soc_management(&self) -> &SocManagement;
    fn _soc_management_mut(&mut self) -> &mut SocManagement;
    fn soc_dead_band_minus(&self) -> &f32 {
        self._soc_management().soc_dead_band_minus.as_ref().unwrap_or(&soc_management::SOC_DEAD_BAND_MINUS)
    }
    fn soc_dead_band_minus_mut(&mut self) -> &mut f32 {
        self._soc_management_mut().soc_dead_band_minus.get_or_insert(Default::default())
    }
    fn soc_dead_band_plus(&self) -> &f32 {
        self._soc_management().soc_dead_band_plus.as_ref().unwrap_or(&soc_management::SOC_DEAD_BAND_PLUS)
    }
    fn soc_dead_band_plus_mut(&mut self) -> &mut f32 {
        self._soc_management_mut().soc_dead_band_plus.get_or_insert(Default::default())
    }
    fn soc_management_ctl(&self) -> &bool {
        self._soc_management().soc_management_ctl.as_ref().unwrap_or(&soc_management::SOC_MANAGEMENT_CTL)
    }
    fn soc_management_ctl_mut(&mut self) -> &mut bool {
        self._soc_management_mut().soc_management_ctl.get_or_insert(Default::default())
    }
    fn soc_power_set_point(&self) -> &f32 {
        self._soc_management().soc_power_set_point.as_ref().unwrap_or(&soc_management::SOC_POWER_SET_POINT)
    }
    fn soc_power_set_point_mut(&mut self) -> &mut f32 {
        self._soc_management_mut().soc_power_set_point.get_or_insert(Default::default())
    }
    fn soc_set_point(&self) -> &f32 {
        self._soc_management().soc_set_point.as_ref().unwrap_or(&soc_management::SOC_SET_POINT)
    }
    fn soc_set_point_mut(&mut self) -> &mut f32 {
        self._soc_management_mut().soc_set_point.get_or_insert(Default::default())
    }
}
impl IsSocManagement for SocManagement {
    fn _soc_management(&self) -> &SocManagement {
        self
    }
    fn _soc_management_mut(&mut self) -> &mut SocManagement {
        self
    }
}
/// Voltage regulation function
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize)]
pub struct VoltageRegulation {
    /// uint/0.1%  The droops define the reaction of the PCS to under/over voltage events. A droop of 1%
    /// means that the PCS will output 100% power if the voltage is 1% of the nominal voltage away from the
    /// upper or lower dead band. The minimum droop value possible is 0.8%.
    #[prost(message, optional, tag="1")]
    #[serde(default, rename = "overVoltageDroop")]
    pub over_voltage_droop: ::std::option::Option<f32>,
    /// uint/0.1%  The droops define the reaction of the PCS to under/over voltage events. A droop of 1%
    /// means that the PCS will output 100% power if the voltage is 1% of the nominal voltage away from the
    /// upper or lower dead band. The minimum droop value possible is 0.8%.
    #[prost(message, optional, tag="2")]
    #[serde(default, rename = "underVoltageDroop")]
    pub under_voltage_droop: ::std::option::Option<f32>,
    /// uint/0.1V  Voltage regulation is performed when the grid voltage goes beyond the dead bands. The
    /// dead bands are defined as follows: Upper DB = voltage set point + dead band plus Lower DB = voltage
    /// set point – dead band minus
    #[prost(message, optional, tag="3")]
    #[serde(default, rename = "voltageDeadBandMinus")]
    pub voltage_dead_band_minus: ::std::option::Option<f32>,
    /// uint/0.1V  Voltage regulation is performed when the grid voltage goes beyond the dead bands. The
    /// dead bands are defined as follows: Upper DB = voltage set point + dead band plus Lower DB = voltage
    /// set point – dead band minus
    #[prost(message, optional, tag="4")]
    #[serde(default, rename = "voltageDeadBandPlus")]
    pub voltage_dead_band_plus: ::std::option::Option<f32>,
    /// uint/0.1V  Other modes of operation, such as peak shaving, smoothing or SOC management may
    /// operate if the grid frequency is within the stable band. Upper stable band = frequency set point +
    /// band plus Lower stable band = frequency set point – band minus
    #[prost(message, optional, tag="5")]
    #[serde(default, rename = "voltageSetPoint")]
    pub voltage_set_point: ::std::option::Option<f32>,
}
mod voltage_regulation {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref OVER_VOLTAGE_DROOP: f32 = Default::default();
        pub(super) static ref UNDER_VOLTAGE_DROOP: f32 = Default::default();
        pub(super) static ref VOLTAGE_DEAD_BAND_MINUS: f32 = Default::default();
        pub(super) static ref VOLTAGE_DEAD_BAND_PLUS: f32 = Default::default();
        pub(super) static ref VOLTAGE_SET_POINT: f32 = Default::default();
    }
}
impl VoltageRegulation {
}
pub trait IsVoltageRegulation {
    fn _voltage_regulation(&self) -> &VoltageRegulation;
    fn _voltage_regulation_mut(&mut self) -> &mut VoltageRegulation;
    fn over_voltage_droop(&self) -> &f32 {
        self._voltage_regulation().over_voltage_droop.as_ref().unwrap_or(&voltage_regulation::OVER_VOLTAGE_DROOP)
    }
    fn over_voltage_droop_mut(&mut self) -> &mut f32 {
        self._voltage_regulation_mut().over_voltage_droop.get_or_insert(Default::default())
    }
    fn under_voltage_droop(&self) -> &f32 {
        self._voltage_regulation().under_voltage_droop.as_ref().unwrap_or(&voltage_regulation::UNDER_VOLTAGE_DROOP)
    }
    fn under_voltage_droop_mut(&mut self) -> &mut f32 {
        self._voltage_regulation_mut().under_voltage_droop.get_or_insert(Default::default())
    }
    fn voltage_dead_band_minus(&self) -> &f32 {
        self._voltage_regulation().voltage_dead_band_minus.as_ref().unwrap_or(&voltage_regulation::VOLTAGE_DEAD_BAND_MINUS)
    }
    fn voltage_dead_band_minus_mut(&mut self) -> &mut f32 {
        self._voltage_regulation_mut().voltage_dead_band_minus.get_or_insert(Default::default())
    }
    fn voltage_dead_band_plus(&self) -> &f32 {
        self._voltage_regulation().voltage_dead_band_plus.as_ref().unwrap_or(&voltage_regulation::VOLTAGE_DEAD_BAND_PLUS)
    }
    fn voltage_dead_band_plus_mut(&mut self) -> &mut f32 {
        self._voltage_regulation_mut().voltage_dead_band_plus.get_or_insert(Default::default())
    }
    fn voltage_set_point(&self) -> &f32 {
        self._voltage_regulation().voltage_set_point.as_ref().unwrap_or(&voltage_regulation::VOLTAGE_SET_POINT)
    }
    fn voltage_set_point_mut(&mut self) -> &mut f32 {
        self._voltage_regulation_mut().voltage_set_point.get_or_insert(Default::default())
    }
}
impl IsVoltageRegulation for VoltageRegulation {
    fn _voltage_regulation(&self) -> &VoltageRegulation {
        self
    }
    fn _voltage_regulation_mut(&mut self) -> &mut VoltageRegulation {
        self
    }
}
/// ESS inverter high level function to maintain voltage within droop dead bands.
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize)]
pub struct VoltageDroop {
    /// Control value (TRUE or FALSE)
    #[prost(message, optional, tag="1")]
    #[serde(default, rename = "voltageDroopCtl")]
    pub voltage_droop_ctl: ::std::option::Option<bool>,
    /// Voltage regulation
    #[prost(message, optional, tag="2")]
    #[serde(default, rename = "voltageRegulation")]
    pub voltage_regulation: ::std::option::Option<VoltageRegulation>,
}
mod voltage_droop {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref VOLTAGE_DROOP_CTL: bool = Default::default();
        pub(super) static ref VOLTAGE_REGULATION: crate::essmodule::VoltageRegulation = Default::default();
    }
}
impl VoltageDroop {
}
pub trait IsVoltageDroop {
    fn _voltage_droop(&self) -> &VoltageDroop;
    fn _voltage_droop_mut(&mut self) -> &mut VoltageDroop;
    fn voltage_droop_ctl(&self) -> &bool {
        self._voltage_droop().voltage_droop_ctl.as_ref().unwrap_or(&voltage_droop::VOLTAGE_DROOP_CTL)
    }
    fn voltage_droop_ctl_mut(&mut self) -> &mut bool {
        self._voltage_droop_mut().voltage_droop_ctl.get_or_insert(Default::default())
    }
    fn voltage_regulation(&self) -> &VoltageRegulation {
        self._voltage_droop().voltage_regulation.as_ref().unwrap_or(&voltage_droop::VOLTAGE_REGULATION)
    }
    fn voltage_regulation_mut(&mut self) -> &mut VoltageRegulation {
        self._voltage_droop_mut().voltage_regulation.get_or_insert(Default::default())
    }
}
impl IsVoltageDroop for VoltageDroop {
    fn _voltage_droop(&self) -> &VoltageDroop {
        self
    }
    fn _voltage_droop_mut(&mut self) -> &mut VoltageDroop {
        self
    }
}
/// ESS inverter high level function to maintain voltage within dead bands.
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize)]
pub struct VoltagePi {
    /// Control value (TRUE or FALSE)
    #[prost(message, optional, tag="1")]
    #[serde(default, rename = "voltagePICtl")]
    pub voltage_pi_ctl: ::std::option::Option<bool>,
    /// Voltage regulation
    #[prost(message, optional, tag="2")]
    #[serde(default, rename = "voltageRegulation")]
    pub voltage_regulation: ::std::option::Option<VoltageRegulation>,
}
mod voltage_pi {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref VOLTAGE_PI_CTL: bool = Default::default();
        pub(super) static ref VOLTAGE_REGULATION: crate::essmodule::VoltageRegulation = Default::default();
    }
}
impl VoltagePi {
}
pub trait IsVoltagePi {
    fn _voltage_pi(&self) -> &VoltagePi;
    fn _voltage_pi_mut(&mut self) -> &mut VoltagePi;
    fn voltage_pi_ctl(&self) -> &bool {
        self._voltage_pi().voltage_pi_ctl.as_ref().unwrap_or(&voltage_pi::VOLTAGE_PI_CTL)
    }
    fn voltage_pi_ctl_mut(&mut self) -> &mut bool {
        self._voltage_pi_mut().voltage_pi_ctl.get_or_insert(Default::default())
    }
    fn voltage_regulation(&self) -> &VoltageRegulation {
        self._voltage_pi().voltage_regulation.as_ref().unwrap_or(&voltage_pi::VOLTAGE_REGULATION)
    }
    fn voltage_regulation_mut(&mut self) -> &mut VoltageRegulation {
        self._voltage_pi_mut().voltage_regulation.get_or_insert(Default::default())
    }
}
impl IsVoltagePi for VoltagePi {
    fn _voltage_pi(&self) -> &VoltagePi {
        self
    }
    fn _voltage_pi_mut(&mut self) -> &mut VoltagePi {
        self
    }
}
/// ESS inverter high level function to reduce (smooth) charging or discharging rate of change.
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize)]
pub struct CapacityFirming {
    /// Control value (TRUE or FALSE)
    #[prost(message, optional, tag="1")]
    #[serde(default, rename = "capacityFirmingCtl")]
    pub capacity_firming_ctl: ::std::option::Option<bool>,
    /// uint/1kW/min  If the supervised power increases at a rate higher that the rate defined by these
    /// limits, the ESS will discharge/charge at an opposite dp/dt to reduce (smooth) the rate of change at
    /// the PCC
    #[prost(message, optional, tag="2")]
    #[serde(default, rename = "limitNegative_dp_dt")]
    pub limit_negative_dp_dt: ::std::option::Option<f32>,
    /// uint/1kW/min  If the supervised power increases at a rate higher that the rate defined by these
    /// limits, the ESS will discharge/charge at an opposite dp/dt to reduce (smooth) the rate of change at
    /// the PCC
    #[prost(message, optional, tag="3")]
    #[serde(default, rename = "limitPositive_dp_dt")]
    pub limit_positive_dp_dt: ::std::option::Option<f32>,
}
mod capacity_firming {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref CAPACITY_FIRMING_CTL: bool = Default::default();
        pub(super) static ref LIMIT_NEGATIVE_DP_DT: f32 = Default::default();
        pub(super) static ref LIMIT_POSITIVE_DP_DT: f32 = Default::default();
    }
}
impl CapacityFirming {
}
pub trait IsCapacityFirming {
    fn _capacity_firming(&self) -> &CapacityFirming;
    fn _capacity_firming_mut(&mut self) -> &mut CapacityFirming;
    fn capacity_firming_ctl(&self) -> &bool {
        self._capacity_firming().capacity_firming_ctl.as_ref().unwrap_or(&capacity_firming::CAPACITY_FIRMING_CTL)
    }
    fn capacity_firming_ctl_mut(&mut self) -> &mut bool {
        self._capacity_firming_mut().capacity_firming_ctl.get_or_insert(Default::default())
    }
    fn limit_negative_dp_dt(&self) -> &f32 {
        self._capacity_firming().limit_negative_dp_dt.as_ref().unwrap_or(&capacity_firming::LIMIT_NEGATIVE_DP_DT)
    }
    fn limit_negative_dp_dt_mut(&mut self) -> &mut f32 {
        self._capacity_firming_mut().limit_negative_dp_dt.get_or_insert(Default::default())
    }
    fn limit_positive_dp_dt(&self) -> &f32 {
        self._capacity_firming().limit_positive_dp_dt.as_ref().unwrap_or(&capacity_firming::LIMIT_POSITIVE_DP_DT)
    }
    fn limit_positive_dp_dt_mut(&mut self) -> &mut f32 {
        self._capacity_firming_mut().limit_positive_dp_dt.get_or_insert(Default::default())
    }
}
impl IsCapacityFirming for CapacityFirming {
    fn _capacity_firming(&self) -> &CapacityFirming {
        self
    }
    fn _capacity_firming_mut(&mut self) -> &mut CapacityFirming {
        self
    }
}
/// ESS inverter high level functions.
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize)]
pub struct EssFunction {
    /// ESS inverter high level function to reduce (smooth) charging or discharging rate of change.
    #[prost(message, optional, tag="1")]
    #[serde(default, rename = "capacityFirming")]
    pub capacity_firming: ::std::option::Option<CapacityFirming>,
    /// ESS inverter high level function to maintain frequency within dead bands.
    #[prost(message, optional, tag="2")]
    #[serde(default, rename = "frequencyRegulation")]
    pub frequency_regulation: ::std::option::Option<FrequencyRegulation>,
    /// ESS inverter high level function to maintain power level by charging or discharging
    #[prost(message, optional, tag="3")]
    #[serde(default, rename = "peakShaving")]
    pub peak_shaving: ::std::option::Option<PeakShaving>,
    /// ESS inverter high level function to shut down ESS if SOC exceeds high or low limits.
    #[prost(message, optional, tag="4")]
    #[serde(default, rename = "socLimit")]
    pub soc_limit: ::std::option::Option<SocLimit>,
    /// ESS inverter high level function to maintain SOC within dead bands
    #[prost(message, optional, tag="5")]
    #[serde(default, rename = "socManagement")]
    pub soc_management: ::std::option::Option<SocManagement>,
    /// ESS inverter high level function to maintain voltage within droop dead bands.
    #[prost(message, optional, tag="6")]
    #[serde(default, rename = "voltageDroop")]
    pub voltage_droop: ::std::option::Option<VoltageDroop>,
    /// ESS inverter high level function to maintain voltage within dead bands.
    #[prost(message, optional, tag="7")]
    #[serde(default, rename = "voltagePI")]
    pub voltage_pi: ::std::option::Option<VoltagePi>,
}
mod ess_function {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref CAPACITY_FIRMING: crate::essmodule::CapacityFirming = Default::default();
        pub(super) static ref FREQUENCY_REGULATION: crate::essmodule::FrequencyRegulation = Default::default();
        pub(super) static ref PEAK_SHAVING: crate::essmodule::PeakShaving = Default::default();
        pub(super) static ref SOC_LIMIT: crate::essmodule::SocLimit = Default::default();
        pub(super) static ref SOC_MANAGEMENT: crate::essmodule::SocManagement = Default::default();
        pub(super) static ref VOLTAGE_DROOP: crate::essmodule::VoltageDroop = Default::default();
        pub(super) static ref VOLTAGE_PI: crate::essmodule::VoltagePi = Default::default();
    }
}
impl EssFunction {
}
pub trait IsEssFunction {
    fn _ess_function(&self) -> &EssFunction;
    fn _ess_function_mut(&mut self) -> &mut EssFunction;
    fn capacity_firming(&self) -> &CapacityFirming {
        self._ess_function().capacity_firming.as_ref().unwrap_or(&ess_function::CAPACITY_FIRMING)
    }
    fn capacity_firming_mut(&mut self) -> &mut CapacityFirming {
        self._ess_function_mut().capacity_firming.get_or_insert(Default::default())
    }
    fn frequency_regulation(&self) -> &FrequencyRegulation {
        self._ess_function().frequency_regulation.as_ref().unwrap_or(&ess_function::FREQUENCY_REGULATION)
    }
    fn frequency_regulation_mut(&mut self) -> &mut FrequencyRegulation {
        self._ess_function_mut().frequency_regulation.get_or_insert(Default::default())
    }
    fn peak_shaving(&self) -> &PeakShaving {
        self._ess_function().peak_shaving.as_ref().unwrap_or(&ess_function::PEAK_SHAVING)
    }
    fn peak_shaving_mut(&mut self) -> &mut PeakShaving {
        self._ess_function_mut().peak_shaving.get_or_insert(Default::default())
    }
    fn soc_limit(&self) -> &SocLimit {
        self._ess_function().soc_limit.as_ref().unwrap_or(&ess_function::SOC_LIMIT)
    }
    fn soc_limit_mut(&mut self) -> &mut SocLimit {
        self._ess_function_mut().soc_limit.get_or_insert(Default::default())
    }
    fn soc_management(&self) -> &SocManagement {
        self._ess_function().soc_management.as_ref().unwrap_or(&ess_function::SOC_MANAGEMENT)
    }
    fn soc_management_mut(&mut self) -> &mut SocManagement {
        self._ess_function_mut().soc_management.get_or_insert(Default::default())
    }
    fn voltage_droop(&self) -> &VoltageDroop {
        self._ess_function().voltage_droop.as_ref().unwrap_or(&ess_function::VOLTAGE_DROOP)
    }
    fn voltage_droop_mut(&mut self) -> &mut VoltageDroop {
        self._ess_function_mut().voltage_droop.get_or_insert(Default::default())
    }
    fn voltage_pi(&self) -> &VoltagePi {
        self._ess_function().voltage_pi.as_ref().unwrap_or(&ess_function::VOLTAGE_PI)
    }
    fn voltage_pi_mut(&mut self) -> &mut VoltagePi {
        self._ess_function_mut().voltage_pi.get_or_insert(Default::default())
    }
}
impl IsEssFunction for EssFunction {
    fn _ess_function(&self) -> &EssFunction {
        self
    }
    fn _ess_function_mut(&mut self) -> &mut EssFunction {
        self
    }
}
/// Point definition (Point)
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize)]
pub struct EssPointStatus {
    /// Black start enable
    #[prost(message, optional, tag="1")]
    #[serde(default, rename = "blackStartEnabled")]
    pub black_start_enabled: ::std::option::Option<super::commonmodule::StatusDps>,
    /// Enable frequency set point
    #[prost(message, optional, tag="2")]
    #[serde(default, rename = "frequencySetPointEnabled")]
    pub frequency_set_point_enabled: ::std::option::Option<super::commonmodule::StatusDps>,
    /// ESS function parameter
    #[prost(message, optional, tag="3")]
    #[serde(default)]
    pub function: ::std::option::Option<EssFunction>,
    /// Grid connect mode
    #[prost(message, optional, tag="4")]
    #[serde(default)]
    pub mode: ::std::option::Option<super::commonmodule::EngGridConnectModeKind>,
    /// Black start enable
    #[prost(message, optional, tag="5")]
    #[serde(default, rename = "pctHzDroop")]
    pub pct_hz_droop: ::std::option::Option<f32>,
    /// Black start enable
    #[prost(message, optional, tag="6")]
    #[serde(default, rename = "pctVDroop")]
    pub pct_v_droop: ::std::option::Option<f32>,
    /// Ramp rates
    #[prost(message, optional, tag="7")]
    #[serde(default, rename = "rampRates")]
    pub ramp_rates: ::std::option::Option<super::commonmodule::RampRate>,
    /// Enable reactive power set point
    #[prost(message, optional, tag="8")]
    #[serde(default, rename = "reactivePwrSetPointEnabled")]
    pub reactive_pwr_set_point_enabled: ::std::option::Option<super::commonmodule::StatusDps>,
    /// Enable real power set point
    #[prost(message, optional, tag="9")]
    #[serde(default, rename = "realPwrSetPointEnabled")]
    pub real_pwr_set_point_enabled: ::std::option::Option<super::commonmodule::StatusDps>,
    /// ESS state
    #[prost(message, optional, tag="10")]
    #[serde(default)]
    pub state: ::std::option::Option<super::commonmodule::OptionalStateKind>,
    /// Synchronize back to grid
    #[prost(message, optional, tag="11")]
    #[serde(default, rename = "syncBackToGrid")]
    pub sync_back_to_grid: ::std::option::Option<super::commonmodule::StatusDps>,
    /// Transition to island on grid loss enable
    #[prost(message, optional, tag="12")]
    #[serde(default, rename = "transToIslndOnGridLossEnabled")]
    pub trans_to_islnd_on_grid_loss_enabled: ::std::option::Option<super::commonmodule::StatusDps>,
    /// Enable voltage set point
    #[prost(message, optional, tag="13")]
    #[serde(default, rename = "voltageSetPointEnabled")]
    pub voltage_set_point_enabled: ::std::option::Option<super::commonmodule::StatusDps>,
}
mod ess_point_status {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref BLACK_START_ENABLED: crate::commonmodule::StatusDps = Default::default();
        pub(super) static ref FREQUENCY_SET_POINT_ENABLED: crate::commonmodule::StatusDps = Default::default();
        pub(super) static ref FUNCTION: crate::essmodule::EssFunction = Default::default();
        pub(super) static ref MODE: crate::commonmodule::EngGridConnectModeKind = Default::default();
        pub(super) static ref PCT_HZ_DROOP: f32 = Default::default();
        pub(super) static ref PCT_V_DROOP: f32 = Default::default();
        pub(super) static ref RAMP_RATES: crate::commonmodule::RampRate = Default::default();
        pub(super) static ref REACTIVE_PWR_SET_POINT_ENABLED: crate::commonmodule::StatusDps = Default::default();
        pub(super) static ref REAL_PWR_SET_POINT_ENABLED: crate::commonmodule::StatusDps = Default::default();
        pub(super) static ref STATE: crate::commonmodule::OptionalStateKind = Default::default();
        pub(super) static ref SYNC_BACK_TO_GRID: crate::commonmodule::StatusDps = Default::default();
        pub(super) static ref TRANS_TO_ISLND_ON_GRID_LOSS_ENABLED: crate::commonmodule::StatusDps = Default::default();
        pub(super) static ref VOLTAGE_SET_POINT_ENABLED: crate::commonmodule::StatusDps = Default::default();
    }
}
impl EssPointStatus {
}
pub trait IsEssPointStatus {
    fn _ess_point_status(&self) -> &EssPointStatus;
    fn _ess_point_status_mut(&mut self) -> &mut EssPointStatus;
    fn black_start_enabled(&self) -> &super::commonmodule::StatusDps {
        self._ess_point_status().black_start_enabled.as_ref().unwrap_or(&ess_point_status::BLACK_START_ENABLED)
    }
    fn black_start_enabled_mut(&mut self) -> &mut super::commonmodule::StatusDps {
        self._ess_point_status_mut().black_start_enabled.get_or_insert(Default::default())
    }
    fn frequency_set_point_enabled(&self) -> &super::commonmodule::StatusDps {
        self._ess_point_status().frequency_set_point_enabled.as_ref().unwrap_or(&ess_point_status::FREQUENCY_SET_POINT_ENABLED)
    }
    fn frequency_set_point_enabled_mut(&mut self) -> &mut super::commonmodule::StatusDps {
        self._ess_point_status_mut().frequency_set_point_enabled.get_or_insert(Default::default())
    }
    fn function(&self) -> &EssFunction {
        self._ess_point_status().function.as_ref().unwrap_or(&ess_point_status::FUNCTION)
    }
    fn function_mut(&mut self) -> &mut EssFunction {
        self._ess_point_status_mut().function.get_or_insert(Default::default())
    }
    fn mode(&self) -> &super::commonmodule::EngGridConnectModeKind {
        self._ess_point_status().mode.as_ref().unwrap_or(&ess_point_status::MODE)
    }
    fn mode_mut(&mut self) -> &mut super::commonmodule::EngGridConnectModeKind {
        self._ess_point_status_mut().mode.get_or_insert(Default::default())
    }
    fn pct_hz_droop(&self) -> &f32 {
        self._ess_point_status().pct_hz_droop.as_ref().unwrap_or(&ess_point_status::PCT_HZ_DROOP)
    }
    fn pct_hz_droop_mut(&mut self) -> &mut f32 {
        self._ess_point_status_mut().pct_hz_droop.get_or_insert(Default::default())
    }
    fn pct_v_droop(&self) -> &f32 {
        self._ess_point_status().pct_v_droop.as_ref().unwrap_or(&ess_point_status::PCT_V_DROOP)
    }
    fn pct_v_droop_mut(&mut self) -> &mut f32 {
        self._ess_point_status_mut().pct_v_droop.get_or_insert(Default::default())
    }
    fn ramp_rates(&self) -> &super::commonmodule::RampRate {
        self._ess_point_status().ramp_rates.as_ref().unwrap_or(&ess_point_status::RAMP_RATES)
    }
    fn ramp_rates_mut(&mut self) -> &mut super::commonmodule::RampRate {
        self._ess_point_status_mut().ramp_rates.get_or_insert(Default::default())
    }
    fn reactive_pwr_set_point_enabled(&self) -> &super::commonmodule::StatusDps {
        self._ess_point_status().reactive_pwr_set_point_enabled.as_ref().unwrap_or(&ess_point_status::REACTIVE_PWR_SET_POINT_ENABLED)
    }
    fn reactive_pwr_set_point_enabled_mut(&mut self) -> &mut super::commonmodule::StatusDps {
        self._ess_point_status_mut().reactive_pwr_set_point_enabled.get_or_insert(Default::default())
    }
    fn real_pwr_set_point_enabled(&self) -> &super::commonmodule::StatusDps {
        self._ess_point_status().real_pwr_set_point_enabled.as_ref().unwrap_or(&ess_point_status::REAL_PWR_SET_POINT_ENABLED)
    }
    fn real_pwr_set_point_enabled_mut(&mut self) -> &mut super::commonmodule::StatusDps {
        self._ess_point_status_mut().real_pwr_set_point_enabled.get_or_insert(Default::default())
    }
    fn state(&self) -> &super::commonmodule::OptionalStateKind {
        self._ess_point_status().state.as_ref().unwrap_or(&ess_point_status::STATE)
    }
    fn state_mut(&mut self) -> &mut super::commonmodule::OptionalStateKind {
        self._ess_point_status_mut().state.get_or_insert(Default::default())
    }
    fn sync_back_to_grid(&self) -> &super::commonmodule::StatusDps {
        self._ess_point_status().sync_back_to_grid.as_ref().unwrap_or(&ess_point_status::SYNC_BACK_TO_GRID)
    }
    fn sync_back_to_grid_mut(&mut self) -> &mut super::commonmodule::StatusDps {
        self._ess_point_status_mut().sync_back_to_grid.get_or_insert(Default::default())
    }
    fn trans_to_islnd_on_grid_loss_enabled(&self) -> &super::commonmodule::StatusDps {
        self._ess_point_status().trans_to_islnd_on_grid_loss_enabled.as_ref().unwrap_or(&ess_point_status::TRANS_TO_ISLND_ON_GRID_LOSS_ENABLED)
    }
    fn trans_to_islnd_on_grid_loss_enabled_mut(&mut self) -> &mut super::commonmodule::StatusDps {
        self._ess_point_status_mut().trans_to_islnd_on_grid_loss_enabled.get_or_insert(Default::default())
    }
    fn voltage_set_point_enabled(&self) -> &super::commonmodule::StatusDps {
        self._ess_point_status().voltage_set_point_enabled.as_ref().unwrap_or(&ess_point_status::VOLTAGE_SET_POINT_ENABLED)
    }
    fn voltage_set_point_enabled_mut(&mut self) -> &mut super::commonmodule::StatusDps {
        self._ess_point_status_mut().voltage_set_point_enabled.get_or_insert(Default::default())
    }
}
impl IsEssPointStatus for EssPointStatus {
    fn _ess_point_status(&self) -> &EssPointStatus {
        self
    }
    fn _ess_point_status_mut(&mut self) -> &mut EssPointStatus {
        self
    }
}
/// Specialized 61850 ZGEN class
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize)]
pub struct EssEventAndStatusZgen {
    /// UML inherited base object
    // parent_message: true
    // required_field: false
    // multiplicity_min: None
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, optional, tag="1")]
    #[serde(default, rename = "logicalNodeForEventAndStatus")]
    pub logical_node_for_event_and_status: ::std::option::Option<super::commonmodule::LogicalNodeForEventAndStatus>,
    /// DC Power On/Off Status; True = DC power on
    #[prost(message, optional, tag="2")]
    #[serde(default, rename = "AuxPwrSt")]
    pub aux_pwr_st: ::std::option::Option<super::commonmodule::StatusSps>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    #[serde(default, rename = "DynamicTest")]
    pub dynamic_test: ::std::option::Option<super::commonmodule::EnsDynamicTestKind>,
    /// Emergency stop
    #[prost(message, optional, tag="4")]
    #[serde(default, rename = "EmgStop")]
    pub emg_stop: ::std::option::Option<super::commonmodule::StatusSps>,
    /// Generator is synchronized to EPS, or not; True = Synchronized
    #[prost(message, optional, tag="5")]
    #[serde(default, rename = "GnSynSt")]
    pub gn_syn_st: ::std::option::Option<super::commonmodule::StatusSps>,
    /// Point status
    #[prost(message, optional, tag="6")]
    #[serde(default, rename = "PointStatus")]
    pub point_status: ::std::option::Option<EssPointStatus>,
}
mod ess_event_and_status_zgen {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref LOGICAL_NODE_FOR_EVENT_AND_STATUS: crate::commonmodule::LogicalNodeForEventAndStatus = Default::default();
        pub(super) static ref AUX_PWR_ST: crate::commonmodule::StatusSps = Default::default();
        pub(super) static ref DYNAMIC_TEST: crate::commonmodule::EnsDynamicTestKind = Default::default();
        pub(super) static ref EMG_STOP: crate::commonmodule::StatusSps = Default::default();
        pub(super) static ref GN_SYN_ST: crate::commonmodule::StatusSps = Default::default();
        pub(super) static ref POINT_STATUS: crate::essmodule::EssPointStatus = Default::default();
    }
}
impl EssEventAndStatusZgen {
    pub(crate) fn parent(&self) -> &super::commonmodule::LogicalNodeForEventAndStatus {
        self.logical_node_for_event_and_status.as_ref().unwrap_or(&ess_event_and_status_zgen::LOGICAL_NODE_FOR_EVENT_AND_STATUS)
    }
    pub(crate) fn parent_mut(&mut self) -> &mut super::commonmodule::LogicalNodeForEventAndStatus {
        self.logical_node_for_event_and_status.get_or_insert(Default::default())
    }
}
pub trait IsEssEventAndStatusZgen {
    fn _ess_event_and_status_zgen(&self) -> &EssEventAndStatusZgen;
    fn _ess_event_and_status_zgen_mut(&mut self) -> &mut EssEventAndStatusZgen;
    fn logical_node_for_event_and_status(&self) -> &super::commonmodule::LogicalNodeForEventAndStatus {
        self._ess_event_and_status_zgen().logical_node_for_event_and_status.as_ref().unwrap_or(&ess_event_and_status_zgen::LOGICAL_NODE_FOR_EVENT_AND_STATUS)
    }
    fn logical_node_for_event_and_status_mut(&mut self) -> &mut super::commonmodule::LogicalNodeForEventAndStatus {
        self._ess_event_and_status_zgen_mut().logical_node_for_event_and_status.get_or_insert(Default::default())
    }
    fn aux_pwr_st(&self) -> &super::commonmodule::StatusSps {
        self._ess_event_and_status_zgen().aux_pwr_st.as_ref().unwrap_or(&ess_event_and_status_zgen::AUX_PWR_ST)
    }
    fn aux_pwr_st_mut(&mut self) -> &mut super::commonmodule::StatusSps {
        self._ess_event_and_status_zgen_mut().aux_pwr_st.get_or_insert(Default::default())
    }
    fn dynamic_test(&self) -> &super::commonmodule::EnsDynamicTestKind {
        self._ess_event_and_status_zgen().dynamic_test.as_ref().unwrap_or(&ess_event_and_status_zgen::DYNAMIC_TEST)
    }
    fn dynamic_test_mut(&mut self) -> &mut super::commonmodule::EnsDynamicTestKind {
        self._ess_event_and_status_zgen_mut().dynamic_test.get_or_insert(Default::default())
    }
    fn emg_stop(&self) -> &super::commonmodule::StatusSps {
        self._ess_event_and_status_zgen().emg_stop.as_ref().unwrap_or(&ess_event_and_status_zgen::EMG_STOP)
    }
    fn emg_stop_mut(&mut self) -> &mut super::commonmodule::StatusSps {
        self._ess_event_and_status_zgen_mut().emg_stop.get_or_insert(Default::default())
    }
    fn gn_syn_st(&self) -> &super::commonmodule::StatusSps {
        self._ess_event_and_status_zgen().gn_syn_st.as_ref().unwrap_or(&ess_event_and_status_zgen::GN_SYN_ST)
    }
    fn gn_syn_st_mut(&mut self) -> &mut super::commonmodule::StatusSps {
        self._ess_event_and_status_zgen_mut().gn_syn_st.get_or_insert(Default::default())
    }
    fn point_status(&self) -> &EssPointStatus {
        self._ess_event_and_status_zgen().point_status.as_ref().unwrap_or(&ess_event_and_status_zgen::POINT_STATUS)
    }
    fn point_status_mut(&mut self) -> &mut EssPointStatus {
        self._ess_event_and_status_zgen_mut().point_status.get_or_insert(Default::default())
    }
}
impl IsEssEventAndStatusZgen for EssEventAndStatusZgen {
    fn _ess_event_and_status_zgen(&self) -> &EssEventAndStatusZgen {
        self
    }
    fn _ess_event_and_status_zgen_mut(&mut self) -> &mut EssEventAndStatusZgen {
        self
    }
}
impl IsLogicalNodeForEventAndStatus for EssEventAndStatusZgen {
    fn _logical_node_for_event_and_status(&self) -> &super::commonmodule::LogicalNodeForEventAndStatus {
        self.parent()
    }
    fn _logical_node_for_event_and_status_mut(&mut self) -> &mut LogicalNodeForEventAndStatus {
        self.parent_mut()
    }
}
impl IsLogicalNode for EssEventAndStatusZgen {
    fn _logical_node(&self) -> &super::commonmodule::LogicalNode {
        self.parent().parent()
    }
    fn _logical_node_mut(&mut self) -> &mut LogicalNode {
        self.parent_mut().parent_mut()
    }
}
impl IsIdentifiedObject for EssEventAndStatusZgen {
    fn _identified_object(&self) -> &super::commonmodule::IdentifiedObject {
        self.parent().parent().parent()
    }
    fn _identified_object_mut(&mut self) -> &mut IdentifiedObject {
        self.parent_mut().parent_mut().parent_mut()
    }
}
/// Specialized 61850 ZGEN class for ESS event profile
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize)]
pub struct EssEventZgen {
    /// UML inherited base object
    // parent_message: true
    // required_field: false
    // multiplicity_min: None
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, optional, tag="1")]
    #[serde(default, rename = "eSSEventAndStatusZGEN")]
    pub e_ss_event_and_status_zgen: ::std::option::Option<EssEventAndStatusZgen>,
}
mod ess_event_zgen {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref E_SS_EVENT_AND_STATUS_ZGEN: crate::essmodule::EssEventAndStatusZgen = Default::default();
    }
}
impl EssEventZgen {
    pub(crate) fn parent(&self) -> &EssEventAndStatusZgen {
        self.e_ss_event_and_status_zgen.as_ref().unwrap_or(&ess_event_zgen::E_SS_EVENT_AND_STATUS_ZGEN)
    }
    pub(crate) fn parent_mut(&mut self) -> &mut EssEventAndStatusZgen {
        self.e_ss_event_and_status_zgen.get_or_insert(Default::default())
    }
}
pub trait IsEssEventZgen {
    fn _ess_event_zgen(&self) -> &EssEventZgen;
    fn _ess_event_zgen_mut(&mut self) -> &mut EssEventZgen;
    fn e_ss_event_and_status_zgen(&self) -> &EssEventAndStatusZgen {
        self._ess_event_zgen().e_ss_event_and_status_zgen.as_ref().unwrap_or(&ess_event_zgen::E_SS_EVENT_AND_STATUS_ZGEN)
    }
    fn e_ss_event_and_status_zgen_mut(&mut self) -> &mut EssEventAndStatusZgen {
        self._ess_event_zgen_mut().e_ss_event_and_status_zgen.get_or_insert(Default::default())
    }
}
impl IsEssEventZgen for EssEventZgen {
    fn _ess_event_zgen(&self) -> &EssEventZgen {
        self
    }
    fn _ess_event_zgen_mut(&mut self) -> &mut EssEventZgen {
        self
    }
}
impl IsEssEventAndStatusZgen for EssEventZgen {
    fn _ess_event_and_status_zgen(&self) -> &EssEventAndStatusZgen {
        self.parent()
    }
    fn _ess_event_and_status_zgen_mut(&mut self) -> &mut EssEventAndStatusZgen {
        self.parent_mut()
    }
}
impl IsLogicalNodeForEventAndStatus for EssEventZgen {
    fn _logical_node_for_event_and_status(&self) -> &super::commonmodule::LogicalNodeForEventAndStatus {
        self.parent().parent()
    }
    fn _logical_node_for_event_and_status_mut(&mut self) -> &mut LogicalNodeForEventAndStatus {
        self.parent_mut().parent_mut()
    }
}
impl IsLogicalNode for EssEventZgen {
    fn _logical_node(&self) -> &super::commonmodule::LogicalNode {
        self.parent().parent().parent()
    }
    fn _logical_node_mut(&mut self) -> &mut LogicalNode {
        self.parent_mut().parent_mut().parent_mut()
    }
}
impl IsIdentifiedObject for EssEventZgen {
    fn _identified_object(&self) -> &super::commonmodule::IdentifiedObject {
        self.parent().parent().parent().parent()
    }
    fn _identified_object_mut(&mut self) -> &mut IdentifiedObject {
        self.parent_mut().parent_mut().parent_mut().parent_mut()
    }
}
/// ESS event
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize)]
pub struct EssEvent {
    /// UML inherited base object
    // parent_message: true
    // required_field: false
    // multiplicity_min: None
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, optional, tag="1")]
    #[serde(default, rename = "eventValue")]
    pub event_value: ::std::option::Option<super::commonmodule::EventValue>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    #[serde(default, rename = "essEventZBAT")]
    pub ess_event_zbat: ::std::option::Option<EssEventZbat>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    #[serde(default, rename = "essEventZGEN")]
    pub ess_event_zgen: ::std::option::Option<EssEventZgen>,
}
mod ess_event {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref EVENT_VALUE: crate::commonmodule::EventValue = Default::default();
        pub(super) static ref ESS_EVENT_ZBAT: crate::essmodule::EssEventZbat = Default::default();
        pub(super) static ref ESS_EVENT_ZGEN: crate::essmodule::EssEventZgen = Default::default();
    }
}
impl EssEvent {
    pub(crate) fn parent(&self) -> &super::commonmodule::EventValue {
        self.event_value.as_ref().unwrap_or(&ess_event::EVENT_VALUE)
    }
    pub(crate) fn parent_mut(&mut self) -> &mut super::commonmodule::EventValue {
        self.event_value.get_or_insert(Default::default())
    }
}
pub trait IsEssEvent {
    fn _ess_event(&self) -> &EssEvent;
    fn _ess_event_mut(&mut self) -> &mut EssEvent;
    fn event_value(&self) -> &super::commonmodule::EventValue {
        self._ess_event().event_value.as_ref().unwrap_or(&ess_event::EVENT_VALUE)
    }
    fn event_value_mut(&mut self) -> &mut super::commonmodule::EventValue {
        self._ess_event_mut().event_value.get_or_insert(Default::default())
    }
    fn ess_event_zbat(&self) -> &EssEventZbat {
        self._ess_event().ess_event_zbat.as_ref().unwrap_or(&ess_event::ESS_EVENT_ZBAT)
    }
    fn ess_event_zbat_mut(&mut self) -> &mut EssEventZbat {
        self._ess_event_mut().ess_event_zbat.get_or_insert(Default::default())
    }
    fn ess_event_zgen(&self) -> &EssEventZgen {
        self._ess_event().ess_event_zgen.as_ref().unwrap_or(&ess_event::ESS_EVENT_ZGEN)
    }
    fn ess_event_zgen_mut(&mut self) -> &mut EssEventZgen {
        self._ess_event_mut().ess_event_zgen.get_or_insert(Default::default())
    }
}
impl IsEssEvent for EssEvent {
    fn _ess_event(&self) -> &EssEvent {
        self
    }
    fn _ess_event_mut(&mut self) -> &mut EssEvent {
        self
    }
}
impl IsEventValue for EssEvent {
    fn _event_value(&self) -> &super::commonmodule::EventValue {
        self.parent()
    }
    fn _event_value_mut(&mut self) -> &mut EventValue {
        self.parent_mut()
    }
}
impl IsIdentifiedObject for EssEvent {
    fn _identified_object(&self) -> &super::commonmodule::IdentifiedObject {
        self.parent().parent()
    }
    fn _identified_object_mut(&mut self) -> &mut IdentifiedObject {
        self.parent_mut().parent_mut()
    }
}
/// ESS event profile
/// OpenFMB Profile Message: true
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize)]
pub struct EssEventProfile {
    /// UML inherited base object
    // parent_message: true
    // required_field: false
    // multiplicity_min: None
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, optional, tag="1")]
    #[serde(default, rename = "eventMessageInfo")]
    pub event_message_info: ::std::option::Option<super::commonmodule::EventMessageInfo>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: Some(1)
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, optional, tag="2")]
    #[serde(default)]
    pub ess: ::std::option::Option<super::commonmodule::Ess>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: Some(1)
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, optional, tag="3")]
    #[serde(default, rename = "essEvent")]
    pub ess_event: ::std::option::Option<EssEvent>,
}
mod ess_event_profile {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref EVENT_MESSAGE_INFO: crate::commonmodule::EventMessageInfo = Default::default();
        pub(super) static ref ESS: crate::commonmodule::Ess = Default::default();
        pub(super) static ref ESS_EVENT: crate::essmodule::EssEvent = Default::default();
    }
}
impl EssEventProfile {
    pub(crate) fn parent(&self) -> &super::commonmodule::EventMessageInfo {
        self.event_message_info.as_ref().unwrap_or(&ess_event_profile::EVENT_MESSAGE_INFO)
    }
    pub(crate) fn parent_mut(&mut self) -> &mut super::commonmodule::EventMessageInfo {
        self.event_message_info.get_or_insert(Default::default())
    }
}
pub trait IsEssEventProfile {
    fn _ess_event_profile(&self) -> &EssEventProfile;
    fn _ess_event_profile_mut(&mut self) -> &mut EssEventProfile;
    fn event_message_info(&self) -> &super::commonmodule::EventMessageInfo {
        self._ess_event_profile().event_message_info.as_ref().unwrap_or(&ess_event_profile::EVENT_MESSAGE_INFO)
    }
    fn event_message_info_mut(&mut self) -> &mut super::commonmodule::EventMessageInfo {
        self._ess_event_profile_mut().event_message_info.get_or_insert(Default::default())
    }
    fn ess(&self) -> &super::commonmodule::Ess {
        self._ess_event_profile().ess.as_ref().unwrap_or(&ess_event_profile::ESS)
    }
    fn ess_mut(&mut self) -> &mut super::commonmodule::Ess {
        self._ess_event_profile_mut().ess.get_or_insert(Default::default())
    }
    fn ess_event(&self) -> &EssEvent {
        self._ess_event_profile().ess_event.as_ref().unwrap_or(&ess_event_profile::ESS_EVENT)
    }
    fn ess_event_mut(&mut self) -> &mut EssEvent {
        self._ess_event_profile_mut().ess_event.get_or_insert(Default::default())
    }
}
impl IsEssEventProfile for EssEventProfile {
    fn _ess_event_profile(&self) -> &EssEventProfile {
        self
    }
    fn _ess_event_profile_mut(&mut self) -> &mut EssEventProfile {
        self
    }
}
impl IsEventMessageInfo for EssEventProfile {
    fn _event_message_info(&self) -> &super::commonmodule::EventMessageInfo {
        self.parent()
    }
    fn _event_message_info_mut(&mut self) -> &mut EventMessageInfo {
        self.parent_mut()
    }
}
impl IsMessageInfo for EssEventProfile {
    fn _message_info(&self) -> &super::commonmodule::MessageInfo {
        self.parent().parent()
    }
    fn _message_info_mut(&mut self) -> &mut MessageInfo {
        self.parent_mut().parent_mut()
    }
}
impl IsIdentifiedObject for EssEventProfile {
    fn _identified_object(&self) -> &super::commonmodule::IdentifiedObject {
        self.parent().parent().parent()
    }
    fn _identified_object_mut(&mut self) -> &mut IdentifiedObject {
        self.parent_mut().parent_mut().parent_mut()
    }
}
/// ESS reading value
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize)]
pub struct EssReading {
    /// UML inherited base object
    // parent_message: true
    // required_field: false
    // multiplicity_min: None
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, optional, tag="1")]
    #[serde(default, rename = "conductingEquipmentTerminalReading")]
    pub conducting_equipment_terminal_reading: ::std::option::Option<super::commonmodule::ConductingEquipmentTerminalReading>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    #[serde(default, rename = "phaseMMTN")]
    pub phase_mmtn: ::std::option::Option<super::commonmodule::PhaseMmtn>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    #[serde(default, rename = "readingMMTR")]
    pub reading_mmtr: ::std::option::Option<super::commonmodule::ReadingMmtr>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="4")]
    #[serde(default, rename = "readingMMXU")]
    pub reading_mmxu: ::std::option::Option<super::commonmodule::ReadingMmxu>,
}
mod ess_reading {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref CONDUCTING_EQUIPMENT_TERMINAL_READING: crate::commonmodule::ConductingEquipmentTerminalReading = Default::default();
        pub(super) static ref PHASE_MMTN: crate::commonmodule::PhaseMmtn = Default::default();
        pub(super) static ref READING_MMTR: crate::commonmodule::ReadingMmtr = Default::default();
        pub(super) static ref READING_MMXU: crate::commonmodule::ReadingMmxu = Default::default();
    }
}
impl EssReading {
    pub(crate) fn parent(&self) -> &super::commonmodule::ConductingEquipmentTerminalReading {
        self.conducting_equipment_terminal_reading.as_ref().unwrap_or(&ess_reading::CONDUCTING_EQUIPMENT_TERMINAL_READING)
    }
    pub(crate) fn parent_mut(&mut self) -> &mut super::commonmodule::ConductingEquipmentTerminalReading {
        self.conducting_equipment_terminal_reading.get_or_insert(Default::default())
    }
}
pub trait IsEssReading {
    fn _ess_reading(&self) -> &EssReading;
    fn _ess_reading_mut(&mut self) -> &mut EssReading;
    fn conducting_equipment_terminal_reading(&self) -> &super::commonmodule::ConductingEquipmentTerminalReading {
        self._ess_reading().conducting_equipment_terminal_reading.as_ref().unwrap_or(&ess_reading::CONDUCTING_EQUIPMENT_TERMINAL_READING)
    }
    fn conducting_equipment_terminal_reading_mut(&mut self) -> &mut super::commonmodule::ConductingEquipmentTerminalReading {
        self._ess_reading_mut().conducting_equipment_terminal_reading.get_or_insert(Default::default())
    }
    fn phase_mmtn(&self) -> &super::commonmodule::PhaseMmtn {
        self._ess_reading().phase_mmtn.as_ref().unwrap_or(&ess_reading::PHASE_MMTN)
    }
    fn phase_mmtn_mut(&mut self) -> &mut super::commonmodule::PhaseMmtn {
        self._ess_reading_mut().phase_mmtn.get_or_insert(Default::default())
    }
    fn reading_mmtr(&self) -> &super::commonmodule::ReadingMmtr {
        self._ess_reading().reading_mmtr.as_ref().unwrap_or(&ess_reading::READING_MMTR)
    }
    fn reading_mmtr_mut(&mut self) -> &mut super::commonmodule::ReadingMmtr {
        self._ess_reading_mut().reading_mmtr.get_or_insert(Default::default())
    }
    fn reading_mmxu(&self) -> &super::commonmodule::ReadingMmxu {
        self._ess_reading().reading_mmxu.as_ref().unwrap_or(&ess_reading::READING_MMXU)
    }
    fn reading_mmxu_mut(&mut self) -> &mut super::commonmodule::ReadingMmxu {
        self._ess_reading_mut().reading_mmxu.get_or_insert(Default::default())
    }
}
impl IsEssReading for EssReading {
    fn _ess_reading(&self) -> &EssReading {
        self
    }
    fn _ess_reading_mut(&mut self) -> &mut EssReading {
        self
    }
}
impl IsConductingEquipmentTerminalReading for EssReading {
    fn _conducting_equipment_terminal_reading(&self) -> &super::commonmodule::ConductingEquipmentTerminalReading {
        self.parent()
    }
    fn _conducting_equipment_terminal_reading_mut(&mut self) -> &mut ConductingEquipmentTerminalReading {
        self.parent_mut()
    }
}
/// ESS reading profile
/// OpenFMB Profile Message: true
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize)]
pub struct EssReadingProfile {
    /// UML inherited base object
    // parent_message: true
    // required_field: false
    // multiplicity_min: None
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, optional, tag="1")]
    #[serde(default, rename = "readingMessageInfo")]
    pub reading_message_info: ::std::option::Option<super::commonmodule::ReadingMessageInfo>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: Some(1)
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, optional, tag="2")]
    #[serde(default)]
    pub ess: ::std::option::Option<super::commonmodule::Ess>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: Some(1)
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, optional, tag="3")]
    #[serde(default, rename = "essReading")]
    pub ess_reading: ::std::option::Option<EssReading>,
}
mod ess_reading_profile {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref READING_MESSAGE_INFO: crate::commonmodule::ReadingMessageInfo = Default::default();
        pub(super) static ref ESS: crate::commonmodule::Ess = Default::default();
        pub(super) static ref ESS_READING: crate::essmodule::EssReading = Default::default();
    }
}
impl EssReadingProfile {
    pub(crate) fn parent(&self) -> &super::commonmodule::ReadingMessageInfo {
        self.reading_message_info.as_ref().unwrap_or(&ess_reading_profile::READING_MESSAGE_INFO)
    }
    pub(crate) fn parent_mut(&mut self) -> &mut super::commonmodule::ReadingMessageInfo {
        self.reading_message_info.get_or_insert(Default::default())
    }
}
pub trait IsEssReadingProfile {
    fn _ess_reading_profile(&self) -> &EssReadingProfile;
    fn _ess_reading_profile_mut(&mut self) -> &mut EssReadingProfile;
    fn reading_message_info(&self) -> &super::commonmodule::ReadingMessageInfo {
        self._ess_reading_profile().reading_message_info.as_ref().unwrap_or(&ess_reading_profile::READING_MESSAGE_INFO)
    }
    fn reading_message_info_mut(&mut self) -> &mut super::commonmodule::ReadingMessageInfo {
        self._ess_reading_profile_mut().reading_message_info.get_or_insert(Default::default())
    }
    fn ess(&self) -> &super::commonmodule::Ess {
        self._ess_reading_profile().ess.as_ref().unwrap_or(&ess_reading_profile::ESS)
    }
    fn ess_mut(&mut self) -> &mut super::commonmodule::Ess {
        self._ess_reading_profile_mut().ess.get_or_insert(Default::default())
    }
    fn ess_reading(&self) -> &EssReading {
        self._ess_reading_profile().ess_reading.as_ref().unwrap_or(&ess_reading_profile::ESS_READING)
    }
    fn ess_reading_mut(&mut self) -> &mut EssReading {
        self._ess_reading_profile_mut().ess_reading.get_or_insert(Default::default())
    }
}
impl IsEssReadingProfile for EssReadingProfile {
    fn _ess_reading_profile(&self) -> &EssReadingProfile {
        self
    }
    fn _ess_reading_profile_mut(&mut self) -> &mut EssReadingProfile {
        self
    }
}
impl IsReadingMessageInfo for EssReadingProfile {
    fn _reading_message_info(&self) -> &super::commonmodule::ReadingMessageInfo {
        self.parent()
    }
    fn _reading_message_info_mut(&mut self) -> &mut ReadingMessageInfo {
        self.parent_mut()
    }
}
impl IsMessageInfo for EssReadingProfile {
    fn _message_info(&self) -> &super::commonmodule::MessageInfo {
        self.parent().parent()
    }
    fn _message_info_mut(&mut self) -> &mut MessageInfo {
        self.parent_mut().parent_mut()
    }
}
impl IsIdentifiedObject for EssReadingProfile {
    fn _identified_object(&self) -> &super::commonmodule::IdentifiedObject {
        self.parent().parent().parent()
    }
    fn _identified_object_mut(&mut self) -> &mut IdentifiedObject {
        self.parent_mut().parent_mut().parent_mut()
    }
}
/// Specialized 61850 ZBAT
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize)]
pub struct EssStatusZbat {
    /// UML inherited base object
    // parent_message: true
    // required_field: false
    // multiplicity_min: None
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, optional, tag="1")]
    #[serde(default, rename = "logicalNodeForEventAndStatus")]
    pub logical_node_for_event_and_status: ::std::option::Option<super::commonmodule::LogicalNodeForEventAndStatus>,
    /// Battery system status &ndash; True: on
    #[prost(message, optional, tag="2")]
    #[serde(default, rename = "BatSt")]
    pub bat_st: ::std::option::Option<super::commonmodule::StatusSps>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    #[serde(default, rename = "GriMod")]
    pub gri_mod: ::std::option::Option<super::commonmodule::EngGridConnectModeKind>,
    /// State of charge (in percentage)
    #[prost(message, optional, tag="4")]
    #[serde(default, rename = "Soc")]
    pub soc: ::std::option::Option<super::commonmodule::Mv>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="5")]
    #[serde(default, rename = "Stdby")]
    pub stdby: ::std::option::Option<super::commonmodule::StatusSps>,
}
mod ess_status_zbat {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref LOGICAL_NODE_FOR_EVENT_AND_STATUS: crate::commonmodule::LogicalNodeForEventAndStatus = Default::default();
        pub(super) static ref BAT_ST: crate::commonmodule::StatusSps = Default::default();
        pub(super) static ref GRI_MOD: crate::commonmodule::EngGridConnectModeKind = Default::default();
        pub(super) static ref SOC: crate::commonmodule::Mv = Default::default();
        pub(super) static ref STDBY: crate::commonmodule::StatusSps = Default::default();
    }
}
impl EssStatusZbat {
    pub(crate) fn parent(&self) -> &super::commonmodule::LogicalNodeForEventAndStatus {
        self.logical_node_for_event_and_status.as_ref().unwrap_or(&ess_status_zbat::LOGICAL_NODE_FOR_EVENT_AND_STATUS)
    }
    pub(crate) fn parent_mut(&mut self) -> &mut super::commonmodule::LogicalNodeForEventAndStatus {
        self.logical_node_for_event_and_status.get_or_insert(Default::default())
    }
}
pub trait IsEssStatusZbat {
    fn _ess_status_zbat(&self) -> &EssStatusZbat;
    fn _ess_status_zbat_mut(&mut self) -> &mut EssStatusZbat;
    fn logical_node_for_event_and_status(&self) -> &super::commonmodule::LogicalNodeForEventAndStatus {
        self._ess_status_zbat().logical_node_for_event_and_status.as_ref().unwrap_or(&ess_status_zbat::LOGICAL_NODE_FOR_EVENT_AND_STATUS)
    }
    fn logical_node_for_event_and_status_mut(&mut self) -> &mut super::commonmodule::LogicalNodeForEventAndStatus {
        self._ess_status_zbat_mut().logical_node_for_event_and_status.get_or_insert(Default::default())
    }
    fn bat_st(&self) -> &super::commonmodule::StatusSps {
        self._ess_status_zbat().bat_st.as_ref().unwrap_or(&ess_status_zbat::BAT_ST)
    }
    fn bat_st_mut(&mut self) -> &mut super::commonmodule::StatusSps {
        self._ess_status_zbat_mut().bat_st.get_or_insert(Default::default())
    }
    fn gri_mod(&self) -> &super::commonmodule::EngGridConnectModeKind {
        self._ess_status_zbat().gri_mod.as_ref().unwrap_or(&ess_status_zbat::GRI_MOD)
    }
    fn gri_mod_mut(&mut self) -> &mut super::commonmodule::EngGridConnectModeKind {
        self._ess_status_zbat_mut().gri_mod.get_or_insert(Default::default())
    }
    fn soc(&self) -> &super::commonmodule::Mv {
        self._ess_status_zbat().soc.as_ref().unwrap_or(&ess_status_zbat::SOC)
    }
    fn soc_mut(&mut self) -> &mut super::commonmodule::Mv {
        self._ess_status_zbat_mut().soc.get_or_insert(Default::default())
    }
    fn stdby(&self) -> &super::commonmodule::StatusSps {
        self._ess_status_zbat().stdby.as_ref().unwrap_or(&ess_status_zbat::STDBY)
    }
    fn stdby_mut(&mut self) -> &mut super::commonmodule::StatusSps {
        self._ess_status_zbat_mut().stdby.get_or_insert(Default::default())
    }
}
impl IsEssStatusZbat for EssStatusZbat {
    fn _ess_status_zbat(&self) -> &EssStatusZbat {
        self
    }
    fn _ess_status_zbat_mut(&mut self) -> &mut EssStatusZbat {
        self
    }
}
impl IsLogicalNodeForEventAndStatus for EssStatusZbat {
    fn _logical_node_for_event_and_status(&self) -> &super::commonmodule::LogicalNodeForEventAndStatus {
        self.parent()
    }
    fn _logical_node_for_event_and_status_mut(&mut self) -> &mut LogicalNodeForEventAndStatus {
        self.parent_mut()
    }
}
impl IsLogicalNode for EssStatusZbat {
    fn _logical_node(&self) -> &super::commonmodule::LogicalNode {
        self.parent().parent()
    }
    fn _logical_node_mut(&mut self) -> &mut LogicalNode {
        self.parent_mut().parent_mut()
    }
}
impl IsIdentifiedObject for EssStatusZbat {
    fn _identified_object(&self) -> &super::commonmodule::IdentifiedObject {
        self.parent().parent().parent()
    }
    fn _identified_object_mut(&mut self) -> &mut IdentifiedObject {
        self.parent_mut().parent_mut().parent_mut()
    }
}
/// Specialized 61850 ZGEN class
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize)]
pub struct EssStatusZgen {
    /// UML inherited base object
    // parent_message: true
    // required_field: false
    // multiplicity_min: None
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, optional, tag="1")]
    #[serde(default, rename = "eSSEventAndStatusZGEN")]
    pub e_ss_event_and_status_zgen: ::std::option::Option<EssEventAndStatusZgen>,
}
mod ess_status_zgen {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref E_SS_EVENT_AND_STATUS_ZGEN: crate::essmodule::EssEventAndStatusZgen = Default::default();
    }
}
impl EssStatusZgen {
    pub(crate) fn parent(&self) -> &EssEventAndStatusZgen {
        self.e_ss_event_and_status_zgen.as_ref().unwrap_or(&ess_status_zgen::E_SS_EVENT_AND_STATUS_ZGEN)
    }
    pub(crate) fn parent_mut(&mut self) -> &mut EssEventAndStatusZgen {
        self.e_ss_event_and_status_zgen.get_or_insert(Default::default())
    }
}
pub trait IsEssStatusZgen {
    fn _ess_status_zgen(&self) -> &EssStatusZgen;
    fn _ess_status_zgen_mut(&mut self) -> &mut EssStatusZgen;
    fn e_ss_event_and_status_zgen(&self) -> &EssEventAndStatusZgen {
        self._ess_status_zgen().e_ss_event_and_status_zgen.as_ref().unwrap_or(&ess_status_zgen::E_SS_EVENT_AND_STATUS_ZGEN)
    }
    fn e_ss_event_and_status_zgen_mut(&mut self) -> &mut EssEventAndStatusZgen {
        self._ess_status_zgen_mut().e_ss_event_and_status_zgen.get_or_insert(Default::default())
    }
}
impl IsEssStatusZgen for EssStatusZgen {
    fn _ess_status_zgen(&self) -> &EssStatusZgen {
        self
    }
    fn _ess_status_zgen_mut(&mut self) -> &mut EssStatusZgen {
        self
    }
}
impl IsEssEventAndStatusZgen for EssStatusZgen {
    fn _ess_event_and_status_zgen(&self) -> &EssEventAndStatusZgen {
        self.parent()
    }
    fn _ess_event_and_status_zgen_mut(&mut self) -> &mut EssEventAndStatusZgen {
        self.parent_mut()
    }
}
impl IsLogicalNodeForEventAndStatus for EssStatusZgen {
    fn _logical_node_for_event_and_status(&self) -> &super::commonmodule::LogicalNodeForEventAndStatus {
        self.parent().parent()
    }
    fn _logical_node_for_event_and_status_mut(&mut self) -> &mut LogicalNodeForEventAndStatus {
        self.parent_mut().parent_mut()
    }
}
impl IsLogicalNode for EssStatusZgen {
    fn _logical_node(&self) -> &super::commonmodule::LogicalNode {
        self.parent().parent().parent()
    }
    fn _logical_node_mut(&mut self) -> &mut LogicalNode {
        self.parent_mut().parent_mut().parent_mut()
    }
}
impl IsIdentifiedObject for EssStatusZgen {
    fn _identified_object(&self) -> &super::commonmodule::IdentifiedObject {
        self.parent().parent().parent().parent()
    }
    fn _identified_object_mut(&mut self) -> &mut IdentifiedObject {
        self.parent_mut().parent_mut().parent_mut().parent_mut()
    }
}
/// ESS status
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize)]
pub struct EssStatus {
    /// UML inherited base object
    // parent_message: true
    // required_field: false
    // multiplicity_min: None
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, optional, tag="1")]
    #[serde(default, rename = "statusValue")]
    pub status_value: ::std::option::Option<super::commonmodule::StatusValue>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    #[serde(default, rename = "essStatusZBAT")]
    pub ess_status_zbat: ::std::option::Option<EssStatusZbat>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    #[serde(default, rename = "essStatusZGEN")]
    pub ess_status_zgen: ::std::option::Option<EssStatusZgen>,
}
mod ess_status {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref STATUS_VALUE: crate::commonmodule::StatusValue = Default::default();
        pub(super) static ref ESS_STATUS_ZBAT: crate::essmodule::EssStatusZbat = Default::default();
        pub(super) static ref ESS_STATUS_ZGEN: crate::essmodule::EssStatusZgen = Default::default();
    }
}
impl EssStatus {
    pub(crate) fn parent(&self) -> &super::commonmodule::StatusValue {
        self.status_value.as_ref().unwrap_or(&ess_status::STATUS_VALUE)
    }
    pub(crate) fn parent_mut(&mut self) -> &mut super::commonmodule::StatusValue {
        self.status_value.get_or_insert(Default::default())
    }
}
pub trait IsEssStatus {
    fn _ess_status(&self) -> &EssStatus;
    fn _ess_status_mut(&mut self) -> &mut EssStatus;
    fn status_value(&self) -> &super::commonmodule::StatusValue {
        self._ess_status().status_value.as_ref().unwrap_or(&ess_status::STATUS_VALUE)
    }
    fn status_value_mut(&mut self) -> &mut super::commonmodule::StatusValue {
        self._ess_status_mut().status_value.get_or_insert(Default::default())
    }
    fn ess_status_zbat(&self) -> &EssStatusZbat {
        self._ess_status().ess_status_zbat.as_ref().unwrap_or(&ess_status::ESS_STATUS_ZBAT)
    }
    fn ess_status_zbat_mut(&mut self) -> &mut EssStatusZbat {
        self._ess_status_mut().ess_status_zbat.get_or_insert(Default::default())
    }
    fn ess_status_zgen(&self) -> &EssStatusZgen {
        self._ess_status().ess_status_zgen.as_ref().unwrap_or(&ess_status::ESS_STATUS_ZGEN)
    }
    fn ess_status_zgen_mut(&mut self) -> &mut EssStatusZgen {
        self._ess_status_mut().ess_status_zgen.get_or_insert(Default::default())
    }
}
impl IsEssStatus for EssStatus {
    fn _ess_status(&self) -> &EssStatus {
        self
    }
    fn _ess_status_mut(&mut self) -> &mut EssStatus {
        self
    }
}
impl IsStatusValue for EssStatus {
    fn _status_value(&self) -> &super::commonmodule::StatusValue {
        self.parent()
    }
    fn _status_value_mut(&mut self) -> &mut StatusValue {
        self.parent_mut()
    }
}
impl IsIdentifiedObject for EssStatus {
    fn _identified_object(&self) -> &super::commonmodule::IdentifiedObject {
        self.parent().parent()
    }
    fn _identified_object_mut(&mut self) -> &mut IdentifiedObject {
        self.parent_mut().parent_mut()
    }
}
/// ESS status profile
/// OpenFMB Profile Message: true
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize)]
pub struct EssStatusProfile {
    /// UML inherited base object
    // parent_message: true
    // required_field: false
    // multiplicity_min: None
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, optional, tag="1")]
    #[serde(default, rename = "statusMessageInfo")]
    pub status_message_info: ::std::option::Option<super::commonmodule::StatusMessageInfo>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: Some(1)
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, optional, tag="2")]
    #[serde(default)]
    pub ess: ::std::option::Option<super::commonmodule::Ess>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: Some(1)
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, optional, tag="3")]
    #[serde(default, rename = "essStatus")]
    pub ess_status: ::std::option::Option<EssStatus>,
}
mod ess_status_profile {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref STATUS_MESSAGE_INFO: crate::commonmodule::StatusMessageInfo = Default::default();
        pub(super) static ref ESS: crate::commonmodule::Ess = Default::default();
        pub(super) static ref ESS_STATUS: crate::essmodule::EssStatus = Default::default();
    }
}
impl EssStatusProfile {
    pub(crate) fn parent(&self) -> &super::commonmodule::StatusMessageInfo {
        self.status_message_info.as_ref().unwrap_or(&ess_status_profile::STATUS_MESSAGE_INFO)
    }
    pub(crate) fn parent_mut(&mut self) -> &mut super::commonmodule::StatusMessageInfo {
        self.status_message_info.get_or_insert(Default::default())
    }
}
pub trait IsEssStatusProfile {
    fn _ess_status_profile(&self) -> &EssStatusProfile;
    fn _ess_status_profile_mut(&mut self) -> &mut EssStatusProfile;
    fn status_message_info(&self) -> &super::commonmodule::StatusMessageInfo {
        self._ess_status_profile().status_message_info.as_ref().unwrap_or(&ess_status_profile::STATUS_MESSAGE_INFO)
    }
    fn status_message_info_mut(&mut self) -> &mut super::commonmodule::StatusMessageInfo {
        self._ess_status_profile_mut().status_message_info.get_or_insert(Default::default())
    }
    fn ess(&self) -> &super::commonmodule::Ess {
        self._ess_status_profile().ess.as_ref().unwrap_or(&ess_status_profile::ESS)
    }
    fn ess_mut(&mut self) -> &mut super::commonmodule::Ess {
        self._ess_status_profile_mut().ess.get_or_insert(Default::default())
    }
    fn ess_status(&self) -> &EssStatus {
        self._ess_status_profile().ess_status.as_ref().unwrap_or(&ess_status_profile::ESS_STATUS)
    }
    fn ess_status_mut(&mut self) -> &mut EssStatus {
        self._ess_status_profile_mut().ess_status.get_or_insert(Default::default())
    }
}
impl IsEssStatusProfile for EssStatusProfile {
    fn _ess_status_profile(&self) -> &EssStatusProfile {
        self
    }
    fn _ess_status_profile_mut(&mut self) -> &mut EssStatusProfile {
        self
    }
}
impl IsStatusMessageInfo for EssStatusProfile {
    fn _status_message_info(&self) -> &super::commonmodule::StatusMessageInfo {
        self.parent()
    }
    fn _status_message_info_mut(&mut self) -> &mut StatusMessageInfo {
        self.parent_mut()
    }
}
impl IsMessageInfo for EssStatusProfile {
    fn _message_info(&self) -> &super::commonmodule::MessageInfo {
        self.parent().parent()
    }
    fn _message_info_mut(&mut self) -> &mut MessageInfo {
        self.parent_mut().parent_mut()
    }
}
impl IsIdentifiedObject for EssStatusProfile {
    fn _identified_object(&self) -> &super::commonmodule::IdentifiedObject {
        self.parent().parent().parent()
    }
    fn _identified_object_mut(&mut self) -> &mut IdentifiedObject {
        self.parent_mut().parent_mut().parent_mut()
    }
}
/// Point definition (Point)
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize)]
pub struct EssPoint {
    /// Black start enable
    #[prost(message, optional, tag="1")]
    #[serde(default, rename = "blackStartEnabled")]
    pub black_start_enabled: ::std::option::Option<super::commonmodule::ControlDpc>,
    /// Enable frequency set point
    #[prost(message, optional, tag="2")]
    #[serde(default, rename = "frequencySetPointEnabled")]
    pub frequency_set_point_enabled: ::std::option::Option<super::commonmodule::ControlDpc>,
    /// ESS function parameter
    #[prost(message, optional, tag="3")]
    #[serde(default)]
    pub function: ::std::option::Option<EssFunction>,
    /// Grid connect mode
    #[prost(message, optional, tag="4")]
    #[serde(default)]
    pub mode: ::std::option::Option<super::commonmodule::EngGridConnectModeKind>,
    /// Black start enable
    #[prost(message, optional, tag="5")]
    #[serde(default, rename = "pctHzDroop")]
    pub pct_hz_droop: ::std::option::Option<f32>,
    /// Black start enable
    #[prost(message, optional, tag="6")]
    #[serde(default, rename = "pctVDroop")]
    pub pct_v_droop: ::std::option::Option<f32>,
    /// Ramp rates
    #[prost(message, optional, tag="7")]
    #[serde(default, rename = "rampRates")]
    pub ramp_rates: ::std::option::Option<super::commonmodule::RampRate>,
    /// Enable reactive power set point
    #[prost(message, optional, tag="8")]
    #[serde(default, rename = "reactivePwrSetPointEnabled")]
    pub reactive_pwr_set_point_enabled: ::std::option::Option<super::commonmodule::ControlDpc>,
    /// Enable real power set point
    #[prost(message, optional, tag="9")]
    #[serde(default, rename = "realPwrSetPointEnabled")]
    pub real_pwr_set_point_enabled: ::std::option::Option<super::commonmodule::ControlDpc>,
    /// Reset device
    #[prost(message, optional, tag="10")]
    #[serde(default)]
    pub reset: ::std::option::Option<super::commonmodule::ControlDpc>,
    /// ESS state
    #[prost(message, optional, tag="11")]
    #[serde(default)]
    pub state: ::std::option::Option<super::commonmodule::OptionalStateKind>,
    /// Synchronize back to grid
    #[prost(message, optional, tag="12")]
    #[serde(default, rename = "syncBackToGrid")]
    pub sync_back_to_grid: ::std::option::Option<super::commonmodule::ControlDpc>,
    /// Transition to island on grid loss enable
    #[prost(message, optional, tag="13")]
    #[serde(default, rename = "transToIslndOnGridLossEnabled")]
    pub trans_to_islnd_on_grid_loss_enabled: ::std::option::Option<super::commonmodule::ControlDpc>,
    /// Enable voltage set point
    #[prost(message, optional, tag="14")]
    #[serde(default, rename = "voltageSetPointEnabled")]
    pub voltage_set_point_enabled: ::std::option::Option<super::commonmodule::ControlDpc>,
    /// Start time
    // parent_message: false
    // required_field: true
    // multiplicity_min: Some(1)
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, optional, tag="15")]
    #[serde(default, rename = "startTime")]
    pub start_time: ::std::option::Option<super::commonmodule::ControlTimestamp>,
}
mod ess_point {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref BLACK_START_ENABLED: crate::commonmodule::ControlDpc = Default::default();
        pub(super) static ref FREQUENCY_SET_POINT_ENABLED: crate::commonmodule::ControlDpc = Default::default();
        pub(super) static ref FUNCTION: crate::essmodule::EssFunction = Default::default();
        pub(super) static ref MODE: crate::commonmodule::EngGridConnectModeKind = Default::default();
        pub(super) static ref PCT_HZ_DROOP: f32 = Default::default();
        pub(super) static ref PCT_V_DROOP: f32 = Default::default();
        pub(super) static ref RAMP_RATES: crate::commonmodule::RampRate = Default::default();
        pub(super) static ref REACTIVE_PWR_SET_POINT_ENABLED: crate::commonmodule::ControlDpc = Default::default();
        pub(super) static ref REAL_PWR_SET_POINT_ENABLED: crate::commonmodule::ControlDpc = Default::default();
        pub(super) static ref RESET: crate::commonmodule::ControlDpc = Default::default();
        pub(super) static ref STATE: crate::commonmodule::OptionalStateKind = Default::default();
        pub(super) static ref SYNC_BACK_TO_GRID: crate::commonmodule::ControlDpc = Default::default();
        pub(super) static ref TRANS_TO_ISLND_ON_GRID_LOSS_ENABLED: crate::commonmodule::ControlDpc = Default::default();
        pub(super) static ref VOLTAGE_SET_POINT_ENABLED: crate::commonmodule::ControlDpc = Default::default();
        pub(super) static ref START_TIME: crate::commonmodule::ControlTimestamp = Default::default();
    }
}
impl EssPoint {
}
pub trait IsEssPoint {
    fn _ess_point(&self) -> &EssPoint;
    fn _ess_point_mut(&mut self) -> &mut EssPoint;
    fn black_start_enabled(&self) -> &super::commonmodule::ControlDpc {
        self._ess_point().black_start_enabled.as_ref().unwrap_or(&ess_point::BLACK_START_ENABLED)
    }
    fn black_start_enabled_mut(&mut self) -> &mut super::commonmodule::ControlDpc {
        self._ess_point_mut().black_start_enabled.get_or_insert(Default::default())
    }
    fn frequency_set_point_enabled(&self) -> &super::commonmodule::ControlDpc {
        self._ess_point().frequency_set_point_enabled.as_ref().unwrap_or(&ess_point::FREQUENCY_SET_POINT_ENABLED)
    }
    fn frequency_set_point_enabled_mut(&mut self) -> &mut super::commonmodule::ControlDpc {
        self._ess_point_mut().frequency_set_point_enabled.get_or_insert(Default::default())
    }
    fn function(&self) -> &EssFunction {
        self._ess_point().function.as_ref().unwrap_or(&ess_point::FUNCTION)
    }
    fn function_mut(&mut self) -> &mut EssFunction {
        self._ess_point_mut().function.get_or_insert(Default::default())
    }
    fn mode(&self) -> &super::commonmodule::EngGridConnectModeKind {
        self._ess_point().mode.as_ref().unwrap_or(&ess_point::MODE)
    }
    fn mode_mut(&mut self) -> &mut super::commonmodule::EngGridConnectModeKind {
        self._ess_point_mut().mode.get_or_insert(Default::default())
    }
    fn pct_hz_droop(&self) -> &f32 {
        self._ess_point().pct_hz_droop.as_ref().unwrap_or(&ess_point::PCT_HZ_DROOP)
    }
    fn pct_hz_droop_mut(&mut self) -> &mut f32 {
        self._ess_point_mut().pct_hz_droop.get_or_insert(Default::default())
    }
    fn pct_v_droop(&self) -> &f32 {
        self._ess_point().pct_v_droop.as_ref().unwrap_or(&ess_point::PCT_V_DROOP)
    }
    fn pct_v_droop_mut(&mut self) -> &mut f32 {
        self._ess_point_mut().pct_v_droop.get_or_insert(Default::default())
    }
    fn ramp_rates(&self) -> &super::commonmodule::RampRate {
        self._ess_point().ramp_rates.as_ref().unwrap_or(&ess_point::RAMP_RATES)
    }
    fn ramp_rates_mut(&mut self) -> &mut super::commonmodule::RampRate {
        self._ess_point_mut().ramp_rates.get_or_insert(Default::default())
    }
    fn reactive_pwr_set_point_enabled(&self) -> &super::commonmodule::ControlDpc {
        self._ess_point().reactive_pwr_set_point_enabled.as_ref().unwrap_or(&ess_point::REACTIVE_PWR_SET_POINT_ENABLED)
    }
    fn reactive_pwr_set_point_enabled_mut(&mut self) -> &mut super::commonmodule::ControlDpc {
        self._ess_point_mut().reactive_pwr_set_point_enabled.get_or_insert(Default::default())
    }
    fn real_pwr_set_point_enabled(&self) -> &super::commonmodule::ControlDpc {
        self._ess_point().real_pwr_set_point_enabled.as_ref().unwrap_or(&ess_point::REAL_PWR_SET_POINT_ENABLED)
    }
    fn real_pwr_set_point_enabled_mut(&mut self) -> &mut super::commonmodule::ControlDpc {
        self._ess_point_mut().real_pwr_set_point_enabled.get_or_insert(Default::default())
    }
    fn reset(&self) -> &super::commonmodule::ControlDpc {
        self._ess_point().reset.as_ref().unwrap_or(&ess_point::RESET)
    }
    fn reset_mut(&mut self) -> &mut super::commonmodule::ControlDpc {
        self._ess_point_mut().reset.get_or_insert(Default::default())
    }
    fn state(&self) -> &super::commonmodule::OptionalStateKind {
        self._ess_point().state.as_ref().unwrap_or(&ess_point::STATE)
    }
    fn state_mut(&mut self) -> &mut super::commonmodule::OptionalStateKind {
        self._ess_point_mut().state.get_or_insert(Default::default())
    }
    fn sync_back_to_grid(&self) -> &super::commonmodule::ControlDpc {
        self._ess_point().sync_back_to_grid.as_ref().unwrap_or(&ess_point::SYNC_BACK_TO_GRID)
    }
    fn sync_back_to_grid_mut(&mut self) -> &mut super::commonmodule::ControlDpc {
        self._ess_point_mut().sync_back_to_grid.get_or_insert(Default::default())
    }
    fn trans_to_islnd_on_grid_loss_enabled(&self) -> &super::commonmodule::ControlDpc {
        self._ess_point().trans_to_islnd_on_grid_loss_enabled.as_ref().unwrap_or(&ess_point::TRANS_TO_ISLND_ON_GRID_LOSS_ENABLED)
    }
    fn trans_to_islnd_on_grid_loss_enabled_mut(&mut self) -> &mut super::commonmodule::ControlDpc {
        self._ess_point_mut().trans_to_islnd_on_grid_loss_enabled.get_or_insert(Default::default())
    }
    fn voltage_set_point_enabled(&self) -> &super::commonmodule::ControlDpc {
        self._ess_point().voltage_set_point_enabled.as_ref().unwrap_or(&ess_point::VOLTAGE_SET_POINT_ENABLED)
    }
    fn voltage_set_point_enabled_mut(&mut self) -> &mut super::commonmodule::ControlDpc {
        self._ess_point_mut().voltage_set_point_enabled.get_or_insert(Default::default())
    }
    fn start_time(&self) -> &super::commonmodule::ControlTimestamp {
        self._ess_point().start_time.as_ref().unwrap_or(&ess_point::START_TIME)
    }
    fn start_time_mut(&mut self) -> &mut super::commonmodule::ControlTimestamp {
        self._ess_point_mut().start_time.get_or_insert(Default::default())
    }
}
impl IsEssPoint for EssPoint {
    fn _ess_point(&self) -> &EssPoint {
        self
    }
    fn _ess_point_mut(&mut self) -> &mut EssPoint {
        self
    }
}
/// Curve shape setting (FC=SP) (CSG_SP)
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize)]
pub struct Esscsg {
    /// The array with the points specifying a curve shape.
    // parent_message: false
    // required_field: true
    // multiplicity_min: Some(1)
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, repeated, tag="1")]
    #[serde(default, rename = "crvPts")]
    pub crv_pts: ::std::vec::Vec<EssPoint>,
}
mod esscsg {
    use lazy_static::lazy_static;
    lazy_static! {
    }
}
impl Esscsg {
}
pub trait IsEsscsg {
    fn _esscsg(&self) -> &Esscsg;
    fn _esscsg_mut(&mut self) -> &mut Esscsg;
    fn crv_pts(&self) -> &::std::vec::Vec<EssPoint> {
        &self._esscsg().crv_pts
    }
    fn crv_pts_mut(&mut self) -> &mut ::std::vec::Vec<EssPoint> {
        &mut self._esscsg_mut().crv_pts
    }
}
impl IsEsscsg for Esscsg {
    fn _esscsg(&self) -> &Esscsg {
        self
    }
    fn _esscsg_mut(&mut self) -> &mut Esscsg {
        self
    }
}
/// OpenFMB specialization for control schedule using:  LN: Schedule   Name: FSCH
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize)]
pub struct EssControlScheduleFsch {
    /// Discrete value in ESSCSG type
    // parent_message: false
    // required_field: true
    // multiplicity_min: Some(1)
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, optional, tag="1")]
    #[serde(default, rename = "ValDCSG")]
    pub val_dcsg: ::std::option::Option<Esscsg>,
}
mod ess_control_schedule_fsch {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref VAL_DCSG: crate::essmodule::Esscsg = Default::default();
    }
}
impl EssControlScheduleFsch {
}
pub trait IsEssControlScheduleFsch {
    fn _ess_control_schedule_fsch(&self) -> &EssControlScheduleFsch;
    fn _ess_control_schedule_fsch_mut(&mut self) -> &mut EssControlScheduleFsch;
    fn val_dcsg(&self) -> &Esscsg {
        self._ess_control_schedule_fsch().val_dcsg.as_ref().unwrap_or(&ess_control_schedule_fsch::VAL_DCSG)
    }
    fn val_dcsg_mut(&mut self) -> &mut Esscsg {
        self._ess_control_schedule_fsch_mut().val_dcsg.get_or_insert(Default::default())
    }
}
impl IsEssControlScheduleFsch for EssControlScheduleFsch {
    fn _ess_control_schedule_fsch(&self) -> &EssControlScheduleFsch {
        self
    }
    fn _ess_control_schedule_fsch_mut(&mut self) -> &mut EssControlScheduleFsch {
        self
    }
}
/// Specialized 61850 FSCC class.  LN: Schedule controller   Name: FSCC
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize)]
pub struct EssControlFscc {
    /// UML inherited base object
    // parent_message: true
    // required_field: false
    // multiplicity_min: None
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, optional, tag="1")]
    #[serde(default, rename = "controlFSCC")]
    pub control_fscc: ::std::option::Option<super::commonmodule::ControlFscc>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    #[serde(default, rename = "essControlScheduleFSCH")]
    pub ess_control_schedule_fsch: ::std::option::Option<EssControlScheduleFsch>,
}
mod ess_control_fscc {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref CONTROL_FSCC: crate::commonmodule::ControlFscc = Default::default();
        pub(super) static ref ESS_CONTROL_SCHEDULE_FSCH: crate::essmodule::EssControlScheduleFsch = Default::default();
    }
}
impl EssControlFscc {
    pub(crate) fn parent(&self) -> &super::commonmodule::ControlFscc {
        self.control_fscc.as_ref().unwrap_or(&ess_control_fscc::CONTROL_FSCC)
    }
    pub(crate) fn parent_mut(&mut self) -> &mut super::commonmodule::ControlFscc {
        self.control_fscc.get_or_insert(Default::default())
    }
}
pub trait IsEssControlFscc {
    fn _ess_control_fscc(&self) -> &EssControlFscc;
    fn _ess_control_fscc_mut(&mut self) -> &mut EssControlFscc;
    fn control_fscc(&self) -> &super::commonmodule::ControlFscc {
        self._ess_control_fscc().control_fscc.as_ref().unwrap_or(&ess_control_fscc::CONTROL_FSCC)
    }
    fn control_fscc_mut(&mut self) -> &mut super::commonmodule::ControlFscc {
        self._ess_control_fscc_mut().control_fscc.get_or_insert(Default::default())
    }
    fn ess_control_schedule_fsch(&self) -> &EssControlScheduleFsch {
        self._ess_control_fscc().ess_control_schedule_fsch.as_ref().unwrap_or(&ess_control_fscc::ESS_CONTROL_SCHEDULE_FSCH)
    }
    fn ess_control_schedule_fsch_mut(&mut self) -> &mut EssControlScheduleFsch {
        self._ess_control_fscc_mut().ess_control_schedule_fsch.get_or_insert(Default::default())
    }
}
impl IsEssControlFscc for EssControlFscc {
    fn _ess_control_fscc(&self) -> &EssControlFscc {
        self
    }
    fn _ess_control_fscc_mut(&mut self) -> &mut EssControlFscc {
        self
    }
}
impl IsControlFscc for EssControlFscc {
    fn _control_fscc(&self) -> &super::commonmodule::ControlFscc {
        self.parent()
    }
    fn _control_fscc_mut(&mut self) -> &mut ControlFscc {
        self.parent_mut()
    }
}
impl IsLogicalNodeForControl for EssControlFscc {
    fn _logical_node_for_control(&self) -> &super::commonmodule::LogicalNodeForControl {
        self.parent().parent()
    }
    fn _logical_node_for_control_mut(&mut self) -> &mut LogicalNodeForControl {
        self.parent_mut().parent_mut()
    }
}
impl IsLogicalNode for EssControlFscc {
    fn _logical_node(&self) -> &super::commonmodule::LogicalNode {
        self.parent().parent().parent()
    }
    fn _logical_node_mut(&mut self) -> &mut LogicalNode {
        self.parent_mut().parent_mut().parent_mut()
    }
}
impl IsIdentifiedObject for EssControlFscc {
    fn _identified_object(&self) -> &super::commonmodule::IdentifiedObject {
        self.parent().parent().parent().parent()
    }
    fn _identified_object_mut(&mut self) -> &mut IdentifiedObject {
        self.parent_mut().parent_mut().parent_mut().parent_mut()
    }
}
/// ESS control class
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize)]
pub struct EssControl {
    /// UML inherited base object
    // parent_message: true
    // required_field: false
    // multiplicity_min: None
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, optional, tag="1")]
    #[serde(default, rename = "controlValue")]
    pub control_value: ::std::option::Option<super::commonmodule::ControlValue>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    #[serde(default)]
    pub check: ::std::option::Option<super::commonmodule::CheckConditions>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    #[serde(default, rename = "essControlFSCC")]
    pub ess_control_fscc: ::std::option::Option<EssControlFscc>,
}
mod ess_control {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref CONTROL_VALUE: crate::commonmodule::ControlValue = Default::default();
        pub(super) static ref CHECK: crate::commonmodule::CheckConditions = Default::default();
        pub(super) static ref ESS_CONTROL_FSCC: crate::essmodule::EssControlFscc = Default::default();
    }
}
impl EssControl {
    pub(crate) fn parent(&self) -> &super::commonmodule::ControlValue {
        self.control_value.as_ref().unwrap_or(&ess_control::CONTROL_VALUE)
    }
    pub(crate) fn parent_mut(&mut self) -> &mut super::commonmodule::ControlValue {
        self.control_value.get_or_insert(Default::default())
    }
}
pub trait IsEssControl {
    fn _ess_control(&self) -> &EssControl;
    fn _ess_control_mut(&mut self) -> &mut EssControl;
    fn control_value(&self) -> &super::commonmodule::ControlValue {
        self._ess_control().control_value.as_ref().unwrap_or(&ess_control::CONTROL_VALUE)
    }
    fn control_value_mut(&mut self) -> &mut super::commonmodule::ControlValue {
        self._ess_control_mut().control_value.get_or_insert(Default::default())
    }
    fn check(&self) -> &super::commonmodule::CheckConditions {
        self._ess_control().check.as_ref().unwrap_or(&ess_control::CHECK)
    }
    fn check_mut(&mut self) -> &mut super::commonmodule::CheckConditions {
        self._ess_control_mut().check.get_or_insert(Default::default())
    }
    fn ess_control_fscc(&self) -> &EssControlFscc {
        self._ess_control().ess_control_fscc.as_ref().unwrap_or(&ess_control::ESS_CONTROL_FSCC)
    }
    fn ess_control_fscc_mut(&mut self) -> &mut EssControlFscc {
        self._ess_control_mut().ess_control_fscc.get_or_insert(Default::default())
    }
}
impl IsEssControl for EssControl {
    fn _ess_control(&self) -> &EssControl {
        self
    }
    fn _ess_control_mut(&mut self) -> &mut EssControl {
        self
    }
}
impl IsControlValue for EssControl {
    fn _control_value(&self) -> &super::commonmodule::ControlValue {
        self.parent()
    }
    fn _control_value_mut(&mut self) -> &mut ControlValue {
        self.parent_mut()
    }
}
impl IsIdentifiedObject for EssControl {
    fn _identified_object(&self) -> &super::commonmodule::IdentifiedObject {
        self.parent().parent()
    }
    fn _identified_object_mut(&mut self) -> &mut IdentifiedObject {
        self.parent_mut().parent_mut()
    }
}
/// ESS control profile
/// OpenFMB Profile Message: true
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize)]
pub struct EssControlProfile {
    /// UML inherited base object
    // parent_message: true
    // required_field: false
    // multiplicity_min: None
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, optional, tag="1")]
    #[serde(default, rename = "controlMessageInfo")]
    pub control_message_info: ::std::option::Option<super::commonmodule::ControlMessageInfo>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: Some(1)
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, optional, tag="2")]
    #[serde(default)]
    pub ess: ::std::option::Option<super::commonmodule::Ess>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: Some(1)
    // multiplicity_max: None
    // uuid: false
    // key: false
    #[prost(message, optional, tag="3")]
    #[serde(default, rename = "essControl")]
    pub ess_control: ::std::option::Option<EssControl>,
}
mod ess_control_profile {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref CONTROL_MESSAGE_INFO: crate::commonmodule::ControlMessageInfo = Default::default();
        pub(super) static ref ESS: crate::commonmodule::Ess = Default::default();
        pub(super) static ref ESS_CONTROL: crate::essmodule::EssControl = Default::default();
    }
}
impl EssControlProfile {
    pub(crate) fn parent(&self) -> &super::commonmodule::ControlMessageInfo {
        self.control_message_info.as_ref().unwrap_or(&ess_control_profile::CONTROL_MESSAGE_INFO)
    }
    pub(crate) fn parent_mut(&mut self) -> &mut super::commonmodule::ControlMessageInfo {
        self.control_message_info.get_or_insert(Default::default())
    }
}
pub trait IsEssControlProfile {
    fn _ess_control_profile(&self) -> &EssControlProfile;
    fn _ess_control_profile_mut(&mut self) -> &mut EssControlProfile;
    fn control_message_info(&self) -> &super::commonmodule::ControlMessageInfo {
        self._ess_control_profile().control_message_info.as_ref().unwrap_or(&ess_control_profile::CONTROL_MESSAGE_INFO)
    }
    fn control_message_info_mut(&mut self) -> &mut super::commonmodule::ControlMessageInfo {
        self._ess_control_profile_mut().control_message_info.get_or_insert(Default::default())
    }
    fn ess(&self) -> &super::commonmodule::Ess {
        self._ess_control_profile().ess.as_ref().unwrap_or(&ess_control_profile::ESS)
    }
    fn ess_mut(&mut self) -> &mut super::commonmodule::Ess {
        self._ess_control_profile_mut().ess.get_or_insert(Default::default())
    }
    fn ess_control(&self) -> &EssControl {
        self._ess_control_profile().ess_control.as_ref().unwrap_or(&ess_control_profile::ESS_CONTROL)
    }
    fn ess_control_mut(&mut self) -> &mut EssControl {
        self._ess_control_profile_mut().ess_control.get_or_insert(Default::default())
    }
}
impl IsEssControlProfile for EssControlProfile {
    fn _ess_control_profile(&self) -> &EssControlProfile {
        self
    }
    fn _ess_control_profile_mut(&mut self) -> &mut EssControlProfile {
        self
    }
}
impl IsControlMessageInfo for EssControlProfile {
    fn _control_message_info(&self) -> &super::commonmodule::ControlMessageInfo {
        self.parent()
    }
    fn _control_message_info_mut(&mut self) -> &mut ControlMessageInfo {
        self.parent_mut()
    }
}
impl IsMessageInfo for EssControlProfile {
    fn _message_info(&self) -> &super::commonmodule::MessageInfo {
        self.parent().parent()
    }
    fn _message_info_mut(&mut self) -> &mut MessageInfo {
        self.parent_mut().parent_mut()
    }
}
impl IsIdentifiedObject for EssControlProfile {
    fn _identified_object(&self) -> &super::commonmodule::IdentifiedObject {
        self.parent().parent().parent()
    }
    fn _identified_object_mut(&mut self) -> &mut IdentifiedObject {
        self.parent_mut().parent_mut().parent_mut()
    }
}