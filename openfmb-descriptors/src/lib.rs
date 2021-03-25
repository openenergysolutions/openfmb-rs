use prost::Message;
use proto_types::FileDescriptorSet;
use lazy_static::lazy_static;

pub static OPENFMB_DESCRIPTORS_BUF: &'static [u8] = include_bytes!(concat!(env!("OUT_DIR"), "/openfmb_descriptors.pb"));

lazy_static! {
    pub static ref OPENFMB_DESCRIPTORS: FileDescriptorSet = {
        FileDescriptorSet::decode(OPENFMB_DESCRIPTORS_BUF).unwrap()
    };
}
