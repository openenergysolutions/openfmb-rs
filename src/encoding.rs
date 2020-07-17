/// A MessageEncoding describes error types associated with a particular
/// encoding (such as protocol buffers). It also typifies the various ways a
/// message may be encoded such that a MessageBus may use any MessageEncoding.
pub trait MessageEncoding: std::fmt::Debug {
    type EncodeError: Send + std::error::Error;
    type DecodeError: Send + std::error::Error;
}

#[cfg(feature = "protobufs")]
mod protobufs;

#[cfg(feature = "protobufs")]
pub use protobufs::ProtobufEncoding;
