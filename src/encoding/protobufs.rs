// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

use crate::prelude::*;
use bytes::{Buf, BufMut};
use std::str::FromStr;

/// Sadly we need a type alias until specialization lands in stable
/// to do what we really want, which is create another impl Message for the variant ProfileMessage type
///
/// So instead this alias is used for the time being. This forces *two* buses to be created if you wish to have type specific
/// subscribes/publishes or use the variant containing ProfileMessage type
///
/// TODO: remove this type alias when specialization lands
#[derive(Debug, Clone)]
pub enum ProtobufVariantEncoding {}

#[derive(Debug, Clone)]
pub enum ProtobufVariantDecodeError {
    ProstDecodeError(prost::DecodeError),
    InvalidTopic,
    InvalidProfile(String),
}

impl std::fmt::Display for ProtobufVariantDecodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for ProtobufVariantDecodeError {}

impl MessageEncoding for ProtobufVariantEncoding {
    type DecodeError = ProtobufVariantDecodeError;
    type EncodeError = prost::EncodeError;
}

/// Protobuf encoding for the ProfileMessage variant enum
impl Message<ProtobufVariantEncoding> for openfmb_messages::ProfileMessage {
    fn encode<B: BufMut>(&self, buf: &mut B) -> Result<(), prost::EncodeError> {
        openfmb_messages::ProfileMessage::encode(self, buf)
    }

    fn decode<S: AsRef<str>, T: Topic<S>, B: Buf>(
        mut topic: T,
        buf: B,
    ) -> Result<openfmb_messages::ProfileMessage, ProtobufVariantDecodeError> {
        // maybe decode and validate the topic here into a ProfileTopic instead?
        let profile = if let Some(last) = topic.nth(2) {
            match last {
                TopicLevel::Exact(profile_str) => {
                    Profile::from_str(profile_str.as_ref()).map_err(|_err| {
                        ProtobufVariantDecodeError::InvalidProfile(format!(
                            "{:?}",
                            profile_str.as_ref()
                        ))
                    })?
                }
                TopicLevel::WildCard => return Err(ProtobufVariantDecodeError::InvalidProfile(
                    "Wildcard given for last topic level, not convertable to an OpenFMB Profile"
                        .to_string(),
                )),
            }
        } else {
            // NOTE uses a hidden API here
            return Err(ProtobufVariantDecodeError::InvalidTopic);
        };
        openfmb_messages::ProfileMessage::decode(profile, buf)
            .map_err(ProtobufVariantDecodeError::ProstDecodeError)
    }
}

/// ProtobufEncoding (zero sized, non-constructable type)
#[derive(Debug, Clone)]
pub enum ProtobufEncoding {}

impl MessageEncoding for ProtobufEncoding {
    type DecodeError = prost::DecodeError;
    type EncodeError = prost::EncodeError;
}

impl<M: prost::Message + Default> Message<ProtobufEncoding> for M {
    fn encode<B: BufMut>(&self, buf: &mut B) -> Result<(), prost::EncodeError> {
        M::encode(self, buf)
    }

    fn decode<S: AsRef<str>, T: Topic<S>, B: Buf>(
        _topic: T,
        buf: B,
    ) -> Result<M, prost::DecodeError> {
        M::decode(buf)
    }
}
