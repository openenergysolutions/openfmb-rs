// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

use lazy_static::lazy_static;
use prost::Message;
use proto_types::FileDescriptorSet;

pub static OPENFMB_DESCRIPTORS_BUF: &'static [u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/openfmb_descriptors.pb"));

lazy_static! {
    pub static ref OPENFMB_DESCRIPTORS: FileDescriptorSet =
        { FileDescriptorSet::decode(OPENFMB_DESCRIPTORS_BUF).unwrap() };
}
