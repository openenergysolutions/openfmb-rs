use std::process::Command;
use std::{env, fs};
use std::error::Error;
use std::ffi::OsStr;

fn proto_paths() -> Result<Vec<String>, Box<dyn Error>> {
    let mut proto_paths: Vec<String> = Vec::new();
    let current_dir = env::current_dir()?;
    let mut proto_path = current_dir.clone();
    proto_path.push("proto");
    println!("searching for proto files in {:?}", proto_path);
    println!("cargo:rerun-if-changed={:?}", proto_path);
    for entry in fs::read_dir(proto_path.clone())? {
        let proto_ext = OsStr::new("proto");
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            for sentry in fs::read_dir(path)? {
                let sentry = sentry?;
                let path = sentry.path();
                if path.is_file() && path.extension() == Some(proto_ext) {
                    proto_paths.push(path.strip_prefix(&proto_path).unwrap().to_str().unwrap().to_string())
                }
            }
        } else {
            if path.is_file() && path.extension() == Some(proto_ext) {
                proto_paths.push(path.strip_prefix(&proto_path).unwrap().to_str().unwrap().to_string())
            }
        }
    }

    Ok(proto_paths)
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("looking up paths");
    let paths = proto_paths()?;
    println!("proto paths {:?}", paths);
    let mut protoc = Command::new("protoc");
    protoc.current_dir("proto");
    protoc.arg("-oopenfmb_descriptors.pb");
    for proto_path in paths {
        protoc.arg(proto_path.clone());
    }

    let output = protoc.output()?;
    println!("output of command {:?}", output);


    Ok(())
}
