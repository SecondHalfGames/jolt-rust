use std::env;
use std::path::Path;

use anyhow::Context;
use walkdir::WalkDir;

fn main() {
    let flags = build_flags();

    build_jolt(&flags);
    build_joltc(&flags);

    link();

    generate_bindings(&flags).unwrap();
}

fn build_flags() -> Vec<(&'static str, &'static str)> {
    let mut flags = Vec::new();

    flags.push(("JPH_DEBUG_RENDERER", "ON"));

    if cfg!(feature = "double-precision") {
        flags.push(("JPC_DOUBLE_PRECISION", "ON"));
        flags.push(("JPH_DOUBLE_PRECISION", "ON"));
    }

    if cfg!(feature = "object-layer-u32") {
        flags.push(("JPC_OBJECT_LAYER_BITS", "32"));
        flags.push(("JPH_OBJECT_LAYER_BITS", "32"));
    }

    flags
}

fn build_joltc(flags: &[(&'static str, &'static str)]) {
    let mut build = cc::Build::new();

    for entry in WalkDir::new("JoltC/JoltC") {
        let entry = entry.unwrap();
        let file_name = entry
            .file_name()
            .to_str()
            .expect("file was not valid UTF-8");

        if file_name.ends_with(".cpp") {
            build.file(entry.path());
        }
    }

    for (key, value) in flags {
        build.define(key, *value);
    }

    build
        .std("c++17")
        .include("JoltC")
        .include("JoltC/JoltPhysics")
        .cpp(true)
        .compile("JoltC");
}

fn build_jolt(flags: &[(&'static str, &'static str)]) {
    let mut build = cc::Build::new();

    for entry in WalkDir::new("JoltC/JoltPhysics/Jolt") {
        let entry = entry.unwrap();
        let file_name = entry
            .file_name()
            .to_str()
            .expect("file was not valid UTF-8");

        if file_name.ends_with(".cpp") {
            build.file(entry.path());
        }
    }

    for (key, value) in flags {
        build.define(key, *value);
    }

    build
        .std("c++17")
        .include("JoltC/JoltPhysics")
        .cpp(true)
        .compile("Jolt");
}

fn link() {
    println!("cargo:rustc-link-lib=Jolt");
    println!("cargo:rustc-link-lib=JoltC");
}

fn generate_bindings(flags: &[(&'static str, &'static str)]) -> anyhow::Result<()> {
    let mut builder = bindgen::Builder::default()
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .header("JoltC/JoltC/JoltC.h")
        .clang_arg("-IJoltC")
        .allowlist_item("JPC_.*")
        .default_enum_style(bindgen::EnumVariation::Consts)
        .prepend_enum_name(false);

    for (key, value) in flags {
        builder = builder.clang_arg(format!("-D{key}={value}"));
    }

    let bindings = builder
        .generate()
        .context("failed to generate JoltC bindings")?;

    let out_path = Path::new(&env::var("OUT_DIR").unwrap()).join("bindings.rs");
    bindings
        .write_to_file(out_path)
        .context("Couldn't write bindings!")
}
