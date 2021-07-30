// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

use crate::encoding::MessageEncoding;
use crate::topic::Topic;
use bytes::{Buf, BufMut};

/// A Message provides functionality to encode/decode (serialize/deserialize)
/// into something that acts like the Buf or BufMut interfaces provided by
/// the bytes crate such as using the prost library to encode into protocol
/// buffers.
///
/// Since the types may vary of over pub/sub topics we allow for an enum type
/// result using the given topic the message was published on.
pub trait Message<E: MessageEncoding>: Sized + Send {
    fn encode<B: BufMut>(&self, buf: &mut B) -> Result<(), E::EncodeError>;
    fn decode<S: AsRef<str>, T: Topic<S>, B: Buf>(topic: T, buf: B)
        -> Result<Self, E::DecodeError>;
}
