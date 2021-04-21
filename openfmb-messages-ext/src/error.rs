// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

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
    NoMeterReading,
    NoSolarInverter,
    NoEnergyConsumer,
    NoRegulatorSystem,
    NoRegulatorStatus,
    NoRegulatorEvent,
    NoRegulatorControl,
    NoEss,
    NoGenerationStatus,
    NoGenerationEvent,
    NoGenerationStatusZGen,
    NoGenerationEventZGen,
    NoGenerationEventAndStatusZGen,
    NoGenerationControl,
    NoGenerationReading,
    NoRecloser,
    NoRecloserReading,
    NoRecloserStatus,
    NoRecloserEvent,
    NoControlValue,
    NoShuntSystem,
    NoBreaker,
    NoName,
    NoPos,
    NoLoadControl,
    NoLoadEvent,
    NoLoadEventZGld,
    NoLoadStatus,
    NoLoadStatusZGld,
    NoLoadEventAndStatusZGld,
    NoNamedObject,
    NoGeneratingUnit,
    NoSolarStatus,
    NoSolarEvent,
    NoSolarReading,
    NoSolarStatusZGen,
    NoSolarEventZGen,
    NoSolarEventAndStatusZGen,
    NoPointStatus,
    NoState,
    NoBreakerDiscreteControl,
    NoBreakerDiscreteControlXcbr,
    NoDiscreteControlXcbr,
    NoBreakerStatus,
    NoBreakerEvent,
    NoBreakerReading,
    NoPhs3,
    NoStatusAndEventXcbr,
    NoEssStatus,
    NoEssStatusZBat,
    NoEssStatusZGen,
    NoEssEventAndStatusZGen,
    NoEssEvent,
    NoEssEventZGen,
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
    NoEventMessageInfo,
    NoCapBankSystem,
    NoApplicationSystem,
    NoSwitchStatus,
    NoSwitchStatusXswi,
    NoSwitchEvent,
    NoSwitchEventXswi,
    NoSwitchReading,
    InvalidValue,
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
