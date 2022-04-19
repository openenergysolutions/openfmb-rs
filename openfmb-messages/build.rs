// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0
//
use env_logger;
use std::env;

fn main() {
    env_logger::init();
    let out_dir = env::var("OUT_DIR").unwrap();
    println!("OUT DIR {:?}", env::var("OUT_DIR"));    
    openfmb_codegen::Config::new()
        .btree_map(&["."])
        .out_dir(&out_dir)
        .compile_protos()
        .unwrap();
}
