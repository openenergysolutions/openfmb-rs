/// Specialized 61850 ZBAT class  LN: Battery   Name: ZBAT
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct EssEventZbat {
    /// UML inherited base object
    // parent_message: true
    // required_field: false
    // multiplicity_min: 0
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="1")]
    pub logical_node_for_event_and_status: ::std::option::Option<super::commonmodule::LogicalNodeForEventAndStatus>,
    /// If true, the battery is in overcharge (voltage or current) condition.
    #[prost(message, optional, tag="2")]
    pub bat_hi: ::std::option::Option<super::commonmodule::StatusSps>,
    /// If true, the battery voltage or charge has dropped below a pre-set level.
    #[prost(message, optional, tag="3")]
    pub bat_lo: ::std::option::Option<super::commonmodule::StatusSps>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="4")]
    pub bat_st: ::std::option::Option<super::commonmodule::StatusSps>,
    /// State of charge (in percentage)
    #[prost(message, optional, tag="5")]
    pub soc: ::std::option::Option<super::commonmodule::Mv>,
    /// If stVal TRUE, the device is in standby.
    #[prost(message, optional, tag="6")]
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
trait IsEssEventZbat {
    fn _ess_event_zbat(&self) -> &EssEventZbat;
    fn logical_node_for_event_and_status(&self) -> &super::commonmodule::LogicalNodeForEventAndStatus {
        self._ess_event_zbat().logical_node_for_event_and_status.as_ref().unwrap_or(&ess_event_zbat::LOGICAL_NODE_FOR_EVENT_AND_STATUS)
    }
    fn bat_hi(&self) -> &super::commonmodule::StatusSps {
        self._ess_event_zbat().bat_hi.as_ref().unwrap_or(&ess_event_zbat::BAT_HI)
    }
    fn bat_lo(&self) -> &super::commonmodule::StatusSps {
        self._ess_event_zbat().bat_lo.as_ref().unwrap_or(&ess_event_zbat::BAT_LO)
    }
    fn bat_st(&self) -> &super::commonmodule::StatusSps {
        self._ess_event_zbat().bat_st.as_ref().unwrap_or(&ess_event_zbat::BAT_ST)
    }
    fn soc(&self) -> &super::commonmodule::Mv {
        self._ess_event_zbat().soc.as_ref().unwrap_or(&ess_event_zbat::SOC)
    }
    fn stdby(&self) -> &super::commonmodule::StatusSps {
        self._ess_event_zbat().stdby.as_ref().unwrap_or(&ess_event_zbat::STDBY)
    }
}
impl IsEssEventZbat for EssEventZbat {
    fn _ess_event_zbat(&self) -> &EssEventZbat {
        self
    }
}
/// ESS inverter high level function to maintain frequency within dead bands.
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct FrequencyRegulation {
    /// uint/0.01Hz  Frequency regulation is performed when the grid frequency goes beyond the dead
    /// bands. The dead bands are defined as follows: Upper DB = frequency set point + dead band plus Lower
    /// DB = frequency set point – dead band minus
    #[prost(message, optional, tag="1")]
    pub frequency_dead_band_minus: ::std::option::Option<f32>,
    /// uint/0.01Hz  Frequency regulation is performed when the grid frequency goes beyond the dead
    /// bands. The dead bands are defined as follows: Upper DB = frequency set point + dead band plus Lower
    /// DB = frequency set point – dead band minus
    #[prost(message, optional, tag="2")]
    pub frequency_dead_band_plus: ::std::option::Option<f32>,
    /// Control value (TRUE or FALSE)
    #[prost(message, optional, tag="3")]
    pub frequency_regulation_ctl: ::std::option::Option<bool>,
    /// uint/0.01Hz  Target frequency
    #[prost(message, optional, tag="4")]
    pub frequency_set_point: ::std::option::Option<f32>,
    /// uint/0.01Hz  Other modes of operation, such as peak shaving, smoothing or SOC management may
    /// operate if the grid frequency is within the stable band. Upper stable band = frequency set point +
    /// band plus Lower stable band = frequency set point – band minus
    #[prost(message, optional, tag="5")]
    pub grid_frequency_stable_band_minus: ::std::option::Option<f32>,
    /// uint/0.01Hz  Other modes of operation, such as peak shaving, smoothing or SOC management may
    /// operate if the grid frequency is within the stable band. Upper stable band = frequency set point +
    /// band plus Lower stable band = frequency set point – band minus
    #[prost(message, optional, tag="6")]
    pub grid_frequency_stable_band_plus: ::std::option::Option<f32>,
    /// uint/0.1%  The droops define the reaction of the PCS to under/over frequency events. A droop of
    /// 1% means that the PCS will output 100% power if the frequency is 1% of the nominal frequency away
    /// from the upper or lower dead band. The minimum droop value possible is 0.8%.
    #[prost(message, optional, tag="7")]
    pub over_frequency_droop: ::std::option::Option<f32>,
    /// uint/0.1%  The droops define the reaction of the PCS to under/over voltage events. A droop of 1%
    /// means that the PCS will output 100% power if the voltage is 1% of the nominal voltage away from the
    /// upper or lower dead band. The minimum droop value possible is 0.8%.
    #[prost(message, optional, tag="8")]
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
trait IsFrequencyRegulation {
    fn _frequency_regulation(&self) -> &FrequencyRegulation;
    fn frequency_dead_band_minus(&self) -> &f32 {
        self._frequency_regulation().frequency_dead_band_minus.as_ref().unwrap_or(&frequency_regulation::FREQUENCY_DEAD_BAND_MINUS)
    }
    fn frequency_dead_band_plus(&self) -> &f32 {
        self._frequency_regulation().frequency_dead_band_plus.as_ref().unwrap_or(&frequency_regulation::FREQUENCY_DEAD_BAND_PLUS)
    }
    fn frequency_regulation_ctl(&self) -> &bool {
        self._frequency_regulation().frequency_regulation_ctl.as_ref().unwrap_or(&frequency_regulation::FREQUENCY_REGULATION_CTL)
    }
    fn frequency_set_point(&self) -> &f32 {
        self._frequency_regulation().frequency_set_point.as_ref().unwrap_or(&frequency_regulation::FREQUENCY_SET_POINT)
    }
    fn grid_frequency_stable_band_minus(&self) -> &f32 {
        self._frequency_regulation().grid_frequency_stable_band_minus.as_ref().unwrap_or(&frequency_regulation::GRID_FREQUENCY_STABLE_BAND_MINUS)
    }
    fn grid_frequency_stable_band_plus(&self) -> &f32 {
        self._frequency_regulation().grid_frequency_stable_band_plus.as_ref().unwrap_or(&frequency_regulation::GRID_FREQUENCY_STABLE_BAND_PLUS)
    }
    fn over_frequency_droop(&self) -> &f32 {
        self._frequency_regulation().over_frequency_droop.as_ref().unwrap_or(&frequency_regulation::OVER_FREQUENCY_DROOP)
    }
    fn under_frequency_droop(&self) -> &f32 {
        self._frequency_regulation().under_frequency_droop.as_ref().unwrap_or(&frequency_regulation::UNDER_FREQUENCY_DROOP)
    }
}
impl IsFrequencyRegulation for FrequencyRegulation {
    fn _frequency_regulation(&self) -> &FrequencyRegulation {
        self
    }
}
/// ESS inverter high level function to maintain power level by charging or discharging
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PeakShaving {
    /// uint/1kW  If the supervised power goes below this limit, the ESS will charge to maintain this limit.
    #[prost(message, optional, tag="1")]
    pub base_shaving_limit: ::std::option::Option<f32>,
    /// Control value (TRUE or FALSE)
    #[prost(message, optional, tag="2")]
    pub peak_shaving_ctl: ::std::option::Option<bool>,
    /// uint/1kW  If the supervised power goes above this limit, the ESS will discharge to maintain this
    /// limit.
    #[prost(message, optional, tag="3")]
    pub peak_shaving_limit: ::std::option::Option<f32>,
    /// uint/1kW  If the supervised power is between the band defined by these two limits then SOC
    /// management is allowed.
    #[prost(message, optional, tag="4")]
    pub soc_management_allowed_high_limit: ::std::option::Option<f32>,
    /// uint/1kW  If the supervised power is between the band defined by these two limits then SOC
    /// management is allowed.
    #[prost(message, optional, tag="5")]
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
trait IsPeakShaving {
    fn _peak_shaving(&self) -> &PeakShaving;
    fn base_shaving_limit(&self) -> &f32 {
        self._peak_shaving().base_shaving_limit.as_ref().unwrap_or(&peak_shaving::BASE_SHAVING_LIMIT)
    }
    fn peak_shaving_ctl(&self) -> &bool {
        self._peak_shaving().peak_shaving_ctl.as_ref().unwrap_or(&peak_shaving::PEAK_SHAVING_CTL)
    }
    fn peak_shaving_limit(&self) -> &f32 {
        self._peak_shaving().peak_shaving_limit.as_ref().unwrap_or(&peak_shaving::PEAK_SHAVING_LIMIT)
    }
    fn soc_management_allowed_high_limit(&self) -> &f32 {
        self._peak_shaving().soc_management_allowed_high_limit.as_ref().unwrap_or(&peak_shaving::SOC_MANAGEMENT_ALLOWED_HIGH_LIMIT)
    }
    fn soc_management_allowed_low_limit(&self) -> &f32 {
        self._peak_shaving().soc_management_allowed_low_limit.as_ref().unwrap_or(&peak_shaving::SOC_MANAGEMENT_ALLOWED_LOW_LIMIT)
    }
}
impl IsPeakShaving for PeakShaving {
    fn _peak_shaving(&self) -> &PeakShaving {
        self
    }
}
/// ESS inverter high level function to shut down ESS if SOC exceeds high or low limits.
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SocLimit {
    /// uint/1%  These limits define the operational range of the battery. If a lineup reaches the SOC
    /// high limit, the inverter’s output is reduced to 0. Charging is then blocked until the hysteresis is
    /// overcome. The same logic applies to the SOC low limit, except that after the ramp down is complete,
    /// discharging is blocked until the hysteresis is overcome.
    #[prost(message, optional, tag="1")]
    pub soc_high_limit: ::std::option::Option<f32>,
    /// uint/1%  These limits define the operational range of the battery. If a lineup reaches the SOC
    /// high limit, the inverter’s output is reduced to 0. Charging is then blocked until the hysteresis is
    /// overcome. The same logic applies to the SOC low limit, except that after the ramp down is complete,
    /// discharging is blocked until the hysteresis is overcome.
    #[prost(message, optional, tag="2")]
    pub soc_high_limit_hysteresis: ::std::option::Option<f32>,
    /// Control value (TRUE or FALSE)
    #[prost(message, optional, tag="3")]
    pub soc_limit_ctl: ::std::option::Option<bool>,
    /// uint/1%  These limits define the operational range of the battery. If a lineup reaches the SOC
    /// high limit, the inverter’s output is reduced to 0. Charging is then blocked until the hysteresis is
    /// overcome. The same logic applies to the SOC low limit, except that after the ramp down is complete,
    /// discharging is blocked until the hysteresis is overcome.
    #[prost(message, optional, tag="4")]
    pub soc_low_limit: ::std::option::Option<f32>,
    /// uint/1%  These hysteresis define the release conditions for the block charge or discharge
    /// initiated by the SOC limits.For example, assume a SOC low limit of 10% and a SOC low limit
    /// hysteresis of 2% and that discharging is blocked because the batteries SOC reached the SOC low
    /// limit, discharging will only be allowed again after the battery’s SOC reaches 13%.
    #[prost(message, optional, tag="5")]
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
trait IsSocLimit {
    fn _soc_limit(&self) -> &SocLimit;
    fn soc_high_limit(&self) -> &f32 {
        self._soc_limit().soc_high_limit.as_ref().unwrap_or(&soc_limit::SOC_HIGH_LIMIT)
    }
    fn soc_high_limit_hysteresis(&self) -> &f32 {
        self._soc_limit().soc_high_limit_hysteresis.as_ref().unwrap_or(&soc_limit::SOC_HIGH_LIMIT_HYSTERESIS)
    }
    fn soc_limit_ctl(&self) -> &bool {
        self._soc_limit().soc_limit_ctl.as_ref().unwrap_or(&soc_limit::SOC_LIMIT_CTL)
    }
    fn soc_low_limit(&self) -> &f32 {
        self._soc_limit().soc_low_limit.as_ref().unwrap_or(&soc_limit::SOC_LOW_LIMIT)
    }
    fn soc_low_limit_hysteresis(&self) -> &f32 {
        self._soc_limit().soc_low_limit_hysteresis.as_ref().unwrap_or(&soc_limit::SOC_LOW_LIMIT_HYSTERESIS)
    }
}
impl IsSocLimit for SocLimit {
    fn _soc_limit(&self) -> &SocLimit {
        self
    }
}
/// ESS inverter high level function to maintain SOC within dead bands
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SocManagement {
    /// uint/1%  Define a dead band (DB) around the SOC set point. When the battery SOC goes outside the
    /// dead band, the SOC management executes and bring the SOC back to the set point. Upper DB = set point
    /// + dead band plus Lower DB = set point – dead band minus
    #[prost(message, optional, tag="1")]
    pub soc_dead_band_minus: ::std::option::Option<f32>,
    /// uint/1%  Define a dead band (DB) around the SOC set point. When the battery SOC goes outside the
    /// dead band, the SOC management executes and bring the SOC back to the set point. Upper DB = set point
    /// + dead band plus Lower DB = set point – dead band minus
    #[prost(message, optional, tag="2")]
    pub soc_dead_band_plus: ::std::option::Option<f32>,
    /// Control value (TRUE or FALSE)
    #[prost(message, optional, tag="3")]
    pub soc_management_ctl: ::std::option::Option<bool>,
    /// uint/1kW  Set point used for SOC maintenance
    #[prost(message, optional, tag="4")]
    pub soc_power_set_point: ::std::option::Option<f32>,
    /// uint/1%  SOC Target in percentage (%).
    #[prost(message, optional, tag="5")]
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
trait IsSocManagement {
    fn _soc_management(&self) -> &SocManagement;
    fn soc_dead_band_minus(&self) -> &f32 {
        self._soc_management().soc_dead_band_minus.as_ref().unwrap_or(&soc_management::SOC_DEAD_BAND_MINUS)
    }
    fn soc_dead_band_plus(&self) -> &f32 {
        self._soc_management().soc_dead_band_plus.as_ref().unwrap_or(&soc_management::SOC_DEAD_BAND_PLUS)
    }
    fn soc_management_ctl(&self) -> &bool {
        self._soc_management().soc_management_ctl.as_ref().unwrap_or(&soc_management::SOC_MANAGEMENT_CTL)
    }
    fn soc_power_set_point(&self) -> &f32 {
        self._soc_management().soc_power_set_point.as_ref().unwrap_or(&soc_management::SOC_POWER_SET_POINT)
    }
    fn soc_set_point(&self) -> &f32 {
        self._soc_management().soc_set_point.as_ref().unwrap_or(&soc_management::SOC_SET_POINT)
    }
}
impl IsSocManagement for SocManagement {
    fn _soc_management(&self) -> &SocManagement {
        self
    }
}
/// Voltage regulation function
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct VoltageRegulation {
    /// uint/0.1%  The droops define the reaction of the PCS to under/over voltage events. A droop of 1%
    /// means that the PCS will output 100% power if the voltage is 1% of the nominal voltage away from the
    /// upper or lower dead band. The minimum droop value possible is 0.8%.
    #[prost(message, optional, tag="1")]
    pub over_voltage_droop: ::std::option::Option<f32>,
    /// uint/0.1%  The droops define the reaction of the PCS to under/over voltage events. A droop of 1%
    /// means that the PCS will output 100% power if the voltage is 1% of the nominal voltage away from the
    /// upper or lower dead band. The minimum droop value possible is 0.8%.
    #[prost(message, optional, tag="2")]
    pub under_voltage_droop: ::std::option::Option<f32>,
    /// uint/0.1V  Voltage regulation is performed when the grid voltage goes beyond the dead bands. The
    /// dead bands are defined as follows: Upper DB = voltage set point + dead band plus Lower DB = voltage
    /// set point – dead band minus
    #[prost(message, optional, tag="3")]
    pub voltage_dead_band_minus: ::std::option::Option<f32>,
    /// uint/0.1V  Voltage regulation is performed when the grid voltage goes beyond the dead bands. The
    /// dead bands are defined as follows: Upper DB = voltage set point + dead band plus Lower DB = voltage
    /// set point – dead band minus
    #[prost(message, optional, tag="4")]
    pub voltage_dead_band_plus: ::std::option::Option<f32>,
    /// uint/0.1V  Other modes of operation, such as peak shaving, smoothing or SOC management may
    /// operate if the grid frequency is within the stable band. Upper stable band = frequency set point +
    /// band plus Lower stable band = frequency set point – band minus
    #[prost(message, optional, tag="5")]
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
trait IsVoltageRegulation {
    fn _voltage_regulation(&self) -> &VoltageRegulation;
    fn over_voltage_droop(&self) -> &f32 {
        self._voltage_regulation().over_voltage_droop.as_ref().unwrap_or(&voltage_regulation::OVER_VOLTAGE_DROOP)
    }
    fn under_voltage_droop(&self) -> &f32 {
        self._voltage_regulation().under_voltage_droop.as_ref().unwrap_or(&voltage_regulation::UNDER_VOLTAGE_DROOP)
    }
    fn voltage_dead_band_minus(&self) -> &f32 {
        self._voltage_regulation().voltage_dead_band_minus.as_ref().unwrap_or(&voltage_regulation::VOLTAGE_DEAD_BAND_MINUS)
    }
    fn voltage_dead_band_plus(&self) -> &f32 {
        self._voltage_regulation().voltage_dead_band_plus.as_ref().unwrap_or(&voltage_regulation::VOLTAGE_DEAD_BAND_PLUS)
    }
    fn voltage_set_point(&self) -> &f32 {
        self._voltage_regulation().voltage_set_point.as_ref().unwrap_or(&voltage_regulation::VOLTAGE_SET_POINT)
    }
}
impl IsVoltageRegulation for VoltageRegulation {
    fn _voltage_regulation(&self) -> &VoltageRegulation {
        self
    }
}
/// ESS inverter high level function to maintain voltage within droop dead bands.
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct VoltageDroop {
    /// Control value (TRUE or FALSE)
    #[prost(message, optional, tag="1")]
    pub voltage_droop_ctl: ::std::option::Option<bool>,
    /// Voltage regulation
    #[prost(message, optional, tag="2")]
    pub voltage_regulation: ::std::option::Option<VoltageRegulation>,
}
mod voltage_droop {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref VOLTAGE_DROOP_CTL: bool = Default::default();
        pub(super) static ref VOLTAGE_REGULATION: crate::essmodule::VoltageRegulation = Default::default();
    }
}
trait IsVoltageDroop {
    fn _voltage_droop(&self) -> &VoltageDroop;
    fn voltage_droop_ctl(&self) -> &bool {
        self._voltage_droop().voltage_droop_ctl.as_ref().unwrap_or(&voltage_droop::VOLTAGE_DROOP_CTL)
    }
    fn voltage_regulation(&self) -> &VoltageRegulation {
        self._voltage_droop().voltage_regulation.as_ref().unwrap_or(&voltage_droop::VOLTAGE_REGULATION)
    }
}
impl IsVoltageDroop for VoltageDroop {
    fn _voltage_droop(&self) -> &VoltageDroop {
        self
    }
}
/// ESS inverter high level function to maintain voltage within dead bands.
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct VoltagePi {
    /// Control value (TRUE or FALSE)
    #[prost(message, optional, tag="1")]
    pub voltage_pi_ctl: ::std::option::Option<bool>,
    /// Voltage regulation
    #[prost(message, optional, tag="2")]
    pub voltage_regulation: ::std::option::Option<VoltageRegulation>,
}
mod voltage_pi {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref VOLTAGE_PI_CTL: bool = Default::default();
        pub(super) static ref VOLTAGE_REGULATION: crate::essmodule::VoltageRegulation = Default::default();
    }
}
trait IsVoltagePi {
    fn _voltage_pi(&self) -> &VoltagePi;
    fn voltage_pi_ctl(&self) -> &bool {
        self._voltage_pi().voltage_pi_ctl.as_ref().unwrap_or(&voltage_pi::VOLTAGE_PI_CTL)
    }
    fn voltage_regulation(&self) -> &VoltageRegulation {
        self._voltage_pi().voltage_regulation.as_ref().unwrap_or(&voltage_pi::VOLTAGE_REGULATION)
    }
}
impl IsVoltagePi for VoltagePi {
    fn _voltage_pi(&self) -> &VoltagePi {
        self
    }
}
/// ESS inverter high level function to reduce (smooth) charging or discharging rate of change.
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct CapacityFirming {
    /// Control value (TRUE or FALSE)
    #[prost(message, optional, tag="1")]
    pub capacity_firming_ctl: ::std::option::Option<bool>,
    /// uint/1kW/min  If the supervised power increases at a rate higher that the rate defined by these
    /// limits, the ESS will discharge/charge at an opposite dp/dt to reduce (smooth) the rate of change at
    /// the PCC
    #[prost(message, optional, tag="2")]
    pub limit_negative_dp_dt: ::std::option::Option<f32>,
    /// uint/1kW/min  If the supervised power increases at a rate higher that the rate defined by these
    /// limits, the ESS will discharge/charge at an opposite dp/dt to reduce (smooth) the rate of change at
    /// the PCC
    #[prost(message, optional, tag="3")]
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
trait IsCapacityFirming {
    fn _capacity_firming(&self) -> &CapacityFirming;
    fn capacity_firming_ctl(&self) -> &bool {
        self._capacity_firming().capacity_firming_ctl.as_ref().unwrap_or(&capacity_firming::CAPACITY_FIRMING_CTL)
    }
    fn limit_negative_dp_dt(&self) -> &f32 {
        self._capacity_firming().limit_negative_dp_dt.as_ref().unwrap_or(&capacity_firming::LIMIT_NEGATIVE_DP_DT)
    }
    fn limit_positive_dp_dt(&self) -> &f32 {
        self._capacity_firming().limit_positive_dp_dt.as_ref().unwrap_or(&capacity_firming::LIMIT_POSITIVE_DP_DT)
    }
}
impl IsCapacityFirming for CapacityFirming {
    fn _capacity_firming(&self) -> &CapacityFirming {
        self
    }
}
/// ESS inverter high level functions.
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct EssFunction {
    /// ESS inverter high level function to reduce (smooth) charging or discharging rate of change.
    #[prost(message, optional, tag="1")]
    pub capacity_firming: ::std::option::Option<CapacityFirming>,
    /// ESS inverter high level function to maintain frequency within dead bands.
    #[prost(message, optional, tag="2")]
    pub frequency_regulation: ::std::option::Option<FrequencyRegulation>,
    /// ESS inverter high level function to maintain power level by charging or discharging
    #[prost(message, optional, tag="3")]
    pub peak_shaving: ::std::option::Option<PeakShaving>,
    /// ESS inverter high level function to shut down ESS if SOC exceeds high or low limits.
    #[prost(message, optional, tag="4")]
    pub soc_limit: ::std::option::Option<SocLimit>,
    /// ESS inverter high level function to maintain SOC within dead bands
    #[prost(message, optional, tag="5")]
    pub soc_management: ::std::option::Option<SocManagement>,
    /// ESS inverter high level function to maintain voltage within droop dead bands.
    #[prost(message, optional, tag="6")]
    pub voltage_droop: ::std::option::Option<VoltageDroop>,
    /// ESS inverter high level function to maintain voltage within dead bands.
    #[prost(message, optional, tag="7")]
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
trait IsEssFunction {
    fn _ess_function(&self) -> &EssFunction;
    fn capacity_firming(&self) -> &CapacityFirming {
        self._ess_function().capacity_firming.as_ref().unwrap_or(&ess_function::CAPACITY_FIRMING)
    }
    fn frequency_regulation(&self) -> &FrequencyRegulation {
        self._ess_function().frequency_regulation.as_ref().unwrap_or(&ess_function::FREQUENCY_REGULATION)
    }
    fn peak_shaving(&self) -> &PeakShaving {
        self._ess_function().peak_shaving.as_ref().unwrap_or(&ess_function::PEAK_SHAVING)
    }
    fn soc_limit(&self) -> &SocLimit {
        self._ess_function().soc_limit.as_ref().unwrap_or(&ess_function::SOC_LIMIT)
    }
    fn soc_management(&self) -> &SocManagement {
        self._ess_function().soc_management.as_ref().unwrap_or(&ess_function::SOC_MANAGEMENT)
    }
    fn voltage_droop(&self) -> &VoltageDroop {
        self._ess_function().voltage_droop.as_ref().unwrap_or(&ess_function::VOLTAGE_DROOP)
    }
    fn voltage_pi(&self) -> &VoltagePi {
        self._ess_function().voltage_pi.as_ref().unwrap_or(&ess_function::VOLTAGE_PI)
    }
}
impl IsEssFunction for EssFunction {
    fn _ess_function(&self) -> &EssFunction {
        self
    }
}
/// Point definition (Point)
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct EssPointStatus {
    /// Black start enable
    #[prost(message, optional, tag="1")]
    pub black_start_enabled: ::std::option::Option<super::commonmodule::ControlDpc>,
    /// Enable frequency set point
    #[prost(message, optional, tag="2")]
    pub frequency_set_point_enabled: ::std::option::Option<super::commonmodule::ControlDpc>,
    /// ESS function parameter
    #[prost(message, optional, tag="3")]
    pub function: ::std::option::Option<EssFunction>,
    /// Grid connect mode
    #[prost(message, optional, tag="4")]
    pub mode: ::std::option::Option<super::commonmodule::EngGridConnectModeKind>,
    /// Black start enable
    #[prost(message, optional, tag="5")]
    pub pct_hz_droop: ::std::option::Option<f32>,
    /// Black start enable
    #[prost(message, optional, tag="6")]
    pub pct_v_droop: ::std::option::Option<f32>,
    /// Ramp rates
    #[prost(message, optional, tag="7")]
    pub ramp_rates: ::std::option::Option<super::commonmodule::RampRate>,
    /// Enable reactive power set point
    #[prost(message, optional, tag="8")]
    pub reactive_pwr_set_point_enabled: ::std::option::Option<super::commonmodule::ControlDpc>,
    /// Enable real power set point
    #[prost(message, optional, tag="9")]
    pub real_pwr_set_point_enabled: ::std::option::Option<super::commonmodule::ControlDpc>,
    /// ESS state
    #[prost(message, optional, tag="10")]
    pub state: ::std::option::Option<super::commonmodule::OptionalStateKind>,
    /// Synchronize back to grid
    #[prost(message, optional, tag="11")]
    pub sync_back_to_grid: ::std::option::Option<super::commonmodule::ControlDpc>,
    /// Transition to island on grid loss enable
    #[prost(message, optional, tag="12")]
    pub trans_to_islnd_on_grid_loss_enabled: ::std::option::Option<super::commonmodule::ControlDpc>,
    /// Enable voltage set point
    #[prost(message, optional, tag="13")]
    pub voltage_set_point_enabled: ::std::option::Option<super::commonmodule::ControlDpc>,
}
mod ess_point_status {
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
        pub(super) static ref STATE: crate::commonmodule::OptionalStateKind = Default::default();
        pub(super) static ref SYNC_BACK_TO_GRID: crate::commonmodule::ControlDpc = Default::default();
        pub(super) static ref TRANS_TO_ISLND_ON_GRID_LOSS_ENABLED: crate::commonmodule::ControlDpc = Default::default();
        pub(super) static ref VOLTAGE_SET_POINT_ENABLED: crate::commonmodule::ControlDpc = Default::default();
    }
}
trait IsEssPointStatus {
    fn _ess_point_status(&self) -> &EssPointStatus;
    fn black_start_enabled(&self) -> &super::commonmodule::ControlDpc {
        self._ess_point_status().black_start_enabled.as_ref().unwrap_or(&ess_point_status::BLACK_START_ENABLED)
    }
    fn frequency_set_point_enabled(&self) -> &super::commonmodule::ControlDpc {
        self._ess_point_status().frequency_set_point_enabled.as_ref().unwrap_or(&ess_point_status::FREQUENCY_SET_POINT_ENABLED)
    }
    fn function(&self) -> &EssFunction {
        self._ess_point_status().function.as_ref().unwrap_or(&ess_point_status::FUNCTION)
    }
    fn mode(&self) -> &super::commonmodule::EngGridConnectModeKind {
        self._ess_point_status().mode.as_ref().unwrap_or(&ess_point_status::MODE)
    }
    fn pct_hz_droop(&self) -> &f32 {
        self._ess_point_status().pct_hz_droop.as_ref().unwrap_or(&ess_point_status::PCT_HZ_DROOP)
    }
    fn pct_v_droop(&self) -> &f32 {
        self._ess_point_status().pct_v_droop.as_ref().unwrap_or(&ess_point_status::PCT_V_DROOP)
    }
    fn ramp_rates(&self) -> &super::commonmodule::RampRate {
        self._ess_point_status().ramp_rates.as_ref().unwrap_or(&ess_point_status::RAMP_RATES)
    }
    fn reactive_pwr_set_point_enabled(&self) -> &super::commonmodule::ControlDpc {
        self._ess_point_status().reactive_pwr_set_point_enabled.as_ref().unwrap_or(&ess_point_status::REACTIVE_PWR_SET_POINT_ENABLED)
    }
    fn real_pwr_set_point_enabled(&self) -> &super::commonmodule::ControlDpc {
        self._ess_point_status().real_pwr_set_point_enabled.as_ref().unwrap_or(&ess_point_status::REAL_PWR_SET_POINT_ENABLED)
    }
    fn state(&self) -> &super::commonmodule::OptionalStateKind {
        self._ess_point_status().state.as_ref().unwrap_or(&ess_point_status::STATE)
    }
    fn sync_back_to_grid(&self) -> &super::commonmodule::ControlDpc {
        self._ess_point_status().sync_back_to_grid.as_ref().unwrap_or(&ess_point_status::SYNC_BACK_TO_GRID)
    }
    fn trans_to_islnd_on_grid_loss_enabled(&self) -> &super::commonmodule::ControlDpc {
        self._ess_point_status().trans_to_islnd_on_grid_loss_enabled.as_ref().unwrap_or(&ess_point_status::TRANS_TO_ISLND_ON_GRID_LOSS_ENABLED)
    }
    fn voltage_set_point_enabled(&self) -> &super::commonmodule::ControlDpc {
        self._ess_point_status().voltage_set_point_enabled.as_ref().unwrap_or(&ess_point_status::VOLTAGE_SET_POINT_ENABLED)
    }
}
impl IsEssPointStatus for EssPointStatus {
    fn _ess_point_status(&self) -> &EssPointStatus {
        self
    }
}
/// Specialized 61850 ZGEN class
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct EssEventAndStatusZgen {
    /// UML inherited base object
    // parent_message: true
    // required_field: false
    // multiplicity_min: 0
    // multiplicity_max: 0
    // uuid: false
    // key: false
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
    /// Generator is synchronized to EPS, or not; True = Synchronized
    #[prost(message, optional, tag="5")]
    pub gn_syn_st: ::std::option::Option<super::commonmodule::StatusSps>,
    /// Point status
    #[prost(message, optional, tag="6")]
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
trait IsEssEventAndStatusZgen {
    fn _ess_event_and_status_zgen(&self) -> &EssEventAndStatusZgen;
    fn logical_node_for_event_and_status(&self) -> &super::commonmodule::LogicalNodeForEventAndStatus {
        self._ess_event_and_status_zgen().logical_node_for_event_and_status.as_ref().unwrap_or(&ess_event_and_status_zgen::LOGICAL_NODE_FOR_EVENT_AND_STATUS)
    }
    fn aux_pwr_st(&self) -> &super::commonmodule::StatusSps {
        self._ess_event_and_status_zgen().aux_pwr_st.as_ref().unwrap_or(&ess_event_and_status_zgen::AUX_PWR_ST)
    }
    fn dynamic_test(&self) -> &super::commonmodule::EnsDynamicTestKind {
        self._ess_event_and_status_zgen().dynamic_test.as_ref().unwrap_or(&ess_event_and_status_zgen::DYNAMIC_TEST)
    }
    fn emg_stop(&self) -> &super::commonmodule::StatusSps {
        self._ess_event_and_status_zgen().emg_stop.as_ref().unwrap_or(&ess_event_and_status_zgen::EMG_STOP)
    }
    fn gn_syn_st(&self) -> &super::commonmodule::StatusSps {
        self._ess_event_and_status_zgen().gn_syn_st.as_ref().unwrap_or(&ess_event_and_status_zgen::GN_SYN_ST)
    }
    fn point_status(&self) -> &EssPointStatus {
        self._ess_event_and_status_zgen().point_status.as_ref().unwrap_or(&ess_event_and_status_zgen::POINT_STATUS)
    }
}
impl IsEssEventAndStatusZgen for EssEventAndStatusZgen {
    fn _ess_event_and_status_zgen(&self) -> &EssEventAndStatusZgen {
        self
    }
}
/// Specialized 61850 ZGEN class for ESS event profile
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct EssEventZgen {
    /// UML inherited base object
    // parent_message: true
    // required_field: false
    // multiplicity_min: 0
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="1")]
    pub e_ss_event_and_status_zgen: ::std::option::Option<EssEventAndStatusZgen>,
}
mod ess_event_zgen {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref E_SS_EVENT_AND_STATUS_ZGEN: crate::essmodule::EssEventAndStatusZgen = Default::default();
    }
}
trait IsEssEventZgen {
    fn _ess_event_zgen(&self) -> &EssEventZgen;
    fn e_ss_event_and_status_zgen(&self) -> &EssEventAndStatusZgen {
        self._ess_event_zgen().e_ss_event_and_status_zgen.as_ref().unwrap_or(&ess_event_zgen::E_SS_EVENT_AND_STATUS_ZGEN)
    }
}
impl IsEssEventZgen for EssEventZgen {
    fn _ess_event_zgen(&self) -> &EssEventZgen {
        self
    }
}
/// ESS event
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct EssEvent {
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
    #[prost(message, optional, tag="2")]
    pub ess_event_zbat: ::std::option::Option<EssEventZbat>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
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
trait IsEssEvent {
    fn _ess_event(&self) -> &EssEvent;
    fn event_value(&self) -> &super::commonmodule::EventValue {
        self._ess_event().event_value.as_ref().unwrap_or(&ess_event::EVENT_VALUE)
    }
    fn ess_event_zbat(&self) -> &EssEventZbat {
        self._ess_event().ess_event_zbat.as_ref().unwrap_or(&ess_event::ESS_EVENT_ZBAT)
    }
    fn ess_event_zgen(&self) -> &EssEventZgen {
        self._ess_event().ess_event_zgen.as_ref().unwrap_or(&ess_event::ESS_EVENT_ZGEN)
    }
}
impl IsEssEvent for EssEvent {
    fn _ess_event(&self) -> &EssEvent {
        self
    }
}
/// ESS event profile
/// OpenFMB Profile Message: true
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct EssEventProfile {
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
    pub ess: ::std::option::Option<super::commonmodule::Ess>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="3")]
    pub ess_event: ::std::option::Option<EssEvent>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="4")]
    pub ied: ::std::option::Option<super::commonmodule::Ied>,
}
mod ess_event_profile {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref EVENT_MESSAGE_INFO: crate::commonmodule::EventMessageInfo = Default::default();
        pub(super) static ref ESS: crate::commonmodule::Ess = Default::default();
        pub(super) static ref ESS_EVENT: crate::essmodule::EssEvent = Default::default();
        pub(super) static ref IED: crate::commonmodule::Ied = Default::default();
    }
}
trait IsEssEventProfile {
    fn _ess_event_profile(&self) -> &EssEventProfile;
    fn event_message_info(&self) -> &super::commonmodule::EventMessageInfo {
        self._ess_event_profile().event_message_info.as_ref().unwrap_or(&ess_event_profile::EVENT_MESSAGE_INFO)
    }
    fn ess(&self) -> &super::commonmodule::Ess {
        self._ess_event_profile().ess.as_ref().unwrap_or(&ess_event_profile::ESS)
    }
    fn ess_event(&self) -> &EssEvent {
        self._ess_event_profile().ess_event.as_ref().unwrap_or(&ess_event_profile::ESS_EVENT)
    }
    fn ied(&self) -> &super::commonmodule::Ied {
        self._ess_event_profile().ied.as_ref().unwrap_or(&ess_event_profile::IED)
    }
}
impl IsEssEventProfile for EssEventProfile {
    fn _ess_event_profile(&self) -> &EssEventProfile {
        self
    }
}
/// ESS reading value
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct EssReading {
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
mod ess_reading {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref CONDUCTING_EQUIPMENT_TERMINAL_READING: crate::commonmodule::ConductingEquipmentTerminalReading = Default::default();
        pub(super) static ref PHASE_MMTN: crate::commonmodule::PhaseMmtn = Default::default();
        pub(super) static ref READING_MMTR: crate::commonmodule::ReadingMmtr = Default::default();
        pub(super) static ref READING_MMXU: crate::commonmodule::ReadingMmxu = Default::default();
    }
}
trait IsEssReading {
    fn _ess_reading(&self) -> &EssReading;
    fn conducting_equipment_terminal_reading(&self) -> &super::commonmodule::ConductingEquipmentTerminalReading {
        self._ess_reading().conducting_equipment_terminal_reading.as_ref().unwrap_or(&ess_reading::CONDUCTING_EQUIPMENT_TERMINAL_READING)
    }
    fn phase_mmtn(&self) -> &super::commonmodule::PhaseMmtn {
        self._ess_reading().phase_mmtn.as_ref().unwrap_or(&ess_reading::PHASE_MMTN)
    }
    fn reading_mmtr(&self) -> &super::commonmodule::ReadingMmtr {
        self._ess_reading().reading_mmtr.as_ref().unwrap_or(&ess_reading::READING_MMTR)
    }
    fn reading_mmxu(&self) -> &super::commonmodule::ReadingMmxu {
        self._ess_reading().reading_mmxu.as_ref().unwrap_or(&ess_reading::READING_MMXU)
    }
}
impl IsEssReading for EssReading {
    fn _ess_reading(&self) -> &EssReading {
        self
    }
}
/// ESS reading profile
/// OpenFMB Profile Message: true
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct EssReadingProfile {
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
    pub ess: ::std::option::Option<super::commonmodule::Ess>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="3")]
    pub ess_reading: ::std::option::Option<EssReading>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="4")]
    pub ied: ::std::option::Option<super::commonmodule::Ied>,
}
mod ess_reading_profile {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref READING_MESSAGE_INFO: crate::commonmodule::ReadingMessageInfo = Default::default();
        pub(super) static ref ESS: crate::commonmodule::Ess = Default::default();
        pub(super) static ref ESS_READING: crate::essmodule::EssReading = Default::default();
        pub(super) static ref IED: crate::commonmodule::Ied = Default::default();
    }
}
trait IsEssReadingProfile {
    fn _ess_reading_profile(&self) -> &EssReadingProfile;
    fn reading_message_info(&self) -> &super::commonmodule::ReadingMessageInfo {
        self._ess_reading_profile().reading_message_info.as_ref().unwrap_or(&ess_reading_profile::READING_MESSAGE_INFO)
    }
    fn ess(&self) -> &super::commonmodule::Ess {
        self._ess_reading_profile().ess.as_ref().unwrap_or(&ess_reading_profile::ESS)
    }
    fn ess_reading(&self) -> &EssReading {
        self._ess_reading_profile().ess_reading.as_ref().unwrap_or(&ess_reading_profile::ESS_READING)
    }
    fn ied(&self) -> &super::commonmodule::Ied {
        self._ess_reading_profile().ied.as_ref().unwrap_or(&ess_reading_profile::IED)
    }
}
impl IsEssReadingProfile for EssReadingProfile {
    fn _ess_reading_profile(&self) -> &EssReadingProfile {
        self
    }
}
/// Specialized 61850 ZBAT
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct EssStatusZbat {
    /// UML inherited base object
    // parent_message: true
    // required_field: false
    // multiplicity_min: 0
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="1")]
    pub logical_node_for_event_and_status: ::std::option::Option<super::commonmodule::LogicalNodeForEventAndStatus>,
    /// Battery system status &ndash; True: on
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="2")]
    pub bat_st: ::std::option::Option<super::commonmodule::StatusSps>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub gri_mod: ::std::option::Option<super::commonmodule::EngGridConnectModeKind>,
    /// State of charge (in percentage)
    #[prost(message, optional, tag="4")]
    pub soc: ::std::option::Option<super::commonmodule::Mv>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="5")]
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
trait IsEssStatusZbat {
    fn _ess_status_zbat(&self) -> &EssStatusZbat;
    fn logical_node_for_event_and_status(&self) -> &super::commonmodule::LogicalNodeForEventAndStatus {
        self._ess_status_zbat().logical_node_for_event_and_status.as_ref().unwrap_or(&ess_status_zbat::LOGICAL_NODE_FOR_EVENT_AND_STATUS)
    }
    fn bat_st(&self) -> &super::commonmodule::StatusSps {
        self._ess_status_zbat().bat_st.as_ref().unwrap_or(&ess_status_zbat::BAT_ST)
    }
    fn gri_mod(&self) -> &super::commonmodule::EngGridConnectModeKind {
        self._ess_status_zbat().gri_mod.as_ref().unwrap_or(&ess_status_zbat::GRI_MOD)
    }
    fn soc(&self) -> &super::commonmodule::Mv {
        self._ess_status_zbat().soc.as_ref().unwrap_or(&ess_status_zbat::SOC)
    }
    fn stdby(&self) -> &super::commonmodule::StatusSps {
        self._ess_status_zbat().stdby.as_ref().unwrap_or(&ess_status_zbat::STDBY)
    }
}
impl IsEssStatusZbat for EssStatusZbat {
    fn _ess_status_zbat(&self) -> &EssStatusZbat {
        self
    }
}
/// Specialized 61850 ZGEN class
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct EssStatusZgen {
    /// UML inherited base object
    // parent_message: true
    // required_field: false
    // multiplicity_min: 0
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="1")]
    pub e_ss_event_and_status_zgen: ::std::option::Option<EssEventAndStatusZgen>,
}
mod ess_status_zgen {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref E_SS_EVENT_AND_STATUS_ZGEN: crate::essmodule::EssEventAndStatusZgen = Default::default();
    }
}
trait IsEssStatusZgen {
    fn _ess_status_zgen(&self) -> &EssStatusZgen;
    fn e_ss_event_and_status_zgen(&self) -> &EssEventAndStatusZgen {
        self._ess_status_zgen().e_ss_event_and_status_zgen.as_ref().unwrap_or(&ess_status_zgen::E_SS_EVENT_AND_STATUS_ZGEN)
    }
}
impl IsEssStatusZgen for EssStatusZgen {
    fn _ess_status_zgen(&self) -> &EssStatusZgen {
        self
    }
}
/// ESS status
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct EssStatus {
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
    #[prost(message, optional, tag="2")]
    pub ess_status_zbat: ::std::option::Option<EssStatusZbat>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
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
trait IsEssStatus {
    fn _ess_status(&self) -> &EssStatus;
    fn status_value(&self) -> &super::commonmodule::StatusValue {
        self._ess_status().status_value.as_ref().unwrap_or(&ess_status::STATUS_VALUE)
    }
    fn ess_status_zbat(&self) -> &EssStatusZbat {
        self._ess_status().ess_status_zbat.as_ref().unwrap_or(&ess_status::ESS_STATUS_ZBAT)
    }
    fn ess_status_zgen(&self) -> &EssStatusZgen {
        self._ess_status().ess_status_zgen.as_ref().unwrap_or(&ess_status::ESS_STATUS_ZGEN)
    }
}
impl IsEssStatus for EssStatus {
    fn _ess_status(&self) -> &EssStatus {
        self
    }
}
/// ESS status profile
/// OpenFMB Profile Message: true
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct EssStatusProfile {
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
    pub ess: ::std::option::Option<super::commonmodule::Ess>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="3")]
    pub ess_status: ::std::option::Option<EssStatus>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="4")]
    pub ied: ::std::option::Option<super::commonmodule::Ied>,
}
mod ess_status_profile {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref STATUS_MESSAGE_INFO: crate::commonmodule::StatusMessageInfo = Default::default();
        pub(super) static ref ESS: crate::commonmodule::Ess = Default::default();
        pub(super) static ref ESS_STATUS: crate::essmodule::EssStatus = Default::default();
        pub(super) static ref IED: crate::commonmodule::Ied = Default::default();
    }
}
trait IsEssStatusProfile {
    fn _ess_status_profile(&self) -> &EssStatusProfile;
    fn status_message_info(&self) -> &super::commonmodule::StatusMessageInfo {
        self._ess_status_profile().status_message_info.as_ref().unwrap_or(&ess_status_profile::STATUS_MESSAGE_INFO)
    }
    fn ess(&self) -> &super::commonmodule::Ess {
        self._ess_status_profile().ess.as_ref().unwrap_or(&ess_status_profile::ESS)
    }
    fn ess_status(&self) -> &EssStatus {
        self._ess_status_profile().ess_status.as_ref().unwrap_or(&ess_status_profile::ESS_STATUS)
    }
    fn ied(&self) -> &super::commonmodule::Ied {
        self._ess_status_profile().ied.as_ref().unwrap_or(&ess_status_profile::IED)
    }
}
impl IsEssStatusProfile for EssStatusProfile {
    fn _ess_status_profile(&self) -> &EssStatusProfile {
        self
    }
}
/// Point definition (Point)
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct EssPoint {
    /// Black start enable
    #[prost(message, optional, tag="1")]
    pub black_start_enabled: ::std::option::Option<super::commonmodule::ControlDpc>,
    /// Enable frequency set point
    #[prost(message, optional, tag="2")]
    pub frequency_set_point_enabled: ::std::option::Option<super::commonmodule::ControlDpc>,
    /// ESS function parameter
    #[prost(message, optional, tag="3")]
    pub function: ::std::option::Option<EssFunction>,
    /// Grid connect mode
    #[prost(message, optional, tag="4")]
    pub mode: ::std::option::Option<super::commonmodule::EngGridConnectModeKind>,
    /// Black start enable
    #[prost(message, optional, tag="5")]
    pub pct_hz_droop: ::std::option::Option<f32>,
    /// Black start enable
    #[prost(message, optional, tag="6")]
    pub pct_v_droop: ::std::option::Option<f32>,
    /// Ramp rates
    #[prost(message, optional, tag="7")]
    pub ramp_rates: ::std::option::Option<super::commonmodule::RampRate>,
    /// Enable reactive power set point
    #[prost(message, optional, tag="8")]
    pub reactive_pwr_set_point_enabled: ::std::option::Option<super::commonmodule::ControlDpc>,
    /// Enable real power set point
    #[prost(message, optional, tag="9")]
    pub real_pwr_set_point_enabled: ::std::option::Option<super::commonmodule::ControlDpc>,
    /// Reset device
    #[prost(message, optional, tag="10")]
    pub reset: ::std::option::Option<super::commonmodule::ControlDpc>,
    /// ESS state
    #[prost(message, optional, tag="11")]
    pub state: ::std::option::Option<super::commonmodule::OptionalStateKind>,
    /// Synchronize back to grid
    #[prost(message, optional, tag="12")]
    pub sync_back_to_grid: ::std::option::Option<super::commonmodule::ControlDpc>,
    /// Transition to island on grid loss enable
    #[prost(message, optional, tag="13")]
    pub trans_to_islnd_on_grid_loss_enabled: ::std::option::Option<super::commonmodule::ControlDpc>,
    /// Enable voltage set point
    #[prost(message, optional, tag="14")]
    pub voltage_set_point_enabled: ::std::option::Option<super::commonmodule::ControlDpc>,
    /// Start time
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="15")]
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
trait IsEssPoint {
    fn _ess_point(&self) -> &EssPoint;
    fn black_start_enabled(&self) -> &super::commonmodule::ControlDpc {
        self._ess_point().black_start_enabled.as_ref().unwrap_or(&ess_point::BLACK_START_ENABLED)
    }
    fn frequency_set_point_enabled(&self) -> &super::commonmodule::ControlDpc {
        self._ess_point().frequency_set_point_enabled.as_ref().unwrap_or(&ess_point::FREQUENCY_SET_POINT_ENABLED)
    }
    fn function(&self) -> &EssFunction {
        self._ess_point().function.as_ref().unwrap_or(&ess_point::FUNCTION)
    }
    fn mode(&self) -> &super::commonmodule::EngGridConnectModeKind {
        self._ess_point().mode.as_ref().unwrap_or(&ess_point::MODE)
    }
    fn pct_hz_droop(&self) -> &f32 {
        self._ess_point().pct_hz_droop.as_ref().unwrap_or(&ess_point::PCT_HZ_DROOP)
    }
    fn pct_v_droop(&self) -> &f32 {
        self._ess_point().pct_v_droop.as_ref().unwrap_or(&ess_point::PCT_V_DROOP)
    }
    fn ramp_rates(&self) -> &super::commonmodule::RampRate {
        self._ess_point().ramp_rates.as_ref().unwrap_or(&ess_point::RAMP_RATES)
    }
    fn reactive_pwr_set_point_enabled(&self) -> &super::commonmodule::ControlDpc {
        self._ess_point().reactive_pwr_set_point_enabled.as_ref().unwrap_or(&ess_point::REACTIVE_PWR_SET_POINT_ENABLED)
    }
    fn real_pwr_set_point_enabled(&self) -> &super::commonmodule::ControlDpc {
        self._ess_point().real_pwr_set_point_enabled.as_ref().unwrap_or(&ess_point::REAL_PWR_SET_POINT_ENABLED)
    }
    fn reset(&self) -> &super::commonmodule::ControlDpc {
        self._ess_point().reset.as_ref().unwrap_or(&ess_point::RESET)
    }
    fn state(&self) -> &super::commonmodule::OptionalStateKind {
        self._ess_point().state.as_ref().unwrap_or(&ess_point::STATE)
    }
    fn sync_back_to_grid(&self) -> &super::commonmodule::ControlDpc {
        self._ess_point().sync_back_to_grid.as_ref().unwrap_or(&ess_point::SYNC_BACK_TO_GRID)
    }
    fn trans_to_islnd_on_grid_loss_enabled(&self) -> &super::commonmodule::ControlDpc {
        self._ess_point().trans_to_islnd_on_grid_loss_enabled.as_ref().unwrap_or(&ess_point::TRANS_TO_ISLND_ON_GRID_LOSS_ENABLED)
    }
    fn voltage_set_point_enabled(&self) -> &super::commonmodule::ControlDpc {
        self._ess_point().voltage_set_point_enabled.as_ref().unwrap_or(&ess_point::VOLTAGE_SET_POINT_ENABLED)
    }
    fn start_time(&self) -> &super::commonmodule::ControlTimestamp {
        self._ess_point().start_time.as_ref().unwrap_or(&ess_point::START_TIME)
    }
}
impl IsEssPoint for EssPoint {
    fn _ess_point(&self) -> &EssPoint {
        self
    }
}
/// Curve shape setting (FC=SP) (CSG_SP)
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Esscsg {
    /// The array with the points specifying a curve shape.
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, repeated, tag="1")]
    pub crv_pts: ::std::vec::Vec<EssPoint>,
}
mod esscsg {
    use lazy_static::lazy_static;
    lazy_static! {
    }
}
trait IsEsscsg {
    fn _esscsg(&self) -> &Esscsg;
    fn crv_pts(&self) -> &::std::vec::Vec<EssPoint> {
        &self._esscsg().crv_pts    }
}
impl IsEsscsg for Esscsg {
    fn _esscsg(&self) -> &Esscsg {
        self
    }
}
/// OpenFMB specialization for control schedule using:  LN: Schedule   Name: FSCH
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct EssControlScheduleFsch {
    /// Discrete value in ESSCSG type
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="1")]
    pub val_dcsg: ::std::option::Option<Esscsg>,
}
mod ess_control_schedule_fsch {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref VAL_DCSG: crate::essmodule::Esscsg = Default::default();
    }
}
trait IsEssControlScheduleFsch {
    fn _ess_control_schedule_fsch(&self) -> &EssControlScheduleFsch;
    fn val_dcsg(&self) -> &Esscsg {
        self._ess_control_schedule_fsch().val_dcsg.as_ref().unwrap_or(&ess_control_schedule_fsch::VAL_DCSG)
    }
}
impl IsEssControlScheduleFsch for EssControlScheduleFsch {
    fn _ess_control_schedule_fsch(&self) -> &EssControlScheduleFsch {
        self
    }
}
/// Specialized 61850 FSCC class.  LN: Schedule controller   Name: FSCC
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct EssControlFscc {
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
    pub ess_control_schedule_fsch: ::std::option::Option<EssControlScheduleFsch>,
}
mod ess_control_fscc {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref CONTROL_FSCC: crate::commonmodule::ControlFscc = Default::default();
        pub(super) static ref ESS_CONTROL_SCHEDULE_FSCH: crate::essmodule::EssControlScheduleFsch = Default::default();
    }
}
trait IsEssControlFscc {
    fn _ess_control_fscc(&self) -> &EssControlFscc;
    fn control_fscc(&self) -> &super::commonmodule::ControlFscc {
        self._ess_control_fscc().control_fscc.as_ref().unwrap_or(&ess_control_fscc::CONTROL_FSCC)
    }
    fn ess_control_schedule_fsch(&self) -> &EssControlScheduleFsch {
        self._ess_control_fscc().ess_control_schedule_fsch.as_ref().unwrap_or(&ess_control_fscc::ESS_CONTROL_SCHEDULE_FSCH)
    }
}
impl IsEssControlFscc for EssControlFscc {
    fn _ess_control_fscc(&self) -> &EssControlFscc {
        self
    }
}
/// ESS control class
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct EssControl {
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
trait IsEssControl {
    fn _ess_control(&self) -> &EssControl;
    fn control_value(&self) -> &super::commonmodule::ControlValue {
        self._ess_control().control_value.as_ref().unwrap_or(&ess_control::CONTROL_VALUE)
    }
    fn check(&self) -> &super::commonmodule::CheckConditions {
        self._ess_control().check.as_ref().unwrap_or(&ess_control::CHECK)
    }
    fn ess_control_fscc(&self) -> &EssControlFscc {
        self._ess_control().ess_control_fscc.as_ref().unwrap_or(&ess_control::ESS_CONTROL_FSCC)
    }
}
impl IsEssControl for EssControl {
    fn _ess_control(&self) -> &EssControl {
        self
    }
}
/// ESS control profile
/// OpenFMB Profile Message: true
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct EssControlProfile {
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
    pub ess: ::std::option::Option<super::commonmodule::Ess>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="3")]
    pub ess_control: ::std::option::Option<EssControl>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="4")]
    pub ied: ::std::option::Option<super::commonmodule::Ied>,
}
mod ess_control_profile {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref CONTROL_MESSAGE_INFO: crate::commonmodule::ControlMessageInfo = Default::default();
        pub(super) static ref ESS: crate::commonmodule::Ess = Default::default();
        pub(super) static ref ESS_CONTROL: crate::essmodule::EssControl = Default::default();
        pub(super) static ref IED: crate::commonmodule::Ied = Default::default();
    }
}
trait IsEssControlProfile {
    fn _ess_control_profile(&self) -> &EssControlProfile;
    fn control_message_info(&self) -> &super::commonmodule::ControlMessageInfo {
        self._ess_control_profile().control_message_info.as_ref().unwrap_or(&ess_control_profile::CONTROL_MESSAGE_INFO)
    }
    fn ess(&self) -> &super::commonmodule::Ess {
        self._ess_control_profile().ess.as_ref().unwrap_or(&ess_control_profile::ESS)
    }
    fn ess_control(&self) -> &EssControl {
        self._ess_control_profile().ess_control.as_ref().unwrap_or(&ess_control_profile::ESS_CONTROL)
    }
    fn ied(&self) -> &super::commonmodule::Ied {
        self._ess_control_profile().ied.as_ref().unwrap_or(&ess_control_profile::IED)
    }
}
impl IsEssControlProfile for EssControlProfile {
    fn _ess_control_profile(&self) -> &EssControlProfile {
        self
    }
}
