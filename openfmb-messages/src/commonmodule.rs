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
    #[prost(message, optional, tag="2")]
    pub m_rid: ::std::option::Option<::std::string::String>,
    /// The name is any free human readable and possibly non unique text naming the object.
    #[prost(message, optional, tag="3")]
    pub name: ::std::option::Option<::std::string::String>,
}
/// An electrical connection point (AC or DC) to a piece of conducting equipment. Terminals are
/// connected at physical connection points called connectivity nodes.
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct AcdcTerminal {
    /// UML inherited base object
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
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct OptionalUnitSymbolKind {
    #[prost(enumeration="UnitSymbolKind", tag="1")]
    pub value: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct OptionalUnitMultiplierKind {
    #[prost(enumeration="UnitMultiplierKind", tag="1")]
    pub value: i32,
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
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct OptionalPhaseCodeKind {
    #[prost(enumeration="PhaseCodeKind", tag="1")]
    pub value: i32,
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
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct OptionalValidityKind {
    #[prost(enumeration="ValidityKind", tag="1")]
    pub value: i32,
}
/// Describes some reasons in case 'validity' is not 'good'.
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct DetailQual {
    /// (default=false) If true, the value may not be a correct value due to a reference being out of
    /// calibration. The server shall decide if validity shall be set to invalid or questionable (used for
    /// measurand information and binary counter information only).
    #[prost(bool, tag="1")]
    pub bad_reference: bool,
    /// (default=false) If true, a supervision function has detected an internal or external failure.
    #[prost(bool, tag="2")]
    pub failure: bool,
    /// (default=false) If true, the value does not meet the stated accuracy of the source. EXAMPLE The
    /// measured value of power factor may be noisy (inaccurate) when the current is very small.
    #[prost(bool, tag="3")]
    pub inaccurate: bool,
    /// (default=false) If true, an evaluation function has detected an inconsistency.
    #[prost(bool, tag="4")]
    pub inconsistent: bool,
    /// (default=false) If true, an update is not made during a specific time interval. The value may be
    /// an old value that may have changed in the meantime. This specific time interval may be defined by an
    /// allowed-age attribute. NOTE "Fail silent" errors, where the equipment stops sending data, will cause
    /// setting this flag to true. In this case, the last received information was correct.
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
    #[prost(bool, tag="6")]
    pub oscillatory: bool,
    /// (default=false) If true, the attribute to which the quality has been associated is beyond a
    /// predefined range of values. The server shall decide if validity shall be set to invalid or
    /// questionable (used for measurand information only). EXAMPLE A measured value may exceed a predefined
    /// range, however the selected data type can still represent the value, for example the data type is a
    /// 16-bit unsigned integer, the predefined range is 0 to 40 000, if the value is between 40 001 and 65
    /// 535 it is considered to be out of range.
    #[prost(bool, tag="7")]
    pub out_of_range: bool,
    /// (default=false) If true, the value of the attribute to which the quality has been associated is
    /// beyond the capability of being represented properly (used for measurand information only). EXAMPLE A
    /// measured value may exceed the range that may be represented by the selected data type, for example
    /// the data type is a 16-bit unsigned integer and the value exceeds 65 535.
    #[prost(bool, tag="8")]
    pub overflow: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct OptionalSourceKind {
    #[prost(enumeration="SourceKind", tag="1")]
    pub value: i32,
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
    #[prost(message, optional, tag="1")]
    pub detail_qual: ::std::option::Option<DetailQual>,
    /// (default=false) If true, further update of the value has been blocked by an operator. The value
    /// shall be the information that was acquired before blocking. If this flag is set, then the
    /// 'detailQual.oldData' shall also be set. The operator shall use the data attribute 'CDC.blkEna' to
    /// block the update of the value. NOTE Both an operator as well as an automatic function may freeze
    /// communication updating as well as input updating. In both cases, 'detailQual.oldData' will be set.
    /// If the blocking is done by an operator, then this flag is set additionally, and an operator activity
    /// is required to clear the condition.
    #[prost(bool, tag="2")]
    pub operator_blocked: bool,
    /// (default=process) Defines the source of a value. NOTE 1 Substitution may be done locally or via
    /// the communication services. In the second case, specific attributes with a FC=SV are used. NOTE 2
    /// There are various means to clear a substitution. As an example, a substitution that was done
    /// following an invalid condition may be cleared automatically if the invalid condition is cleared.
    /// However, this is a local issue and therefore not within the scope of this standard.
    #[prost(enumeration="SourceKind", tag="3")]
    pub source: i32,
    /// (default=false) If true, the value is a test value. The processing of the test quality in the
    /// client shall be as described in IEC 61850-7-4. This bit shall be completely independent from the
    /// other bits within the quality descriptor.
    #[prost(bool, tag="4")]
    pub test: bool,
    /// Validity of the value, as condensed information for the client. In case this value is not
    /// 'good', some reasons may be found in the 'detailQual'.
    #[prost(enumeration="ValidityKind", tag="5")]
    pub validity: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct OptionalTimeAccuracyKind {
    #[prost(enumeration="TimeAccuracyKind", tag="1")]
    pub value: i32,
}
/// Information about the quality of the time source of the sending IED.
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct TimeQuality {
    /// If true, the time source of the sending device is unreliable and the value of the time stamp
    /// shall be ignored.
    #[prost(bool, tag="1")]
    pub clock_failure: bool,
    /// If true, the time source of the sending device is not synchronised with the external UTC time.
    #[prost(bool, tag="2")]
    pub clock_not_synchronized: bool,
    /// If true, the value in 'P_Timestamp.SecondSinceEpoch' contains all leap seconds occurred.
    /// Otherwise, it does not take into account the leap seconds that occurred before the initialization of
    /// the time source of the device. Instead, the seconds since start of the epoch are calculated from the
    /// current date assuming a constant day length of 86 400 seconds. Note: If a UTC time master clock is
    /// used and accessible, this value should always be true.
    #[prost(bool, tag="3")]
    pub leap_seconds_known: bool,
    /// Information about the quality of the time source of the sending IED.
    #[prost(enumeration="TimeAccuracyKind", tag="4")]
    pub time_accuracy: i32,
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
    #[prost(uint32, tag="1")]
    pub fraction: u32,
    /// Second since epoch (1970-01-01T00:00:00Z)
    #[prost(uint64, tag="2")]
    pub seconds: u64,
    /// IEC61850 time quality
    #[prost(message, optional, tag="3")]
    pub tq: ::std::option::Option<TimeQuality>,
}
/// Unit definition (Unit)
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Unit {
    /// (default='') Unit multiplier.
    #[prost(message, optional, tag="1")]
    pub multiplier: ::std::option::Option<OptionalUnitMultiplierKind>,
    /// SI unit of measure.
    #[prost(enumeration="UnitSymbolKind", tag="2")]
    pub si_unit: i32,
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
    #[prost(message, optional, tag="1")]
    pub mag: ::std::option::Option<AnalogueValue>,
    /// Quality of the values in 'instMag', 'mag', 'range'.
    #[prost(message, optional, tag="2")]
    pub q: ::std::option::Option<Quality>,
    /// Timestamp of the last refresh of the value in 'mag' or of the last change of the value in any of
    /// 'range' or 'q'.
    #[prost(message, optional, tag="3")]
    pub t: ::std::option::Option<Timestamp>,
    /// Unit for: 'instMag', 'mag', 'subMag', 'rangeC'.
    #[prost(message, optional, tag="4")]
    pub units: ::std::option::Option<Unit>,
}
/// IEC61850 logical node.
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct LogicalNode {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub identified_object: ::std::option::Option<IdentifiedObject>,
}
/// LN: Generic process I/O   Name: GGIO
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct AnalogStatusGgio {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub logical_node: ::std::option::Option<LogicalNode>,
    /// Generic analogue input <i>n</i>.
    #[prost(message, optional, tag="2")]
    pub an_in: ::std::option::Option<Mv>,
    /// Phase code
    #[prost(message, optional, tag="3")]
    pub phase: ::std::option::Option<OptionalPhaseCodeKind>,
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
/// The parts of a power system that are physical devices, electronic or mechanical.
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ApplicationSystem {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub named_object: ::std::option::Option<NamedObject>,
    /// MISSING DOCUMENTATION!!!
    #[prost(string, tag="2")]
    pub m_rid: std::string::String,
}
/// Analogue setting (FC=SP) (ASG_SP)
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Asg {
    /// The value of the analogue setting.
    #[prost(message, optional, tag="1")]
    pub set_mag: ::std::option::Option<AnalogueValueCtl>,
    /// Unit for 'setMag', 'minVal', 'maxVal', 'stepSize'.
    #[prost(message, optional, tag="2")]
    pub units: ::std::option::Option<Unit>,
}
/// Binary counter reading (BCR)
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Bcr {
    /// Binary counter status represented as an integer value; wraps to 0 at the maximum or minimum
    /// value of INT64.
    #[prost(int64, tag="1")]
    pub act_val: i64,
    /// Quality of the values in 'actVal', 'frVal'.
    #[prost(message, optional, tag="2")]
    pub q: ::std::option::Option<Quality>,
    /// Timestamp of the last change of value in 'actVal' or 'q'.
    #[prost(message, optional, tag="3")]
    pub t: ::std::option::Option<Timestamp>,
    /// Unit for value in 'pulsQty'.
    #[prost(message, optional, tag="4")]
    pub units: ::std::option::Option<OptionalUnitSymbolKind>,
}
/// Specialized 61850 SPS class
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct StatusSps {
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="1")]
    pub q: ::std::option::Option<Quality>,
    /// MISSING DOCUMENTATION!!!
    #[prost(bool, tag="2")]
    pub st_val: bool,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub t: ::std::option::Option<Timestamp>,
}
/// LN: Generic process I/O   Name: GGIO
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct BooleanStatusGgio {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub logical_node: ::std::option::Option<LogicalNode>,
    /// If true, indication <i>n</i> is present.
    #[prost(message, optional, tag="2")]
    pub ind: ::std::option::Option<StatusSps>,
    /// Phase code
    #[prost(message, optional, tag="3")]
    pub phase: ::std::option::Option<OptionalPhaseCodeKind>,
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
/// Vector definition (Vector)
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Vector {
    /// (range=[-180...180]) Angle of the complex value (Unit.SIUnit='deg' and Unit.multiplier='');
    /// angle reference is defined in the context where this type is used.
    #[prost(message, optional, tag="1")]
    pub ang: ::std::option::Option<AnalogueValue>,
    /// Magnitude of the complex value.
    #[prost(message, optional, tag="2")]
    pub mag: ::std::option::Option<AnalogueValue>,
}
/// Complex measured value (CMV)
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Cmv {
    /// Complex value based on a deadband calculation from the instantaneous value 'instCVal.mag'. The
    /// deadband calculation is done both on 'instCVal.mag' (based on 'db') and on 'instCVal.ang' (based on
    /// 'dbAng'), independently. See  'MV.mag'.
    #[prost(message, optional, tag="1")]
    pub c_val: ::std::option::Option<Vector>,
    /// Quality of the values in 'instCVal', 'cVal', 'range', ‘rangeAng’.
    #[prost(message, optional, tag="2")]
    pub q: ::std::option::Option<Quality>,
    /// Timestamp of the last refresh of the value in 'cVal' or of the last change of the value in any
    /// of 'range', 'rangeAng' or 'q'.
    #[prost(message, optional, tag="3")]
    pub t: ::std::option::Option<Timestamp>,
    /// Units for: 'instCVal.mag', 'cVal.mag', 'subCVal.mag', 'rangeC'.
    #[prost(message, optional, tag="4")]
    pub units: ::std::option::Option<Unit>,
}
/// Asset representation of a ConductingEquipment.
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ConductingEquipment {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub named_object: ::std::option::Option<NamedObject>,
    /// MISSING DOCUMENTATION!!!
    #[prost(string, tag="2")]
    pub m_rid: std::string::String,
}
/// An AC electrical connection point to a piece of conducting equipment. Terminals are connected at
/// physical connection points called connectivity nodes.
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Terminal {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub a_cdc_terminal: ::std::option::Option<AcdcTerminal>,
    /// Represents the normal network phasing condition. If the attribute is missing three phases (ABC
    /// or ABCN) shall be assumed.
    #[prost(message, optional, tag="2")]
    pub phases: ::std::option::Option<OptionalPhaseCodeKind>,
}
/// Reading associated with an equipment such as a recloser.
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ConductingEquipmentTerminalReading {
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="1")]
    pub terminal: ::std::option::Option<Terminal>,
}
/// Specialized DPC 61850 CDC class
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ControlDpc {
    /// Service parameter that determines the control activity ('false' for off, 'true' for on).
    #[prost(bool, tag="1")]
    pub ctl_val: bool,
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
    #[prost(uint32, tag="1")]
    pub fraction: u32,
    /// Second since epoch (1970-01-01T00:00:00Z)
    #[prost(uint64, tag="2")]
    pub seconds: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct OptionalScheduleParameterKind {
    #[prost(enumeration="ScheduleParameterKind", tag="1")]
    pub value: i32,
}
/// Grid connect mode kind
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct EngScheduleParameter {
    /// Schedule parameter type
    #[prost(enumeration="ScheduleParameterKind", tag="1")]
    pub schedule_parameter_type: i32,
    /// MISSING DOCUMENTATION!!!
    #[prost(float, tag="2")]
    pub value: f32,
}
/// Point definition (Point)
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SchedulePoint {
    /// Schedule parameter
    #[prost(message, repeated, tag="1")]
    pub schedule_parameter: ::std::vec::Vec<EngScheduleParameter>,
    /// Start time
    #[prost(message, optional, tag="2")]
    pub start_time: ::std::option::Option<ControlTimestamp>,
}
/// Curve shape setting (FC=SP) (CSG_SP)
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ScheduleCsg {
    /// The array with the points specifying a time schedule
    #[prost(message, repeated, tag="1")]
    pub sch_pts: ::std::vec::Vec<SchedulePoint>,
}
/// OpenFMB specialization for control schedule using:  LN: Schedule   Name: FSCH
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ControlScheduleFsch {
    /// Analog CSG
    #[prost(message, optional, tag="1")]
    pub val_acsg: ::std::option::Option<ScheduleCsg>,
}
/// OpenFMB specialization for logical node control
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct LogicalNodeForControl {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub logical_node: ::std::option::Option<LogicalNode>,
}
/// LN: Schedule controller   Name: FSCC  F:    Function (generic) SC:  Schedule Controller C:
/// Control (execution)
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ControlFscc {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub logical_node_for_control: ::std::option::Option<LogicalNodeForControl>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub control_schedule_fsch: ::std::option::Option<ControlScheduleFsch>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub island_control_schedule_fsch: ::std::option::Option<ControlScheduleFsch>,
}
/// Integer status setting (FC=SP) (ING_SP)
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ControlIng {
    /// The value of the status setting.
    #[prost(int32, tag="1")]
    pub set_val: i32,
    /// Unit for 'setVal', 'minVal', 'maxVal', 'stepSize'.
    #[prost(message, optional, tag="2")]
    pub units: ::std::option::Option<Unit>,
}
/// &lt;&lt;statistics&gt;&gt; Integer controlled step position information (ISC)
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ControlIsc {
    /// Service parameter that determines the control activity.
    #[prost(int32, tag="1")]
    pub ctl_val: i32,
}
/// Generic control message info.
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct MessageInfo {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub identified_object: ::std::option::Option<IdentifiedObject>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub message_time_stamp: ::std::option::Option<Timestamp>,
}
/// Generic control message info.
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ControlMessageInfo {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub message_info: ::std::option::Option<MessageInfo>,
}
/// Controllable single point (SPC)
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ControlSpc {
    /// Service parameter that determines the control activity ('false' for off or deactivation, 'true'
    /// for on or activation).
    #[prost(bool, tag="1")]
    pub ctl_val: bool,
}
/// The value of a control command which could either be a setpoint or a control schedule in curve.
/// The attribute modBlk is used to tag out a device. if it is TRUE, any setpoints and control schedule
/// in a message payload should be ignored.   It should also be presented in a status profile.  Any
/// modBlk value change (i.e. TRUE to FALSE and vice versa) should trigger an event.
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ControlValue {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub identified_object: ::std::option::Option<IdentifiedObject>,
    /// The attribute modBlk is used to tag out a device. if it is TRUE, any setpoints and control
    /// schedule in a message payload should be ignored.   It should also be presented in a status profile.
    /// Any modBlk value change (i.e. TRUE to FALSE and vice versa) should trigger an event.
    #[prost(message, optional, tag="2")]
    pub mod_blk: ::std::option::Option<bool>,
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
/// Generic user of energy - a  point of consumption on the power system model.
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct EnergyConsumer {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub conducting_equipment: ::std::option::Option<ConductingEquipment>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub operating_limit: ::std::option::Option<::std::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct OptionalCalcMethodKind {
    #[prost(enumeration="CalcMethodKind", tag="1")]
    pub value: i32,
}
/// Calc method kind
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct EngCalcMethodKind {
    /// The value of the status setting.
    #[prost(enumeration="CalcMethodKind", tag="1")]
    pub set_val: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct OptionalGridConnectModeKind {
    #[prost(enumeration="GridConnectModeKind", tag="1")]
    pub value: i32,
}
/// Grid connect mode kind
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct EngGridConnectModeKind {
    /// The value of the status setting.
    #[prost(enumeration="GridConnectModeKind", tag="1")]
    pub set_val: i32,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub set_val_extension: ::std::option::Option<::std::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct OptionalPfSignKind {
    #[prost(enumeration="PfSignKind", tag="1")]
    pub value: i32,
}
/// Enumerated status setting (FC=SP) (ENG_SP)
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct EngPfSignKind {
    /// The value of the status setting.
    #[prost(enumeration="PfSignKind", tag="1")]
    pub set_val: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct OptionalBehaviourModeKind {
    #[prost(enumeration="BehaviourModeKind", tag="1")]
    pub value: i32,
}
/// Behavior mode kind. ENS stands for Enumerated status
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct EnsBehaviourModeKind {
    /// Quality of the value in 'stVal'.
    #[prost(message, optional, tag="1")]
    pub q: ::std::option::Option<Quality>,
    /// Value of the data.
    #[prost(enumeration="BehaviourModeKind", tag="2")]
    pub st_val: i32,
    /// Timestamp of the last change or update event of 'stVal' or the last change of value in 'q'.
    #[prost(message, optional, tag="3")]
    pub t: ::std::option::Option<Timestamp>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct OptionalDerGeneratorStateKind {
    #[prost(enumeration="DerGeneratorStateKind", tag="1")]
    pub value: i32,
}
/// DER generation state kind. ENS stands for Enumerated status
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct EnsDerGeneratorStateKind {
    /// Quality of the value in 'stVal'.
    #[prost(message, optional, tag="1")]
    pub q: ::std::option::Option<Quality>,
    /// Value of the data.
    #[prost(enumeration="DerGeneratorStateKind", tag="2")]
    pub st_val: i32,
    /// Timestamp of the last change or update event of 'stVal' or the last change of value in 'q'.
    #[prost(message, optional, tag="3")]
    pub t: ::std::option::Option<Timestamp>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct OptionalDynamicTestKind {
    #[prost(enumeration="DynamicTestKind", tag="1")]
    pub value: i32,
}
/// Dynamic test kind. ENS stands for Enumerated status
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct EnsDynamicTestKind {
    /// Quality of the value in 'stVal'.
    #[prost(message, optional, tag="1")]
    pub q: ::std::option::Option<Quality>,
    /// Value of the data.
    #[prost(enumeration="DynamicTestKind", tag="2")]
    pub st_val: i32,
    /// Timestamp of the last change or update event of 'stVal' or the last change of value in 'q'.
    #[prost(message, optional, tag="3")]
    pub t: ::std::option::Option<Timestamp>,
}
/// Grid connect event &amp; status mode kind
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct EnsGridConnectModeKind {
    /// Actual Grid Connection Mode
    #[prost(enumeration="GridConnectModeKind", tag="1")]
    pub st_val: i32,
    /// MISSING DOCUMENTATION!!!
    #[prost(string, tag="2")]
    pub st_val_extension: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct OptionalHealthKind {
    #[prost(enumeration="HealthKind", tag="1")]
    pub value: i32,
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
    #[prost(enumeration="HealthKind", tag="2")]
    pub st_val: i32,
}
/// MISSING DOCUMENTATION!!!
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Ess {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub conducting_equipment: ::std::option::Option<ConductingEquipment>,
}
/// Generic event message information
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct EventMessageInfo {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub message_info: ::std::option::Option<MessageInfo>,
}
/// Event value
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct EventValue {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub identified_object: ::std::option::Option<IdentifiedObject>,
}
/// The source where a forecast value is issued.
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ForecastValueSource {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub identified_object: ::std::option::Option<IdentifiedObject>,
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
    #[prost(message, optional, tag="1")]
    pub forecast_value_source: ::std::option::Option<ForecastValueSource>,
    /// For control, this is an application ID, unique within communication system, and if the message
    /// is transformed between gateway the original source application ID should be kept.
    #[prost(string, tag="2")]
    pub source_application_id: std::string::String,
    /// Message publication date time
    #[prost(int64, tag="3")]
    pub source_date_time: i64,
}
/// Forecast value
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ForecastValue {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub identified_object: ::std::option::Option<IdentifiedObject>,
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
    #[prost(message, optional, tag="1")]
    pub identified_object: ::std::option::Option<IdentifiedObject>,
}
/// <<statistics>> Integer status (INS)
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct StatusIns {
    /// Quality of the value in 'stVal'.
    #[prost(message, optional, tag="1")]
    pub q: ::std::option::Option<Quality>,
    /// Value of the data.
    #[prost(int32, tag="2")]
    pub st_val: i32,
    /// Timestamp of the last change or update event of 'stVal' or the last change of value in 'q'.
    #[prost(message, optional, tag="3")]
    pub t: ::std::option::Option<Timestamp>,
    /// Units for: 'stVal', 'subVal'.
    #[prost(message, optional, tag="4")]
    pub units: ::std::option::Option<Unit>,
}
/// Status expressed in integer based on IEC61850 GGIO.
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct IntegerStatusGgio {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub logical_node: ::std::option::Option<LogicalNode>,
    /// Generic integer status input <i>n</i>.
    #[prost(message, optional, tag="2")]
    pub int_in: ::std::option::Option<StatusIns>,
    /// Phase code
    #[prost(message, optional, tag="3")]
    pub phase: ::std::option::Option<OptionalPhaseCodeKind>,
}
/// Logical node for event and status
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct LogicalNodeForEventAndStatus {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub logical_node: ::std::option::Option<LogicalNode>,
    /// Behavior of the function
    #[prost(message, optional, tag="2")]
    pub beh: ::std::option::Option<EnsBehaviourModeKind>,
    /// Asset health
    #[prost(message, optional, tag="3")]
    pub ee_health: ::std::option::Option<EnsHealthKind>,
}
/// The current state for a measurement. A state value is an instance of a measurement from a
/// specific source. Measurements can be associated with many state values, each representing a
/// different source for the measurement.
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct MeasurementValue {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub identified_object: ::std::option::Option<IdentifiedObject>,
}
/// Physical asset that performs the metering role of the usage point. Used for measuring
/// consumption and detection of events.
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Meter {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub conducting_equipment: ::std::option::Option<ConductingEquipment>,
}
/// Generic event message information
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct OptimizationMessageInfo {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub message_info: ::std::option::Option<MessageInfo>,
}
/// Specialized 61850 MMTN LN class
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ReadingMmtn {
    /// UML inherited base object
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
/// Generic reading message information
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ReadingMessageInfo {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub message_info: ::std::option::Option<MessageInfo>,
}
/// Specialized 61850 MMTR class
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ReadingMmtr {
    /// UML inherited base object
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
/// Specialized 61850 MMXU LN class
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ReadingMmxu {
    /// UML inherited base object
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
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct OptionalDbPosKind {
    #[prost(enumeration="DbPosKind", tag="1")]
    pub value: i32,
}
/// Specialized 61850 DPS class
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct StatusDps {
    /// Quality of the value in 'stVal'.
    #[prost(message, optional, tag="1")]
    pub q: ::std::option::Option<Quality>,
    /// Status value of the controllable data object.
    #[prost(enumeration="DbPosKind", tag="2")]
    pub st_val: i32,
    /// Timestamp of the last change of the value in any of 'stVal' or 'q'.
    #[prost(message, optional, tag="3")]
    pub t: ::std::option::Option<Timestamp>,
}
/// OpenFMB specialization for breaker, recloser and switch status and event profiles:  LN: Circuit
/// breaker   Name: XCBR
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct StatusAndEventXcbr {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub logical_node_for_event_and_status: ::std::option::Option<LogicalNodeForEventAndStatus>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub dynamic_test: ::std::option::Option<EnsDynamicTestKind>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub pos: ::std::option::Option<StatusDps>,
}
/// &lt;&lt;statistics&gt;&gt; Integer controlled step position information (ISC)
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct StatusIsc {
    /// Quality of the value in 'valWTr'.
    #[prost(message, optional, tag="1")]
    pub q: ::std::option::Option<Quality>,
    /// Status value
    #[prost(int32, tag="2")]
    pub st_val: i32,
    /// Timestamp of the last change of the value in any of 'valWTr' or 'q'.
    #[prost(message, optional, tag="3")]
    pub t: ::std::option::Option<Timestamp>,
}
/// Generic status message information
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct StatusMessageInfo {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub message_info: ::std::option::Option<MessageInfo>,
}
/// Controllable single point (SPC)
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct StatusSpc {
    /// Quality of the value in 'stVal'.
    #[prost(message, optional, tag="1")]
    pub q: ::std::option::Option<Quality>,
    /// Status value of the controllable data object.
    #[prost(bool, tag="2")]
    pub st_val: bool,
    /// Timestamp of the last change of the value in any of 'stVal' or 'q'.
    #[prost(message, optional, tag="3")]
    pub t: ::std::option::Option<Timestamp>,
}
/// Status value
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct StatusValue {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub identified_object: ::std::option::Option<IdentifiedObject>,
}
/// Visible string status (VSS)
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Vss {
    /// Quality of the value in 'stVal'.
    #[prost(message, optional, tag="1")]
    pub q: ::std::option::Option<Quality>,
    /// Value of the data.
    #[prost(string, tag="2")]
    pub st_val: std::string::String,
    /// Timestamp of the last change of the value in any of 'stVal' or 'q'.
    #[prost(message, optional, tag="3")]
    pub t: ::std::option::Option<Timestamp>,
}
/// LN: Generic process I/O   Name: GGIO
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct StringStatusGgio {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub logical_node: ::std::option::Option<LogicalNode>,
    /// Phase code
    #[prost(message, optional, tag="2")]
    pub phase: ::std::option::Option<OptionalPhaseCodeKind>,
    /// String status
    #[prost(message, optional, tag="3")]
    pub str_in: ::std::option::Option<Vss>,
}
/// Point definition (Point)
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SwitchPoint {
    /// Switch position
    #[prost(message, optional, tag="1")]
    pub pos: ::std::option::Option<ControlDpc>,
    /// Start time
    #[prost(message, optional, tag="2")]
    pub start_time: ::std::option::Option<ControlTimestamp>,
}
/// Curve shape setting (FC=SP) (CSG_SP)
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SwitchCsg {
    /// The array with the points specifying a curve shape.
    #[prost(message, repeated, tag="1")]
    pub crv_pts: ::std::vec::Vec<SwitchPoint>,
}
/// OpenFMB specialization for control schedule using:  LN: Schedule   Name: FSCH
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SwitchControlScheduleFsch {
    /// Discrete value in SwitchCSG type
    #[prost(message, optional, tag="1")]
    pub val_dcsg: ::std::option::Option<SwitchCsg>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct OptionalStateKind {
    #[prost(enumeration="StateKind", tag="1")]
    pub value: i32,
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
