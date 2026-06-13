use std::{env, path::PathBuf};

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=proto/helloworld/helloworld.proto");
    println!("cargo:rerun-if-changed=proto/profilerequest/profilerequest.proto");

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    tonic_build::configure()
        .file_descriptor_set_path(out_dir.join("helloworld_descriptor.bin"))
        .compile_protos(&["proto/helloworld/helloworld.proto"], &["proto"])
        .unwrap();

    tonic_build::configure()
        .file_descriptor_set_path(out_dir.join("profilerequest_descriptor.bin"))
        .compile_protos(&["proto/profilerequest/profilerequest.proto"], &["proto"])
        .unwrap();
}
