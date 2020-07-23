use futures::stream::BoxStream;
use std::error::Error;
use std::fmt;
use std::time::Duration;

/// A common publish error type with erased enclosed error types
#[derive(Debug)]
pub enum PublishError {
    IoError(std::io::Error),
    EncodeError(Box<dyn Error>),
    BusError(Box<dyn Error>),
}

impl fmt::Display for PublishError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for PublishError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            PublishError::IoError(ref err) => Some(err),
            PublishError::EncodeError(ref err) => Some(&**err),
            PublishError::BusError(ref err) => Some(&**err),
        }
    }
}

impl From<std::io::Error> for PublishError {
    fn from(err: std::io::Error) -> PublishError {
        PublishError::IoError(err)
    }
}

/// Type alias for a publish result
pub type PublishResult<T, E = PublishError> = Result<T, E>;

/// A common publish error type with erased enclosed error types, useful
/// for matching on and correcting issues potentially.
#[derive(Debug)]
pub enum SubscriptionError {
    IoError(std::io::Error),
    DecodeError(Box<(dyn Error + Send)>),
    BusError(Box<(dyn Error + Send)>),
    Unsubscribed,
}

impl fmt::Display for SubscriptionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for SubscriptionError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            SubscriptionError::IoError(ref err) => Some(err),
            SubscriptionError::DecodeError(ref err) => Some(&**err),
            SubscriptionError::BusError(ref err) => Some(&**err),
            _ => None,
        }
    }
}

impl From<std::io::Error> for SubscriptionError {
    fn from(err: std::io::Error) -> SubscriptionError {
        SubscriptionError::IoError(err)
    }
}

/// A common publish error type with erased enclosed error types, useful
/// for matching on and correcting issues potentially.
#[derive(Debug)]
pub enum SubscribeError {
    IoError(std::io::Error),
    BusError(Box<dyn Error>),
    InvalidTopic(String),
}

impl fmt::Display for SubscribeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for SubscribeError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            SubscribeError::IoError(ref err) => Some(err),
            SubscribeError::BusError(ref err) => Some(&**err),
            _ => None,
        }
    }
}

impl From<std::io::Error> for SubscribeError {
    fn from(err: std::io::Error) -> SubscribeError {
        SubscribeError::IoError(err)
    }
}

/// Type alias for the subscription stream
pub type Subscription<T> = BoxStream<'static, Result<T, SubscriptionError>>;

/// Type alias for a publish result
pub type SubscribeResult<T, E = SubscribeError> = Result<Subscription<T>, E>;

/// A commonly useful Error type for control functions
#[derive(Debug)]
pub enum ControlError {
    PublishError(PublishError),
    SubscribeError(SubscribeError),
    SubscriptionError(SubscriptionError),
    RetryError(usize),
    TimeoutError(Duration),
}

impl fmt::Display for ControlError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for ControlError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            ControlError::PublishError(ref err) => Some(err),
            ControlError::SubscribeError(ref err) => Some(err),
            ControlError::SubscriptionError(ref err) => Some(err),
            _ => None,
        }
    }
}

impl From<PublishError> for ControlError {
    fn from(err: PublishError) -> ControlError {
        ControlError::PublishError(err)
    }
}

impl From<SubscribeError> for ControlError {
    fn from(err: SubscribeError) -> ControlError {
        ControlError::SubscribeError(err)
    }
}

impl From<SubscriptionError> for ControlError {
    fn from(err: SubscriptionError) -> ControlError {
        ControlError::SubscriptionError(err)
    }
}

/// Type alias for a control result
pub type ControlResult<T, E = ControlError> = Result<T, E>;
