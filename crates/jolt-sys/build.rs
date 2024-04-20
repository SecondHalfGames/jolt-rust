use std::env;
use std::path::Path;

use anyhow::Context;

fn main() {
    let flags = build_flags();

    build_joltc();

    link();

    generate_bindings(&flags).unwrap();
}

fn build_joltc() {
    let mut config = cmake::Config::new("JoltC");

    if cfg!(windows) {
        config.cxxflag("/EHsc");
    }

    if cfg!(feature = "double-precision") {
        config.configure_arg("-DDOUBLE_PRECISION=ON");
    }

    if cfg!(feature = "object-layer-u32") {
        config.configure_arg("-DOBJECT_LAYER_BITS=32");
    }

    let dst = config.build();

    println!("cargo:rustc-link-search=native={}", dst.display());
}

fn link() {
    println!("cargo:rustc-link-lib=Jolt");
    println!("cargo:rustc-link-lib=joltc");
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
