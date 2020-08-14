/// Specialized 61850 ZBAT class  LN: Battery   Name: ZBAT
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct EssEventZbat {
    /// UML inherited base object
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
/// Specialized 61850 ZGEN class
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct EssEventAndStatusZgen {
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
    /// Generator is synchronized to EPS, or not; True = Synchronized
    #[prost(message, optional, tag="5")]
    pub gn_syn_st: ::std::option::Option<super::commonmodule::StatusSps>,
    /// Point status
    #[prost(message, optional, tag="6")]
    pub point_status: ::std::option::Option<EssPointStatus>,
}
/// Specialized 61850 ZGEN class for ESS event profile
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct EssEventZgen {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub e_ss_event_and_status_zgen: ::std::option::Option<EssEventAndStatusZgen>,
}
/// ESS event
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct EssEvent {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub event_value: ::std::option::Option<super::commonmodule::EventValue>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub ess_event_zbat: ::std::option::Option<EssEventZbat>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub ess_event_zgen: ::std::option::Option<EssEventZgen>,
}
/// ESS event profile
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct EssEventProfile {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub event_message_info: ::std::option::Option<super::commonmodule::EventMessageInfo>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub ess: ::std::option::Option<super::commonmodule::Ess>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub ess_event: ::std::option::Option<EssEvent>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="4")]
    pub ied: ::std::option::Option<super::commonmodule::Ied>,
}
/// ESS reading value
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct EssReading {
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
/// ESS reading profile
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct EssReadingProfile {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub reading_message_info: ::std::option::Option<super::commonmodule::ReadingMessageInfo>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub ess: ::std::option::Option<super::commonmodule::Ess>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub ess_reading: ::std::option::Option<EssReading>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="4")]
    pub ied: ::std::option::Option<super::commonmodule::Ied>,
}
/// Specialized 61850 ZBAT
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct EssStatusZbat {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub logical_node_for_event_and_status: ::std::option::Option<super::commonmodule::LogicalNodeForEventAndStatus>,
    /// Battery system status &ndash; True: on
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
/// Specialized 61850 ZGEN class
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct EssStatusZgen {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub e_ss_event_and_status_zgen: ::std::option::Option<EssEventAndStatusZgen>,
}
/// ESS status
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct EssStatus {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub status_value: ::std::option::Option<super::commonmodule::StatusValue>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub ess_status_zbat: ::std::option::Option<EssStatusZbat>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub ess_status_zgen: ::std::option::Option<EssStatusZgen>,
}
/// ESS status profile
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct EssStatusProfile {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub status_message_info: ::std::option::Option<super::commonmodule::StatusMessageInfo>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub ess: ::std::option::Option<super::commonmodule::Ess>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub ess_status: ::std::option::Option<EssStatus>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="4")]
    pub ied: ::std::option::Option<super::commonmodule::Ied>,
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
    #[prost(message, optional, tag="15")]
    pub start_time: ::std::option::Option<super::commonmodule::ControlTimestamp>,
}
/// Curve shape setting (FC=SP) (CSG_SP)
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Esscsg {
    /// The array with the points specifying a curve shape.
    #[prost(message, repeated, tag="1")]
    pub crv_pts: ::std::vec::Vec<EssPoint>,
}
/// OpenFMB specialization for control schedule using:  LN: Schedule   Name: FSCH
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct EssControlScheduleFsch {
    /// Discrete value in ESSCSG type
    #[prost(message, optional, tag="1")]
    pub val_dcsg: ::std::option::Option<Esscsg>,
}
/// Specialized 61850 FSCC class.  LN: Schedule controller   Name: FSCC
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct EssControlFscc {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub control_fscc: ::std::option::Option<super::commonmodule::ControlFscc>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub ess_control_schedule_fsch: ::std::option::Option<EssControlScheduleFsch>,
}
/// ESS control class
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct EssControl {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub control_value: ::std::option::Option<super::commonmodule::ControlValue>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub check: ::std::option::Option<super::commonmodule::CheckConditions>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub ess_control_fscc: ::std::option::Option<EssControlFscc>,
}
/// ESS control profile
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct EssControlProfile {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub control_message_info: ::std::option::Option<super::commonmodule::ControlMessageInfo>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub ess: ::std::option::Option<super::commonmodule::Ess>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub ess_control: ::std::option::Option<EssControl>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="4")]
    pub ied: ::std::option::Option<super::commonmodule::Ied>,
}
