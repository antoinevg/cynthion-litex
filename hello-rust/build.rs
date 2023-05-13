use std::env;
use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;

fn main() {
    let out_dir = &PathBuf::from(env::var_os("OUT_DIR").unwrap());

    // memory.x
    #[cfg(feature = "bootrom")]
    let memory_x = include_bytes!("memory-bootrom.x");
    #[cfg(feature = "cynthion")]
    let memory_x = include_bytes!("memory-gsg_cynthion.x");
    #[cfg(feature = "ulx3s")]
    let memory_x = include_bytes!("memory-radiona_ulx3s.x");
    fs::write(
        out_dir.join("memory.x"),
        memory_x,
    ).unwrap();
    println!("cargo:rerun-if-changed=memory.x");
    println!("cargo:rerun-if-changed=memory-bootrom.x");
    println!("cargo:rerun-if-changed=memory-gsg_cynthion.x");
    println!("cargo:rerun-if-changed=memory-radiona_ulx3s.x");

    // device.x
    File::create(out_dir.join("device.x"))
        .unwrap()
        .write_all(include_bytes!("device.x"))
        .unwrap();
    println!("cargo:rerun-if-changed=device.x");

    // link.rs
    #[cfg(feature = "bootrom")]
    let link_x = include_bytes!("link-bootrom.x");
    #[cfg(not(feature = "bootrom"))]
    let link_x = include_bytes!("link-generic.x");
    fs::write(
        out_dir.join("link.x"),
        link_x,
    ).unwrap();
    println!("cargo:rustc-link-search={}", out_dir.display());
    println!("cargo:rerun-if-changed=link.x");
    println!("cargo:rerun-if-changed=link-bootrom.x");
    println!("cargo:rerun-if-changed=link-generic.x");

    // build.rs
    println!("cargo:rerun-if-changed=build.rs");
}
