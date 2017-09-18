extern crate rustc_version;
use std::env;
use rustc_version::{version_matches};

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    match (version_matches("1.15.0"), env::var("CARGO_FEATURE_USE_PROC_MACROS").is_ok()) {
        (true, true) => println!("cargo:rustc-cfg=use_proc_macros"),
        (false, true) => println!("cargo:warning=`use-proc-macros` ignored: this Rust version does not support procedural macros"),
        (true, false) | (false, false) => (),
    }
}
