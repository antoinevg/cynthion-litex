use std::env;
use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;

fn main() {
    let out_dir = &PathBuf::from(env::var_os("OUT_DIR").unwrap());

    // memory.x
    #[cfg(feature = "cynthion")]
    let memory_x = include_bytes!("gsg_cynthion-memory.x");
    #[cfg(feature = "ulx3s")]
    let memory_x = include_bytes!("radiona_ulx3s-memory.x");
    fs::write(
        out_dir.join("memory.x"),
        memory_x,
    ).unwrap();
    println!("cargo:rerun-if-changed=memory.x");

    // device.x
    File::create(out_dir.join("device.x"))
        .unwrap()
        .write_all(include_bytes!("device.x"))
        .unwrap();
    println!("cargo:rerun-if-changed=device.x");

    // link.rs
    println!("cargo:rustc-link-search={}", out_dir.display());

    // build.rs
    println!("cargo:rerun-if-changed=build.rs");
}
