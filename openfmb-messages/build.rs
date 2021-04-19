// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

use env_logger;

fn main() {
    env_logger::init();
    //println!("cargo:rerun-if-changed=build.rs");
    const OUT: &str = "src";
    openfmb_codegen::Config::new()
        .btree_map(&["."])
        .out_dir(OUT)
        .compile_protos()
        .unwrap();
}
