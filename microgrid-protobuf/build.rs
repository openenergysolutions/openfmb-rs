// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

fn main() {
    prost_build::compile_protos(&["src/microgrid.proto"], &["src/"]).unwrap();
}
