// build.rs
use std::env;
use std::path::PathBuf;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    
    // Build C utilities if needed
    cc::Build::new()
        .file("src/c/pdf_utils.c")
        .compile("pdf_utils");

    println!("cargo:rerun-if-changed=src/c/pdf_utils.c");
    println!("cargo:rerun-if-changed=src/c/pdf_utils.h");
}