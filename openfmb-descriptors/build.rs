// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

use std::error::Error;
use std::ffi::OsStr;
use std::path::PathBuf;
use std::process::Command;
use std::{env, fs};

/// Returns the path to the location of the bundled Protobuf artifacts.
fn bundle_path() -> PathBuf {
    env::current_dir()
        .unwrap()
        .join("third-party")
        .join("protobuf")
}

/// Returns the path to the `protoc` pointed to by the `PROTOC` environment variable, if it is set.
fn env_protoc() -> Option<PathBuf> {
    let protoc = match env::var_os("PROTOC") {
        Some(path) => PathBuf::from(path),
        None => return None,
    };

    Some(protoc)
}

/// Returns the path to the bundled `protoc`, if it is available for the host platform.
#[cfg(not(target_env = "musl"))]
fn bundled_protoc() -> Option<PathBuf> {
    let protoc_bin_name = match (env::consts::OS, env::consts::ARCH) {
        ("linux", "x86") => "protoc-linux-x86_32",
        ("linux", "x86_64") => "protoc-linux-x86_64",
        ("linux", "aarch64") => "protoc-linux-aarch_64",
        ("macos", "x86_64") => "protoc-osx-x86_64",
        ("macos", "aarch64") => "protoc-osx-x86_64", // will be translated to aarch64 by Rosetta
        ("windows", _) => "protoc-win32.exe",
        _ => return None,
    };

    Some(bundle_path().join(protoc_bin_name))
}

/// `musl` build hosts do not have a bundled `protoc`.
///
/// Note: this checks the target of the `prost-build` build.rs, which is ultimately the host architecture.
#[cfg(target_env = "musl")]
fn bundled_protoc() -> Option<PathBuf> {
    None
}

/// Returns the path to the `protoc` included on the `PATH`, if it exists.
fn path_protoc() -> Option<PathBuf> {
    which::which("protoc").ok()
}

/// Returns the path to the Protobuf include directory pointed to by the `PROTOC_INCLUDE`
/// environment variable, if it is set.
fn env_protoc_include() -> Option<PathBuf> {
    let protoc_include = match env::var_os("PROTOC_INCLUDE") {
        Some(path) => PathBuf::from(path),
        None => return None,
    };

    if !protoc_include.exists() {
        panic!(
            "PROTOC_INCLUDE environment variable points to non-existent directory ({:?})",
            protoc_include
        );
    }
    if !protoc_include.is_dir() {
        panic!(
            "PROTOC_INCLUDE environment variable points to a non-directory file ({:?})",
            protoc_include
        );
    }

    Some(protoc_include)
}

/// Returns the path to the bundled Protobuf include directory.
fn bundled_protoc_include() -> PathBuf {
    bundle_path().join("include")
}

fn proto_paths() -> Result<Vec<String>, Box<dyn Error>> {
    let mut proto_paths: Vec<String> = Vec::new();
    let current_dir = env::current_dir()?;
    let mut proto_path = current_dir.clone();
    proto_path.push("proto");
    //println!("searching for proto files in {:?}", proto_path);
    //println!("cargo:rerun-if-changed={:?}", proto_path);
    for entry in fs::read_dir(proto_path.clone())? {
        let proto_ext = OsStr::new("proto");
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            for sentry in fs::read_dir(path)? {
                let sentry = sentry?;
                let path = sentry.path();
                if path.is_file() && path.extension() == Some(proto_ext) {
                    proto_paths.push(
                        path.strip_prefix(&proto_path)
                            .unwrap()
                            .to_str()
                            .unwrap()
                            .to_string(),
                    )
                }
            }
        } else {
            if path.is_file() && path.extension() == Some(proto_ext) {
                proto_paths.push(
                    path.strip_prefix(&proto_path)
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .to_string(),
                )
            }
        }
    }

    Ok(proto_paths)
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("cargo:rerun-if-changed=proto");
    let protoc = env_protoc()
        .or_else(bundled_protoc)
        .or_else(path_protoc)
        .expect(
            "Failed to find the protoc binary. The PROTOC environment variable is not set, \
             there is no bundled protoc for this platform, and protoc is not in the PATH",
        );

    let protoc_include = env_protoc_include().unwrap_or_else(bundled_protoc_include);

    println!("cargo:rustc-env=PROTOC={}", protoc.display());
    println!(
        "cargo:rustc-env=PROTOC_INCLUDE={}",
        protoc_include.display()
    );
    println!("cargo:rerun-if-env-changed=PROTOC");
    println!("cargo:rerun-if-env-changed=PROTOC_INCLUDE");

    let out_dir = env::var("OUT_DIR").unwrap();
    //println!("looking up paths");
    let paths = proto_paths()?;

    //println!("proto paths {:?}", paths);
    let mut protoc_cmd = Command::new(protoc);
    protoc_cmd.current_dir("proto");
    protoc_cmd
        .arg("--include_imports")
        .arg("--include_source_info")
        .arg(format!("-o{}/openfmb_descriptors.pb", out_dir))
        .arg("-I.")
        .arg("-I")
        .arg(protoc_include);
    for proto_path in paths {
        protoc_cmd.arg(proto_path.clone());
    }

    //println!("cmd {:?}", protoc_cmd);
    let _output = protoc_cmd.output()?;
    //println!("output of command {:?}", output);

    Ok(())
}
