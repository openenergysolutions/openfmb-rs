use snafu::Snafu;

/// OpenFMBError type provides a very specific cause for a failure
///
/// This is useful over the typical Option<T> in many scenarios as a deeply
/// nested Option<T> may fail at one of many levels of Option<T> unwrapping,
/// instead we provide a Result<T, E> with an error that self describes what was missing.
#[derive(Debug, Snafu)]
#[snafu(visibility = "pub(crate)")]
pub enum OpenFMBError {
    #[snafu(display("Unsupported OpenFMBProfile"))]
    UnsupportedOpenFMBProfileError {
        profile: String,
    },
    #[snafu(display("Unsupported OpenFMB type"))]
    UnsupportedOpenFMBTypeError {
        fmb_type: String,
    },
    NoProtectedSwitch,
    NoDiscreteBreaker,
    NoConductingEquipment,
    NoReadingMessageInfo,
    NoControlMessageInfo,
    NoMessageInfo,
    NoIdentifiedObject,
    NoLogicalNode,
    NoLogicalControlNode,
    NoIED,
    InvalidOpenFMBMessage,
    NoMRID,
    NoStatusMessageInfo,
    NoMessageTimestamp,
    NoMeter,
    NoSolarInverter,
    NoEnergyConsumer,
    NoRegulatorSystem,
    NoEss,
    NoGenerationStatus,
    NoGenerationStatusZGen,
    NoRecloser,
    NoControlValue,
    NoShuntSystem,
    NoBreaker,
    NoName,
    NoPos,
    NoLoadControl,
    NoNamedObject,
    NoGeneratingUnit,
    NoSolarStatus,
    NoSolarStatusZGen,
    NoSolarEventAndStatusZGen,
    NoPointStatus,
    NoState,
    NoBreakerDiscreteControl,
    NoBreakerDiscreteControlXcbr,
    NoDiscreteControlXcbr,
    NoBreakerStatus,
    NoPhs3,
    NoStatusAndEventXcbr,
    NoEssStatus,
    NoEssStatusZBat,
    NoEssStatusZGen,
    NoEssEventAndStatusZGen,
    NoMode,
    NoSoc,
    NoEssReading,
    NoReadingMmxu,
    NoW,
    NoNet,
    NoCVal,
    NoMag,
    NoF,
    NoLogicalNodeForEventAndStatus,
    NoEeHealth,
    NoBeh,
    #[snafu(display("Actor System Error"))]
    IOError {
        source: std::io::Error,
    },
    UuidError {
        source: uuid::Error,
    },
}

/// OpenFMBExt Result Type Alias
pub type OpenFMBResult<T> = std::result::Result<T, OpenFMBError>;