// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

use proto_types::FileDescriptorProto;

pub type Module = Vec<String>;

pub fn module(file: &FileDescriptorProto) -> Module {
    file.package()
        .split('.')
        .filter(|s| !s.is_empty())
        .map(to_snake)
        .collect()
}
