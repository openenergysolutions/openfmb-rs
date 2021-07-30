// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

use crate::prelude::*;
use bytes::{Buf, BufMut};

/// ProtobufEncoding (zero sized, non-constructable type)
#[derive(Debug, Clone)]
pub enum ProtobufEncoding {}

impl MessageEncoding for ProtobufEncoding {
    type DecodeError = prost::DecodeError;
    type EncodeError = prost::EncodeError;
}

impl<M: prost::Message + Default> Message<ProtobufEncoding> for M {
    fn encode<B: BufMut>(self: &M, buf: &mut B) -> Result<(), prost::EncodeError> {
        M::encode(self, buf)
    }

    fn decode<S: AsRef<str>, T: Topic<S>, B: Buf>(
        _topic: T,
        buf: B,
    ) -> Result<M, prost::DecodeError> {
        M::decode(buf)
    }
}

//TODO create an enum with all possible message variants and decode the appropriate variant based on topic
