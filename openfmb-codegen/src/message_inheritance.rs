// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

use std::collections::{HashMap, HashSet};
use std::iter;

use itertools::Itertools;

use proto_types::field_descriptor_proto::Type;
use proto_types::{FieldDescriptorProto, FileDescriptorProto};

use crate::extern_paths::ExternPaths;
use crate::ident::{to_snake, to_upper_camel};

fn module(file: &FileDescriptorProto) -> Vec<String> {
    file.package()
        .split('.')
        .filter(|s| !s.is_empty())
        .map(to_snake)
        .collect()
}

fn field_type_ident(paths: &ExternPaths, package: &str, field: &FieldDescriptorProto) -> String {
    match field.r#type() {
        Type::Float => String::from("f32"),
        Type::Double => String::from("f64"),
        Type::Uint32 | Type::Fixed32 => String::from("u32"),
        Type::Uint64 | Type::Fixed64 => String::from("u64"),
        Type::Int32 | Type::Sfixed32 | Type::Sint32 | Type::Enum => String::from("i32"),
        Type::Int64 | Type::Sfixed64 | Type::Sint64 => String::from("i64"),
        Type::Bool => String::from("bool"),
        Type::String => String::from("std::string::String"),
        Type::Bytes => String::from("std::vec::Vec<u8>"),
        Type::Group | Type::Message => field_type_name_ident(paths, package, field.type_name()),
    }
}

fn field_type_name_ident(paths: &ExternPaths, package: &str, type_name: &str) -> String {
    // protoc should always give fully qualified identifiers.
    assert_eq!(".", &type_name[..1]);

    if let Some(proto_ident) = paths.resolve_ident(type_name) {
        return proto_ident;
    }

    let local_path = package.split('.').peekable();

    let mut ident_path = type_name[1..].split('.');
    let ident_type = ident_path.next_back().unwrap();
    let ident_path = ident_path.peekable();

    // Skip path elements in common.
    //while local_path.peek().is_some() && local_path.peek() == ident_path.peek() {
    //    local_path.next();
    //    ident_path.next();
    //}

    local_path
        .map(|_| "crate".to_string())
        .chain(ident_path.map(to_snake))
        .chain(iter::once(to_upper_camel(ident_type)))
        .join("::")
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct MessageInheritance {
    /// A mapping from full protobuf type to another full protobuf type
    /// described by the OpenFMB UML as a Parent type
    pub parent_typemap: HashMap<String, (String, FieldDescriptorProto)>,

    /// Any type found to be a parent of another is added here
    pub parent_types: HashSet<String>,
}

/// Return a mapping of (package, type) to parent (inherited) message (package, type) which may be
/// traversed recursively to determine all inherited functionality
pub fn message_inheritance(
    paths: &ExternPaths,
    files: &[FileDescriptorProto],
) -> MessageInheritance {
    let mut parent_typemap = HashMap::new();
    let mut parent_types = HashSet::new();

    for file in files {
        let package = file.package.as_ref().unwrap();
        let _module = module(file);
        for message in file.message_type.iter() {
            for field in message.field.iter() {
                if let Some(ref field_options) = field.options {
                    if let Some(true) = field_options.parent_message {
                        //TODO add minimum stuff to mapping to impl IsXyz for Abc {}
                        let _field_type = field_type_ident(paths, package, field);
                        //trace!("package {:?}, message name: {:?}, field name: {:?}", package, message.name(), field.type_name());
                        let message_type = format!(".{}.{}", package, message.name());
                        assert_eq!(
                            parent_typemap
                                .insert(message_type, (field.type_name().into(), field.clone())),
                            None
                        );
                        parent_types.insert(field.type_name().into());
                    }
                }
            }
        }
    }
    MessageInheritance {
        parent_typemap,
        parent_types,
    }
}
