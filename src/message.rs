use bytes::{Buf, BufMut};

/// A MessageEncoding type describes the error types associated with encoding
/// and decoding message. It also typifies the various ways a message may
/// be encoded such that a MessageBus may use any MessageEncoding to encode
/// messages into buffers.
pub trait MessageEncoding: 'static + Send {
    type EncodeError: Send;
    type DecodeError: Send;
}

/// A Message provide functionality to serialize/deserialize (encode/decode)
/// into something that acts like the Buf or BufMut interfaces provides by
/// the bytes crate such as using the prost library to encode into protocol
/// buffers.
pub trait Message<M: MessageEncoding>: Sized + Send {
    fn encode<B: BufMut>(&self, buf: &mut B) -> Result<(), M::EncodeError>;
    fn decode<B: Buf>(buf: B) -> Result<Self, M::DecodeError>;
}
