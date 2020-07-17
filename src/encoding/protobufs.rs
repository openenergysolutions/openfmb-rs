use crate::prelude::*;
use bytes::{Buf, BufMut};

/// ProtobufEncoding (zero sized, non-constructable type)
#[derive(Debug, Clone)]
pub enum ProtobufEncoding {}

impl MessageEncoding for ProtobufEncoding {
    type DecodeError = prost::DecodeError;
    type EncodeError = prost::EncodeError;
}

impl<T: prost::Message + Default> Message<ProtobufEncoding> for T {
    fn encode<B: BufMut>(self: &T, buf: &mut B) -> Result<(), prost::EncodeError> {
        T::encode(self, buf)
    }

    fn decode<B: Buf>(buf: B) -> Result<T, prost::DecodeError> {
        T::decode(buf)
    }
}
