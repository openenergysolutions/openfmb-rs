// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

use openfmb_descriptors::OPENFMB_DESCRIPTORS;

fn main() {
    println!("traversing FileDescriptorSet for OpenFMB...");
    for file_descriptor in OPENFMB_DESCRIPTORS.file.iter() {
        println!("{:?}", file_descriptor);
    }
}
