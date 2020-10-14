/// This is a root class to provide common identification for all classes needing identification and
/// naming attributes.
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct IdentifiedObject {
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="1")]
    pub description: ::std::option::Option<::std::string::String>,
    /// Master resource identifier issued by a model authority. The mRID must semantically be a UUID as
    /// specified in RFC 4122. The mRID is globally unique.
    // parent_message: false
    // required_field: false
    // multiplicity_min: 0
    // multiplicity_max: 0
    // uuid: true
    // key: false
    #[prost(message, optional, tag="2")]
    pub m_rid: ::std::option::Option<::std::string::String>,
    /// The name is any free human readable and possibly non unique text naming the object.
    #[prost(message, optional, tag="3")]
    pub name: ::std::option::Option<::std::string::String>,
}
mod identified_object {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref DESCRIPTION: ::std::string::String = Default::default();
        pub(super) static ref M_RID: ::std::string::String = Default::default();
        pub(super) static ref NAME: ::std::string::String = Default::default();
    }
}
pub trait IsIdentifiedObject {
    fn _identified_object(&self) -> &IdentifiedObject;
    fn description(&self) -> &::std::string::String {
        self._identified_object().description.as_ref().unwrap_or(&identified_object::DESCRIPTION)
    }
    fn m_rid(&self) -> &::std::string::String {
        self._identified_object().m_rid.as_ref().unwrap_or(&identified_object::M_RID)
    }
    fn name(&self) -> &::std::string::String {
        self._identified_object().name.as_ref().unwrap_or(&identified_object::NAME)
    }
}
impl IsIdentifiedObject for IdentifiedObject {
    fn _identified_object(&self) -> &IdentifiedObject {
        self
    }
}
/// An electrical connection point (AC or DC) to a piece of conducting equipment. Terminals are
/// connected at physical connection points called connectivity nodes.
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct AcdcTerminal {
    /// UML inherited base object
    // parent_message: true
    // required_field: false
    // multiplicity_min: 0
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="1")]
    pub identified_object: ::std::option::Option<IdentifiedObject>,
    /// The connected status is related to a bus-branch model and the topological node to terminal
    /// relation.  True implies the terminal is connected to the related topological node and false implies
    /// it is not.  In a bus-branch model, the connected status is used to tell if equipment is disconnected
    /// without having to change the connectivity described by the topological node to terminal relation. A
    /// valid case is that conducting equipment can be connected in one end and open in the other. In
    /// particular for an AC line segment, where the reactive line charging can be significant, this is a
    /// relevant case.
    #[prost(message, optional, tag="2")]
    pub connected: ::std::option::Option<bool>,
    /// The orientation of the terminal connections for a multiple terminal conducting equipment.  The
    /// sequence numbering starts with 1 and additional terminals should follow in increasing order.   The
    /// first terminal is the "starting point" for a two terminal branch.
    #[prost(message, optional, tag="3")]
    pub sequence_number: ::std::option::Option<i32>,
}
mod acdc_terminal {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref IDENTIFIED_OBJECT: crate::commonmodule::IdentifiedObject = Default::default();
        pub(super) static ref CONNECTED: bool = Default::default();
        pub(super) static ref SEQUENCE_NUMBER: i32 = Default::default();
    }
}
pub trait IsAcdcTerminal {
    fn _acdc_terminal(&self) -> &AcdcTerminal;
    fn identified_object(&self) -> &IdentifiedObject {
        self._acdc_terminal().identified_object.as_ref().unwrap_or(&acdc_terminal::IDENTIFIED_OBJECT)
    }
    fn connected(&self) -> &bool {
        self._acdc_terminal().connected.as_ref().unwrap_or(&acdc_terminal::CONNECTED)
    }
    fn sequence_number(&self) -> &i32 {
        self._acdc_terminal().sequence_number.as_ref().unwrap_or(&acdc_terminal::SEQUENCE_NUMBER)
    }
}
impl IsAcdcTerminal for AcdcTerminal {
    fn _acdc_terminal(&self) -> &AcdcTerminal {
        self
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct OptionalUnitSymbolKind {
    #[prost(enumeration="UnitSymbolKind", tag="1")]
    pub value: i32,
}
mod optional_unit_symbol_kind {
    use lazy_static::lazy_static;
    lazy_static! {
    }
}
pub trait IsOptionalUnitSymbolKind {
    fn _optional_unit_symbol_kind(&self) -> &OptionalUnitSymbolKind;
    fn value(&self) -> &i32 {
        &self._optional_unit_symbol_kind().value    }
}
impl IsOptionalUnitSymbolKind for OptionalUnitSymbolKind {
    fn _optional_unit_symbol_kind(&self) -> &OptionalUnitSymbolKind {
        self
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct OptionalUnitMultiplierKind {
    #[prost(enumeration="UnitMultiplierKind", tag="1")]
    pub value: i32,
}
mod optional_unit_multiplier_kind {
    use lazy_static::lazy_static;
    lazy_static! {
    }
}
pub trait IsOptionalUnitMultiplierKind {
    fn _optional_unit_multiplier_kind(&self) -> &OptionalUnitMultiplierKind;
    fn value(&self) -> &i32 {
        &self._optional_unit_multiplier_kind().value    }
}
impl IsOptionalUnitMultiplierKind for OptionalUnitMultiplierKind {
    fn _optional_unit_multiplier_kind(&self) -> &OptionalUnitMultiplierKind {
        self
    }
}
/// MISSING DOCUMENTATION!!!
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ActivePower {
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="1")]
    pub multiplier: ::std::option::Option<OptionalUnitMultiplierKind>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub unit: ::std::option::Option<OptionalUnitSymbolKind>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub value: ::std::option::Option<f32>,
}
mod active_power {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref MULTIPLIER: crate::commonmodule::OptionalUnitMultiplierKind = Default::default();
        pub(super) static ref UNIT: crate::commonmodule::OptionalUnitSymbolKind = Default::default();
        pub(super) static ref VALUE: f32 = Default::default();
    }
}
pub trait IsActivePower {
    fn _active_power(&self) -> &ActivePower;
    fn multiplier(&self) -> &OptionalUnitMultiplierKind {
        self._active_power().multiplier.as_ref().unwrap_or(&active_power::MULTIPLIER)
    }
    fn unit(&self) -> &OptionalUnitSymbolKind {
        self._active_power().unit.as_ref().unwrap_or(&active_power::UNIT)
    }
    fn value(&self) -> &f32 {
        self._active_power().value.as_ref().unwrap_or(&active_power::VALUE)
    }
}
impl IsActivePower for ActivePower {
    fn _active_power(&self) -> &ActivePower {
        self
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct OptionalPhaseCodeKind {
    #[prost(enumeration="PhaseCodeKind", tag="1")]
    pub value: i32,
}
mod optional_phase_code_kind {
    use lazy_static::lazy_static;
    lazy_static! {
    }
}
pub trait IsOptionalPhaseCodeKind {
    fn _optional_phase_code_kind(&self) -> &OptionalPhaseCodeKind;
    fn value(&self) -> &i32 {
        &self._optional_phase_code_kind().value    }
}
impl IsOptionalPhaseCodeKind for OptionalPhaseCodeKind {
    fn _optional_phase_code_kind(&self) -> &OptionalPhaseCodeKind {
        self
    }
}
/// Analogue value (AnalogueValue)
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct AnalogueValue {
    /// Floating point representation of the measured value. The formula to convert between 'f' and the
    /// process value (pVal) shall be: <i>pVal</i>='f'*10exp('Unit.multiplier') in ['Unit.SIUnit'].
    #[prost(message, optional, tag="1")]
    pub f: ::std::option::Option<f32>,
    /// Integer representation of the measured value. The formula to convert between 'i' and the process
    /// value (pVal) shall be: <i>pVal</i>=(('i'*'ScaledValueConfig.scaleFactor')+'ScaledValueConfig.offset'
    ///  in ['Unit.SIUnit'].
    #[prost(message, optional, tag="2")]
    pub i: ::std::option::Option<i32>,
}
mod analogue_value {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref F: f32 = Default::default();
        pub(super) static ref I: i32 = Default::default();
    }
}
pub trait IsAnalogueValue {
    fn _analogue_value(&self) -> &AnalogueValue;
    fn f(&self) -> &f32 {
        self._analogue_value().f.as_ref().unwrap_or(&analogue_value::F)
    }
    fn i(&self) -> &i32 {
        self._analogue_value().i.as_ref().unwrap_or(&analogue_value::I)
    }
}
impl IsAnalogueValue for AnalogueValue {
    fn _analogue_value(&self) -> &AnalogueValue {
        self
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct OptionalValidityKind {
    #[prost(enumeration="ValidityKind", tag="1")]
    pub value: i32,
}
mod optional_validity_kind {
    use lazy_static::lazy_static;
    lazy_static! {
    }
}
pub trait IsOptionalValidityKind {
    fn _optional_validity_kind(&self) -> &OptionalValidityKind;
    fn value(&self) -> &i32 {
        &self._optional_validity_kind().value    }
}
impl IsOptionalValidityKind for OptionalValidityKind {
    fn _optional_validity_kind(&self) -> &OptionalValidityKind {
        self
    }
}
/// Describes some reasons in case 'validity' is not 'good'.
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct DetailQual {
    /// (default=false) If true, the value may not be a correct value due to a reference being out of
    /// calibration. The server shall decide if validity shall be set to invalid or questionable (used for
    /// measurand information and binary counter information only).
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(bool, tag="1")]
    pub bad_reference: bool,
    /// (default=false) If true, a supervision function has detected an internal or external failure.
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(bool, tag="2")]
    pub failure: bool,
    /// (default=false) If true, the value does not meet the stated accuracy of the source. EXAMPLE The
    /// measured value of power factor may be noisy (inaccurate) when the current is very small.
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(bool, tag="3")]
    pub inaccurate: bool,
    /// (default=false) If true, an evaluation function has detected an inconsistency.
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(bool, tag="4")]
    pub inconsistent: bool,
    /// (default=false) If true, an update is not made during a specific time interval. The value may be
    /// an old value that may have changed in the meantime. This specific time interval may be defined by an
    /// allowed-age attribute. NOTE "Fail silent" errors, where the equipment stops sending data, will cause
    /// setting this flag to true. In this case, the last received information was correct.
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(bool, tag="5")]
    pub old_data: bool,
    /// (default=false) To prevent overloading of event driven communication channels, it is desirable
    /// to detect and suppress oscillating (fast changing) binary inputs. If a signal changes in a defined
    /// time (tosc) twice in the same direction (from 0 to 1 or from 1 to 0), then it shall be defined as an
    /// oscillation and this attribute shall be set to true. If a configured number of transient changes is
    /// detected, they shall be suppressed. In this time, the 'Quality.validity' shall be set to
    /// 'questionable'. If the signal is still in the oscillating state after the defined number of changes,
    /// the value shall be left in the state it was in when this flag was set. In this case, the 'Quality
    /// validity' shall be changed from 'questionable' to 'invalid' and kept so as long as the signal is
    /// oscillating.  If the configuration is such that all transient changes should be suppressed, the
    /// 'Quality.validity' shall be set immediately to 'invalid' and this flag to true (used for status
    /// information only).
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(bool, tag="6")]
    pub oscillatory: bool,
    /// (default=false) If true, the attribute to which the quality has been associated is beyond a
    /// predefined range of values. The server shall decide if validity shall be set to invalid or
    /// questionable (used for measurand information only). EXAMPLE A measured value may exceed a predefined
    /// range, however the selected data type can still represent the value, for example the data type is a
    /// 16-bit unsigned integer, the predefined range is 0 to 40 000, if the value is between 40 001 and 65
    /// 535 it is considered to be out of range.
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(bool, tag="7")]
    pub out_of_range: bool,
    /// (default=false) If true, the value of the attribute to which the quality has been associated is
    /// beyond the capability of being represented properly (used for measurand information only). EXAMPLE A
    /// measured value may exceed the range that may be represented by the selected data type, for example
    /// the data type is a 16-bit unsigned integer and the value exceeds 65 535.
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(bool, tag="8")]
    pub overflow: bool,
}
mod detail_qual {
    use lazy_static::lazy_static;
    lazy_static! {
    }
}
pub trait IsDetailQual {
    fn _detail_qual(&self) -> &DetailQual;
    fn bad_reference(&self) -> &bool {
        &self._detail_qual().bad_reference    }
    fn failure(&self) -> &bool {
        &self._detail_qual().failure    }
    fn inaccurate(&self) -> &bool {
        &self._detail_qual().inaccurate    }
    fn inconsistent(&self) -> &bool {
        &self._detail_qual().inconsistent    }
    fn old_data(&self) -> &bool {
        &self._detail_qual().old_data    }
    fn oscillatory(&self) -> &bool {
        &self._detail_qual().oscillatory    }
    fn out_of_range(&self) -> &bool {
        &self._detail_qual().out_of_range    }
    fn overflow(&self) -> &bool {
        &self._detail_qual().overflow    }
}
impl IsDetailQual for DetailQual {
    fn _detail_qual(&self) -> &DetailQual {
        self
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct OptionalSourceKind {
    #[prost(enumeration="SourceKind", tag="1")]
    pub value: i32,
}
mod optional_source_kind {
    use lazy_static::lazy_static;
    lazy_static! {
    }
}
pub trait IsOptionalSourceKind {
    fn _optional_source_kind(&self) -> &OptionalSourceKind;
    fn value(&self) -> &i32 {
        &self._optional_source_kind().value    }
}
impl IsOptionalSourceKind for OptionalSourceKind {
    fn _optional_source_kind(&self) -> &OptionalSourceKind {
        self
    }
}
/// Quality contains data that describe the quality of the data from the server. Quality of the data
/// is also related to the mode of a logical node. Further details can be found in IEC 61850-7-4. The
/// different quality attributes are <i>not</i> independent.The default value shall be applied if the
/// functionality of the related attribute is not supported. The mapping may specify to exclude the
/// attribute from the message if it is not supported or if the default value applies.
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Quality {
    /// Describes some reasons in case 'validity' is not 'good'.
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="1")]
    pub detail_qual: ::std::option::Option<DetailQual>,
    /// (default=false) If true, further update of the value has been blocked by an operator. The value
    /// shall be the information that was acquired before blocking. If this flag is set, then the
    /// 'detailQual.oldData' shall also be set. The operator shall use the data attribute 'CDC.blkEna' to
    /// block the update of the value. NOTE Both an operator as well as an automatic function may freeze
    /// communication updating as well as input updating. In both cases, 'detailQual.oldData' will be set.
    /// If the blocking is done by an operator, then this flag is set additionally, and an operator activity
    /// is required to clear the condition.
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(bool, tag="2")]
    pub operator_blocked: bool,
    /// (default=process) Defines the source of a value. NOTE 1 Substitution may be done locally or via
    /// the communication services. In the second case, specific attributes with a FC=SV are used. NOTE 2
    /// There are various means to clear a substitution. As an example, a substitution that was done
    /// following an invalid condition may be cleared automatically if the invalid condition is cleared.
    /// However, this is a local issue and therefore not within the scope of this standard.
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(enumeration="SourceKind", tag="3")]
    pub source: i32,
    /// (default=false) If true, the value is a test value. The processing of the test quality in the
    /// client shall be as described in IEC 61850-7-4. This bit shall be completely independent from the
    /// other bits within the quality descriptor.
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(bool, tag="4")]
    pub test: bool,
    /// Validity of the value, as condensed information for the client. In case this value is not
    /// 'good', some reasons may be found in the 'detailQual'.
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(enumeration="ValidityKind", tag="5")]
    pub validity: i32,
}
mod quality {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref DETAIL_QUAL: crate::commonmodule::DetailQual = Default::default();
    }
}
pub trait IsQuality {
    fn _quality(&self) -> &Quality;
    fn detail_qual(&self) -> &DetailQual {
        self._quality().detail_qual.as_ref().unwrap_or(&quality::DETAIL_QUAL)
    }
    fn operator_blocked(&self) -> &bool {
        &self._quality().operator_blocked    }
    fn source(&self) -> &i32 {
        &self._quality().source    }
    fn test(&self) -> &bool {
        &self._quality().test    }
    fn validity(&self) -> &i32 {
        &self._quality().validity    }
}
impl IsQuality for Quality {
    fn _quality(&self) -> &Quality {
        self
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct OptionalTimeAccuracyKind {
    #[prost(enumeration="TimeAccuracyKind", tag="1")]
    pub value: i32,
}
mod optional_time_accuracy_kind {
    use lazy_static::lazy_static;
    lazy_static! {
    }
}
pub trait IsOptionalTimeAccuracyKind {
    fn _optional_time_accuracy_kind(&self) -> &OptionalTimeAccuracyKind;
    fn value(&self) -> &i32 {
        &self._optional_time_accuracy_kind().value    }
}
impl IsOptionalTimeAccuracyKind for OptionalTimeAccuracyKind {
    fn _optional_time_accuracy_kind(&self) -> &OptionalTimeAccuracyKind {
        self
    }
}
/// Information about the quality of the time source of the sending IED.
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct TimeQuality {
    /// If true, the time source of the sending device is unreliable and the value of the time stamp
    /// shall be ignored.
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(bool, tag="1")]
    pub clock_failure: bool,
    /// If true, the time source of the sending device is not synchronised with the external UTC time.
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(bool, tag="2")]
    pub clock_not_synchronized: bool,
    /// If true, the value in 'P_Timestamp.SecondSinceEpoch' contains all leap seconds occurred.
    /// Otherwise, it does not take into account the leap seconds that occurred before the initialization of
    /// the time source of the device. Instead, the seconds since start of the epoch are calculated from the
    /// current date assuming a constant day length of 86 400 seconds. Note: If a UTC time master clock is
    /// used and accessible, this value should always be true.
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(bool, tag="3")]
    pub leap_seconds_known: bool,
    /// Information about the quality of the time source of the sending IED.
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(enumeration="TimeAccuracyKind", tag="4")]
    pub time_accuracy: i32,
}
mod time_quality {
    use lazy_static::lazy_static;
    lazy_static! {
    }
}
pub trait IsTimeQuality {
    fn _time_quality(&self) -> &TimeQuality;
    fn clock_failure(&self) -> &bool {
        &self._time_quality().clock_failure    }
    fn clock_not_synchronized(&self) -> &bool {
        &self._time_quality().clock_not_synchronized    }
    fn leap_seconds_known(&self) -> &bool {
        &self._time_quality().leap_seconds_known    }
    fn time_accuracy(&self) -> &i32 {
        &self._time_quality().time_accuracy    }
}
impl IsTimeQuality for TimeQuality {
    fn _time_quality(&self) -> &TimeQuality {
        self
    }
}
/// UTC time with the epoch of midnight (00:00:00) of 1970-01-01. The presentation is defined in the
/// SCSMs.The NULL time stamp has all fields set to 0 (zero).The relation between a timestamp value, the
/// synchronization of an internal time with an external time source (for example, UTC time), and other
/// information related to time model are available as requirements in Clause 21.
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Timestamp {
    /// Fractional of the current second when the value of time stamp has been determined. It shall be
    /// calculated as (SUM from i=0 to 31 of bi*2**-(i+1)).
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(uint32, tag="1")]
    pub fraction: u32,
    /// Second since epoch (1970-01-01T00:00:00Z)
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(uint64, tag="2")]
    pub seconds: u64,
    /// IEC61850 time quality
    #[prost(message, optional, tag="3")]
    pub tq: ::std::option::Option<TimeQuality>,
}
mod timestamp {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref TQ: crate::commonmodule::TimeQuality = Default::default();
    }
}
pub trait IsTimestamp {
    fn _timestamp(&self) -> &Timestamp;
    fn fraction(&self) -> &u32 {
        &self._timestamp().fraction    }
    fn seconds(&self) -> &u64 {
        &self._timestamp().seconds    }
    fn tq(&self) -> &TimeQuality {
        self._timestamp().tq.as_ref().unwrap_or(&timestamp::TQ)
    }
}
impl IsTimestamp for Timestamp {
    fn _timestamp(&self) -> &Timestamp {
        self
    }
}
/// Unit definition (Unit)
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Unit {
    /// (default='') Unit multiplier.
    #[prost(message, optional, tag="1")]
    pub multiplier: ::std::option::Option<OptionalUnitMultiplierKind>,
    /// SI unit of measure.
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(enumeration="UnitSymbolKind", tag="2")]
    pub si_unit: i32,
}
mod unit {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref MULTIPLIER: crate::commonmodule::OptionalUnitMultiplierKind = Default::default();
    }
}
pub trait IsUnit {
    fn _unit(&self) -> &Unit;
    fn multiplier(&self) -> &OptionalUnitMultiplierKind {
        self._unit().multiplier.as_ref().unwrap_or(&unit::MULTIPLIER)
    }
    fn si_unit(&self) -> &i32 {
        &self._unit().si_unit    }
}
impl IsUnit for Unit {
    fn _unit(&self) -> &Unit {
        self
    }
}
/// Measured value (MV)
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Mv {
    /// Value of the magnitude based on a deadband calculation from the instantaneous value 'instMag'.
    /// The value of 'mag' shall be updated to the current instantaneous value 'instMag' when the value has
    /// changed according to the configuration parameter 'db'. If 'db'=0, 'mag'='instMag'.NOTE 1 This value
    /// is typically used to create reports for analogue values. Such a report sent "by exception" is not
    /// comparable to the transfer of sampled measured values as supported by the CDC SAV.NOTE 2 This 'mag'
    /// is not the same as 'mag' of the constructed attribute class 'Vector'.
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="1")]
    pub mag: ::std::option::Option<AnalogueValue>,
    /// Quality of the values in 'instMag', 'mag', 'range'.
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="2")]
    pub q: ::std::option::Option<Quality>,
    /// Timestamp of the last refresh of the value in 'mag' or of the last change of the value in any of
    /// 'range' or 'q'.
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="3")]
    pub t: ::std::option::Option<Timestamp>,
    /// Unit for: 'instMag', 'mag', 'subMag', 'rangeC'.
    #[prost(message, optional, tag="4")]
    pub units: ::std::option::Option<Unit>,
}
mod mv {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref MAG: crate::commonmodule::AnalogueValue = Default::default();
        pub(super) static ref Q: crate::commonmodule::Quality = Default::default();
        pub(super) static ref T: crate::commonmodule::Timestamp = Default::default();
        pub(super) static ref UNITS: crate::commonmodule::Unit = Default::default();
    }
}
pub trait IsMv {
    fn _mv(&self) -> &Mv;
    fn mag(&self) -> &AnalogueValue {
        self._mv().mag.as_ref().unwrap_or(&mv::MAG)
    }
    fn q(&self) -> &Quality {
        self._mv().q.as_ref().unwrap_or(&mv::Q)
    }
    fn t(&self) -> &Timestamp {
        self._mv().t.as_ref().unwrap_or(&mv::T)
    }
    fn units(&self) -> &Unit {
        self._mv().units.as_ref().unwrap_or(&mv::UNITS)
    }
}
impl IsMv for Mv {
    fn _mv(&self) -> &Mv {
        self
    }
}
/// IEC61850 logical node.
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct LogicalNode {
    /// UML inherited base object
    // parent_message: true
    // required_field: false
    // multiplicity_min: 0
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="1")]
    pub identified_object: ::std::option::Option<IdentifiedObject>,
}
mod logical_node {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref IDENTIFIED_OBJECT: crate::commonmodule::IdentifiedObject = Default::default();
    }
}
pub trait IsLogicalNode {
    fn _logical_node(&self) -> &LogicalNode;
    fn identified_object(&self) -> &IdentifiedObject {
        self._logical_node().identified_object.as_ref().unwrap_or(&logical_node::IDENTIFIED_OBJECT)
    }
}
impl IsLogicalNode for LogicalNode {
    fn _logical_node(&self) -> &LogicalNode {
        self
    }
}
/// LN: Generic process I/O   Name: GGIO
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct AnalogStatusGgio {
    /// UML inherited base object
    // parent_message: true
    // required_field: false
    // multiplicity_min: 0
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="1")]
    pub logical_node: ::std::option::Option<LogicalNode>,
    /// Generic analogue input <i>n</i>.
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="2")]
    pub an_in: ::std::option::Option<Mv>,
    /// Phase code
    #[prost(message, optional, tag="3")]
    pub phase: ::std::option::Option<OptionalPhaseCodeKind>,
}
mod analog_status_ggio {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref LOGICAL_NODE: crate::commonmodule::LogicalNode = Default::default();
        pub(super) static ref AN_IN: crate::commonmodule::Mv = Default::default();
        pub(super) static ref PHASE: crate::commonmodule::OptionalPhaseCodeKind = Default::default();
    }
}
pub trait IsAnalogStatusGgio {
    fn _analog_status_ggio(&self) -> &AnalogStatusGgio;
    fn logical_node(&self) -> &LogicalNode {
        self._analog_status_ggio().logical_node.as_ref().unwrap_or(&analog_status_ggio::LOGICAL_NODE)
    }
    fn an_in(&self) -> &Mv {
        self._analog_status_ggio().an_in.as_ref().unwrap_or(&analog_status_ggio::AN_IN)
    }
    fn phase(&self) -> &OptionalPhaseCodeKind {
        self._analog_status_ggio().phase.as_ref().unwrap_or(&analog_status_ggio::PHASE)
    }
}
impl IsAnalogStatusGgio for AnalogStatusGgio {
    fn _analog_status_ggio(&self) -> &AnalogStatusGgio {
        self
    }
}
/// Analogue value control (AnalogueValueCtl)
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct AnalogueValueCtl {
    /// Floating point representation of the measured value.  See 'AnalogueValue.f'.
    #[prost(message, optional, tag="1")]
    pub f: ::std::option::Option<f32>,
    /// Integer representation of the measured value. See 'AnalogueValue.i'.
    #[prost(message, optional, tag="2")]
    pub i: ::std::option::Option<i32>,
}
mod analogue_value_ctl {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref F: f32 = Default::default();
        pub(super) static ref I: i32 = Default::default();
    }
}
pub trait IsAnalogueValueCtl {
    fn _analogue_value_ctl(&self) -> &AnalogueValueCtl;
    fn f(&self) -> &f32 {
        self._analogue_value_ctl().f.as_ref().unwrap_or(&analogue_value_ctl::F)
    }
    fn i(&self) -> &i32 {
        self._analogue_value_ctl().i.as_ref().unwrap_or(&analogue_value_ctl::I)
    }
}
impl IsAnalogueValueCtl for AnalogueValueCtl {
    fn _analogue_value_ctl(&self) -> &AnalogueValueCtl {
        self
    }
}
/// This is a root class similar to IdentifiedObject but without the mRID. The reason to separate
/// the two classes is because the mRID may need to be defined as a separate key field for technology
/// such as the DDS implementation.
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct NamedObject {
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="1")]
    pub description: ::std::option::Option<::std::string::String>,
    /// The name is any free human readable and possibly non unique text naming the object.
    #[prost(message, optional, tag="2")]
    pub name: ::std::option::Option<::std::string::String>,
}
mod named_object {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref DESCRIPTION: ::std::string::String = Default::default();
        pub(super) static ref NAME: ::std::string::String = Default::default();
    }
}
pub trait IsNamedObject {
    fn _named_object(&self) -> &NamedObject;
    fn description(&self) -> &::std::string::String {
        self._named_object().description.as_ref().unwrap_or(&named_object::DESCRIPTION)
    }
    fn name(&self) -> &::std::string::String {
        self._named_object().name.as_ref().unwrap_or(&named_object::NAME)
    }
}
impl IsNamedObject for NamedObject {
    fn _named_object(&self) -> &NamedObject {
        self
    }
}
/// The parts of a power system that are physical devices, electronic or mechanical.
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ApplicationSystem {
    /// UML inherited base object
    // parent_message: true
    // required_field: false
    // multiplicity_min: 0
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="1")]
    pub named_object: ::std::option::Option<NamedObject>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: true
    // key: false
    #[prost(string, tag="2")]
    pub m_rid: std::string::String,
}
mod application_system {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref NAMED_OBJECT: crate::commonmodule::NamedObject = Default::default();
    }
}
pub trait IsApplicationSystem {
    fn _application_system(&self) -> &ApplicationSystem;
    fn named_object(&self) -> &NamedObject {
        self._application_system().named_object.as_ref().unwrap_or(&application_system::NAMED_OBJECT)
    }
    fn m_rid(&self) -> &std::string::String {
        &self._application_system().m_rid    }
}
impl IsApplicationSystem for ApplicationSystem {
    fn _application_system(&self) -> &ApplicationSystem {
        self
    }
}
/// Analogue setting (FC=SP) (ASG_SP)
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Asg {
    /// The value of the analogue setting.
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="1")]
    pub set_mag: ::std::option::Option<AnalogueValueCtl>,
    /// Unit for 'setMag', 'minVal', 'maxVal', 'stepSize'.
    #[prost(message, optional, tag="2")]
    pub units: ::std::option::Option<Unit>,
}
mod asg {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref SET_MAG: crate::commonmodule::AnalogueValueCtl = Default::default();
        pub(super) static ref UNITS: crate::commonmodule::Unit = Default::default();
    }
}
pub trait IsAsg {
    fn _asg(&self) -> &Asg;
    fn set_mag(&self) -> &AnalogueValueCtl {
        self._asg().set_mag.as_ref().unwrap_or(&asg::SET_MAG)
    }
    fn units(&self) -> &Unit {
        self._asg().units.as_ref().unwrap_or(&asg::UNITS)
    }
}
impl IsAsg for Asg {
    fn _asg(&self) -> &Asg {
        self
    }
}
/// Binary counter reading (BCR)
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Bcr {
    /// Binary counter status represented as an integer value; wraps to 0 at the maximum or minimum
    /// value of INT64.
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(int64, tag="1")]
    pub act_val: i64,
    /// Quality of the values in 'actVal', 'frVal'.
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="2")]
    pub q: ::std::option::Option<Quality>,
    /// Timestamp of the last change of value in 'actVal' or 'q'.
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="3")]
    pub t: ::std::option::Option<Timestamp>,
    /// Unit for value in 'pulsQty'.
    #[prost(message, optional, tag="4")]
    pub units: ::std::option::Option<OptionalUnitSymbolKind>,
}
mod bcr {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref Q: crate::commonmodule::Quality = Default::default();
        pub(super) static ref T: crate::commonmodule::Timestamp = Default::default();
        pub(super) static ref UNITS: crate::commonmodule::OptionalUnitSymbolKind = Default::default();
    }
}
pub trait IsBcr {
    fn _bcr(&self) -> &Bcr;
    fn act_val(&self) -> &i64 {
        &self._bcr().act_val    }
    fn q(&self) -> &Quality {
        self._bcr().q.as_ref().unwrap_or(&bcr::Q)
    }
    fn t(&self) -> &Timestamp {
        self._bcr().t.as_ref().unwrap_or(&bcr::T)
    }
    fn units(&self) -> &OptionalUnitSymbolKind {
        self._bcr().units.as_ref().unwrap_or(&bcr::UNITS)
    }
}
impl IsBcr for Bcr {
    fn _bcr(&self) -> &Bcr {
        self
    }
}
/// Specialized 61850 SPS class
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct StatusSps {
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="1")]
    pub q: ::std::option::Option<Quality>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(bool, tag="2")]
    pub st_val: bool,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub t: ::std::option::Option<Timestamp>,
}
mod status_sps {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref Q: crate::commonmodule::Quality = Default::default();
        pub(super) static ref T: crate::commonmodule::Timestamp = Default::default();
    }
}
pub trait IsStatusSps {
    fn _status_sps(&self) -> &StatusSps;
    fn q(&self) -> &Quality {
        self._status_sps().q.as_ref().unwrap_or(&status_sps::Q)
    }
    fn st_val(&self) -> &bool {
        &self._status_sps().st_val    }
    fn t(&self) -> &Timestamp {
        self._status_sps().t.as_ref().unwrap_or(&status_sps::T)
    }
}
impl IsStatusSps for StatusSps {
    fn _status_sps(&self) -> &StatusSps {
        self
    }
}
/// LN: Generic process I/O   Name: GGIO
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct BooleanStatusGgio {
    /// UML inherited base object
    // parent_message: true
    // required_field: false
    // multiplicity_min: 0
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="1")]
    pub logical_node: ::std::option::Option<LogicalNode>,
    /// If true, indication <i>n</i> is present.
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="2")]
    pub ind: ::std::option::Option<StatusSps>,
    /// Phase code
    #[prost(message, optional, tag="3")]
    pub phase: ::std::option::Option<OptionalPhaseCodeKind>,
}
mod boolean_status_ggio {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref LOGICAL_NODE: crate::commonmodule::LogicalNode = Default::default();
        pub(super) static ref IND: crate::commonmodule::StatusSps = Default::default();
        pub(super) static ref PHASE: crate::commonmodule::OptionalPhaseCodeKind = Default::default();
    }
}
pub trait IsBooleanStatusGgio {
    fn _boolean_status_ggio(&self) -> &BooleanStatusGgio;
    fn logical_node(&self) -> &LogicalNode {
        self._boolean_status_ggio().logical_node.as_ref().unwrap_or(&boolean_status_ggio::LOGICAL_NODE)
    }
    fn ind(&self) -> &StatusSps {
        self._boolean_status_ggio().ind.as_ref().unwrap_or(&boolean_status_ggio::IND)
    }
    fn phase(&self) -> &OptionalPhaseCodeKind {
        self._boolean_status_ggio().phase.as_ref().unwrap_or(&boolean_status_ggio::PHASE)
    }
}
impl IsBooleanStatusGgio for BooleanStatusGgio {
    fn _boolean_status_ggio(&self) -> &BooleanStatusGgio {
        self
    }
}
/// IEC61850-7-2 Service parameter for conditions checking
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct CheckConditions {
    /// InterlockCheck is used for the device to be controlled to check if other devices are in proper
    /// state for the control command.  One example is that 2 circuit breakers on a busbar need to be
    /// interlocked so one is open and one is closed, but not both on.
    #[prost(message, optional, tag="1")]
    pub interlock_check: ::std::option::Option<bool>,
    /// To check if a device is synchrous to the system.
    #[prost(message, optional, tag="2")]
    pub synchro_check: ::std::option::Option<bool>,
}
mod check_conditions {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref INTERLOCK_CHECK: bool = Default::default();
        pub(super) static ref SYNCHRO_CHECK: bool = Default::default();
    }
}
pub trait IsCheckConditions {
    fn _check_conditions(&self) -> &CheckConditions;
    fn interlock_check(&self) -> &bool {
        self._check_conditions().interlock_check.as_ref().unwrap_or(&check_conditions::INTERLOCK_CHECK)
    }
    fn synchro_check(&self) -> &bool {
        self._check_conditions().synchro_check.as_ref().unwrap_or(&check_conditions::SYNCHRO_CHECK)
    }
}
impl IsCheckConditions for CheckConditions {
    fn _check_conditions(&self) -> &CheckConditions {
        self
    }
}
/// Vector definition (Vector)
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Vector {
    /// (range=[-180...180]) Angle of the complex value (Unit.SIUnit='deg' and Unit.multiplier='');
    /// angle reference is defined in the context where this type is used.
    #[prost(message, optional, tag="1")]
    pub ang: ::std::option::Option<AnalogueValue>,
    /// Magnitude of the complex value.
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="2")]
    pub mag: ::std::option::Option<AnalogueValue>,
}
mod vector {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref ANG: crate::commonmodule::AnalogueValue = Default::default();
        pub(super) static ref MAG: crate::commonmodule::AnalogueValue = Default::default();
    }
}
pub trait IsVector {
    fn _vector(&self) -> &Vector;
    fn ang(&self) -> &AnalogueValue {
        self._vector().ang.as_ref().unwrap_or(&vector::ANG)
    }
    fn mag(&self) -> &AnalogueValue {
        self._vector().mag.as_ref().unwrap_or(&vector::MAG)
    }
}
impl IsVector for Vector {
    fn _vector(&self) -> &Vector {
        self
    }
}
/// Complex measured value (CMV)
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Cmv {
    /// Complex value based on a deadband calculation from the instantaneous value 'instCVal.mag'. The
    /// deadband calculation is done both on 'instCVal.mag' (based on 'db') and on 'instCVal.ang' (based on
    /// 'dbAng'), independently. See  'MV.mag'.
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="1")]
    pub c_val: ::std::option::Option<Vector>,
    /// Quality of the values in 'instCVal', 'cVal', 'range', ‘rangeAng’.
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="2")]
    pub q: ::std::option::Option<Quality>,
    /// Timestamp of the last refresh of the value in 'cVal' or of the last change of the value in any
    /// of 'range', 'rangeAng' or 'q'.
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="3")]
    pub t: ::std::option::Option<Timestamp>,
    /// Units for: 'instCVal.mag', 'cVal.mag', 'subCVal.mag', 'rangeC'.
    #[prost(message, optional, tag="4")]
    pub units: ::std::option::Option<Unit>,
}
mod cmv {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref C_VAL: crate::commonmodule::Vector = Default::default();
        pub(super) static ref Q: crate::commonmodule::Quality = Default::default();
        pub(super) static ref T: crate::commonmodule::Timestamp = Default::default();
        pub(super) static ref UNITS: crate::commonmodule::Unit = Default::default();
    }
}
pub trait IsCmv {
    fn _cmv(&self) -> &Cmv;
    fn c_val(&self) -> &Vector {
        self._cmv().c_val.as_ref().unwrap_or(&cmv::C_VAL)
    }
    fn q(&self) -> &Quality {
        self._cmv().q.as_ref().unwrap_or(&cmv::Q)
    }
    fn t(&self) -> &Timestamp {
        self._cmv().t.as_ref().unwrap_or(&cmv::T)
    }
    fn units(&self) -> &Unit {
        self._cmv().units.as_ref().unwrap_or(&cmv::UNITS)
    }
}
impl IsCmv for Cmv {
    fn _cmv(&self) -> &Cmv {
        self
    }
}
/// Asset representation of a ConductingEquipment.
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ConductingEquipment {
    /// UML inherited base object
    // parent_message: true
    // required_field: false
    // multiplicity_min: 0
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="1")]
    pub named_object: ::std::option::Option<NamedObject>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: true
    // key: true
    #[prost(string, tag="2")]
    pub m_rid: std::string::String,
}
mod conducting_equipment {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref NAMED_OBJECT: crate::commonmodule::NamedObject = Default::default();
    }
}
pub trait IsConductingEquipment {
    fn _conducting_equipment(&self) -> &ConductingEquipment;
    fn named_object(&self) -> &NamedObject {
        self._conducting_equipment().named_object.as_ref().unwrap_or(&conducting_equipment::NAMED_OBJECT)
    }
    fn m_rid(&self) -> &std::string::String {
        &self._conducting_equipment().m_rid    }
}
impl IsConductingEquipment for ConductingEquipment {
    fn _conducting_equipment(&self) -> &ConductingEquipment {
        self
    }
}
/// An AC electrical connection point to a piece of conducting equipment. Terminals are connected at
/// physical connection points called connectivity nodes.
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Terminal {
    /// UML inherited base object
    // parent_message: true
    // required_field: false
    // multiplicity_min: 0
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="1")]
    pub a_cdc_terminal: ::std::option::Option<AcdcTerminal>,
    /// Represents the normal network phasing condition. If the attribute is missing three phases (ABC
    /// or ABCN) shall be assumed.
    #[prost(message, optional, tag="2")]
    pub phases: ::std::option::Option<OptionalPhaseCodeKind>,
}
mod terminal {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref A_CDC_TERMINAL: crate::commonmodule::AcdcTerminal = Default::default();
        pub(super) static ref PHASES: crate::commonmodule::OptionalPhaseCodeKind = Default::default();
    }
}
pub trait IsTerminal {
    fn _terminal(&self) -> &Terminal;
    fn a_cdc_terminal(&self) -> &AcdcTerminal {
        self._terminal().a_cdc_terminal.as_ref().unwrap_or(&terminal::A_CDC_TERMINAL)
    }
    fn phases(&self) -> &OptionalPhaseCodeKind {
        self._terminal().phases.as_ref().unwrap_or(&terminal::PHASES)
    }
}
impl IsTerminal for Terminal {
    fn _terminal(&self) -> &Terminal {
        self
    }
}
/// Reading associated with an equipment such as a recloser.
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ConductingEquipmentTerminalReading {
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="1")]
    pub terminal: ::std::option::Option<Terminal>,
}
mod conducting_equipment_terminal_reading {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref TERMINAL: crate::commonmodule::Terminal = Default::default();
    }
}
pub trait IsConductingEquipmentTerminalReading {
    fn _conducting_equipment_terminal_reading(&self) -> &ConductingEquipmentTerminalReading;
    fn terminal(&self) -> &Terminal {
        self._conducting_equipment_terminal_reading().terminal.as_ref().unwrap_or(&conducting_equipment_terminal_reading::TERMINAL)
    }
}
impl IsConductingEquipmentTerminalReading for ConductingEquipmentTerminalReading {
    fn _conducting_equipment_terminal_reading(&self) -> &ConductingEquipmentTerminalReading {
        self
    }
}
/// Specialized DPC 61850 CDC class
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ControlDpc {
    /// Service parameter that determines the control activity ('false' for off, 'true' for on).
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(bool, tag="1")]
    pub ctl_val: bool,
}
mod control_dpc {
    use lazy_static::lazy_static;
    lazy_static! {
    }
}
pub trait IsControlDpc {
    fn _control_dpc(&self) -> &ControlDpc;
    fn ctl_val(&self) -> &bool {
        &self._control_dpc().ctl_val    }
}
impl IsControlDpc for ControlDpc {
    fn _control_dpc(&self) -> &ControlDpc {
        self
    }
}
/// UTC time with the epoch of midnight (00:00:00) of 1970-01-01. The presentation is defined in the
/// SCSMs.The NULL time stamp has all fields set to 0 (zero).The relation between a timestamp value, the
/// synchronization of an internal time with an external time source (for example, UTC time), and other
/// information related to time model are available as requirements in Clause 21.  ControlTimestamp is a
/// timestamp for future time point so it does not contain the time quality as the one contained in the
/// normal Timestamp data type.
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ControlTimestamp {
    /// Fractional of the current second when the value of time stamp has been determined. It shall be
    /// calculated as (SUM from i=0 to 31 of bi*2**-(i+1)).
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(uint32, tag="1")]
    pub fraction: u32,
    /// Second since epoch (1970-01-01T00:00:00Z)
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(uint64, tag="2")]
    pub seconds: u64,
}
mod control_timestamp {
    use lazy_static::lazy_static;
    lazy_static! {
    }
}
pub trait IsControlTimestamp {
    fn _control_timestamp(&self) -> &ControlTimestamp;
    fn fraction(&self) -> &u32 {
        &self._control_timestamp().fraction    }
    fn seconds(&self) -> &u64 {
        &self._control_timestamp().seconds    }
}
impl IsControlTimestamp for ControlTimestamp {
    fn _control_timestamp(&self) -> &ControlTimestamp {
        self
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct OptionalScheduleParameterKind {
    #[prost(enumeration="ScheduleParameterKind", tag="1")]
    pub value: i32,
}
mod optional_schedule_parameter_kind {
    use lazy_static::lazy_static;
    lazy_static! {
    }
}
pub trait IsOptionalScheduleParameterKind {
    fn _optional_schedule_parameter_kind(&self) -> &OptionalScheduleParameterKind;
    fn value(&self) -> &i32 {
        &self._optional_schedule_parameter_kind().value    }
}
impl IsOptionalScheduleParameterKind for OptionalScheduleParameterKind {
    fn _optional_schedule_parameter_kind(&self) -> &OptionalScheduleParameterKind {
        self
    }
}
/// Grid connect mode kind
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct EngScheduleParameter {
    /// Schedule parameter type
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(enumeration="ScheduleParameterKind", tag="1")]
    pub schedule_parameter_type: i32,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(float, tag="2")]
    pub value: f32,
}
mod eng_schedule_parameter {
    use lazy_static::lazy_static;
    lazy_static! {
    }
}
pub trait IsEngScheduleParameter {
    fn _eng_schedule_parameter(&self) -> &EngScheduleParameter;
    fn schedule_parameter_type(&self) -> &i32 {
        &self._eng_schedule_parameter().schedule_parameter_type    }
    fn value(&self) -> &f32 {
        &self._eng_schedule_parameter().value    }
}
impl IsEngScheduleParameter for EngScheduleParameter {
    fn _eng_schedule_parameter(&self) -> &EngScheduleParameter {
        self
    }
}
/// Point definition (Point)
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SchedulePoint {
    /// Schedule parameter
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, repeated, tag="1")]
    pub schedule_parameter: ::std::vec::Vec<EngScheduleParameter>,
    /// Start time
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="2")]
    pub start_time: ::std::option::Option<ControlTimestamp>,
}
mod schedule_point {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref START_TIME: crate::commonmodule::ControlTimestamp = Default::default();
    }
}
pub trait IsSchedulePoint {
    fn _schedule_point(&self) -> &SchedulePoint;
    fn schedule_parameter(&self) -> &::std::vec::Vec<EngScheduleParameter> {
        &self._schedule_point().schedule_parameter    }
    fn start_time(&self) -> &ControlTimestamp {
        self._schedule_point().start_time.as_ref().unwrap_or(&schedule_point::START_TIME)
    }
}
impl IsSchedulePoint for SchedulePoint {
    fn _schedule_point(&self) -> &SchedulePoint {
        self
    }
}
/// Curve shape setting (FC=SP) (CSG_SP)
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ScheduleCsg {
    /// The array with the points specifying a time schedule
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, repeated, tag="1")]
    pub sch_pts: ::std::vec::Vec<SchedulePoint>,
}
mod schedule_csg {
    use lazy_static::lazy_static;
    lazy_static! {
    }
}
pub trait IsScheduleCsg {
    fn _schedule_csg(&self) -> &ScheduleCsg;
    fn sch_pts(&self) -> &::std::vec::Vec<SchedulePoint> {
        &self._schedule_csg().sch_pts    }
}
impl IsScheduleCsg for ScheduleCsg {
    fn _schedule_csg(&self) -> &ScheduleCsg {
        self
    }
}
/// OpenFMB specialization for control schedule using:  LN: Schedule   Name: FSCH
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ControlScheduleFsch {
    /// Analog CSG
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="1")]
    pub val_acsg: ::std::option::Option<ScheduleCsg>,
}
mod control_schedule_fsch {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref VAL_ACSG: crate::commonmodule::ScheduleCsg = Default::default();
    }
}
pub trait IsControlScheduleFsch {
    fn _control_schedule_fsch(&self) -> &ControlScheduleFsch;
    fn val_acsg(&self) -> &ScheduleCsg {
        self._control_schedule_fsch().val_acsg.as_ref().unwrap_or(&control_schedule_fsch::VAL_ACSG)
    }
}
impl IsControlScheduleFsch for ControlScheduleFsch {
    fn _control_schedule_fsch(&self) -> &ControlScheduleFsch {
        self
    }
}
/// OpenFMB specialization for logical node control
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct LogicalNodeForControl {
    /// UML inherited base object
    // parent_message: true
    // required_field: false
    // multiplicity_min: 0
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="1")]
    pub logical_node: ::std::option::Option<LogicalNode>,
}
mod logical_node_for_control {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref LOGICAL_NODE: crate::commonmodule::LogicalNode = Default::default();
    }
}
pub trait IsLogicalNodeForControl {
    fn _logical_node_for_control(&self) -> &LogicalNodeForControl;
    fn logical_node(&self) -> &LogicalNode {
        self._logical_node_for_control().logical_node.as_ref().unwrap_or(&logical_node_for_control::LOGICAL_NODE)
    }
}
impl IsLogicalNodeForControl for LogicalNodeForControl {
    fn _logical_node_for_control(&self) -> &LogicalNodeForControl {
        self
    }
}
/// LN: Schedule controller   Name: FSCC  F:    Function (generic) SC:  Schedule Controller C:
/// Control (execution)
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ControlFscc {
    /// UML inherited base object
    // parent_message: true
    // required_field: false
    // multiplicity_min: 0
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="1")]
    pub logical_node_for_control: ::std::option::Option<LogicalNodeForControl>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub control_schedule_fsch: ::std::option::Option<ControlScheduleFsch>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub island_control_schedule_fsch: ::std::option::Option<ControlScheduleFsch>,
}
mod control_fscc {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref LOGICAL_NODE_FOR_CONTROL: crate::commonmodule::LogicalNodeForControl = Default::default();
        pub(super) static ref CONTROL_SCHEDULE_FSCH: crate::commonmodule::ControlScheduleFsch = Default::default();
        pub(super) static ref ISLAND_CONTROL_SCHEDULE_FSCH: crate::commonmodule::ControlScheduleFsch = Default::default();
    }
}
pub trait IsControlFscc {
    fn _control_fscc(&self) -> &ControlFscc;
    fn logical_node_for_control(&self) -> &LogicalNodeForControl {
        self._control_fscc().logical_node_for_control.as_ref().unwrap_or(&control_fscc::LOGICAL_NODE_FOR_CONTROL)
    }
    fn control_schedule_fsch(&self) -> &ControlScheduleFsch {
        self._control_fscc().control_schedule_fsch.as_ref().unwrap_or(&control_fscc::CONTROL_SCHEDULE_FSCH)
    }
    fn island_control_schedule_fsch(&self) -> &ControlScheduleFsch {
        self._control_fscc().island_control_schedule_fsch.as_ref().unwrap_or(&control_fscc::ISLAND_CONTROL_SCHEDULE_FSCH)
    }
}
impl IsControlFscc for ControlFscc {
    fn _control_fscc(&self) -> &ControlFscc {
        self
    }
}
/// Integer status setting (FC=SP) (ING_SP)
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ControlIng {
    /// The value of the status setting.
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(int32, tag="1")]
    pub set_val: i32,
    /// Unit for 'setVal', 'minVal', 'maxVal', 'stepSize'.
    #[prost(message, optional, tag="2")]
    pub units: ::std::option::Option<Unit>,
}
mod control_ing {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref UNITS: crate::commonmodule::Unit = Default::default();
    }
}
pub trait IsControlIng {
    fn _control_ing(&self) -> &ControlIng;
    fn set_val(&self) -> &i32 {
        &self._control_ing().set_val    }
    fn units(&self) -> &Unit {
        self._control_ing().units.as_ref().unwrap_or(&control_ing::UNITS)
    }
}
impl IsControlIng for ControlIng {
    fn _control_ing(&self) -> &ControlIng {
        self
    }
}
/// &lt;&lt;statistics&gt;&gt; Integer controlled step position information (ISC)
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ControlIsc {
    /// Service parameter that determines the control activity.
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(int32, tag="1")]
    pub ctl_val: i32,
}
mod control_isc {
    use lazy_static::lazy_static;
    lazy_static! {
    }
}
pub trait IsControlIsc {
    fn _control_isc(&self) -> &ControlIsc;
    fn ctl_val(&self) -> &i32 {
        &self._control_isc().ctl_val    }
}
impl IsControlIsc for ControlIsc {
    fn _control_isc(&self) -> &ControlIsc {
        self
    }
}
/// Generic control message info.
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct MessageInfo {
    /// UML inherited base object
    // parent_message: true
    // required_field: false
    // multiplicity_min: 0
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="1")]
    pub identified_object: ::std::option::Option<IdentifiedObject>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="2")]
    pub message_time_stamp: ::std::option::Option<Timestamp>,
}
mod message_info {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref IDENTIFIED_OBJECT: crate::commonmodule::IdentifiedObject = Default::default();
        pub(super) static ref MESSAGE_TIME_STAMP: crate::commonmodule::Timestamp = Default::default();
    }
}
pub trait IsMessageInfo {
    fn _message_info(&self) -> &MessageInfo;
    fn identified_object(&self) -> &IdentifiedObject {
        self._message_info().identified_object.as_ref().unwrap_or(&message_info::IDENTIFIED_OBJECT)
    }
    fn message_time_stamp(&self) -> &Timestamp {
        self._message_info().message_time_stamp.as_ref().unwrap_or(&message_info::MESSAGE_TIME_STAMP)
    }
}
impl IsMessageInfo for MessageInfo {
    fn _message_info(&self) -> &MessageInfo {
        self
    }
}
/// Generic control message info.
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ControlMessageInfo {
    /// UML inherited base object
    // parent_message: true
    // required_field: false
    // multiplicity_min: 0
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="1")]
    pub message_info: ::std::option::Option<MessageInfo>,
}
mod control_message_info {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref MESSAGE_INFO: crate::commonmodule::MessageInfo = Default::default();
    }
}
pub trait IsControlMessageInfo {
    fn _control_message_info(&self) -> &ControlMessageInfo;
    fn message_info(&self) -> &MessageInfo {
        self._control_message_info().message_info.as_ref().unwrap_or(&control_message_info::MESSAGE_INFO)
    }
}
impl IsControlMessageInfo for ControlMessageInfo {
    fn _control_message_info(&self) -> &ControlMessageInfo {
        self
    }
}
/// Controllable single point (SPC)
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ControlSpc {
    /// Service parameter that determines the control activity ('false' for off or deactivation, 'true'
    /// for on or activation).
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(bool, tag="1")]
    pub ctl_val: bool,
}
mod control_spc {
    use lazy_static::lazy_static;
    lazy_static! {
    }
}
pub trait IsControlSpc {
    fn _control_spc(&self) -> &ControlSpc;
    fn ctl_val(&self) -> &bool {
        &self._control_spc().ctl_val    }
}
impl IsControlSpc for ControlSpc {
    fn _control_spc(&self) -> &ControlSpc {
        self
    }
}
/// The value of a control command which could either be a setpoint or a control schedule in curve.
/// The attribute modBlk is used to tag out a device. if it is TRUE, any setpoints and control schedule
/// in a message payload should be ignored.   It should also be presented in a status profile.  Any
/// modBlk value change (i.e. TRUE to FALSE and vice versa) should trigger an event.
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ControlValue {
    /// UML inherited base object
    // parent_message: true
    // required_field: false
    // multiplicity_min: 0
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="1")]
    pub identified_object: ::std::option::Option<IdentifiedObject>,
    /// The attribute modBlk is used to tag out a device. if it is TRUE, any setpoints and control
    /// schedule in a message payload should be ignored.   It should also be presented in a status profile.
    /// Any modBlk value change (i.e. TRUE to FALSE and vice versa) should trigger an event.
    #[prost(message, optional, tag="2")]
    pub mod_blk: ::std::option::Option<bool>,
}
mod control_value {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref IDENTIFIED_OBJECT: crate::commonmodule::IdentifiedObject = Default::default();
        pub(super) static ref MOD_BLK: bool = Default::default();
    }
}
pub trait IsControlValue {
    fn _control_value(&self) -> &ControlValue;
    fn identified_object(&self) -> &IdentifiedObject {
        self._control_value().identified_object.as_ref().unwrap_or(&control_value::IDENTIFIED_OBJECT)
    }
    fn mod_blk(&self) -> &bool {
        self._control_value().mod_blk.as_ref().unwrap_or(&control_value::MOD_BLK)
    }
}
impl IsControlValue for ControlValue {
    fn _control_value(&self) -> &ControlValue {
        self
    }
}
/// Interval between two date and time points.
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct DateTimeInterval {
    /// End date and time of this interval.
    #[prost(message, optional, tag="1")]
    pub end: ::std::option::Option<i64>,
    /// Start date and time of this interval.
    #[prost(message, optional, tag="2")]
    pub start: ::std::option::Option<i64>,
}
mod date_time_interval {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref END: i64 = Default::default();
        pub(super) static ref START: i64 = Default::default();
    }
}
pub trait IsDateTimeInterval {
    fn _date_time_interval(&self) -> &DateTimeInterval;
    fn end(&self) -> &i64 {
        self._date_time_interval().end.as_ref().unwrap_or(&date_time_interval::END)
    }
    fn start(&self) -> &i64 {
        self._date_time_interval().start.as_ref().unwrap_or(&date_time_interval::START)
    }
}
impl IsDateTimeInterval for DateTimeInterval {
    fn _date_time_interval(&self) -> &DateTimeInterval {
        self
    }
}
/// Phase to phase related measured values of a three-phase system (DEL)
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Del {
    /// Value of phase A to phase B measurement.
    #[prost(message, optional, tag="1")]
    pub phs_ab: ::std::option::Option<Cmv>,
    /// Value of phase B to phase C measurement.
    #[prost(message, optional, tag="2")]
    pub phs_bc: ::std::option::Option<Cmv>,
    /// Value of phase C to phase A measurement.
    #[prost(message, optional, tag="3")]
    pub phs_ca: ::std::option::Option<Cmv>,
}
mod del {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref PHS_AB: crate::commonmodule::Cmv = Default::default();
        pub(super) static ref PHS_BC: crate::commonmodule::Cmv = Default::default();
        pub(super) static ref PHS_CA: crate::commonmodule::Cmv = Default::default();
    }
}
pub trait IsDel {
    fn _del(&self) -> &Del;
    fn phs_ab(&self) -> &Cmv {
        self._del().phs_ab.as_ref().unwrap_or(&del::PHS_AB)
    }
    fn phs_bc(&self) -> &Cmv {
        self._del().phs_bc.as_ref().unwrap_or(&del::PHS_BC)
    }
    fn phs_ca(&self) -> &Cmv {
        self._del().phs_ca.as_ref().unwrap_or(&del::PHS_CA)
    }
}
impl IsDel for Del {
    fn _del(&self) -> &Del {
        self
    }
}
/// Generic user of energy - a  point of consumption on the power system model.
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct EnergyConsumer {
    /// UML inherited base object
    // parent_message: true
    // required_field: false
    // multiplicity_min: 0
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="1")]
    pub conducting_equipment: ::std::option::Option<ConductingEquipment>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub operating_limit: ::std::option::Option<::std::string::String>,
}
mod energy_consumer {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref CONDUCTING_EQUIPMENT: crate::commonmodule::ConductingEquipment = Default::default();
        pub(super) static ref OPERATING_LIMIT: ::std::string::String = Default::default();
    }
}
pub trait IsEnergyConsumer {
    fn _energy_consumer(&self) -> &EnergyConsumer;
    fn conducting_equipment(&self) -> &ConductingEquipment {
        self._energy_consumer().conducting_equipment.as_ref().unwrap_or(&energy_consumer::CONDUCTING_EQUIPMENT)
    }
    fn operating_limit(&self) -> &::std::string::String {
        self._energy_consumer().operating_limit.as_ref().unwrap_or(&energy_consumer::OPERATING_LIMIT)
    }
}
impl IsEnergyConsumer for EnergyConsumer {
    fn _energy_consumer(&self) -> &EnergyConsumer {
        self
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct OptionalCalcMethodKind {
    #[prost(enumeration="CalcMethodKind", tag="1")]
    pub value: i32,
}
mod optional_calc_method_kind {
    use lazy_static::lazy_static;
    lazy_static! {
    }
}
pub trait IsOptionalCalcMethodKind {
    fn _optional_calc_method_kind(&self) -> &OptionalCalcMethodKind;
    fn value(&self) -> &i32 {
        &self._optional_calc_method_kind().value    }
}
impl IsOptionalCalcMethodKind for OptionalCalcMethodKind {
    fn _optional_calc_method_kind(&self) -> &OptionalCalcMethodKind {
        self
    }
}
/// Calc method kind
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct EngCalcMethodKind {
    /// The value of the status setting.
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(enumeration="CalcMethodKind", tag="1")]
    pub set_val: i32,
}
mod eng_calc_method_kind {
    use lazy_static::lazy_static;
    lazy_static! {
    }
}
pub trait IsEngCalcMethodKind {
    fn _eng_calc_method_kind(&self) -> &EngCalcMethodKind;
    fn set_val(&self) -> &i32 {
        &self._eng_calc_method_kind().set_val    }
}
impl IsEngCalcMethodKind for EngCalcMethodKind {
    fn _eng_calc_method_kind(&self) -> &EngCalcMethodKind {
        self
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct OptionalGridConnectModeKind {
    #[prost(enumeration="GridConnectModeKind", tag="1")]
    pub value: i32,
}
mod optional_grid_connect_mode_kind {
    use lazy_static::lazy_static;
    lazy_static! {
    }
}
pub trait IsOptionalGridConnectModeKind {
    fn _optional_grid_connect_mode_kind(&self) -> &OptionalGridConnectModeKind;
    fn value(&self) -> &i32 {
        &self._optional_grid_connect_mode_kind().value    }
}
impl IsOptionalGridConnectModeKind for OptionalGridConnectModeKind {
    fn _optional_grid_connect_mode_kind(&self) -> &OptionalGridConnectModeKind {
        self
    }
}
/// Grid connect mode kind
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct EngGridConnectModeKind {
    /// The value of the status setting.
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(enumeration="GridConnectModeKind", tag="1")]
    pub set_val: i32,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub set_val_extension: ::std::option::Option<::std::string::String>,
}
mod eng_grid_connect_mode_kind {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref SET_VAL_EXTENSION: ::std::string::String = Default::default();
    }
}
pub trait IsEngGridConnectModeKind {
    fn _eng_grid_connect_mode_kind(&self) -> &EngGridConnectModeKind;
    fn set_val(&self) -> &i32 {
        &self._eng_grid_connect_mode_kind().set_val    }
    fn set_val_extension(&self) -> &::std::string::String {
        self._eng_grid_connect_mode_kind().set_val_extension.as_ref().unwrap_or(&eng_grid_connect_mode_kind::SET_VAL_EXTENSION)
    }
}
impl IsEngGridConnectModeKind for EngGridConnectModeKind {
    fn _eng_grid_connect_mode_kind(&self) -> &EngGridConnectModeKind {
        self
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct OptionalPfSignKind {
    #[prost(enumeration="PfSignKind", tag="1")]
    pub value: i32,
}
mod optional_pf_sign_kind {
    use lazy_static::lazy_static;
    lazy_static! {
    }
}
pub trait IsOptionalPfSignKind {
    fn _optional_pf_sign_kind(&self) -> &OptionalPfSignKind;
    fn value(&self) -> &i32 {
        &self._optional_pf_sign_kind().value    }
}
impl IsOptionalPfSignKind for OptionalPfSignKind {
    fn _optional_pf_sign_kind(&self) -> &OptionalPfSignKind {
        self
    }
}
/// Enumerated status setting (FC=SP) (ENG_SP)
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct EngPfSignKind {
    /// The value of the status setting.
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(enumeration="PfSignKind", tag="1")]
    pub set_val: i32,
}
mod eng_pf_sign_kind {
    use lazy_static::lazy_static;
    lazy_static! {
    }
}
pub trait IsEngPfSignKind {
    fn _eng_pf_sign_kind(&self) -> &EngPfSignKind;
    fn set_val(&self) -> &i32 {
        &self._eng_pf_sign_kind().set_val    }
}
impl IsEngPfSignKind for EngPfSignKind {
    fn _eng_pf_sign_kind(&self) -> &EngPfSignKind {
        self
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct OptionalBehaviourModeKind {
    #[prost(enumeration="BehaviourModeKind", tag="1")]
    pub value: i32,
}
mod optional_behaviour_mode_kind {
    use lazy_static::lazy_static;
    lazy_static! {
    }
}
pub trait IsOptionalBehaviourModeKind {
    fn _optional_behaviour_mode_kind(&self) -> &OptionalBehaviourModeKind;
    fn value(&self) -> &i32 {
        &self._optional_behaviour_mode_kind().value    }
}
impl IsOptionalBehaviourModeKind for OptionalBehaviourModeKind {
    fn _optional_behaviour_mode_kind(&self) -> &OptionalBehaviourModeKind {
        self
    }
}
/// Behavior mode kind. ENS stands for Enumerated status
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct EnsBehaviourModeKind {
    /// Quality of the value in 'stVal'.
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="1")]
    pub q: ::std::option::Option<Quality>,
    /// Value of the data.
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(enumeration="BehaviourModeKind", tag="2")]
    pub st_val: i32,
    /// Timestamp of the last change or update event of 'stVal' or the last change of value in 'q'.
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="3")]
    pub t: ::std::option::Option<Timestamp>,
}
mod ens_behaviour_mode_kind {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref Q: crate::commonmodule::Quality = Default::default();
        pub(super) static ref T: crate::commonmodule::Timestamp = Default::default();
    }
}
pub trait IsEnsBehaviourModeKind {
    fn _ens_behaviour_mode_kind(&self) -> &EnsBehaviourModeKind;
    fn q(&self) -> &Quality {
        self._ens_behaviour_mode_kind().q.as_ref().unwrap_or(&ens_behaviour_mode_kind::Q)
    }
    fn st_val(&self) -> &i32 {
        &self._ens_behaviour_mode_kind().st_val    }
    fn t(&self) -> &Timestamp {
        self._ens_behaviour_mode_kind().t.as_ref().unwrap_or(&ens_behaviour_mode_kind::T)
    }
}
impl IsEnsBehaviourModeKind for EnsBehaviourModeKind {
    fn _ens_behaviour_mode_kind(&self) -> &EnsBehaviourModeKind {
        self
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct OptionalDerGeneratorStateKind {
    #[prost(enumeration="DerGeneratorStateKind", tag="1")]
    pub value: i32,
}
mod optional_der_generator_state_kind {
    use lazy_static::lazy_static;
    lazy_static! {
    }
}
pub trait IsOptionalDerGeneratorStateKind {
    fn _optional_der_generator_state_kind(&self) -> &OptionalDerGeneratorStateKind;
    fn value(&self) -> &i32 {
        &self._optional_der_generator_state_kind().value    }
}
impl IsOptionalDerGeneratorStateKind for OptionalDerGeneratorStateKind {
    fn _optional_der_generator_state_kind(&self) -> &OptionalDerGeneratorStateKind {
        self
    }
}
/// DER generation state kind. ENS stands for Enumerated status
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct EnsDerGeneratorStateKind {
    /// Quality of the value in 'stVal'.
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="1")]
    pub q: ::std::option::Option<Quality>,
    /// Value of the data.
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(enumeration="DerGeneratorStateKind", tag="2")]
    pub st_val: i32,
    /// Timestamp of the last change or update event of 'stVal' or the last change of value in 'q'.
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="3")]
    pub t: ::std::option::Option<Timestamp>,
}
mod ens_der_generator_state_kind {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref Q: crate::commonmodule::Quality = Default::default();
        pub(super) static ref T: crate::commonmodule::Timestamp = Default::default();
    }
}
pub trait IsEnsDerGeneratorStateKind {
    fn _ens_der_generator_state_kind(&self) -> &EnsDerGeneratorStateKind;
    fn q(&self) -> &Quality {
        self._ens_der_generator_state_kind().q.as_ref().unwrap_or(&ens_der_generator_state_kind::Q)
    }
    fn st_val(&self) -> &i32 {
        &self._ens_der_generator_state_kind().st_val    }
    fn t(&self) -> &Timestamp {
        self._ens_der_generator_state_kind().t.as_ref().unwrap_or(&ens_der_generator_state_kind::T)
    }
}
impl IsEnsDerGeneratorStateKind for EnsDerGeneratorStateKind {
    fn _ens_der_generator_state_kind(&self) -> &EnsDerGeneratorStateKind {
        self
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct OptionalDynamicTestKind {
    #[prost(enumeration="DynamicTestKind", tag="1")]
    pub value: i32,
}
mod optional_dynamic_test_kind {
    use lazy_static::lazy_static;
    lazy_static! {
    }
}
pub trait IsOptionalDynamicTestKind {
    fn _optional_dynamic_test_kind(&self) -> &OptionalDynamicTestKind;
    fn value(&self) -> &i32 {
        &self._optional_dynamic_test_kind().value    }
}
impl IsOptionalDynamicTestKind for OptionalDynamicTestKind {
    fn _optional_dynamic_test_kind(&self) -> &OptionalDynamicTestKind {
        self
    }
}
/// Dynamic test kind. ENS stands for Enumerated status
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct EnsDynamicTestKind {
    /// Quality of the value in 'stVal'.
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="1")]
    pub q: ::std::option::Option<Quality>,
    /// Value of the data.
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(enumeration="DynamicTestKind", tag="2")]
    pub st_val: i32,
    /// Timestamp of the last change or update event of 'stVal' or the last change of value in 'q'.
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="3")]
    pub t: ::std::option::Option<Timestamp>,
}
mod ens_dynamic_test_kind {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref Q: crate::commonmodule::Quality = Default::default();
        pub(super) static ref T: crate::commonmodule::Timestamp = Default::default();
    }
}
pub trait IsEnsDynamicTestKind {
    fn _ens_dynamic_test_kind(&self) -> &EnsDynamicTestKind;
    fn q(&self) -> &Quality {
        self._ens_dynamic_test_kind().q.as_ref().unwrap_or(&ens_dynamic_test_kind::Q)
    }
    fn st_val(&self) -> &i32 {
        &self._ens_dynamic_test_kind().st_val    }
    fn t(&self) -> &Timestamp {
        self._ens_dynamic_test_kind().t.as_ref().unwrap_or(&ens_dynamic_test_kind::T)
    }
}
impl IsEnsDynamicTestKind for EnsDynamicTestKind {
    fn _ens_dynamic_test_kind(&self) -> &EnsDynamicTestKind {
        self
    }
}
/// Grid connect event &amp; status mode kind
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct EnsGridConnectModeKind {
    /// Actual Grid Connection Mode
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(enumeration="GridConnectModeKind", tag="1")]
    pub st_val: i32,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(string, tag="2")]
    pub st_val_extension: std::string::String,
}
mod ens_grid_connect_mode_kind {
    use lazy_static::lazy_static;
    lazy_static! {
    }
}
pub trait IsEnsGridConnectModeKind {
    fn _ens_grid_connect_mode_kind(&self) -> &EnsGridConnectModeKind;
    fn st_val(&self) -> &i32 {
        &self._ens_grid_connect_mode_kind().st_val    }
    fn st_val_extension(&self) -> &std::string::String {
        &self._ens_grid_connect_mode_kind().st_val_extension    }
}
impl IsEnsGridConnectModeKind for EnsGridConnectModeKind {
    fn _ens_grid_connect_mode_kind(&self) -> &EnsGridConnectModeKind {
        self
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct OptionalHealthKind {
    #[prost(enumeration="HealthKind", tag="1")]
    pub value: i32,
}
mod optional_health_kind {
    use lazy_static::lazy_static;
    lazy_static! {
    }
}
pub trait IsOptionalHealthKind {
    fn _optional_health_kind(&self) -> &OptionalHealthKind;
    fn value(&self) -> &i32 {
        &self._optional_health_kind().value    }
}
impl IsOptionalHealthKind for OptionalHealthKind {
    fn _optional_health_kind(&self) -> &OptionalHealthKind {
        self
    }
}
/// &lt;&gt; Enumerated status (ENS)
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct EnsHealthKind {
    /// Textual description of the data. In case it is used within the CDC LPL, the description refers
    /// to the logical node.
    #[prost(message, optional, tag="1")]
    pub d: ::std::option::Option<::std::string::String>,
    /// Value of the data.
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(enumeration="HealthKind", tag="2")]
    pub st_val: i32,
}
mod ens_health_kind {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref D: ::std::string::String = Default::default();
    }
}
pub trait IsEnsHealthKind {
    fn _ens_health_kind(&self) -> &EnsHealthKind;
    fn d(&self) -> &::std::string::String {
        self._ens_health_kind().d.as_ref().unwrap_or(&ens_health_kind::D)
    }
    fn st_val(&self) -> &i32 {
        &self._ens_health_kind().st_val    }
}
impl IsEnsHealthKind for EnsHealthKind {
    fn _ens_health_kind(&self) -> &EnsHealthKind {
        self
    }
}
/// MISSING DOCUMENTATION!!!
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Ess {
    /// UML inherited base object
    // parent_message: true
    // required_field: false
    // multiplicity_min: 0
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="1")]
    pub conducting_equipment: ::std::option::Option<ConductingEquipment>,
}
mod ess {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref CONDUCTING_EQUIPMENT: crate::commonmodule::ConductingEquipment = Default::default();
    }
}
pub trait IsEss {
    fn _ess(&self) -> &Ess;
    fn conducting_equipment(&self) -> &ConductingEquipment {
        self._ess().conducting_equipment.as_ref().unwrap_or(&ess::CONDUCTING_EQUIPMENT)
    }
}
impl IsEss for Ess {
    fn _ess(&self) -> &Ess {
        self
    }
}
/// Generic event message information
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct EventMessageInfo {
    /// UML inherited base object
    // parent_message: true
    // required_field: false
    // multiplicity_min: 0
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="1")]
    pub message_info: ::std::option::Option<MessageInfo>,
}
mod event_message_info {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref MESSAGE_INFO: crate::commonmodule::MessageInfo = Default::default();
    }
}
pub trait IsEventMessageInfo {
    fn _event_message_info(&self) -> &EventMessageInfo;
    fn message_info(&self) -> &MessageInfo {
        self._event_message_info().message_info.as_ref().unwrap_or(&event_message_info::MESSAGE_INFO)
    }
}
impl IsEventMessageInfo for EventMessageInfo {
    fn _event_message_info(&self) -> &EventMessageInfo {
        self
    }
}
/// Event value
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct EventValue {
    /// UML inherited base object
    // parent_message: true
    // required_field: false
    // multiplicity_min: 0
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="1")]
    pub identified_object: ::std::option::Option<IdentifiedObject>,
}
mod event_value {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref IDENTIFIED_OBJECT: crate::commonmodule::IdentifiedObject = Default::default();
    }
}
pub trait IsEventValue {
    fn _event_value(&self) -> &EventValue;
    fn identified_object(&self) -> &IdentifiedObject {
        self._event_value().identified_object.as_ref().unwrap_or(&event_value::IDENTIFIED_OBJECT)
    }
}
impl IsEventValue for EventValue {
    fn _event_value(&self) -> &EventValue {
        self
    }
}
/// The source where a forecast value is issued.
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ForecastValueSource {
    /// UML inherited base object
    // parent_message: true
    // required_field: false
    // multiplicity_min: 0
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="1")]
    pub identified_object: ::std::option::Option<IdentifiedObject>,
}
mod forecast_value_source {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref IDENTIFIED_OBJECT: crate::commonmodule::IdentifiedObject = Default::default();
    }
}
pub trait IsForecastValueSource {
    fn _forecast_value_source(&self) -> &ForecastValueSource;
    fn identified_object(&self) -> &IdentifiedObject {
        self._forecast_value_source().identified_object.as_ref().unwrap_or(&forecast_value_source::IDENTIFIED_OBJECT)
    }
}
impl IsForecastValueSource for ForecastValueSource {
    fn _forecast_value_source(&self) -> &ForecastValueSource {
        self
    }
}
/// Intelligent Electronic Device is a device with a microprocessor that can contain one or more
/// (IEC61850) SERVERs. In the context of IEC61850, IED could be an electronic protection device, a
/// controller or even a laptop/desktop computer. <b>Modelling note</b>: This class is not explicitly
/// defined in IEC61850-7-2 (but only in SCL: IEC61850-6). However, it is an important concept that
/// deserves its place in the meta-model. When the meta-model gets instantiated from a direct link to an
/// IED with an IEC61850 SERVER, we typically create an instance of the meta-model IED per connection.
/// When the meta-model gets instantiated from an SCL file, there is the full description of IED and its
/// functions.
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ForecastIed {
    /// UML inherited base object
    // parent_message: true
    // required_field: false
    // multiplicity_min: 0
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="1")]
    pub forecast_value_source: ::std::option::Option<ForecastValueSource>,
    /// For control, this is an application ID, unique within communication system, and if the message
    /// is transformed between gateway the original source application ID should be kept.
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(string, tag="2")]
    pub source_application_id: std::string::String,
    /// Message publication date time
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(int64, tag="3")]
    pub source_date_time: i64,
}
mod forecast_ied {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref FORECAST_VALUE_SOURCE: crate::commonmodule::ForecastValueSource = Default::default();
    }
}
pub trait IsForecastIed {
    fn _forecast_ied(&self) -> &ForecastIed;
    fn forecast_value_source(&self) -> &ForecastValueSource {
        self._forecast_ied().forecast_value_source.as_ref().unwrap_or(&forecast_ied::FORECAST_VALUE_SOURCE)
    }
    fn source_application_id(&self) -> &std::string::String {
        &self._forecast_ied().source_application_id    }
    fn source_date_time(&self) -> &i64 {
        &self._forecast_ied().source_date_time    }
}
impl IsForecastIed for ForecastIed {
    fn _forecast_ied(&self) -> &ForecastIed {
        self
    }
}
/// Forecast value
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ForecastValue {
    /// UML inherited base object
    // parent_message: true
    // required_field: false
    // multiplicity_min: 0
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="1")]
    pub identified_object: ::std::option::Option<IdentifiedObject>,
}
mod forecast_value {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref IDENTIFIED_OBJECT: crate::commonmodule::IdentifiedObject = Default::default();
    }
}
pub trait IsForecastValue {
    fn _forecast_value(&self) -> &ForecastValue;
    fn identified_object(&self) -> &IdentifiedObject {
        self._forecast_value().identified_object.as_ref().unwrap_or(&forecast_value::IDENTIFIED_OBJECT)
    }
}
impl IsForecastValue for ForecastValue {
    fn _forecast_value(&self) -> &ForecastValue {
        self
    }
}
/// Intelligent Electronic Device is a device with a microprocessor that can contain one or more
/// (IEC61850) SERVERs. In the context of IEC61850, IED could be an electronic protection device, a
/// controller or even a laptop/desktop computer. <b>Modelling note</b>: This class is not explicitly
/// defined in IEC61850-7-2 (but only in SCL: IEC61850-6). However, it is an important concept that
/// deserves its place in the meta-model. When the meta-model gets instantiated from a direct link to an
/// IED with an IEC61850 SERVER, we typically create an instance of the meta-model IED per connection.
/// When the meta-model gets instantiated from an SCL file, there is the full description of IED and its
/// functions.
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Ied {
    /// UML inherited base object
    // parent_message: true
    // required_field: false
    // multiplicity_min: 0
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="1")]
    pub identified_object: ::std::option::Option<IdentifiedObject>,
}
mod ied {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref IDENTIFIED_OBJECT: crate::commonmodule::IdentifiedObject = Default::default();
    }
}
pub trait IsIed {
    fn _ied(&self) -> &Ied;
    fn identified_object(&self) -> &IdentifiedObject {
        self._ied().identified_object.as_ref().unwrap_or(&ied::IDENTIFIED_OBJECT)
    }
}
impl IsIed for Ied {
    fn _ied(&self) -> &Ied {
        self
    }
}
/// <<statistics>> Integer status (INS)
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct StatusIns {
    /// Quality of the value in 'stVal'.
    #[prost(message, optional, tag="1")]
    pub q: ::std::option::Option<Quality>,
    /// Value of the data.
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(int32, tag="2")]
    pub st_val: i32,
    /// Timestamp of the last change or update event of 'stVal' or the last change of value in 'q'.
    #[prost(message, optional, tag="3")]
    pub t: ::std::option::Option<Timestamp>,
    /// Units for: 'stVal', 'subVal'.
    #[prost(message, optional, tag="4")]
    pub units: ::std::option::Option<Unit>,
}
mod status_ins {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref Q: crate::commonmodule::Quality = Default::default();
        pub(super) static ref T: crate::commonmodule::Timestamp = Default::default();
        pub(super) static ref UNITS: crate::commonmodule::Unit = Default::default();
    }
}
pub trait IsStatusIns {
    fn _status_ins(&self) -> &StatusIns;
    fn q(&self) -> &Quality {
        self._status_ins().q.as_ref().unwrap_or(&status_ins::Q)
    }
    fn st_val(&self) -> &i32 {
        &self._status_ins().st_val    }
    fn t(&self) -> &Timestamp {
        self._status_ins().t.as_ref().unwrap_or(&status_ins::T)
    }
    fn units(&self) -> &Unit {
        self._status_ins().units.as_ref().unwrap_or(&status_ins::UNITS)
    }
}
impl IsStatusIns for StatusIns {
    fn _status_ins(&self) -> &StatusIns {
        self
    }
}
/// Status expressed in integer based on IEC61850 GGIO.
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct IntegerStatusGgio {
    /// UML inherited base object
    // parent_message: true
    // required_field: false
    // multiplicity_min: 0
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="1")]
    pub logical_node: ::std::option::Option<LogicalNode>,
    /// Generic integer status input <i>n</i>.
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="2")]
    pub int_in: ::std::option::Option<StatusIns>,
    /// Phase code
    #[prost(message, optional, tag="3")]
    pub phase: ::std::option::Option<OptionalPhaseCodeKind>,
}
mod integer_status_ggio {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref LOGICAL_NODE: crate::commonmodule::LogicalNode = Default::default();
        pub(super) static ref INT_IN: crate::commonmodule::StatusIns = Default::default();
        pub(super) static ref PHASE: crate::commonmodule::OptionalPhaseCodeKind = Default::default();
    }
}
pub trait IsIntegerStatusGgio {
    fn _integer_status_ggio(&self) -> &IntegerStatusGgio;
    fn logical_node(&self) -> &LogicalNode {
        self._integer_status_ggio().logical_node.as_ref().unwrap_or(&integer_status_ggio::LOGICAL_NODE)
    }
    fn int_in(&self) -> &StatusIns {
        self._integer_status_ggio().int_in.as_ref().unwrap_or(&integer_status_ggio::INT_IN)
    }
    fn phase(&self) -> &OptionalPhaseCodeKind {
        self._integer_status_ggio().phase.as_ref().unwrap_or(&integer_status_ggio::PHASE)
    }
}
impl IsIntegerStatusGgio for IntegerStatusGgio {
    fn _integer_status_ggio(&self) -> &IntegerStatusGgio {
        self
    }
}
/// Logical node for event and status
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct LogicalNodeForEventAndStatus {
    /// UML inherited base object
    // parent_message: true
    // required_field: false
    // multiplicity_min: 0
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="1")]
    pub logical_node: ::std::option::Option<LogicalNode>,
    /// Behavior of the function
    #[prost(message, optional, tag="2")]
    pub beh: ::std::option::Option<EnsBehaviourModeKind>,
    /// Asset health
    #[prost(message, optional, tag="3")]
    pub ee_health: ::std::option::Option<EnsHealthKind>,
}
mod logical_node_for_event_and_status {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref LOGICAL_NODE: crate::commonmodule::LogicalNode = Default::default();
        pub(super) static ref BEH: crate::commonmodule::EnsBehaviourModeKind = Default::default();
        pub(super) static ref EE_HEALTH: crate::commonmodule::EnsHealthKind = Default::default();
    }
}
pub trait IsLogicalNodeForEventAndStatus {
    fn _logical_node_for_event_and_status(&self) -> &LogicalNodeForEventAndStatus;
    fn logical_node(&self) -> &LogicalNode {
        self._logical_node_for_event_and_status().logical_node.as_ref().unwrap_or(&logical_node_for_event_and_status::LOGICAL_NODE)
    }
    fn beh(&self) -> &EnsBehaviourModeKind {
        self._logical_node_for_event_and_status().beh.as_ref().unwrap_or(&logical_node_for_event_and_status::BEH)
    }
    fn ee_health(&self) -> &EnsHealthKind {
        self._logical_node_for_event_and_status().ee_health.as_ref().unwrap_or(&logical_node_for_event_and_status::EE_HEALTH)
    }
}
impl IsLogicalNodeForEventAndStatus for LogicalNodeForEventAndStatus {
    fn _logical_node_for_event_and_status(&self) -> &LogicalNodeForEventAndStatus {
        self
    }
}
/// The current state for a measurement. A state value is an instance of a measurement from a
/// specific source. Measurements can be associated with many state values, each representing a
/// different source for the measurement.
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct MeasurementValue {
    /// UML inherited base object
    // parent_message: true
    // required_field: false
    // multiplicity_min: 0
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="1")]
    pub identified_object: ::std::option::Option<IdentifiedObject>,
}
mod measurement_value {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref IDENTIFIED_OBJECT: crate::commonmodule::IdentifiedObject = Default::default();
    }
}
pub trait IsMeasurementValue {
    fn _measurement_value(&self) -> &MeasurementValue;
    fn identified_object(&self) -> &IdentifiedObject {
        self._measurement_value().identified_object.as_ref().unwrap_or(&measurement_value::IDENTIFIED_OBJECT)
    }
}
impl IsMeasurementValue for MeasurementValue {
    fn _measurement_value(&self) -> &MeasurementValue {
        self
    }
}
/// Physical asset that performs the metering role of the usage point. Used for measuring
/// consumption and detection of events.
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Meter {
    /// UML inherited base object
    // parent_message: true
    // required_field: false
    // multiplicity_min: 0
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="1")]
    pub conducting_equipment: ::std::option::Option<ConductingEquipment>,
}
mod meter {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref CONDUCTING_EQUIPMENT: crate::commonmodule::ConductingEquipment = Default::default();
    }
}
pub trait IsMeter {
    fn _meter(&self) -> &Meter;
    fn conducting_equipment(&self) -> &ConductingEquipment {
        self._meter().conducting_equipment.as_ref().unwrap_or(&meter::CONDUCTING_EQUIPMENT)
    }
}
impl IsMeter for Meter {
    fn _meter(&self) -> &Meter {
        self
    }
}
/// Generic event message information
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct OptimizationMessageInfo {
    /// UML inherited base object
    // parent_message: true
    // required_field: false
    // multiplicity_min: 0
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="1")]
    pub message_info: ::std::option::Option<MessageInfo>,
}
mod optimization_message_info {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref MESSAGE_INFO: crate::commonmodule::MessageInfo = Default::default();
    }
}
pub trait IsOptimizationMessageInfo {
    fn _optimization_message_info(&self) -> &OptimizationMessageInfo;
    fn message_info(&self) -> &MessageInfo {
        self._optimization_message_info().message_info.as_ref().unwrap_or(&optimization_message_info::MESSAGE_INFO)
    }
}
impl IsOptimizationMessageInfo for OptimizationMessageInfo {
    fn _optimization_message_info(&self) -> &OptimizationMessageInfo {
        self
    }
}
/// Specialized 61850 MMTN LN class
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ReadingMmtn {
    /// UML inherited base object
    // parent_message: true
    // required_field: false
    // multiplicity_min: 0
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="1")]
    pub logical_node: ::std::option::Option<LogicalNode>,
    /// Apparent energy demand (direction: from busbar).
    #[prost(message, optional, tag="2")]
    pub dmd_v_ah: ::std::option::Option<Bcr>,
    /// Reactive energy demand (direction: from busbar).
    #[prost(message, optional, tag="3")]
    pub dmd_v_arh: ::std::option::Option<Bcr>,
    /// Real energy demand (direction: from busbar).
    #[prost(message, optional, tag="4")]
    pub dmd_wh: ::std::option::Option<Bcr>,
    /// Apparent energy supply (default direction: towards busbar).
    #[prost(message, optional, tag="5")]
    pub sup_v_ah: ::std::option::Option<Bcr>,
    /// Reactive energy supply (default direction: towards busbar).
    #[prost(message, optional, tag="6")]
    pub sup_v_arh: ::std::option::Option<Bcr>,
    /// Real energy supply (default direction: towards busbar).
    #[prost(message, optional, tag="7")]
    pub sup_wh: ::std::option::Option<Bcr>,
    /// Net apparent energy since last reset.
    #[prost(message, optional, tag="8")]
    pub tot_v_ah: ::std::option::Option<Bcr>,
    /// Net reactive energy since last reset.
    #[prost(message, optional, tag="9")]
    pub tot_v_arh: ::std::option::Option<Bcr>,
    /// Net real energy since last reset.
    #[prost(message, optional, tag="10")]
    pub tot_wh: ::std::option::Option<Bcr>,
}
mod reading_mmtn {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref LOGICAL_NODE: crate::commonmodule::LogicalNode = Default::default();
        pub(super) static ref DMD_V_AH: crate::commonmodule::Bcr = Default::default();
        pub(super) static ref DMD_V_ARH: crate::commonmodule::Bcr = Default::default();
        pub(super) static ref DMD_WH: crate::commonmodule::Bcr = Default::default();
        pub(super) static ref SUP_V_AH: crate::commonmodule::Bcr = Default::default();
        pub(super) static ref SUP_V_ARH: crate::commonmodule::Bcr = Default::default();
        pub(super) static ref SUP_WH: crate::commonmodule::Bcr = Default::default();
        pub(super) static ref TOT_V_AH: crate::commonmodule::Bcr = Default::default();
        pub(super) static ref TOT_V_ARH: crate::commonmodule::Bcr = Default::default();
        pub(super) static ref TOT_WH: crate::commonmodule::Bcr = Default::default();
    }
}
pub trait IsReadingMmtn {
    fn _reading_mmtn(&self) -> &ReadingMmtn;
    fn logical_node(&self) -> &LogicalNode {
        self._reading_mmtn().logical_node.as_ref().unwrap_or(&reading_mmtn::LOGICAL_NODE)
    }
    fn dmd_v_ah(&self) -> &Bcr {
        self._reading_mmtn().dmd_v_ah.as_ref().unwrap_or(&reading_mmtn::DMD_V_AH)
    }
    fn dmd_v_arh(&self) -> &Bcr {
        self._reading_mmtn().dmd_v_arh.as_ref().unwrap_or(&reading_mmtn::DMD_V_ARH)
    }
    fn dmd_wh(&self) -> &Bcr {
        self._reading_mmtn().dmd_wh.as_ref().unwrap_or(&reading_mmtn::DMD_WH)
    }
    fn sup_v_ah(&self) -> &Bcr {
        self._reading_mmtn().sup_v_ah.as_ref().unwrap_or(&reading_mmtn::SUP_V_AH)
    }
    fn sup_v_arh(&self) -> &Bcr {
        self._reading_mmtn().sup_v_arh.as_ref().unwrap_or(&reading_mmtn::SUP_V_ARH)
    }
    fn sup_wh(&self) -> &Bcr {
        self._reading_mmtn().sup_wh.as_ref().unwrap_or(&reading_mmtn::SUP_WH)
    }
    fn tot_v_ah(&self) -> &Bcr {
        self._reading_mmtn().tot_v_ah.as_ref().unwrap_or(&reading_mmtn::TOT_V_AH)
    }
    fn tot_v_arh(&self) -> &Bcr {
        self._reading_mmtn().tot_v_arh.as_ref().unwrap_or(&reading_mmtn::TOT_V_ARH)
    }
    fn tot_wh(&self) -> &Bcr {
        self._reading_mmtn().tot_wh.as_ref().unwrap_or(&reading_mmtn::TOT_WH)
    }
}
impl IsReadingMmtn for ReadingMmtn {
    fn _reading_mmtn(&self) -> &ReadingMmtn {
        self
    }
}
/// Specialized 61850 MMTN LN class
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PhaseMmtn {
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="1")]
    pub phs_a: ::std::option::Option<ReadingMmtn>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub phs_ab: ::std::option::Option<ReadingMmtn>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub phs_b: ::std::option::Option<ReadingMmtn>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="4")]
    pub phs_bc: ::std::option::Option<ReadingMmtn>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="5")]
    pub phs_c: ::std::option::Option<ReadingMmtn>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="6")]
    pub phs_ca: ::std::option::Option<ReadingMmtn>,
}
mod phase_mmtn {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref PHS_A: crate::commonmodule::ReadingMmtn = Default::default();
        pub(super) static ref PHS_AB: crate::commonmodule::ReadingMmtn = Default::default();
        pub(super) static ref PHS_B: crate::commonmodule::ReadingMmtn = Default::default();
        pub(super) static ref PHS_BC: crate::commonmodule::ReadingMmtn = Default::default();
        pub(super) static ref PHS_C: crate::commonmodule::ReadingMmtn = Default::default();
        pub(super) static ref PHS_CA: crate::commonmodule::ReadingMmtn = Default::default();
    }
}
pub trait IsPhaseMmtn {
    fn _phase_mmtn(&self) -> &PhaseMmtn;
    fn phs_a(&self) -> &ReadingMmtn {
        self._phase_mmtn().phs_a.as_ref().unwrap_or(&phase_mmtn::PHS_A)
    }
    fn phs_ab(&self) -> &ReadingMmtn {
        self._phase_mmtn().phs_ab.as_ref().unwrap_or(&phase_mmtn::PHS_AB)
    }
    fn phs_b(&self) -> &ReadingMmtn {
        self._phase_mmtn().phs_b.as_ref().unwrap_or(&phase_mmtn::PHS_B)
    }
    fn phs_bc(&self) -> &ReadingMmtn {
        self._phase_mmtn().phs_bc.as_ref().unwrap_or(&phase_mmtn::PHS_BC)
    }
    fn phs_c(&self) -> &ReadingMmtn {
        self._phase_mmtn().phs_c.as_ref().unwrap_or(&phase_mmtn::PHS_C)
    }
    fn phs_ca(&self) -> &ReadingMmtn {
        self._phase_mmtn().phs_ca.as_ref().unwrap_or(&phase_mmtn::PHS_CA)
    }
}
impl IsPhaseMmtn for PhaseMmtn {
    fn _phase_mmtn(&self) -> &PhaseMmtn {
        self
    }
}
/// Grid connect mode kind
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct RampRate {
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="1")]
    pub negative_reactive_power_kv_ar_per_min: ::std::option::Option<f32>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub negative_real_power_kw_per_min: ::std::option::Option<f32>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub positive_reactive_power_kv_ar_per_min: ::std::option::Option<f32>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="4")]
    pub positive_real_power_kw_per_min: ::std::option::Option<f32>,
}
mod ramp_rate {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref NEGATIVE_REACTIVE_POWER_KV_AR_PER_MIN: f32 = Default::default();
        pub(super) static ref NEGATIVE_REAL_POWER_KW_PER_MIN: f32 = Default::default();
        pub(super) static ref POSITIVE_REACTIVE_POWER_KV_AR_PER_MIN: f32 = Default::default();
        pub(super) static ref POSITIVE_REAL_POWER_KW_PER_MIN: f32 = Default::default();
    }
}
pub trait IsRampRate {
    fn _ramp_rate(&self) -> &RampRate;
    fn negative_reactive_power_kv_ar_per_min(&self) -> &f32 {
        self._ramp_rate().negative_reactive_power_kv_ar_per_min.as_ref().unwrap_or(&ramp_rate::NEGATIVE_REACTIVE_POWER_KV_AR_PER_MIN)
    }
    fn negative_real_power_kw_per_min(&self) -> &f32 {
        self._ramp_rate().negative_real_power_kw_per_min.as_ref().unwrap_or(&ramp_rate::NEGATIVE_REAL_POWER_KW_PER_MIN)
    }
    fn positive_reactive_power_kv_ar_per_min(&self) -> &f32 {
        self._ramp_rate().positive_reactive_power_kv_ar_per_min.as_ref().unwrap_or(&ramp_rate::POSITIVE_REACTIVE_POWER_KV_AR_PER_MIN)
    }
    fn positive_real_power_kw_per_min(&self) -> &f32 {
        self._ramp_rate().positive_real_power_kw_per_min.as_ref().unwrap_or(&ramp_rate::POSITIVE_REAL_POWER_KW_PER_MIN)
    }
}
impl IsRampRate for RampRate {
    fn _ramp_rate(&self) -> &RampRate {
        self
    }
}
/// Generic reading message information
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ReadingMessageInfo {
    /// UML inherited base object
    // parent_message: true
    // required_field: false
    // multiplicity_min: 0
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="1")]
    pub message_info: ::std::option::Option<MessageInfo>,
}
mod reading_message_info {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref MESSAGE_INFO: crate::commonmodule::MessageInfo = Default::default();
    }
}
pub trait IsReadingMessageInfo {
    fn _reading_message_info(&self) -> &ReadingMessageInfo;
    fn message_info(&self) -> &MessageInfo {
        self._reading_message_info().message_info.as_ref().unwrap_or(&reading_message_info::MESSAGE_INFO)
    }
}
impl IsReadingMessageInfo for ReadingMessageInfo {
    fn _reading_message_info(&self) -> &ReadingMessageInfo {
        self
    }
}
/// Specialized 61850 MMTR class
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ReadingMmtr {
    /// UML inherited base object
    // parent_message: true
    // required_field: false
    // multiplicity_min: 0
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="1")]
    pub logical_node: ::std::option::Option<LogicalNode>,
    /// Apparent energy demand (direction: from busbar).
    #[prost(message, optional, tag="2")]
    pub dmd_v_ah: ::std::option::Option<Bcr>,
    /// Reactive energy demand (direction: from busbar).
    #[prost(message, optional, tag="3")]
    pub dmd_v_arh: ::std::option::Option<Bcr>,
    /// Real energy demand (direction: from busbar).
    #[prost(message, optional, tag="4")]
    pub dmd_wh: ::std::option::Option<Bcr>,
    /// Apparent energy supply (default direction: towards busbar).
    #[prost(message, optional, tag="5")]
    pub sup_v_ah: ::std::option::Option<Bcr>,
    /// Reactive energy supply (default direction: towards busbar).
    #[prost(message, optional, tag="6")]
    pub sup_v_arh: ::std::option::Option<Bcr>,
    /// Real energy supply (default direction: towards busbar).
    #[prost(message, optional, tag="7")]
    pub sup_wh: ::std::option::Option<Bcr>,
    /// Net apparent energy since last reset.
    #[prost(message, optional, tag="8")]
    pub tot_v_ah: ::std::option::Option<Bcr>,
    /// Net reactive energy since last reset.
    #[prost(message, optional, tag="9")]
    pub tot_v_arh: ::std::option::Option<Bcr>,
    /// Net real energy since last reset.
    #[prost(message, optional, tag="10")]
    pub tot_wh: ::std::option::Option<Bcr>,
}
mod reading_mmtr {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref LOGICAL_NODE: crate::commonmodule::LogicalNode = Default::default();
        pub(super) static ref DMD_V_AH: crate::commonmodule::Bcr = Default::default();
        pub(super) static ref DMD_V_ARH: crate::commonmodule::Bcr = Default::default();
        pub(super) static ref DMD_WH: crate::commonmodule::Bcr = Default::default();
        pub(super) static ref SUP_V_AH: crate::commonmodule::Bcr = Default::default();
        pub(super) static ref SUP_V_ARH: crate::commonmodule::Bcr = Default::default();
        pub(super) static ref SUP_WH: crate::commonmodule::Bcr = Default::default();
        pub(super) static ref TOT_V_AH: crate::commonmodule::Bcr = Default::default();
        pub(super) static ref TOT_V_ARH: crate::commonmodule::Bcr = Default::default();
        pub(super) static ref TOT_WH: crate::commonmodule::Bcr = Default::default();
    }
}
pub trait IsReadingMmtr {
    fn _reading_mmtr(&self) -> &ReadingMmtr;
    fn logical_node(&self) -> &LogicalNode {
        self._reading_mmtr().logical_node.as_ref().unwrap_or(&reading_mmtr::LOGICAL_NODE)
    }
    fn dmd_v_ah(&self) -> &Bcr {
        self._reading_mmtr().dmd_v_ah.as_ref().unwrap_or(&reading_mmtr::DMD_V_AH)
    }
    fn dmd_v_arh(&self) -> &Bcr {
        self._reading_mmtr().dmd_v_arh.as_ref().unwrap_or(&reading_mmtr::DMD_V_ARH)
    }
    fn dmd_wh(&self) -> &Bcr {
        self._reading_mmtr().dmd_wh.as_ref().unwrap_or(&reading_mmtr::DMD_WH)
    }
    fn sup_v_ah(&self) -> &Bcr {
        self._reading_mmtr().sup_v_ah.as_ref().unwrap_or(&reading_mmtr::SUP_V_AH)
    }
    fn sup_v_arh(&self) -> &Bcr {
        self._reading_mmtr().sup_v_arh.as_ref().unwrap_or(&reading_mmtr::SUP_V_ARH)
    }
    fn sup_wh(&self) -> &Bcr {
        self._reading_mmtr().sup_wh.as_ref().unwrap_or(&reading_mmtr::SUP_WH)
    }
    fn tot_v_ah(&self) -> &Bcr {
        self._reading_mmtr().tot_v_ah.as_ref().unwrap_or(&reading_mmtr::TOT_V_AH)
    }
    fn tot_v_arh(&self) -> &Bcr {
        self._reading_mmtr().tot_v_arh.as_ref().unwrap_or(&reading_mmtr::TOT_V_ARH)
    }
    fn tot_wh(&self) -> &Bcr {
        self._reading_mmtr().tot_wh.as_ref().unwrap_or(&reading_mmtr::TOT_WH)
    }
}
impl IsReadingMmtr for ReadingMmtr {
    fn _reading_mmtr(&self) -> &ReadingMmtr {
        self
    }
}
/// Phase to ground/neutral related measured values of a three-phase system (WYE)
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Wye {
    /// Net current, as the algebraic sum of the instantaneous values of currents flowing through all
    /// live conductors and the neutral of a circuit at one point of the electrical installation ('phsA
    /// instCVal'+'phsB.instCVal'+'phsC.instCVal'+'neut.instCVal').
    #[prost(message, optional, tag="1")]
    pub net: ::std::option::Option<Cmv>,
    /// Value of the measured phase neutral. If a direct measurement of this value is not available, it
    /// is acceptable to substitute an estimate computed by creating the algebraic sum of the instantaneous
    /// values of currents flowing through all live conductors ('phsA.instCVal'+'phsB.instCVal'+'phsC
    /// instCVal'); in that case, 'neut'='res'.
    #[prost(message, optional, tag="2")]
    pub neut: ::std::option::Option<Cmv>,
    /// Value of phase A.
    #[prost(message, optional, tag="3")]
    pub phs_a: ::std::option::Option<Cmv>,
    /// Value of phase B.
    #[prost(message, optional, tag="4")]
    pub phs_b: ::std::option::Option<Cmv>,
    /// Value of phase C.
    #[prost(message, optional, tag="5")]
    pub phs_c: ::std::option::Option<Cmv>,
}
mod wye {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref NET: crate::commonmodule::Cmv = Default::default();
        pub(super) static ref NEUT: crate::commonmodule::Cmv = Default::default();
        pub(super) static ref PHS_A: crate::commonmodule::Cmv = Default::default();
        pub(super) static ref PHS_B: crate::commonmodule::Cmv = Default::default();
        pub(super) static ref PHS_C: crate::commonmodule::Cmv = Default::default();
    }
}
pub trait IsWye {
    fn _wye(&self) -> &Wye;
    fn net(&self) -> &Cmv {
        self._wye().net.as_ref().unwrap_or(&wye::NET)
    }
    fn neut(&self) -> &Cmv {
        self._wye().neut.as_ref().unwrap_or(&wye::NEUT)
    }
    fn phs_a(&self) -> &Cmv {
        self._wye().phs_a.as_ref().unwrap_or(&wye::PHS_A)
    }
    fn phs_b(&self) -> &Cmv {
        self._wye().phs_b.as_ref().unwrap_or(&wye::PHS_B)
    }
    fn phs_c(&self) -> &Cmv {
        self._wye().phs_c.as_ref().unwrap_or(&wye::PHS_C)
    }
}
impl IsWye for Wye {
    fn _wye(&self) -> &Wye {
        self
    }
}
/// Specialized 61850 MMXU LN class
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ReadingMmxu {
    /// UML inherited base object
    // parent_message: true
    // required_field: false
    // multiplicity_min: 0
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="1")]
    pub logical_node: ::std::option::Option<LogicalNode>,
    /// Phase to ground/phase to neutral three phase currents.
    #[prost(message, optional, tag="2")]
    pub a: ::std::option::Option<Wye>,
    /// Kind of statistical calculation, specifying how the data attributes that represent analogue
    /// values have been calculated. The calculation method shall be the same for all data objects of the
    /// logical node instance.If the value is 'PEAK_FUNDAMENTAL', angle may be present in a data object of
    /// complex measured value type (CMV, such as in WYE, DEL, etc.), otherwise angle is not used (if
    /// ‘TRUE_RMS’ and ‘RMS_FUNDAMENTAL’).If the value is 'unspecified', the dependent data objects may be
    /// meaningless.
    #[prost(message, optional, tag="3")]
    pub clc_mth: ::std::option::Option<EngCalcMethodKind>,
    /// Frequency [Hz].
    #[prost(message, optional, tag="4")]
    pub hz: ::std::option::Option<Mv>,
    /// Phase to ground/phase to neutral power factors.The power factor is defined as P (active power) /
    /// S (apparent power), so the value range is 0...1. If current (I) and voltage (U) are sinusoidal and
    /// displaced by the angle phi, then the power factor is |cos phi|, again with the value range 0...1.
    /// Therefore, for the power factor per phase, value is contained in 'mag' and 'ang' is not used.
    #[prost(message, optional, tag="5")]
    pub pf: ::std::option::Option<Wye>,
    /// Sign convention for power factor 'PF' (and reactive power 'VAr').
    #[prost(message, optional, tag="6")]
    pub pf_sign: ::std::option::Option<EngPfSignKind>,
    /// Phase to ground (line) voltages.
    #[prost(message, optional, tag="7")]
    pub ph_v: ::std::option::Option<Wye>,
    /// Phase to phase voltages.
    #[prost(message, optional, tag="8")]
    pub ppv: ::std::option::Option<Del>,
    /// Phase to ground/phase to neutral apparent powers S.
    #[prost(message, optional, tag="9")]
    pub va: ::std::option::Option<Wye>,
    /// Phase to ground/phase to neutral reactive powers Q.
    #[prost(message, optional, tag="10")]
    pub v_ar: ::std::option::Option<Wye>,
    /// Phase to ground/phase to neutral real powers P.
    #[prost(message, optional, tag="11")]
    pub w: ::std::option::Option<Wye>,
}
mod reading_mmxu {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref LOGICAL_NODE: crate::commonmodule::LogicalNode = Default::default();
        pub(super) static ref A: crate::commonmodule::Wye = Default::default();
        pub(super) static ref CLC_MTH: crate::commonmodule::EngCalcMethodKind = Default::default();
        pub(super) static ref HZ: crate::commonmodule::Mv = Default::default();
        pub(super) static ref PF: crate::commonmodule::Wye = Default::default();
        pub(super) static ref PF_SIGN: crate::commonmodule::EngPfSignKind = Default::default();
        pub(super) static ref PH_V: crate::commonmodule::Wye = Default::default();
        pub(super) static ref PPV: crate::commonmodule::Del = Default::default();
        pub(super) static ref VA: crate::commonmodule::Wye = Default::default();
        pub(super) static ref V_AR: crate::commonmodule::Wye = Default::default();
        pub(super) static ref W: crate::commonmodule::Wye = Default::default();
    }
}
pub trait IsReadingMmxu {
    fn _reading_mmxu(&self) -> &ReadingMmxu;
    fn logical_node(&self) -> &LogicalNode {
        self._reading_mmxu().logical_node.as_ref().unwrap_or(&reading_mmxu::LOGICAL_NODE)
    }
    fn a(&self) -> &Wye {
        self._reading_mmxu().a.as_ref().unwrap_or(&reading_mmxu::A)
    }
    fn clc_mth(&self) -> &EngCalcMethodKind {
        self._reading_mmxu().clc_mth.as_ref().unwrap_or(&reading_mmxu::CLC_MTH)
    }
    fn hz(&self) -> &Mv {
        self._reading_mmxu().hz.as_ref().unwrap_or(&reading_mmxu::HZ)
    }
    fn pf(&self) -> &Wye {
        self._reading_mmxu().pf.as_ref().unwrap_or(&reading_mmxu::PF)
    }
    fn pf_sign(&self) -> &EngPfSignKind {
        self._reading_mmxu().pf_sign.as_ref().unwrap_or(&reading_mmxu::PF_SIGN)
    }
    fn ph_v(&self) -> &Wye {
        self._reading_mmxu().ph_v.as_ref().unwrap_or(&reading_mmxu::PH_V)
    }
    fn ppv(&self) -> &Del {
        self._reading_mmxu().ppv.as_ref().unwrap_or(&reading_mmxu::PPV)
    }
    fn va(&self) -> &Wye {
        self._reading_mmxu().va.as_ref().unwrap_or(&reading_mmxu::VA)
    }
    fn v_ar(&self) -> &Wye {
        self._reading_mmxu().v_ar.as_ref().unwrap_or(&reading_mmxu::V_AR)
    }
    fn w(&self) -> &Wye {
        self._reading_mmxu().w.as_ref().unwrap_or(&reading_mmxu::W)
    }
}
impl IsReadingMmxu for ReadingMmxu {
    fn _reading_mmxu(&self) -> &ReadingMmxu {
        self
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct OptionalDbPosKind {
    #[prost(enumeration="DbPosKind", tag="1")]
    pub value: i32,
}
mod optional_db_pos_kind {
    use lazy_static::lazy_static;
    lazy_static! {
    }
}
pub trait IsOptionalDbPosKind {
    fn _optional_db_pos_kind(&self) -> &OptionalDbPosKind;
    fn value(&self) -> &i32 {
        &self._optional_db_pos_kind().value    }
}
impl IsOptionalDbPosKind for OptionalDbPosKind {
    fn _optional_db_pos_kind(&self) -> &OptionalDbPosKind {
        self
    }
}
/// Specialized 61850 DPS class
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct StatusDps {
    /// Quality of the value in 'stVal'.
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="1")]
    pub q: ::std::option::Option<Quality>,
    /// Status value of the controllable data object.
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(enumeration="DbPosKind", tag="2")]
    pub st_val: i32,
    /// Timestamp of the last change of the value in any of 'stVal' or 'q'.
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="3")]
    pub t: ::std::option::Option<Timestamp>,
}
mod status_dps {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref Q: crate::commonmodule::Quality = Default::default();
        pub(super) static ref T: crate::commonmodule::Timestamp = Default::default();
    }
}
pub trait IsStatusDps {
    fn _status_dps(&self) -> &StatusDps;
    fn q(&self) -> &Quality {
        self._status_dps().q.as_ref().unwrap_or(&status_dps::Q)
    }
    fn st_val(&self) -> &i32 {
        &self._status_dps().st_val    }
    fn t(&self) -> &Timestamp {
        self._status_dps().t.as_ref().unwrap_or(&status_dps::T)
    }
}
impl IsStatusDps for StatusDps {
    fn _status_dps(&self) -> &StatusDps {
        self
    }
}
/// OpenFMB specialization for breaker, recloser and switch status and event profiles:  LN: Circuit
/// breaker   Name: XCBR
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct StatusAndEventXcbr {
    /// UML inherited base object
    // parent_message: true
    // required_field: false
    // multiplicity_min: 0
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="1")]
    pub logical_node_for_event_and_status: ::std::option::Option<LogicalNodeForEventAndStatus>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub dynamic_test: ::std::option::Option<EnsDynamicTestKind>,
    /// MISSING DOCUMENTATION!!!
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="3")]
    pub pos: ::std::option::Option<StatusDps>,
}
mod status_and_event_xcbr {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref LOGICAL_NODE_FOR_EVENT_AND_STATUS: crate::commonmodule::LogicalNodeForEventAndStatus = Default::default();
        pub(super) static ref DYNAMIC_TEST: crate::commonmodule::EnsDynamicTestKind = Default::default();
        pub(super) static ref POS: crate::commonmodule::StatusDps = Default::default();
    }
}
pub trait IsStatusAndEventXcbr {
    fn _status_and_event_xcbr(&self) -> &StatusAndEventXcbr;
    fn logical_node_for_event_and_status(&self) -> &LogicalNodeForEventAndStatus {
        self._status_and_event_xcbr().logical_node_for_event_and_status.as_ref().unwrap_or(&status_and_event_xcbr::LOGICAL_NODE_FOR_EVENT_AND_STATUS)
    }
    fn dynamic_test(&self) -> &EnsDynamicTestKind {
        self._status_and_event_xcbr().dynamic_test.as_ref().unwrap_or(&status_and_event_xcbr::DYNAMIC_TEST)
    }
    fn pos(&self) -> &StatusDps {
        self._status_and_event_xcbr().pos.as_ref().unwrap_or(&status_and_event_xcbr::POS)
    }
}
impl IsStatusAndEventXcbr for StatusAndEventXcbr {
    fn _status_and_event_xcbr(&self) -> &StatusAndEventXcbr {
        self
    }
}
/// &lt;&lt;statistics&gt;&gt; Integer controlled step position information (ISC)
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct StatusIsc {
    /// Quality of the value in 'valWTr'.
    #[prost(message, optional, tag="1")]
    pub q: ::std::option::Option<Quality>,
    /// Status value
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(int32, tag="2")]
    pub st_val: i32,
    /// Timestamp of the last change of the value in any of 'valWTr' or 'q'.
    #[prost(message, optional, tag="3")]
    pub t: ::std::option::Option<Timestamp>,
}
mod status_isc {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref Q: crate::commonmodule::Quality = Default::default();
        pub(super) static ref T: crate::commonmodule::Timestamp = Default::default();
    }
}
pub trait IsStatusIsc {
    fn _status_isc(&self) -> &StatusIsc;
    fn q(&self) -> &Quality {
        self._status_isc().q.as_ref().unwrap_or(&status_isc::Q)
    }
    fn st_val(&self) -> &i32 {
        &self._status_isc().st_val    }
    fn t(&self) -> &Timestamp {
        self._status_isc().t.as_ref().unwrap_or(&status_isc::T)
    }
}
impl IsStatusIsc for StatusIsc {
    fn _status_isc(&self) -> &StatusIsc {
        self
    }
}
/// Generic status message information
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct StatusMessageInfo {
    /// UML inherited base object
    // parent_message: true
    // required_field: false
    // multiplicity_min: 0
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="1")]
    pub message_info: ::std::option::Option<MessageInfo>,
}
mod status_message_info {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref MESSAGE_INFO: crate::commonmodule::MessageInfo = Default::default();
    }
}
pub trait IsStatusMessageInfo {
    fn _status_message_info(&self) -> &StatusMessageInfo;
    fn message_info(&self) -> &MessageInfo {
        self._status_message_info().message_info.as_ref().unwrap_or(&status_message_info::MESSAGE_INFO)
    }
}
impl IsStatusMessageInfo for StatusMessageInfo {
    fn _status_message_info(&self) -> &StatusMessageInfo {
        self
    }
}
/// Controllable single point (SPC)
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct StatusSpc {
    /// Quality of the value in 'stVal'.
    #[prost(message, optional, tag="1")]
    pub q: ::std::option::Option<Quality>,
    /// Status value of the controllable data object.
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(bool, tag="2")]
    pub st_val: bool,
    /// Timestamp of the last change of the value in any of 'stVal' or 'q'.
    #[prost(message, optional, tag="3")]
    pub t: ::std::option::Option<Timestamp>,
}
mod status_spc {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref Q: crate::commonmodule::Quality = Default::default();
        pub(super) static ref T: crate::commonmodule::Timestamp = Default::default();
    }
}
pub trait IsStatusSpc {
    fn _status_spc(&self) -> &StatusSpc;
    fn q(&self) -> &Quality {
        self._status_spc().q.as_ref().unwrap_or(&status_spc::Q)
    }
    fn st_val(&self) -> &bool {
        &self._status_spc().st_val    }
    fn t(&self) -> &Timestamp {
        self._status_spc().t.as_ref().unwrap_or(&status_spc::T)
    }
}
impl IsStatusSpc for StatusSpc {
    fn _status_spc(&self) -> &StatusSpc {
        self
    }
}
/// Status value
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct StatusValue {
    /// UML inherited base object
    // parent_message: true
    // required_field: false
    // multiplicity_min: 0
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="1")]
    pub identified_object: ::std::option::Option<IdentifiedObject>,
}
mod status_value {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref IDENTIFIED_OBJECT: crate::commonmodule::IdentifiedObject = Default::default();
    }
}
pub trait IsStatusValue {
    fn _status_value(&self) -> &StatusValue;
    fn identified_object(&self) -> &IdentifiedObject {
        self._status_value().identified_object.as_ref().unwrap_or(&status_value::IDENTIFIED_OBJECT)
    }
}
impl IsStatusValue for StatusValue {
    fn _status_value(&self) -> &StatusValue {
        self
    }
}
/// Visible string status (VSS)
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Vss {
    /// Quality of the value in 'stVal'.
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="1")]
    pub q: ::std::option::Option<Quality>,
    /// Value of the data.
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(string, tag="2")]
    pub st_val: std::string::String,
    /// Timestamp of the last change of the value in any of 'stVal' or 'q'.
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="3")]
    pub t: ::std::option::Option<Timestamp>,
}
mod vss {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref Q: crate::commonmodule::Quality = Default::default();
        pub(super) static ref T: crate::commonmodule::Timestamp = Default::default();
    }
}
pub trait IsVss {
    fn _vss(&self) -> &Vss;
    fn q(&self) -> &Quality {
        self._vss().q.as_ref().unwrap_or(&vss::Q)
    }
    fn st_val(&self) -> &std::string::String {
        &self._vss().st_val    }
    fn t(&self) -> &Timestamp {
        self._vss().t.as_ref().unwrap_or(&vss::T)
    }
}
impl IsVss for Vss {
    fn _vss(&self) -> &Vss {
        self
    }
}
/// LN: Generic process I/O   Name: GGIO
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct StringStatusGgio {
    /// UML inherited base object
    // parent_message: true
    // required_field: false
    // multiplicity_min: 0
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="1")]
    pub logical_node: ::std::option::Option<LogicalNode>,
    /// Phase code
    #[prost(message, optional, tag="2")]
    pub phase: ::std::option::Option<OptionalPhaseCodeKind>,
    /// String status
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="3")]
    pub str_in: ::std::option::Option<Vss>,
}
mod string_status_ggio {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref LOGICAL_NODE: crate::commonmodule::LogicalNode = Default::default();
        pub(super) static ref PHASE: crate::commonmodule::OptionalPhaseCodeKind = Default::default();
        pub(super) static ref STR_IN: crate::commonmodule::Vss = Default::default();
    }
}
pub trait IsStringStatusGgio {
    fn _string_status_ggio(&self) -> &StringStatusGgio;
    fn logical_node(&self) -> &LogicalNode {
        self._string_status_ggio().logical_node.as_ref().unwrap_or(&string_status_ggio::LOGICAL_NODE)
    }
    fn phase(&self) -> &OptionalPhaseCodeKind {
        self._string_status_ggio().phase.as_ref().unwrap_or(&string_status_ggio::PHASE)
    }
    fn str_in(&self) -> &Vss {
        self._string_status_ggio().str_in.as_ref().unwrap_or(&string_status_ggio::STR_IN)
    }
}
impl IsStringStatusGgio for StringStatusGgio {
    fn _string_status_ggio(&self) -> &StringStatusGgio {
        self
    }
}
/// Point definition (Point)
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SwitchPoint {
    /// Switch position
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="1")]
    pub pos: ::std::option::Option<ControlDpc>,
    /// Start time
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="2")]
    pub start_time: ::std::option::Option<ControlTimestamp>,
}
mod switch_point {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref POS: crate::commonmodule::ControlDpc = Default::default();
        pub(super) static ref START_TIME: crate::commonmodule::ControlTimestamp = Default::default();
    }
}
pub trait IsSwitchPoint {
    fn _switch_point(&self) -> &SwitchPoint;
    fn pos(&self) -> &ControlDpc {
        self._switch_point().pos.as_ref().unwrap_or(&switch_point::POS)
    }
    fn start_time(&self) -> &ControlTimestamp {
        self._switch_point().start_time.as_ref().unwrap_or(&switch_point::START_TIME)
    }
}
impl IsSwitchPoint for SwitchPoint {
    fn _switch_point(&self) -> &SwitchPoint {
        self
    }
}
/// Curve shape setting (FC=SP) (CSG_SP)
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SwitchCsg {
    /// The array with the points specifying a curve shape.
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, repeated, tag="1")]
    pub crv_pts: ::std::vec::Vec<SwitchPoint>,
}
mod switch_csg {
    use lazy_static::lazy_static;
    lazy_static! {
    }
}
pub trait IsSwitchCsg {
    fn _switch_csg(&self) -> &SwitchCsg;
    fn crv_pts(&self) -> &::std::vec::Vec<SwitchPoint> {
        &self._switch_csg().crv_pts    }
}
impl IsSwitchCsg for SwitchCsg {
    fn _switch_csg(&self) -> &SwitchCsg {
        self
    }
}
/// OpenFMB specialization for control schedule using:  LN: Schedule   Name: FSCH
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SwitchControlScheduleFsch {
    /// Discrete value in SwitchCSG type
    // parent_message: false
    // required_field: true
    // multiplicity_min: 1
    // multiplicity_max: 0
    // uuid: false
    // key: false
    #[prost(message, optional, tag="1")]
    pub val_dcsg: ::std::option::Option<SwitchCsg>,
}
mod switch_control_schedule_fsch {
    use lazy_static::lazy_static;
    lazy_static! {
        pub(super) static ref VAL_DCSG: crate::commonmodule::SwitchCsg = Default::default();
    }
}
pub trait IsSwitchControlScheduleFsch {
    fn _switch_control_schedule_fsch(&self) -> &SwitchControlScheduleFsch;
    fn val_dcsg(&self) -> &SwitchCsg {
        self._switch_control_schedule_fsch().val_dcsg.as_ref().unwrap_or(&switch_control_schedule_fsch::VAL_DCSG)
    }
}
impl IsSwitchControlScheduleFsch for SwitchControlScheduleFsch {
    fn _switch_control_schedule_fsch(&self) -> &SwitchControlScheduleFsch {
        self
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct OptionalStateKind {
    #[prost(enumeration="StateKind", tag="1")]
    pub value: i32,
}
mod optional_state_kind {
    use lazy_static::lazy_static;
    lazy_static! {
    }
}
pub trait IsOptionalStateKind {
    fn _optional_state_kind(&self) -> &OptionalStateKind;
    fn value(&self) -> &i32 {
        &self._optional_state_kind().value    }
}
impl IsOptionalStateKind for OptionalStateKind {
    fn _optional_state_kind(&self) -> &OptionalStateKind {
        self
    }
}
/// The units defined for usage in the CIM.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum UnitSymbolKind {
    /// Dimension less quantity, e.g. count, per unit, etc.
    None = 0,
    /// Length in meter.
    Meter = 2,
    /// Mass in gram.
    Gram = 3,
    /// Current in ampere.
    Amp = 5,
    /// Plane angle in degrees.
    Deg = 9,
    /// Plane angle in radians.
    Rad = 10,
    /// Relative temperature in degrees Celsius. In the SI unit system the symbol is ºC. Electric charge
    /// is measured in coulomb that has the unit symbol C. To distinguish degree Celsius form coulomb the
    /// symbol used in the UML is degC. Reason for not using ºC is the special character º is difficult to
    /// manage in software.
    DegC = 23,
    /// Capacitance in farad.
    Farad = 25,
    /// Time in seconds.
    Sec = 27,
    /// Inductance in Henry.
    Henry = 28,
    /// Voltage in volt.
    V = 29,
    /// Resistance in ohm.
    Ohm = 30,
    /// Energy in joule.
    Joule = 31,
    /// Force in newton.
    Newton = 32,
    /// Frequency in hertz.
    Hz = 33,
    /// Active power in watt.
    W = 38,
    /// Pressure in pascal (n/m2).
    Pa = 39,
    /// Area in square meters.
    M2 = 41,
    /// Conductance in siemens.
    Siemens = 53,
    /// Apparent power in volt ampere.
    Va = 61,
    /// Reactive power in volt ampere reactive.
    VAr = 63,
    /// Power factor
    WPerVa = 65,
    /// Apparent energy in volt ampere hours.
    VAh = 71,
    /// Real energy in what hours.
    Wh = 72,
    /// Reactive energy in volt ampere reactive hours.
    VArh = 73,
    /// MISSING DOCUMENTATION!!!
    HzPerS = 75,
    /// MISSING DOCUMENTATION!!!
    WPerS = 81,
    /// Other enum not listed
    Other = 100,
    /// Amp hour
    Ah = 106,
    /// Time in minutes.
    Min = 159,
    /// Time in hours.
    Hour = 160,
    /// Volume in cubic meters.
    M3 = 166,
    /// Watts per square meter
    WPerM2 = 179,
    /// Relative temperature in degree fahrenheit.
    DegF = 279,
    /// Mile per hour
    Mph = 500,
}
/// The unit multipliers defined for the CIM.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum UnitMultiplierKind {
    /// No multiplier or equivalently multiply by 1.
    None = 0,
    /// Other enum not listed
    Other = 1,
    /// Centi 10**-2.
    Centi = 2,
    /// Deci 10**-1.
    Deci = 3,
    /// Giga 10**9.
    Giga = 4,
    /// Kilo 10**3.
    Kilo = 5,
    /// Mega 10**6.
    Mega = 6,
    /// Micro 10**-6.
    Micro = 7,
    /// Milli 10**-3.
    Milli = 8,
    /// Nano 10**-9.
    Nano = 9,
    /// Pico 10**-12.
    Pico = 10,
    /// Tera 10**12.
    Tera = 11,
}
/// Enumeration of phase identifiers. Allows designation of phases for both transmission and
/// distribution equipment, circuits and loads. Residential and small commercial loads are often served
/// from single-phase, or split-phase, secondary circuits. For example of s12N, phases 1 and 2 refer to
/// hot wires that are 180 degrees out of phase, while N refers to the neutral wire. Through single
/// phase transformer connections, these secondary circuits may be served from one or two of the primary
/// phases A, B, and C. For three-phase loads, use the A, B, C phase codes instead of s12N.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum PhaseCodeKind {
    /// Not applicable
    None = 0,
    /// Other enum not listed
    Other = 1,
    /// Neutral phase.
    N = 16,
    /// Phase C.
    C = 32,
    /// Phases C and neutral.
    Cn = 33,
    /// Phases A and C.
    Ac = 40,
    /// Phases A, C and neutral.
    Acn = 41,
    /// Phase B.
    B = 64,
    /// Phases B and neutral.
    Bn = 65,
    /// Phases B and C.
    Bc = 66,
    /// Phases B, C, and neutral.
    Bcn = 97,
    /// Phase A.
    A = 128,
    /// Phases A and neutral.
    An = 129,
    /// Phases A and B.
    Ab = 132,
    /// Phases A, B, and neutral.
    Abn = 193,
    /// Phases A, B, and C.
    Abc = 224,
    /// Phases A, B, C, and N.
    Abcn = 225,
    /// Secondary phase 2.
    S2 = 256,
    /// Secondary phase 2 and neutral.
    S2N = 257,
    /// Secondary phase 1.
    S1 = 512,
    /// Secondary phase 1 and neutral.
    S1N = 513,
    /// Secondary phase 1 and 2.
    S12 = 768,
    /// Secondary phases 1, 2, and neutral.
    S12N = 769,
}
/// Validity of the value, as condensed information for the client. In case this value is not
/// 'good', some reasons may be found in the 'detailQual'.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum ValidityKind {
    /// Supervision function has detected no abnormal condition of either the acquisition function or
    /// the information source.
    Good = 0,
    /// Supervision function has detected an abnormal condition of the acquisition function or the
    /// information source (missing or non-operating updating devices). The value is not defined under this
    /// condition. It shall be used to indicate to the client that the value may be incorrect and shall not
    /// be used.  EXAMPLE If an input unit detects an oscillation of one input it will mark the related
    /// information as invalid.
    Invalid = 1,
    /// Reserved
    Reserved = 2,
    /// Supervision function has detected any abnormal behaviour. However, the value could still be
    /// valid. It is client's responsibility to determine whether the values should be used.
    Questionable = 3,
}
/// (default=process) Defines the source of a value. NOTE 1 Substitution may be done locally or via
/// the communication services. In the second case, specific attributes with a FC=SV are used. NOTE 2
/// There are various means to clear a substitution. As an example, a substitution that was done
/// following an invalid condition may be cleared automatically if the invalid condition is cleared.
/// However, this is a local issue and therefore
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum SourceKind {
    /// The value is provided by an input function from the process I/O or is calculated from some
    /// application function.
    Process = 0,
    /// The value is provided by an operator input or by an automatic source.
    Substituted = 1,
}
/// Validity of the value, as condensed information for the client. In case this value is not
/// 'good', some reasons may be found in the 'detailQual'.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum TimeAccuracyKind {
    /// Undefined
    Undefined = 0,
    /// 10 ms (class T0)
    T0 = 7,
    /// 1 ms (class T1)
    T1 = 10,
    /// 100 us (class T2)
    T2 = 14,
    /// 25 us (class T3)
    T3 = 16,
    /// 4 us (class T4)
    T4 = 18,
    /// 1 us (class T5)
    T5 = 20,
    /// Undefined
    Unspecified = 31,
}
/// ESS function kind
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum ScheduleParameterKind {
    /// MISSING DOCUMENTATION!!!
    None = 0,
    /// Other enum not listed
    Other = 1,
    /// MISSING DOCUMENTATION!!!
    ANetMag = 2,
    /// MISSING DOCUMENTATION!!!
    ANeutMag = 3,
    /// MISSING DOCUMENTATION!!!
    APhsAMag = 4,
    /// MISSING DOCUMENTATION!!!
    APhsBMag = 5,
    /// MISSING DOCUMENTATION!!!
    APhsCMag = 6,
    /// MISSING DOCUMENTATION!!!
    HzMag = 7,
    /// MISSING DOCUMENTATION!!!
    PfNetMag = 8,
    /// MISSING DOCUMENTATION!!!
    PfNeutMag = 9,
    /// MISSING DOCUMENTATION!!!
    PfPhsAMag = 10,
    /// MISSING DOCUMENTATION!!!
    PfPhsBMag = 11,
    /// MISSING DOCUMENTATION!!!
    PfPhsCMag = 12,
    /// MISSING DOCUMENTATION!!!
    PhVNetAng = 13,
    /// MISSING DOCUMENTATION!!!
    PhVNetMag = 14,
    /// MISSING DOCUMENTATION!!!
    PhVNeutAng = 15,
    /// MISSING DOCUMENTATION!!!
    PhVNeutMag = 16,
    /// MISSING DOCUMENTATION!!!
    PhVPhsAAng = 17,
    /// MISSING DOCUMENTATION!!!
    PhVPhsAMag = 18,
    /// MISSING DOCUMENTATION!!!
    PhVPhsBAng = 19,
    /// MISSING DOCUMENTATION!!!
    PhVPhsBMag = 20,
    /// MISSING DOCUMENTATION!!!
    PhVPhsCAng = 21,
    /// MISSING DOCUMENTATION!!!
    PhVPhsCMag = 22,
    /// MISSING DOCUMENTATION!!!
    PpvPhsAbAng = 23,
    /// MISSING DOCUMENTATION!!!
    PpvPhsAbMag = 24,
    /// MISSING DOCUMENTATION!!!
    PpvPhsBcAng = 25,
    /// MISSING DOCUMENTATION!!!
    PpvPhsBcMag = 26,
    /// MISSING DOCUMENTATION!!!
    PpvPhsCaAng = 27,
    /// MISSING DOCUMENTATION!!!
    PpvPhsCaMag = 28,
    /// MISSING DOCUMENTATION!!!
    VaNetMag = 29,
    /// MISSING DOCUMENTATION!!!
    VaNeutMag = 30,
    /// MISSING DOCUMENTATION!!!
    VaPhsAMag = 31,
    /// MISSING DOCUMENTATION!!!
    VaPhsBMag = 32,
    /// MISSING DOCUMENTATION!!!
    VaPhsCMag = 33,
    /// MISSING DOCUMENTATION!!!
    VArNetMag = 34,
    /// MISSING DOCUMENTATION!!!
    VArNeutMag = 35,
    /// MISSING DOCUMENTATION!!!
    VArPhsAMag = 36,
    /// MISSING DOCUMENTATION!!!
    VArPhsBMag = 37,
    /// MISSING DOCUMENTATION!!!
    VArPhsCMag = 38,
    /// MISSING DOCUMENTATION!!!
    WNetMag = 39,
    /// MISSING DOCUMENTATION!!!
    WNeutMag = 40,
    /// MISSING DOCUMENTATION!!!
    WPhsAMag = 41,
    /// MISSING DOCUMENTATION!!!
    WPhsBMag = 42,
    /// MISSING DOCUMENTATION!!!
    WPhsCMag = 43,
}
/// Calculation method (CalcMethodKind enumeration)
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum CalcMethodKind {
    /// Undefined enum value which can be used for Protobuf generation and be consistent with other
    /// technologies.
    Undefined = 0,
    /// All analogue values (i.e., all common attributes 'i' and 'f') meet the sampling and filtering
    /// characteristics specified in IEEE C37.118.1 for P-CLASS.
    PClass = 11,
    /// All analogue values (i.e., all common attributes 'i' and 'f') meet the sampling and filtering
    /// characteristics specified in IEEE C37.118.1 for M-CLASS.
    MClass = 12,
    /// All analogue values are [F(t+T)-F(t)] for a calculation interval T (in the same unit as the
    /// original entity). Note: The client can still calculate rate so: RATE = DIFF/T.
    Diff = 13,
}
/// Power system connect modes to the power grid (GridConnectModeKind)
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum GridConnectModeKind {
    /// Undefined
    Undefined = 0,
    /// Current-source inverter (CSI)
    Csi = 1,
    /// Voltage-controlled voltage-source inverter (VC-VSI)
    VcVsi = 2,
    /// Current-controlled voltage-source inverter (CC-VSI)
    CcVsi = 3,
    /// Not applicable / Unknown
    None = 98,
    /// MISSING DOCUMENTATION!!!
    Other = 99,
    /// Voltage source inverter regulating to P and Q references (VSI PQ)
    VsiPq = 2000,
    /// Voltage source inverter regulating to voltage and frequency references paralleling other
    /// generation and not grid forming (VSI VF).
    VsiVf = 2001,
    /// Voltage source inverter regulating to voltage and frequency references as primary grid forming
    /// generation (VSI ISO).
    VsiIso = 2002,
}
/// Power factor sign (PFSignKind enumeration)
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum PfSignKind {
    /// Undefined enum value which can be used for Protobuf generation and be consistent with other
    /// technologies.
    Undefined = 0,
    /// All analogue values are [F(t+T)-F(t)] for a calculation interval T (in the same unit as the
    /// original entity). Note: The client can still calculate rate so: RATE = DIFF/T.
    Iec = 1,
    /// All analogue values (i.e., all common attributes 'i' and 'f') meet the sampling and filtering
    /// characteristics specified in IEEE C37.118.1 for M-CLASS.
    Eei = 2,
}
/// Behaviour or mode (BehaviourModeKind enumeration)
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum BehaviourModeKind {
    /// Undefined
    Undefined = 0,
    /// Normal enabled state.
    On = 1,
    /// Process is passively supervised.
    Blocked = 2,
    /// Function is operated but results are indicated as test results.
    Test = 3,
    /// Function is operated in test mode, but with no impact to the process.
    TestBlocked = 4,
    /// Function is inactive but shows its configuration capability.
    Off = 5,
}
/// DER operational state (DERGeneratorStateKind)
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum DerGeneratorStateKind {
    /// Undefined enum value which can be used for Protobuf generation and be consistent with other
    /// technologies.
    Undefined = 0,
    /// MISSING DOCUMENTATION!!!
    NotOperating = 1,
    /// MISSING DOCUMENTATION!!!
    Operating = 2,
    /// MISSING DOCUMENTATION!!!
    StartingUp = 3,
    /// MISSING DOCUMENTATION!!!
    ShuttingDown = 4,
    /// MISSING DOCUMENTATION!!!
    AtDisconnectLevel = 5,
    /// MISSING DOCUMENTATION!!!
    RampingInPower = 6,
    /// MISSING DOCUMENTATION!!!
    RampingInReactivePower = 7,
    /// MISSING DOCUMENTATION!!!
    Standby = 8,
    /// MISSING DOCUMENTATION!!!
    NotApplicableUnknown = 98,
    /// MISSING DOCUMENTATION!!!
    Other = 99,
}
/// Dynamic test status (see IEC61850-7-2 section 20.2.1 Direct control with normal security, state
/// machine diagram)   A simplified state machine diagram (from Herb F.) is provided.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum DynamicTestKind {
    /// None
    None = 0,
    /// Testing status
    Testing = 1,
    /// Operating status
    Operating = 2,
    /// Failed status
    Failed = 3,
}
/// State kind
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum HealthKind {
    /// MISSING DOCUMENTATION!!!
    None = 0,
    /// No problems, normal operation ("green").
    Ok = 1,
    /// Minor problems, but in safe operating mode ("yellow"). The exact meaning is a local issue,
    /// depending on the dedicated function/device.
    Warning = 2,
    /// Severe problem, no operation possible ("red").
    Alarm = 3,
}
/// Double point position status
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum DbPosKind {
    /// Transient status
    Transient = 0,
    /// Closed status
    Closed = 1,
    /// Open status
    Open = 2,
    /// Invalid status
    Invalid = 3,
}
/// State kind
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum StateKind {
    /// MISSING DOCUMENTATION!!!
    Off = 0,
    /// MISSING DOCUMENTATION!!!
    On = 1,
    /// MISSING DOCUMENTATION!!!
    Standby = 2,
}
