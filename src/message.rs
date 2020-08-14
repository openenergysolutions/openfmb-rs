use crate::encoding::MessageEncoding;
use bytes::{Buf, BufMut};

/// A Message provides functionality to encode/decode (serialize/deserialize)
/// into something that acts like the Buf or BufMut interfaces provided by
/// the bytes crate such as using the prost library to encode into protocol
/// buffers.
pub trait Message<M: MessageEncoding>: Sized + Send {
    fn encode<B: BufMut>(&self, buf: &mut B) -> Result<(), M::EncodeError>;
    fn decode<B: Buf>(buf: B) -> Result<Self, M::DecodeError>;
}


