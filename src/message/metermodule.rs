// This file is generated by rust-protobuf 2.17.0. Do not edit
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![rustfmt::skip]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_imports)]
#![allow(unused_results)]
//! Generated file from `metermodule/metermodule.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_17_0;

#[derive(PartialEq,Clone,Default)]
#[cfg_attr(feature = "with-serde", derive(Serialize, Deserialize))]
pub struct MeterReading {
    // message fields
    conductingEquipmentTerminalReading: ::protobuf::SingularPtrField<super::commonmodule::ConductingEquipmentTerminalReading>,
    phaseMMTN: ::protobuf::SingularPtrField<super::commonmodule::PhaseMMTN>,
    readingMMTR: ::protobuf::SingularPtrField<super::commonmodule::ReadingMMTR>,
    readingMMXU: ::protobuf::SingularPtrField<super::commonmodule::ReadingMMXU>,
    // special fields
    #[cfg_attr(feature = "with-serde", serde(skip))]
    pub unknown_fields: ::protobuf::UnknownFields,
    #[cfg_attr(feature = "with-serde", serde(skip))]
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a MeterReading {
    fn default() -> &'a MeterReading {
        <MeterReading as ::protobuf::Message>::default_instance()
    }
}

impl MeterReading {
    pub fn new() -> MeterReading {
        ::std::default::Default::default()
    }

    // .commonmodule.ConductingEquipmentTerminalReading conductingEquipmentTerminalReading = 1;


    pub fn get_conductingEquipmentTerminalReading(&self) -> &super::commonmodule::ConductingEquipmentTerminalReading {
        self.conductingEquipmentTerminalReading.as_ref().unwrap_or_else(|| <super::commonmodule::ConductingEquipmentTerminalReading as ::protobuf::Message>::default_instance())
    }
    pub fn clear_conductingEquipmentTerminalReading(&mut self) {
        self.conductingEquipmentTerminalReading.clear();
    }

    pub fn has_conductingEquipmentTerminalReading(&self) -> bool {
        self.conductingEquipmentTerminalReading.is_some()
    }

    // Param is passed by value, moved
    pub fn set_conductingEquipmentTerminalReading(&mut self, v: super::commonmodule::ConductingEquipmentTerminalReading) {
        self.conductingEquipmentTerminalReading = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_conductingEquipmentTerminalReading(&mut self) -> &mut super::commonmodule::ConductingEquipmentTerminalReading {
        if self.conductingEquipmentTerminalReading.is_none() {
            self.conductingEquipmentTerminalReading.set_default();
        }
        self.conductingEquipmentTerminalReading.as_mut().unwrap()
    }

    // Take field
    pub fn take_conductingEquipmentTerminalReading(&mut self) -> super::commonmodule::ConductingEquipmentTerminalReading {
        self.conductingEquipmentTerminalReading.take().unwrap_or_else(|| super::commonmodule::ConductingEquipmentTerminalReading::new())
    }

    // .commonmodule.PhaseMMTN phaseMMTN = 2;


    pub fn get_phaseMMTN(&self) -> &super::commonmodule::PhaseMMTN {
        self.phaseMMTN.as_ref().unwrap_or_else(|| <super::commonmodule::PhaseMMTN as ::protobuf::Message>::default_instance())
    }
    pub fn clear_phaseMMTN(&mut self) {
        self.phaseMMTN.clear();
    }

    pub fn has_phaseMMTN(&self) -> bool {
        self.phaseMMTN.is_some()
    }

    // Param is passed by value, moved
    pub fn set_phaseMMTN(&mut self, v: super::commonmodule::PhaseMMTN) {
        self.phaseMMTN = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_phaseMMTN(&mut self) -> &mut super::commonmodule::PhaseMMTN {
        if self.phaseMMTN.is_none() {
            self.phaseMMTN.set_default();
        }
        self.phaseMMTN.as_mut().unwrap()
    }

    // Take field
    pub fn take_phaseMMTN(&mut self) -> super::commonmodule::PhaseMMTN {
        self.phaseMMTN.take().unwrap_or_else(|| super::commonmodule::PhaseMMTN::new())
    }

    // .commonmodule.ReadingMMTR readingMMTR = 3;


    pub fn get_readingMMTR(&self) -> &super::commonmodule::ReadingMMTR {
        self.readingMMTR.as_ref().unwrap_or_else(|| <super::commonmodule::ReadingMMTR as ::protobuf::Message>::default_instance())
    }
    pub fn clear_readingMMTR(&mut self) {
        self.readingMMTR.clear();
    }

    pub fn has_readingMMTR(&self) -> bool {
        self.readingMMTR.is_some()
    }

    // Param is passed by value, moved
    pub fn set_readingMMTR(&mut self, v: super::commonmodule::ReadingMMTR) {
        self.readingMMTR = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_readingMMTR(&mut self) -> &mut super::commonmodule::ReadingMMTR {
        if self.readingMMTR.is_none() {
            self.readingMMTR.set_default();
        }
        self.readingMMTR.as_mut().unwrap()
    }

    // Take field
    pub fn take_readingMMTR(&mut self) -> super::commonmodule::ReadingMMTR {
        self.readingMMTR.take().unwrap_or_else(|| super::commonmodule::ReadingMMTR::new())
    }

    // .commonmodule.ReadingMMXU readingMMXU = 4;


    pub fn get_readingMMXU(&self) -> &super::commonmodule::ReadingMMXU {
        self.readingMMXU.as_ref().unwrap_or_else(|| <super::commonmodule::ReadingMMXU as ::protobuf::Message>::default_instance())
    }
    pub fn clear_readingMMXU(&mut self) {
        self.readingMMXU.clear();
    }

    pub fn has_readingMMXU(&self) -> bool {
        self.readingMMXU.is_some()
    }

    // Param is passed by value, moved
    pub fn set_readingMMXU(&mut self, v: super::commonmodule::ReadingMMXU) {
        self.readingMMXU = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_readingMMXU(&mut self) -> &mut super::commonmodule::ReadingMMXU {
        if self.readingMMXU.is_none() {
            self.readingMMXU.set_default();
        }
        self.readingMMXU.as_mut().unwrap()
    }

    // Take field
    pub fn take_readingMMXU(&mut self) -> super::commonmodule::ReadingMMXU {
        self.readingMMXU.take().unwrap_or_else(|| super::commonmodule::ReadingMMXU::new())
    }
}

impl ::protobuf::Message for MeterReading {
    fn is_initialized(&self) -> bool {
        for v in &self.conductingEquipmentTerminalReading {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.phaseMMTN {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.readingMMTR {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.readingMMXU {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.conductingEquipmentTerminalReading)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.phaseMMTN)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.readingMMTR)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.readingMMXU)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.conductingEquipmentTerminalReading.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.phaseMMTN.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.readingMMTR.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.readingMMXU.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.conductingEquipmentTerminalReading.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.phaseMMTN.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.readingMMTR.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.readingMMXU.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> MeterReading {
        MeterReading::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::commonmodule::ConductingEquipmentTerminalReading>>(
                "conductingEquipmentTerminalReading",
                |m: &MeterReading| { &m.conductingEquipmentTerminalReading },
                |m: &mut MeterReading| { &mut m.conductingEquipmentTerminalReading },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::commonmodule::PhaseMMTN>>(
                "phaseMMTN",
                |m: &MeterReading| { &m.phaseMMTN },
                |m: &mut MeterReading| { &mut m.phaseMMTN },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::commonmodule::ReadingMMTR>>(
                "readingMMTR",
                |m: &MeterReading| { &m.readingMMTR },
                |m: &mut MeterReading| { &mut m.readingMMTR },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::commonmodule::ReadingMMXU>>(
                "readingMMXU",
                |m: &MeterReading| { &m.readingMMXU },
                |m: &mut MeterReading| { &mut m.readingMMXU },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<MeterReading>(
                "MeterReading",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static MeterReading {
        static instance: ::protobuf::rt::LazyV2<MeterReading> = ::protobuf::rt::LazyV2::INIT;
        instance.get(MeterReading::new)
    }
}

impl ::protobuf::Clear for MeterReading {
    fn clear(&mut self) {
        self.conductingEquipmentTerminalReading.clear();
        self.phaseMMTN.clear();
        self.readingMMTR.clear();
        self.readingMMXU.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for MeterReading {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MeterReading {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
#[cfg_attr(feature = "with-serde", derive(Serialize, Deserialize))]
pub struct MeterReadingProfile {
    // message fields
    readingMessageInfo: ::protobuf::SingularPtrField<super::commonmodule::ReadingMessageInfo>,
    ied: ::protobuf::SingularPtrField<super::commonmodule::IED>,
    meter: ::protobuf::SingularPtrField<super::commonmodule::Meter>,
    meterReading: ::protobuf::SingularPtrField<MeterReading>,
    // special fields
    #[cfg_attr(feature = "with-serde", serde(skip))]
    pub unknown_fields: ::protobuf::UnknownFields,
    #[cfg_attr(feature = "with-serde", serde(skip))]
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a MeterReadingProfile {
    fn default() -> &'a MeterReadingProfile {
        <MeterReadingProfile as ::protobuf::Message>::default_instance()
    }
}

impl MeterReadingProfile {
    pub fn new() -> MeterReadingProfile {
        ::std::default::Default::default()
    }

    // .commonmodule.ReadingMessageInfo readingMessageInfo = 1;


    pub fn get_readingMessageInfo(&self) -> &super::commonmodule::ReadingMessageInfo {
        self.readingMessageInfo.as_ref().unwrap_or_else(|| <super::commonmodule::ReadingMessageInfo as ::protobuf::Message>::default_instance())
    }
    pub fn clear_readingMessageInfo(&mut self) {
        self.readingMessageInfo.clear();
    }

    pub fn has_readingMessageInfo(&self) -> bool {
        self.readingMessageInfo.is_some()
    }

    // Param is passed by value, moved
    pub fn set_readingMessageInfo(&mut self, v: super::commonmodule::ReadingMessageInfo) {
        self.readingMessageInfo = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_readingMessageInfo(&mut self) -> &mut super::commonmodule::ReadingMessageInfo {
        if self.readingMessageInfo.is_none() {
            self.readingMessageInfo.set_default();
        }
        self.readingMessageInfo.as_mut().unwrap()
    }

    // Take field
    pub fn take_readingMessageInfo(&mut self) -> super::commonmodule::ReadingMessageInfo {
        self.readingMessageInfo.take().unwrap_or_else(|| super::commonmodule::ReadingMessageInfo::new())
    }

    // .commonmodule.IED ied = 2;


    pub fn get_ied(&self) -> &super::commonmodule::IED {
        self.ied.as_ref().unwrap_or_else(|| <super::commonmodule::IED as ::protobuf::Message>::default_instance())
    }
    pub fn clear_ied(&mut self) {
        self.ied.clear();
    }

    pub fn has_ied(&self) -> bool {
        self.ied.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ied(&mut self, v: super::commonmodule::IED) {
        self.ied = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ied(&mut self) -> &mut super::commonmodule::IED {
        if self.ied.is_none() {
            self.ied.set_default();
        }
        self.ied.as_mut().unwrap()
    }

    // Take field
    pub fn take_ied(&mut self) -> super::commonmodule::IED {
        self.ied.take().unwrap_or_else(|| super::commonmodule::IED::new())
    }

    // .commonmodule.Meter meter = 3;


    pub fn get_meter(&self) -> &super::commonmodule::Meter {
        self.meter.as_ref().unwrap_or_else(|| <super::commonmodule::Meter as ::protobuf::Message>::default_instance())
    }
    pub fn clear_meter(&mut self) {
        self.meter.clear();
    }

    pub fn has_meter(&self) -> bool {
        self.meter.is_some()
    }

    // Param is passed by value, moved
    pub fn set_meter(&mut self, v: super::commonmodule::Meter) {
        self.meter = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_meter(&mut self) -> &mut super::commonmodule::Meter {
        if self.meter.is_none() {
            self.meter.set_default();
        }
        self.meter.as_mut().unwrap()
    }

    // Take field
    pub fn take_meter(&mut self) -> super::commonmodule::Meter {
        self.meter.take().unwrap_or_else(|| super::commonmodule::Meter::new())
    }

    // .metermodule.MeterReading meterReading = 4;


    pub fn get_meterReading(&self) -> &MeterReading {
        self.meterReading.as_ref().unwrap_or_else(|| <MeterReading as ::protobuf::Message>::default_instance())
    }
    pub fn clear_meterReading(&mut self) {
        self.meterReading.clear();
    }

    pub fn has_meterReading(&self) -> bool {
        self.meterReading.is_some()
    }

    // Param is passed by value, moved
    pub fn set_meterReading(&mut self, v: MeterReading) {
        self.meterReading = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_meterReading(&mut self) -> &mut MeterReading {
        if self.meterReading.is_none() {
            self.meterReading.set_default();
        }
        self.meterReading.as_mut().unwrap()
    }

    // Take field
    pub fn take_meterReading(&mut self) -> MeterReading {
        self.meterReading.take().unwrap_or_else(|| MeterReading::new())
    }
}

impl ::protobuf::Message for MeterReadingProfile {
    fn is_initialized(&self) -> bool {
        for v in &self.readingMessageInfo {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.ied {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.meter {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.meterReading {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.readingMessageInfo)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.ied)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.meter)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.meterReading)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.readingMessageInfo.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.ied.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.meter.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.meterReading.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.readingMessageInfo.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.ied.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.meter.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.meterReading.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> MeterReadingProfile {
        MeterReadingProfile::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::commonmodule::ReadingMessageInfo>>(
                "readingMessageInfo",
                |m: &MeterReadingProfile| { &m.readingMessageInfo },
                |m: &mut MeterReadingProfile| { &mut m.readingMessageInfo },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::commonmodule::IED>>(
                "ied",
                |m: &MeterReadingProfile| { &m.ied },
                |m: &mut MeterReadingProfile| { &mut m.ied },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::commonmodule::Meter>>(
                "meter",
                |m: &MeterReadingProfile| { &m.meter },
                |m: &mut MeterReadingProfile| { &mut m.meter },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<MeterReading>>(
                "meterReading",
                |m: &MeterReadingProfile| { &m.meterReading },
                |m: &mut MeterReadingProfile| { &mut m.meterReading },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<MeterReadingProfile>(
                "MeterReadingProfile",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static MeterReadingProfile {
        static instance: ::protobuf::rt::LazyV2<MeterReadingProfile> = ::protobuf::rt::LazyV2::INIT;
        instance.get(MeterReadingProfile::new)
    }
}

impl ::protobuf::Clear for MeterReadingProfile {
    fn clear(&mut self) {
        self.readingMessageInfo.clear();
        self.ied.clear();
        self.meter.clear();
        self.meterReading.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for MeterReadingProfile {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MeterReadingProfile {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1dmetermodule/metermodule.proto\x12\x0bmetermodule\"\xd0\x02\n\x0cMe\
    terReading\x12\x86\x01\n\"conductingEquipmentTerminalReading\x18\x01\x20\
    \x01(\x0b20.commonmodule.ConductingEquipmentTerminalReadingR\"conducting\
    EquipmentTerminalReadingB\x04\x80\xb5\x18\x01\x127\n\tphaseMMTN\x18\x02\
    \x20\x01(\x0b2\x17.commonmodule.PhaseMMTNR\tphaseMMTNB\0\x12=\n\x0breadi\
    ngMMTR\x18\x03\x20\x01(\x0b2\x19.commonmodule.ReadingMMTRR\x0breadingMMT\
    RB\0\x12=\n\x0breadingMMXU\x18\x04\x20\x01(\x0b2\x19.commonmodule.Readin\
    gMMXUR\x0breadingMMXUB\0:\0\"\xa0\x02\n\x13MeterReadingProfile\x12V\n\
    \x12readingMessageInfo\x18\x01\x20\x01(\x0b2\x20.commonmodule.ReadingMes\
    sageInfoR\x12readingMessageInfoB\x04\x80\xb5\x18\x01\x12-\n\x03ied\x18\
    \x02\x20\x01(\x0b2\x11.commonmodule.IEDR\x03iedB\x08\x90\xb5\x18\x01\x88\
    \xb5\x18\x01\x123\n\x05meter\x18\x03\x20\x01(\x0b2\x13.commonmodule.Mete\
    rR\x05meterB\x08\x90\xb5\x18\x01\x88\xb5\x18\x01\x12G\n\x0cmeterReading\
    \x18\x04\x20\x01(\x0b2\x19.metermodule.MeterReadingR\x0cmeterReadingB\
    \x08\x88\xb5\x18\x01\x90\xb5\x18\x01:\x04\xc0\xf3\x18\x01B\0b\x06proto3\
";

static file_descriptor_proto_lazy: ::protobuf::rt::LazyV2<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::LazyV2::INIT;

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    file_descriptor_proto_lazy.get(|| {
        parse_descriptor_proto()
    })
}