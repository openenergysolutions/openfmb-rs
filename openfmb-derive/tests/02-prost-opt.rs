/// Validity of the value, as condensed information for the client. In case this value is not
/// 'good', some reasons may be found in the 'detailQual'.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
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

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OptionalTimeAccuracyKind {
    #[prost(enumeration = "TimeAccuracyKind", tag = "1")]
    pub value: i32,
}

/// Information about the quality of the time source of the sending IED.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TimeQuality {
    /// If true, the time source of the sending device is unreliable and the value of the time stamp
    /// shall be ignored.
    #[prost(bool, tag = "1")]
    pub clock_failure: bool,
    /// If true, the time source of the sending device is not synchronised with the external UTC time.
    #[prost(bool, tag = "2")]
    pub clock_not_synchronized: bool,
    /// If true, the value in 'P_Timestamp.SecondSinceEpoch' contains all leap seconds occurred.
    /// Otherwise, it does not take into account the leap seconds that occurred before the initialization of
    /// the time source of the device. Instead, the seconds since start of the epoch are calculated from the
    /// current date assuming a constant day length of 86 400 seconds. Note: If a UTC time master clock is
    /// used and accessible, this value should always be true.
    #[prost(bool, tag = "3")]
    pub leap_seconds_known: bool,
    /// Information about the quality of the time source of the sending IED.
    #[prost(enumeration = "TimeAccuracyKind", tag = "4")]
    pub time_accuracy: i32,
}

/// This is a root class to provide common identification for all classes needing identification and
/// naming attributes.
#[derive(Clone, PartialEq, ::prost::Message, ::openfmb_derive::OpenFMB)]
pub struct IdentifiedObject {
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag = "1")]
    pub description: ::std::option::Option<::std::string::String>,
    /// Master resource identifier issued by a model authority. The mRID must semantically be a UUID as
    /// specified in RFC 4122. The mRID is globally unique.
    #[prost(message, optional, tag = "2")]
    pub m_rid: ::std::option::Option<::std::string::String>,
    /// The name is any free human readable and possibly non unique text naming the object.
    #[prost(message, optional, tag = "3")]
    pub name: ::std::option::Option<::std::string::String>,
}

/// UTC time with the epoch of midnight (00:00:00) of 1970-01-01. The presentation is defined in the
/// SCSMs.The NULL time stamp has all fields set to 0 (zero).The relation between a timestamp value, the
/// synchronization of an internal time with an external time source (for example, UTC time), and other
/// information related to time model are available as requirements in Clause 21.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Timestamp {
    /// Fractional of the current second when the value of time stamp has been determined. It shall be
    /// calculated as (SUM from i=0 to 31 of bi*2**-(i+1)).
    #[prost(uint32, tag = "1")]
    pub fraction: u32,
    /// Second since epoch (1970-01-01T00:00:00Z)
    #[prost(uint64, tag = "2")]
    pub seconds: u64,
    /// IEC61850 time quality
    #[prost(message, optional, tag = "3")]
    pub tq: ::std::option::Option<TimeQuality>,
}

/// Generic control message info.
#[derive(Clone, PartialEq, ::prost::Message, ::openfmb_derive::OpenFMB)]
pub struct MessageInfo {
    /// UML inherited base object
    #[prost(message, optional, tag = "1")]
    #[openfmb(inherit)]
    pub identified_object: ::std::option::Option<IdentifiedObject>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag = "2")]
    pub message_time_stamp: ::std::option::Option<Timestamp>,
}

/// Generic control message info.
#[derive(Clone, PartialEq, ::prost::Message, ::openfmb_derive::OpenFMB)]
pub struct ControlMessageInfo {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    #[openfmb(inherit)]
    pub message_info: ::std::option::Option<MessageInfo>,
}

/// Generic event message information
#[derive(Clone, PartialEq, ::prost::Message, ::openfmb_derive::OpenFMB)]
pub struct EventMessageInfo {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    #[openfmb(inherit)]
    pub message_info: ::std::option::Option<MessageInfo>,
}

#[test]
fn test_default_accessors() {
    let cmi: ControlMessageInfo = Default::default();
    let default_string: String = Default::default();
    if *cmi.get_message_info().get_description() != default_string {
        panic!("Unexpectedly not equal!");
    }

    if *cmi.get_identified_object().get_description() != default_string {
        panic!("Unexpectedly not equal!");
    }

    let emi: EventMessageInfo = Default::default();
    let default_string: String = Default::default();
    if *emi.get_message_info().get_description() != default_string {
        panic!("Unexpectedly not equal!");
    }

    if *emi.get_identified_object().get_description() != default_string {
        panic!("Unexpectedly not equal!");
    }
}

fn main() {
}
