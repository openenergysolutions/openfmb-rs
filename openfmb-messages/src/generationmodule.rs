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
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="13")]
    pub start_time: ::std::option::Option<super::commonmodule::ControlTimestamp>,
}
mod generation_point {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref BLACK_START_ENABLED: crate::commonmodule::ControlDpc = Default::default();
        pub(super) static ref FREQUENCY_SET_POINT_ENABLED: crate::commonmodule::ControlDpc = Default::default();
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
pub trait IsGenerationPoint {
    fn _generation_point(&self) -> &GenerationPoint;
    fn _mut_generation_point(&mut self) -> &mut GenerationPoint;
    fn black_start_enabled(&self) -> &super::commonmodule::ControlDpc {
        self._generation_point().black_start_enabled.as_ref().unwrap_or(&generation_point::BLACK_START_ENABLED)
    }
    fn mut_black_start_enabled(&mut self) -> &mut super::commonmodule::ControlDpc {
        self._mut_generation_point().black_start_enabled.get_or_insert(generation_point::BLACK_START_ENABLED.clone())
    }
    fn frequency_set_point_enabled(&self) -> &super::commonmodule::ControlDpc {
        self._generation_point().frequency_set_point_enabled.as_ref().unwrap_or(&generation_point::FREQUENCY_SET_POINT_ENABLED)
    }
    fn mut_frequency_set_point_enabled(&mut self) -> &mut super::commonmodule::ControlDpc {
        self._mut_generation_point().frequency_set_point_enabled.get_or_insert(generation_point::FREQUENCY_SET_POINT_ENABLED.clone())
    }
    fn pct_hz_droop(&self) -> &f32 {
        self._generation_point().pct_hz_droop.as_ref().unwrap_or(&generation_point::PCT_HZ_DROOP)
    }
    fn mut_pct_hz_droop(&mut self) -> &mut f32 {
        self._mut_generation_point().pct_hz_droop.get_or_insert(generation_point::PCT_HZ_DROOP.clone())
    }
    fn pct_v_droop(&self) -> &f32 {
        self._generation_point().pct_v_droop.as_ref().unwrap_or(&generation_point::PCT_V_DROOP)
    }
    fn mut_pct_v_droop(&mut self) -> &mut f32 {
        self._mut_generation_point().pct_v_droop.get_or_insert(generation_point::PCT_V_DROOP.clone())
    }
    fn ramp_rates(&self) -> &super::commonmodule::RampRate {
        self._generation_point().ramp_rates.as_ref().unwrap_or(&generation_point::RAMP_RATES)
    }
    fn mut_ramp_rates(&mut self) -> &mut super::commonmodule::RampRate {
        self._mut_generation_point().ramp_rates.get_or_insert(generation_point::RAMP_RATES.clone())
    }
    fn reactive_pwr_set_point_enabled(&self) -> &super::commonmodule::ControlDpc {
        self._generation_point().reactive_pwr_set_point_enabled.as_ref().unwrap_or(&generation_point::REACTIVE_PWR_SET_POINT_ENABLED)
    }
    fn mut_reactive_pwr_set_point_enabled(&mut self) -> &mut super::commonmodule::ControlDpc {
        self._mut_generation_point().reactive_pwr_set_point_enabled.get_or_insert(generation_point::REACTIVE_PWR_SET_POINT_ENABLED.clone())
    }
    fn real_pwr_set_point_enabled(&self) -> &super::commonmodule::ControlDpc {
        self._generation_point().real_pwr_set_point_enabled.as_ref().unwrap_or(&generation_point::REAL_PWR_SET_POINT_ENABLED)
    }
    fn mut_real_pwr_set_point_enabled(&mut self) -> &mut super::commonmodule::ControlDpc {
        self._mut_generation_point().real_pwr_set_point_enabled.get_or_insert(generation_point::REAL_PWR_SET_POINT_ENABLED.clone())
    }
    fn reset(&self) -> &super::commonmodule::ControlDpc {
        self._generation_point().reset.as_ref().unwrap_or(&generation_point::RESET)
    }
    fn mut_reset(&mut self) -> &mut super::commonmodule::ControlDpc {
        self._mut_generation_point().reset.get_or_insert(generation_point::RESET.clone())
    }
    fn state(&self) -> &super::commonmodule::OptionalStateKind {
        self._generation_point().state.as_ref().unwrap_or(&generation_point::STATE)
    }
    fn mut_state(&mut self) -> &mut super::commonmodule::OptionalStateKind {
        self._mut_generation_point().state.get_or_insert(generation_point::STATE.clone())
    }
    fn sync_back_to_grid(&self) -> &super::commonmodule::ControlDpc {
        self._generation_point().sync_back_to_grid.as_ref().unwrap_or(&generation_point::SYNC_BACK_TO_GRID)
    }
    fn mut_sync_back_to_grid(&mut self) -> &mut super::commonmodule::ControlDpc {
        self._mut_generation_point().sync_back_to_grid.get_or_insert(generation_point::SYNC_BACK_TO_GRID.clone())
    }
    fn trans_to_islnd_on_grid_loss_enabled(&self) -> &super::commonmodule::ControlDpc {
        self._generation_point().trans_to_islnd_on_grid_loss_enabled.as_ref().unwrap_or(&generation_point::TRANS_TO_ISLND_ON_GRID_LOSS_ENABLED)
    }
    fn mut_trans_to_islnd_on_grid_loss_enabled(&mut self) -> &mut super::commonmodule::ControlDpc {
        self._mut_generation_point().trans_to_islnd_on_grid_loss_enabled.get_or_insert(generation_point::TRANS_TO_ISLND_ON_GRID_LOSS_ENABLED.clone())
    }
    fn voltage_set_point_enabled(&self) -> &super::commonmodule::ControlDpc {
        self._generation_point().voltage_set_point_enabled.as_ref().unwrap_or(&generation_point::VOLTAGE_SET_POINT_ENABLED)
    }
    fn mut_voltage_set_point_enabled(&mut self) -> &mut super::commonmodule::ControlDpc {
        self._mut_generation_point().voltage_set_point_enabled.get_or_insert(generation_point::VOLTAGE_SET_POINT_ENABLED.clone())
    }
    fn start_time(&self) -> &super::commonmodule::ControlTimestamp {
        self._generation_point().start_time.as_ref().unwrap_or(&generation_point::START_TIME)
    }
    fn mut_start_time(&mut self) -> &mut super::commonmodule::ControlTimestamp {
        self._mut_generation_point().start_time.get_or_insert(generation_point::START_TIME.clone())
    }
}
impl IsGenerationPoint for GenerationPoint {
    fn _generation_point(&self) -> &GenerationPoint {
        self
    }
    fn _mut_generation_point(&mut self) -> &mut GenerationPoint {
        self
    }
}
/// Curve shape setting (FC=SP) (CSG_SP)
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct GenerationCsg {
    /// The array with the points specifying a curve shape.
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, repeated, tag="1")]
    pub crv_pts: ::std::vec::Vec<GenerationPoint>,
}
mod generation_csg {
    use lazy_static::lazy_static;
    lazy_static! {
    }
}
pub trait IsGenerationCsg {
    fn _generation_csg(&self) -> &GenerationCsg;
    fn _mut_generation_csg(&mut self) -> &mut GenerationCsg;
    fn crv_pts(&self) -> &::std::vec::Vec<GenerationPoint> {
        &self._generation_csg().crv_pts    }
    fn mut_crv_pts(&mut self) -> &mut ::std::vec::Vec<GenerationPoint> {
        &mut self._mut_generation_csg().crv_pts    }
}
impl IsGenerationCsg for GenerationCsg {
    fn _generation_csg(&self) -> &GenerationCsg {
        self
    }
    fn _mut_generation_csg(&mut self) -> &mut GenerationCsg {
        self
    }
}
/// OpenFMB specialization for control schedule using:  LN: Schedule   Name: FSCH
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct GenerationControlScheduleFsch {
    /// Discrete value in GenerationCSG type
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="1")]
    pub val_dcsg: ::std::option::Option<GenerationCsg>,
}
mod generation_control_schedule_fsch {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref VAL_DCSG: crate::generationmodule::GenerationCsg = Default::default();
    }
}
pub trait IsGenerationControlScheduleFsch {
    fn _generation_control_schedule_fsch(&self) -> &GenerationControlScheduleFsch;
    fn _mut_generation_control_schedule_fsch(&mut self) -> &mut GenerationControlScheduleFsch;
    fn val_dcsg(&self) -> &GenerationCsg {
        self._generation_control_schedule_fsch().val_dcsg.as_ref().unwrap_or(&generation_control_schedule_fsch::VAL_DCSG)
    }
    fn mut_val_dcsg(&mut self) -> &mut GenerationCsg {
        self._mut_generation_control_schedule_fsch().val_dcsg.get_or_insert(generation_control_schedule_fsch::VAL_DCSG.clone())
    }
}
impl IsGenerationControlScheduleFsch for GenerationControlScheduleFsch {
    fn _generation_control_schedule_fsch(&self) -> &GenerationControlScheduleFsch {
        self
    }
    fn _mut_generation_control_schedule_fsch(&mut self) -> &mut GenerationControlScheduleFsch {
        self
    }
}
/// LN: Schedule controller   Name: FSCC
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct GenerationControlFscc {
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
    pub generation_control_schedule_fsch: ::std::option::Option<GenerationControlScheduleFsch>,
}
mod generation_control_fscc {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref CONTROL_FSCC: crate::commonmodule::ControlFscc = Default::default();
        pub(super) static ref GENERATION_CONTROL_SCHEDULE_FSCH: crate::generationmodule::GenerationControlScheduleFsch = Default::default();
    }
}
pub trait IsGenerationControlFscc {
    fn _generation_control_fscc(&self) -> &GenerationControlFscc;
    fn _mut_generation_control_fscc(&mut self) -> &mut GenerationControlFscc;
    fn control_fscc(&self) -> &super::commonmodule::ControlFscc {
        self._generation_control_fscc().control_fscc.as_ref().unwrap_or(&generation_control_fscc::CONTROL_FSCC)
    }
    fn mut_control_fscc(&mut self) -> &mut super::commonmodule::ControlFscc {
        self._mut_generation_control_fscc().control_fscc.get_or_insert(generation_control_fscc::CONTROL_FSCC.clone())
    }
    fn generation_control_schedule_fsch(&self) -> &GenerationControlScheduleFsch {
        self._generation_control_fscc().generation_control_schedule_fsch.as_ref().unwrap_or(&generation_control_fscc::GENERATION_CONTROL_SCHEDULE_FSCH)
    }
    fn mut_generation_control_schedule_fsch(&mut self) -> &mut GenerationControlScheduleFsch {
        self._mut_generation_control_fscc().generation_control_schedule_fsch.get_or_insert(generation_control_fscc::GENERATION_CONTROL_SCHEDULE_FSCH.clone())
    }
}
impl IsGenerationControlFscc for GenerationControlFscc {
    fn _generation_control_fscc(&self) -> &GenerationControlFscc {
        self
    }
    fn _mut_generation_control_fscc(&mut self) -> &mut GenerationControlFscc {
        self
    }
}
/// Generation control
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct GenerationControl {
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
    pub generation_control_fscc: ::std::option::Option<GenerationControlFscc>,
}
mod generation_control {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref CONTROL_VALUE: crate::commonmodule::ControlValue = Default::default();
        pub(super) static ref CHECK: crate::commonmodule::CheckConditions = Default::default();
        pub(super) static ref GENERATION_CONTROL_FSCC: crate::generationmodule::GenerationControlFscc = Default::default();
    }
}
pub trait IsGenerationControl {
    fn _generation_control(&self) -> &GenerationControl;
    fn _mut_generation_control(&mut self) -> &mut GenerationControl;
    fn control_value(&self) -> &super::commonmodule::ControlValue {
        self._generation_control().control_value.as_ref().unwrap_or(&generation_control::CONTROL_VALUE)
    }
    fn mut_control_value(&mut self) -> &mut super::commonmodule::ControlValue {
        self._mut_generation_control().control_value.get_or_insert(generation_control::CONTROL_VALUE.clone())
    }
    fn check(&self) -> &super::commonmodule::CheckConditions {
        self._generation_control().check.as_ref().unwrap_or(&generation_control::CHECK)
    }
    fn mut_check(&mut self) -> &mut super::commonmodule::CheckConditions {
        self._mut_generation_control().check.get_or_insert(generation_control::CHECK.clone())
    }
    fn generation_control_fscc(&self) -> &GenerationControlFscc {
        self._generation_control().generation_control_fscc.as_ref().unwrap_or(&generation_control::GENERATION_CONTROL_FSCC)
    }
    fn mut_generation_control_fscc(&mut self) -> &mut GenerationControlFscc {
        self._mut_generation_control().generation_control_fscc.get_or_insert(generation_control::GENERATION_CONTROL_FSCC.clone())
    }
}
impl IsGenerationControl for GenerationControl {
    fn _generation_control(&self) -> &GenerationControl {
        self
    }
    fn _mut_generation_control(&mut self) -> &mut GenerationControl {
        self
    }
}
/// A single or set of synchronous machines for converting mechanical power into alternating-current
/// power. For example, individual machines within a set may be defined for scheduling purposes while a
/// single control signal is derived for the set. In this case there would be a GeneratingUnit for each
/// member of the set and an additional GeneratingUnit corresponding to the set.
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct GeneratingUnit {
    /// UML inherited base object
    // parent_message: true
    // required_field: false
    // multiplicity_min: 0
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="1")]
    pub conducting_equipment: ::std::option::Option<super::commonmodule::ConductingEquipment>,
    /// This is the maximum operating active power limit the dispatcher can enter for this unit.
    #[prost(message, optional, tag="2")]
    pub max_operating_p: ::std::option::Option<super::commonmodule::ActivePower>,
}
mod generating_unit {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref CONDUCTING_EQUIPMENT: crate::commonmodule::ConductingEquipment = Default::default();
        pub(super) static ref MAX_OPERATING_P: crate::commonmodule::ActivePower = Default::default();
    }
}
pub trait IsGeneratingUnit {
    fn _generating_unit(&self) -> &GeneratingUnit;
    fn _mut_generating_unit(&mut self) -> &mut GeneratingUnit;
    fn conducting_equipment(&self) -> &super::commonmodule::ConductingEquipment {
        self._generating_unit().conducting_equipment.as_ref().unwrap_or(&generating_unit::CONDUCTING_EQUIPMENT)
    }
    fn mut_conducting_equipment(&mut self) -> &mut super::commonmodule::ConductingEquipment {
        self._mut_generating_unit().conducting_equipment.get_or_insert(generating_unit::CONDUCTING_EQUIPMENT.clone())
    }
    fn max_operating_p(&self) -> &super::commonmodule::ActivePower {
        self._generating_unit().max_operating_p.as_ref().unwrap_or(&generating_unit::MAX_OPERATING_P)
    }
    fn mut_max_operating_p(&mut self) -> &mut super::commonmodule::ActivePower {
        self._mut_generating_unit().max_operating_p.get_or_insert(generating_unit::MAX_OPERATING_P.clone())
    }
}
impl IsGeneratingUnit for GeneratingUnit {
    fn _generating_unit(&self) -> &GeneratingUnit {
        self
    }
    fn _mut_generating_unit(&mut self) -> &mut GeneratingUnit {
        self
    }
}
/// Generation control profile
/// OpenFMB Profile Message: true
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct GenerationControlProfile {
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
    pub generating_unit: ::std::option::Option<GeneratingUnit>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="3")]
    pub generation_control: ::std::option::Option<GenerationControl>,
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
mod generation_control_profile {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref CONTROL_MESSAGE_INFO: crate::commonmodule::ControlMessageInfo = Default::default();
        pub(super) static ref GENERATING_UNIT: crate::generationmodule::GeneratingUnit = Default::default();
        pub(super) static ref GENERATION_CONTROL: crate::generationmodule::GenerationControl = Default::default();
        pub(super) static ref IED: crate::commonmodule::Ied = Default::default();
    }
}
pub trait IsGenerationControlProfile {
    fn _generation_control_profile(&self) -> &GenerationControlProfile;
    fn _mut_generation_control_profile(&mut self) -> &mut GenerationControlProfile;
    fn control_message_info(&self) -> &super::commonmodule::ControlMessageInfo {
        self._generation_control_profile().control_message_info.as_ref().unwrap_or(&generation_control_profile::CONTROL_MESSAGE_INFO)
    }
    fn mut_control_message_info(&mut self) -> &mut super::commonmodule::ControlMessageInfo {
        self._mut_generation_control_profile().control_message_info.get_or_insert(generation_control_profile::CONTROL_MESSAGE_INFO.clone())
    }
    fn generating_unit(&self) -> &GeneratingUnit {
        self._generation_control_profile().generating_unit.as_ref().unwrap_or(&generation_control_profile::GENERATING_UNIT)
    }
    fn mut_generating_unit(&mut self) -> &mut GeneratingUnit {
        self._mut_generation_control_profile().generating_unit.get_or_insert(generation_control_profile::GENERATING_UNIT.clone())
    }
    fn generation_control(&self) -> &GenerationControl {
        self._generation_control_profile().generation_control.as_ref().unwrap_or(&generation_control_profile::GENERATION_CONTROL)
    }
    fn mut_generation_control(&mut self) -> &mut GenerationControl {
        self._mut_generation_control_profile().generation_control.get_or_insert(generation_control_profile::GENERATION_CONTROL.clone())
    }
    fn ied(&self) -> &super::commonmodule::Ied {
        self._generation_control_profile().ied.as_ref().unwrap_or(&generation_control_profile::IED)
    }
    fn mut_ied(&mut self) -> &mut super::commonmodule::Ied {
        self._mut_generation_control_profile().ied.get_or_insert(generation_control_profile::IED.clone())
    }
}
impl IsGenerationControlProfile for GenerationControlProfile {
    fn _generation_control_profile(&self) -> &GenerationControlProfile {
        self
    }
    fn _mut_generation_control_profile(&mut self) -> &mut GenerationControlProfile {
        self
    }
}
/// Generation reading value
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct GenerationReading {
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
mod generation_reading {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref CONDUCTING_EQUIPMENT_TERMINAL_READING: crate::commonmodule::ConductingEquipmentTerminalReading = Default::default();
        pub(super) static ref PHASE_MMTN: crate::commonmodule::PhaseMmtn = Default::default();
        pub(super) static ref READING_MMTR: crate::commonmodule::ReadingMmtr = Default::default();
        pub(super) static ref READING_MMXU: crate::commonmodule::ReadingMmxu = Default::default();
    }
}
pub trait IsGenerationReading {
    fn _generation_reading(&self) -> &GenerationReading;
    fn _mut_generation_reading(&mut self) -> &mut GenerationReading;
    fn conducting_equipment_terminal_reading(&self) -> &super::commonmodule::ConductingEquipmentTerminalReading {
        self._generation_reading().conducting_equipment_terminal_reading.as_ref().unwrap_or(&generation_reading::CONDUCTING_EQUIPMENT_TERMINAL_READING)
    }
    fn mut_conducting_equipment_terminal_reading(&mut self) -> &mut super::commonmodule::ConductingEquipmentTerminalReading {
        self._mut_generation_reading().conducting_equipment_terminal_reading.get_or_insert(generation_reading::CONDUCTING_EQUIPMENT_TERMINAL_READING.clone())
    }
    fn phase_mmtn(&self) -> &super::commonmodule::PhaseMmtn {
        self._generation_reading().phase_mmtn.as_ref().unwrap_or(&generation_reading::PHASE_MMTN)
    }
    fn mut_phase_mmtn(&mut self) -> &mut super::commonmodule::PhaseMmtn {
        self._mut_generation_reading().phase_mmtn.get_or_insert(generation_reading::PHASE_MMTN.clone())
    }
    fn reading_mmtr(&self) -> &super::commonmodule::ReadingMmtr {
        self._generation_reading().reading_mmtr.as_ref().unwrap_or(&generation_reading::READING_MMTR)
    }
    fn mut_reading_mmtr(&mut self) -> &mut super::commonmodule::ReadingMmtr {
        self._mut_generation_reading().reading_mmtr.get_or_insert(generation_reading::READING_MMTR.clone())
    }
    fn reading_mmxu(&self) -> &super::commonmodule::ReadingMmxu {
        self._generation_reading().reading_mmxu.as_ref().unwrap_or(&generation_reading::READING_MMXU)
    }
    fn mut_reading_mmxu(&mut self) -> &mut super::commonmodule::ReadingMmxu {
        self._mut_generation_reading().reading_mmxu.get_or_insert(generation_reading::READING_MMXU.clone())
    }
}
impl IsGenerationReading for GenerationReading {
    fn _generation_reading(&self) -> &GenerationReading {
        self
    }
    fn _mut_generation_reading(&mut self) -> &mut GenerationReading {
        self
    }
}
/// Generation reading profile
/// OpenFMB Profile Message: true
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct GenerationReadingProfile {
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
    pub generating_unit: ::std::option::Option<GeneratingUnit>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="3")]
    pub generation_reading: ::std::option::Option<GenerationReading>,
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
mod generation_reading_profile {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref READING_MESSAGE_INFO: crate::commonmodule::ReadingMessageInfo = Default::default();
        pub(super) static ref GENERATING_UNIT: crate::generationmodule::GeneratingUnit = Default::default();
        pub(super) static ref GENERATION_READING: crate::generationmodule::GenerationReading = Default::default();
        pub(super) static ref IED: crate::commonmodule::Ied = Default::default();
    }
}
pub trait IsGenerationReadingProfile {
    fn _generation_reading_profile(&self) -> &GenerationReadingProfile;
    fn _mut_generation_reading_profile(&mut self) -> &mut GenerationReadingProfile;
    fn reading_message_info(&self) -> &super::commonmodule::ReadingMessageInfo {
        self._generation_reading_profile().reading_message_info.as_ref().unwrap_or(&generation_reading_profile::READING_MESSAGE_INFO)
    }
    fn mut_reading_message_info(&mut self) -> &mut super::commonmodule::ReadingMessageInfo {
        self._mut_generation_reading_profile().reading_message_info.get_or_insert(generation_reading_profile::READING_MESSAGE_INFO.clone())
    }
    fn generating_unit(&self) -> &GeneratingUnit {
        self._generation_reading_profile().generating_unit.as_ref().unwrap_or(&generation_reading_profile::GENERATING_UNIT)
    }
    fn mut_generating_unit(&mut self) -> &mut GeneratingUnit {
        self._mut_generation_reading_profile().generating_unit.get_or_insert(generation_reading_profile::GENERATING_UNIT.clone())
    }
    fn generation_reading(&self) -> &GenerationReading {
        self._generation_reading_profile().generation_reading.as_ref().unwrap_or(&generation_reading_profile::GENERATION_READING)
    }
    fn mut_generation_reading(&mut self) -> &mut GenerationReading {
        self._mut_generation_reading_profile().generation_reading.get_or_insert(generation_reading_profile::GENERATION_READING.clone())
    }
    fn ied(&self) -> &super::commonmodule::Ied {
        self._generation_reading_profile().ied.as_ref().unwrap_or(&generation_reading_profile::IED)
    }
    fn mut_ied(&mut self) -> &mut super::commonmodule::Ied {
        self._mut_generation_reading_profile().ied.get_or_insert(generation_reading_profile::IED.clone())
    }
}
impl IsGenerationReadingProfile for GenerationReadingProfile {
    fn _generation_reading_profile(&self) -> &GenerationReadingProfile {
        self
    }
    fn _mut_generation_reading_profile(&mut self) -> &mut GenerationReadingProfile {
        self
    }
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
mod generation_point_status {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref BLACK_START_ENABLED: crate::commonmodule::ControlDpc = Default::default();
        pub(super) static ref FREQUENCY_SET_POINT_ENABLED: crate::commonmodule::ControlDpc = Default::default();
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
pub trait IsGenerationPointStatus {
    fn _generation_point_status(&self) -> &GenerationPointStatus;
    fn _mut_generation_point_status(&mut self) -> &mut GenerationPointStatus;
    fn black_start_enabled(&self) -> &super::commonmodule::ControlDpc {
        self._generation_point_status().black_start_enabled.as_ref().unwrap_or(&generation_point_status::BLACK_START_ENABLED)
    }
    fn mut_black_start_enabled(&mut self) -> &mut super::commonmodule::ControlDpc {
        self._mut_generation_point_status().black_start_enabled.get_or_insert(generation_point_status::BLACK_START_ENABLED.clone())
    }
    fn frequency_set_point_enabled(&self) -> &super::commonmodule::ControlDpc {
        self._generation_point_status().frequency_set_point_enabled.as_ref().unwrap_or(&generation_point_status::FREQUENCY_SET_POINT_ENABLED)
    }
    fn mut_frequency_set_point_enabled(&mut self) -> &mut super::commonmodule::ControlDpc {
        self._mut_generation_point_status().frequency_set_point_enabled.get_or_insert(generation_point_status::FREQUENCY_SET_POINT_ENABLED.clone())
    }
    fn pct_hz_droop(&self) -> &f32 {
        self._generation_point_status().pct_hz_droop.as_ref().unwrap_or(&generation_point_status::PCT_HZ_DROOP)
    }
    fn mut_pct_hz_droop(&mut self) -> &mut f32 {
        self._mut_generation_point_status().pct_hz_droop.get_or_insert(generation_point_status::PCT_HZ_DROOP.clone())
    }
    fn pct_v_droop(&self) -> &f32 {
        self._generation_point_status().pct_v_droop.as_ref().unwrap_or(&generation_point_status::PCT_V_DROOP)
    }
    fn mut_pct_v_droop(&mut self) -> &mut f32 {
        self._mut_generation_point_status().pct_v_droop.get_or_insert(generation_point_status::PCT_V_DROOP.clone())
    }
    fn ramp_rates(&self) -> &super::commonmodule::RampRate {
        self._generation_point_status().ramp_rates.as_ref().unwrap_or(&generation_point_status::RAMP_RATES)
    }
    fn mut_ramp_rates(&mut self) -> &mut super::commonmodule::RampRate {
        self._mut_generation_point_status().ramp_rates.get_or_insert(generation_point_status::RAMP_RATES.clone())
    }
    fn reactive_pwr_set_point_enabled(&self) -> &super::commonmodule::ControlDpc {
        self._generation_point_status().reactive_pwr_set_point_enabled.as_ref().unwrap_or(&generation_point_status::REACTIVE_PWR_SET_POINT_ENABLED)
    }
    fn mut_reactive_pwr_set_point_enabled(&mut self) -> &mut super::commonmodule::ControlDpc {
        self._mut_generation_point_status().reactive_pwr_set_point_enabled.get_or_insert(generation_point_status::REACTIVE_PWR_SET_POINT_ENABLED.clone())
    }
    fn real_pwr_set_point_enabled(&self) -> &super::commonmodule::ControlDpc {
        self._generation_point_status().real_pwr_set_point_enabled.as_ref().unwrap_or(&generation_point_status::REAL_PWR_SET_POINT_ENABLED)
    }
    fn mut_real_pwr_set_point_enabled(&mut self) -> &mut super::commonmodule::ControlDpc {
        self._mut_generation_point_status().real_pwr_set_point_enabled.get_or_insert(generation_point_status::REAL_PWR_SET_POINT_ENABLED.clone())
    }
    fn state(&self) -> &super::commonmodule::OptionalStateKind {
        self._generation_point_status().state.as_ref().unwrap_or(&generation_point_status::STATE)
    }
    fn mut_state(&mut self) -> &mut super::commonmodule::OptionalStateKind {
        self._mut_generation_point_status().state.get_or_insert(generation_point_status::STATE.clone())
    }
    fn sync_back_to_grid(&self) -> &super::commonmodule::ControlDpc {
        self._generation_point_status().sync_back_to_grid.as_ref().unwrap_or(&generation_point_status::SYNC_BACK_TO_GRID)
    }
    fn mut_sync_back_to_grid(&mut self) -> &mut super::commonmodule::ControlDpc {
        self._mut_generation_point_status().sync_back_to_grid.get_or_insert(generation_point_status::SYNC_BACK_TO_GRID.clone())
    }
    fn trans_to_islnd_on_grid_loss_enabled(&self) -> &super::commonmodule::ControlDpc {
        self._generation_point_status().trans_to_islnd_on_grid_loss_enabled.as_ref().unwrap_or(&generation_point_status::TRANS_TO_ISLND_ON_GRID_LOSS_ENABLED)
    }
    fn mut_trans_to_islnd_on_grid_loss_enabled(&mut self) -> &mut super::commonmodule::ControlDpc {
        self._mut_generation_point_status().trans_to_islnd_on_grid_loss_enabled.get_or_insert(generation_point_status::TRANS_TO_ISLND_ON_GRID_LOSS_ENABLED.clone())
    }
    fn voltage_set_point_enabled(&self) -> &super::commonmodule::ControlDpc {
        self._generation_point_status().voltage_set_point_enabled.as_ref().unwrap_or(&generation_point_status::VOLTAGE_SET_POINT_ENABLED)
    }
    fn mut_voltage_set_point_enabled(&mut self) -> &mut super::commonmodule::ControlDpc {
        self._mut_generation_point_status().voltage_set_point_enabled.get_or_insert(generation_point_status::VOLTAGE_SET_POINT_ENABLED.clone())
    }
}
impl IsGenerationPointStatus for GenerationPointStatus {
    fn _generation_point_status(&self) -> &GenerationPointStatus {
        self
    }
    fn _mut_generation_point_status(&mut self) -> &mut GenerationPointStatus {
        self
    }
}
/// Specialized 61850 ZGEN class
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct GenerationEventAndStatusZgen {
    /// UML inherited base object
    // parent_message: true
    // required_field: false
    // multiplicity_min: 0
    // multiplicity_max: 0
    // uuid: false
    // key: false
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
mod generation_event_and_status_zgen {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref LOGICAL_NODE_FOR_EVENT_AND_STATUS: crate::commonmodule::LogicalNodeForEventAndStatus = Default::default();
        pub(super) static ref AUX_PWR_ST: crate::commonmodule::StatusSps = Default::default();
        pub(super) static ref DYNAMIC_TEST: crate::commonmodule::EnsDynamicTestKind = Default::default();
        pub(super) static ref EMG_STOP: crate::commonmodule::StatusSps = Default::default();
        pub(super) static ref GN_SYN_ST: crate::commonmodule::StatusSps = Default::default();
        pub(super) static ref POINT_STATUS: crate::generationmodule::GenerationPointStatus = Default::default();
    }
}
pub trait IsGenerationEventAndStatusZgen {
    fn _generation_event_and_status_zgen(&self) -> &GenerationEventAndStatusZgen;
    fn _mut_generation_event_and_status_zgen(&mut self) -> &mut GenerationEventAndStatusZgen;
    fn logical_node_for_event_and_status(&self) -> &super::commonmodule::LogicalNodeForEventAndStatus {
        self._generation_event_and_status_zgen().logical_node_for_event_and_status.as_ref().unwrap_or(&generation_event_and_status_zgen::LOGICAL_NODE_FOR_EVENT_AND_STATUS)
    }
    fn mut_logical_node_for_event_and_status(&mut self) -> &mut super::commonmodule::LogicalNodeForEventAndStatus {
        self._mut_generation_event_and_status_zgen().logical_node_for_event_and_status.get_or_insert(generation_event_and_status_zgen::LOGICAL_NODE_FOR_EVENT_AND_STATUS.clone())
    }
    fn aux_pwr_st(&self) -> &super::commonmodule::StatusSps {
        self._generation_event_and_status_zgen().aux_pwr_st.as_ref().unwrap_or(&generation_event_and_status_zgen::AUX_PWR_ST)
    }
    fn mut_aux_pwr_st(&mut self) -> &mut super::commonmodule::StatusSps {
        self._mut_generation_event_and_status_zgen().aux_pwr_st.get_or_insert(generation_event_and_status_zgen::AUX_PWR_ST.clone())
    }
    fn dynamic_test(&self) -> &super::commonmodule::EnsDynamicTestKind {
        self._generation_event_and_status_zgen().dynamic_test.as_ref().unwrap_or(&generation_event_and_status_zgen::DYNAMIC_TEST)
    }
    fn mut_dynamic_test(&mut self) -> &mut super::commonmodule::EnsDynamicTestKind {
        self._mut_generation_event_and_status_zgen().dynamic_test.get_or_insert(generation_event_and_status_zgen::DYNAMIC_TEST.clone())
    }
    fn emg_stop(&self) -> &super::commonmodule::StatusSps {
        self._generation_event_and_status_zgen().emg_stop.as_ref().unwrap_or(&generation_event_and_status_zgen::EMG_STOP)
    }
    fn mut_emg_stop(&mut self) -> &mut super::commonmodule::StatusSps {
        self._mut_generation_event_and_status_zgen().emg_stop.get_or_insert(generation_event_and_status_zgen::EMG_STOP.clone())
    }
    fn gn_syn_st(&self) -> &super::commonmodule::StatusSps {
        self._generation_event_and_status_zgen().gn_syn_st.as_ref().unwrap_or(&generation_event_and_status_zgen::GN_SYN_ST)
    }
    fn mut_gn_syn_st(&mut self) -> &mut super::commonmodule::StatusSps {
        self._mut_generation_event_and_status_zgen().gn_syn_st.get_or_insert(generation_event_and_status_zgen::GN_SYN_ST.clone())
    }
    fn point_status(&self) -> &GenerationPointStatus {
        self._generation_event_and_status_zgen().point_status.as_ref().unwrap_or(&generation_event_and_status_zgen::POINT_STATUS)
    }
    fn mut_point_status(&mut self) -> &mut GenerationPointStatus {
        self._mut_generation_event_and_status_zgen().point_status.get_or_insert(generation_event_and_status_zgen::POINT_STATUS.clone())
    }
}
impl IsGenerationEventAndStatusZgen for GenerationEventAndStatusZgen {
    fn _generation_event_and_status_zgen(&self) -> &GenerationEventAndStatusZgen {
        self
    }
    fn _mut_generation_event_and_status_zgen(&mut self) -> &mut GenerationEventAndStatusZgen {
        self
    }
}
/// Specialized generation event ZGEN
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct GenerationEventZgen {
    /// UML inherited base object
    // parent_message: true
    // required_field: false
    // multiplicity_min: 0
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="1")]
    pub generation_event_and_status_zgen: ::std::option::Option<GenerationEventAndStatusZgen>,
}
mod generation_event_zgen {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref GENERATION_EVENT_AND_STATUS_ZGEN: crate::generationmodule::GenerationEventAndStatusZgen = Default::default();
    }
}
pub trait IsGenerationEventZgen {
    fn _generation_event_zgen(&self) -> &GenerationEventZgen;
    fn _mut_generation_event_zgen(&mut self) -> &mut GenerationEventZgen;
    fn generation_event_and_status_zgen(&self) -> &GenerationEventAndStatusZgen {
        self._generation_event_zgen().generation_event_and_status_zgen.as_ref().unwrap_or(&generation_event_zgen::GENERATION_EVENT_AND_STATUS_ZGEN)
    }
    fn mut_generation_event_and_status_zgen(&mut self) -> &mut GenerationEventAndStatusZgen {
        self._mut_generation_event_zgen().generation_event_and_status_zgen.get_or_insert(generation_event_zgen::GENERATION_EVENT_AND_STATUS_ZGEN.clone())
    }
}
impl IsGenerationEventZgen for GenerationEventZgen {
    fn _generation_event_zgen(&self) -> &GenerationEventZgen {
        self
    }
    fn _mut_generation_event_zgen(&mut self) -> &mut GenerationEventZgen {
        self
    }
}
/// Generation event
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct GenerationEvent {
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
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="2")]
    pub generation_event_zgen: ::std::option::Option<GenerationEventZgen>,
}
mod generation_event {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref EVENT_VALUE: crate::commonmodule::EventValue = Default::default();
        pub(super) static ref GENERATION_EVENT_ZGEN: crate::generationmodule::GenerationEventZgen = Default::default();
    }
}
pub trait IsGenerationEvent {
    fn _generation_event(&self) -> &GenerationEvent;
    fn _mut_generation_event(&mut self) -> &mut GenerationEvent;
    fn event_value(&self) -> &super::commonmodule::EventValue {
        self._generation_event().event_value.as_ref().unwrap_or(&generation_event::EVENT_VALUE)
    }
    fn mut_event_value(&mut self) -> &mut super::commonmodule::EventValue {
        self._mut_generation_event().event_value.get_or_insert(generation_event::EVENT_VALUE.clone())
    }
    fn generation_event_zgen(&self) -> &GenerationEventZgen {
        self._generation_event().generation_event_zgen.as_ref().unwrap_or(&generation_event::GENERATION_EVENT_ZGEN)
    }
    fn mut_generation_event_zgen(&mut self) -> &mut GenerationEventZgen {
        self._mut_generation_event().generation_event_zgen.get_or_insert(generation_event::GENERATION_EVENT_ZGEN.clone())
    }
}
impl IsGenerationEvent for GenerationEvent {
    fn _generation_event(&self) -> &GenerationEvent {
        self
    }
    fn _mut_generation_event(&mut self) -> &mut GenerationEvent {
        self
    }
}
/// Generation event profile
/// OpenFMB Profile Message: true
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct GenerationEventProfile {
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
    pub generating_unit: ::std::option::Option<GeneratingUnit>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="3")]
    pub generation_event: ::std::option::Option<GenerationEvent>,
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
mod generation_event_profile {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref EVENT_MESSAGE_INFO: crate::commonmodule::EventMessageInfo = Default::default();
        pub(super) static ref GENERATING_UNIT: crate::generationmodule::GeneratingUnit = Default::default();
        pub(super) static ref GENERATION_EVENT: crate::generationmodule::GenerationEvent = Default::default();
        pub(super) static ref IED: crate::commonmodule::Ied = Default::default();
    }
}
pub trait IsGenerationEventProfile {
    fn _generation_event_profile(&self) -> &GenerationEventProfile;
    fn _mut_generation_event_profile(&mut self) -> &mut GenerationEventProfile;
    fn event_message_info(&self) -> &super::commonmodule::EventMessageInfo {
        self._generation_event_profile().event_message_info.as_ref().unwrap_or(&generation_event_profile::EVENT_MESSAGE_INFO)
    }
    fn mut_event_message_info(&mut self) -> &mut super::commonmodule::EventMessageInfo {
        self._mut_generation_event_profile().event_message_info.get_or_insert(generation_event_profile::EVENT_MESSAGE_INFO.clone())
    }
    fn generating_unit(&self) -> &GeneratingUnit {
        self._generation_event_profile().generating_unit.as_ref().unwrap_or(&generation_event_profile::GENERATING_UNIT)
    }
    fn mut_generating_unit(&mut self) -> &mut GeneratingUnit {
        self._mut_generation_event_profile().generating_unit.get_or_insert(generation_event_profile::GENERATING_UNIT.clone())
    }
    fn generation_event(&self) -> &GenerationEvent {
        self._generation_event_profile().generation_event.as_ref().unwrap_or(&generation_event_profile::GENERATION_EVENT)
    }
    fn mut_generation_event(&mut self) -> &mut GenerationEvent {
        self._mut_generation_event_profile().generation_event.get_or_insert(generation_event_profile::GENERATION_EVENT.clone())
    }
    fn ied(&self) -> &super::commonmodule::Ied {
        self._generation_event_profile().ied.as_ref().unwrap_or(&generation_event_profile::IED)
    }
    fn mut_ied(&mut self) -> &mut super::commonmodule::Ied {
        self._mut_generation_event_profile().ied.get_or_insert(generation_event_profile::IED.clone())
    }
}
impl IsGenerationEventProfile for GenerationEventProfile {
    fn _generation_event_profile(&self) -> &GenerationEventProfile {
        self
    }
    fn _mut_generation_event_profile(&mut self) -> &mut GenerationEventProfile {
        self
    }
}
/// Specialized 61850 ZGEN class
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct GenerationStatusZgen {
    /// UML inherited base object
    // parent_message: true
    // required_field: false
    // multiplicity_min: 0
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="1")]
    pub generation_event_and_status_zgen: ::std::option::Option<GenerationEventAndStatusZgen>,
}
mod generation_status_zgen {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref GENERATION_EVENT_AND_STATUS_ZGEN: crate::generationmodule::GenerationEventAndStatusZgen = Default::default();
    }
}
pub trait IsGenerationStatusZgen {
    fn _generation_status_zgen(&self) -> &GenerationStatusZgen;
    fn _mut_generation_status_zgen(&mut self) -> &mut GenerationStatusZgen;
    fn generation_event_and_status_zgen(&self) -> &GenerationEventAndStatusZgen {
        self._generation_status_zgen().generation_event_and_status_zgen.as_ref().unwrap_or(&generation_status_zgen::GENERATION_EVENT_AND_STATUS_ZGEN)
    }
    fn mut_generation_event_and_status_zgen(&mut self) -> &mut GenerationEventAndStatusZgen {
        self._mut_generation_status_zgen().generation_event_and_status_zgen.get_or_insert(generation_status_zgen::GENERATION_EVENT_AND_STATUS_ZGEN.clone())
    }
}
impl IsGenerationStatusZgen for GenerationStatusZgen {
    fn _generation_status_zgen(&self) -> &GenerationStatusZgen {
        self
    }
    fn _mut_generation_status_zgen(&mut self) -> &mut GenerationStatusZgen {
        self
    }
}
/// Generation status
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct GenerationStatus {
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
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="2")]
    pub generation_status_zgen: ::std::option::Option<GenerationStatusZgen>,
}
mod generation_status {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref STATUS_VALUE: crate::commonmodule::StatusValue = Default::default();
        pub(super) static ref GENERATION_STATUS_ZGEN: crate::generationmodule::GenerationStatusZgen = Default::default();
    }
}
pub trait IsGenerationStatus {
    fn _generation_status(&self) -> &GenerationStatus;
    fn _mut_generation_status(&mut self) -> &mut GenerationStatus;
    fn status_value(&self) -> &super::commonmodule::StatusValue {
        self._generation_status().status_value.as_ref().unwrap_or(&generation_status::STATUS_VALUE)
    }
    fn mut_status_value(&mut self) -> &mut super::commonmodule::StatusValue {
        self._mut_generation_status().status_value.get_or_insert(generation_status::STATUS_VALUE.clone())
    }
    fn generation_status_zgen(&self) -> &GenerationStatusZgen {
        self._generation_status().generation_status_zgen.as_ref().unwrap_or(&generation_status::GENERATION_STATUS_ZGEN)
    }
    fn mut_generation_status_zgen(&mut self) -> &mut GenerationStatusZgen {
        self._mut_generation_status().generation_status_zgen.get_or_insert(generation_status::GENERATION_STATUS_ZGEN.clone())
    }
}
impl IsGenerationStatus for GenerationStatus {
    fn _generation_status(&self) -> &GenerationStatus {
        self
    }
    fn _mut_generation_status(&mut self) -> &mut GenerationStatus {
        self
    }
}
/// Generation status profile
/// OpenFMB Profile Message: true
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct GenerationStatusProfile {
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
    pub generating_unit: ::std::option::Option<GeneratingUnit>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="3")]
    pub generation_status: ::std::option::Option<GenerationStatus>,
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
mod generation_status_profile {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref STATUS_MESSAGE_INFO: crate::commonmodule::StatusMessageInfo = Default::default();
        pub(super) static ref GENERATING_UNIT: crate::generationmodule::GeneratingUnit = Default::default();
        pub(super) static ref GENERATION_STATUS: crate::generationmodule::GenerationStatus = Default::default();
        pub(super) static ref IED: crate::commonmodule::Ied = Default::default();
    }
}
pub trait IsGenerationStatusProfile {
    fn _generation_status_profile(&self) -> &GenerationStatusProfile;
    fn _mut_generation_status_profile(&mut self) -> &mut GenerationStatusProfile;
    fn status_message_info(&self) -> &super::commonmodule::StatusMessageInfo {
        self._generation_status_profile().status_message_info.as_ref().unwrap_or(&generation_status_profile::STATUS_MESSAGE_INFO)
    }
    fn mut_status_message_info(&mut self) -> &mut super::commonmodule::StatusMessageInfo {
        self._mut_generation_status_profile().status_message_info.get_or_insert(generation_status_profile::STATUS_MESSAGE_INFO.clone())
    }
    fn generating_unit(&self) -> &GeneratingUnit {
        self._generation_status_profile().generating_unit.as_ref().unwrap_or(&generation_status_profile::GENERATING_UNIT)
    }
    fn mut_generating_unit(&mut self) -> &mut GeneratingUnit {
        self._mut_generation_status_profile().generating_unit.get_or_insert(generation_status_profile::GENERATING_UNIT.clone())
    }
    fn generation_status(&self) -> &GenerationStatus {
        self._generation_status_profile().generation_status.as_ref().unwrap_or(&generation_status_profile::GENERATION_STATUS)
    }
    fn mut_generation_status(&mut self) -> &mut GenerationStatus {
        self._mut_generation_status_profile().generation_status.get_or_insert(generation_status_profile::GENERATION_STATUS.clone())
    }
    fn ied(&self) -> &super::commonmodule::Ied {
        self._generation_status_profile().ied.as_ref().unwrap_or(&generation_status_profile::IED)
    }
    fn mut_ied(&mut self) -> &mut super::commonmodule::Ied {
        self._mut_generation_status_profile().ied.get_or_insert(generation_status_profile::IED.clone())
    }
}
impl IsGenerationStatusProfile for GenerationStatusProfile {
    fn _generation_status_profile(&self) -> &GenerationStatusProfile {
        self
    }
    fn _mut_generation_status_profile(&mut self) -> &mut GenerationStatusProfile {
        self
    }
}
