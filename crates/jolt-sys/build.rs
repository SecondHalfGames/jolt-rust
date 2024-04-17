use std::env;
use std::path::{Path, PathBuf};

use anyhow::Context;
use walkdir::WalkDir;

fn main() {
    build();
    link();
    generate_bindings();

    println!("cargo:rerun-if-changed=JoltC/JoltC/.h");
}

fn build() {
    build_jolt();
    build_joltc();
}

fn build_joltc() {
    let mut build = cc::Build::new();

    for entry in WalkDir::new("JoltC") {
        let entry = entry.unwrap();
        let file_name = entry
            .file_name()
            .to_str()
            .expect("file was not valid UTF-8");

        if file_name.ends_with(".cpp") {
            build.file(entry.path());
        }
    }

    build
        .std("c++17")
        .include(".")
        .include("JoltC")
        .cpp(true)
        .compile("JoltC");
}

fn build_jolt() {
    let mut build = cc::Build::new();

    for entry in WalkDir::new("JoltC/JoltPhysics") {
        let entry = entry.unwrap();
        let file_name = entry
            .file_name()
            .to_str()
            .expect("file was not valid UTF-8");

        if file_name.ends_with(".cpp") {
            build.file(entry.path());
        }
    }

    build.std("c++17").include(".").cpp(true).compile("Jolt");
}

fn link() {
    println!("cargo:rustc-link-lib=Jolt");
    println!("cargo:rustc-link-lib=JoltC");
}

fn generate_bindings() -> anyhow::Result<()> {
    let bindings = bindgen::Builder::default()
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .header("JoltC/JoltC.h")
        .allowlist_item("JPC_.*")
        .default_enum_style(bindgen::EnumVariation::Consts)
        .generate()
        .context("failed to generate JoltC bindings")?;

    let out_path = Path::new(&env::var("OUT_DIR").unwrap()).join("bindings.rs");
    bindings
        .write_to_file(out_path)
        .context("Couldn't write bindings!")
}
