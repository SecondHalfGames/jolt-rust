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

    // We always have to build in Release.
    //
    // On Windows, Rust always links against the non-debug CRT. Using the Debug
    // profile (which the cmake crate will sometimes pick by default) causes
    // Jolt/JoltC to be linked against the debug CRT, causing linker issues.
    //
    // Forcing Jolt and JoltC to be compiled with the non-debug CRT (/MT)
    // doesn't change enough about the build to work.
    //
    // As a nice side effect, this ensures that we build with a known
    // configuration instead of accidentally enabling or disabling extra
    // features just based on opt-level.
    config.profile("Release");

    // Read the actual cross-compile target rather than the build host so
    // the rest of this fn can dispatch correctly. `cfg!(...)` would
    // evaluate at build-script-compile time on the HOST.
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default();

    // Jolt fails to compile via the cmake crate without specifying exception
    // handling behavior under MSVC. I'm not sure that this is the correct
    // exception handling mode.
    if target_os == "windows" {
        config.cxxflag("/EHsc");
    }

    // Native Android cross-compile setup. Without this, building
    // joltc-sys for Android via the standard
    // `cargo ndk --target <android-triple> check` flow fails because
    // (a) cmake-rs defaults to MSBuild on Windows hosts which can't
    // target Android; (b) the NDK's `android.toolchain.cmake` requires
    // `ANDROID_ABI` to be set as a CMake variable (env var is
    // ignored); (c) `ANDROID_NDK_HOME` is the canonical env name set
    // by `nttld/setup-ndk` GitHub Action + cargo-ndk's local
    // workflow.
    //
    // We translate cargo's target arch to NDK's ABI string and route
    // CMake through the NDK toolchain file. `ANDROID_PLATFORM=android-21`
    // is the same baseline most cargo-ndk users target.
    if target_os == "android" {
        let android_ndk_home = env::var("ANDROID_NDK_HOME")
            .or_else(|_| env::var("ANDROID_NDK_ROOT"))
            .or_else(|_| env::var("ANDROID_NDK"))
            .expect(
                "Android cross-compile requires ANDROID_NDK_HOME (or ANDROID_NDK_ROOT / \
                 ANDROID_NDK) to point at the NDK install. Install via \
                 `nttld/setup-ndk@v1` in CI or the Android SDK manager locally.",
            );
        let toolchain_file = format!("{android_ndk_home}/build/cmake/android.toolchain.cmake");
        config.define("CMAKE_TOOLCHAIN_FILE", &toolchain_file);
        let android_abi = match target_arch.as_str() {
            "aarch64" => "arm64-v8a",
            "arm" => "armeabi-v7a",
            "x86" => "x86",
            "x86_64" => "x86_64",
            _ => panic!("unsupported Android target arch: {target_arch}"),
        };
        config.define("ANDROID_ABI", android_abi);
        config.define("ANDROID_PLATFORM", "android-21");
        // cmake-rs defaults to the host-OS generator (MSBuild on Win);
        // the NDK toolchain only supports Ninja / Makefiles. Force
        // Ninja explicitly.
        config.generator("Ninja");
    }

    // Having IPO/LTO turned on breaks lld on Windows.
    config.define("INTERPROCEDURAL_OPTIMIZATION", "OFF");

    // Warnings when building Jolt or JoltC don't matter to users of joltc-sys.
    config.define("ENABLE_ALL_WARNINGS", "OFF");

    // These feature flags go through CMake and affect compilation of both Jolt
    // and JoltC.
    if cfg!(feature = "double-precision") {
        config.define("DOUBLE_PRECISION", "ON");
    }
    if cfg!(feature = "object-layer-u32") {
        config.define("OBJECT_LAYER_BITS", "32");
    }

    if cfg!(feature = "asserts") {
        config.define("USE_ASSERTS", "ON");
    }

    let mut dst = config.build();

    // Jolt and JoltC put libraries in the 'lib' subfolder. This goes against
    // the docs of the cmake crate, but it's possible that it's just mishandling
    // an output path and not account for the install target's configurability.
    dst.push("lib");
    println!("cargo:rustc-link-search=native={}", dst.display());

    // On Fedora Workstation 42, it looks like Jolt puts libs in the "lib64"
    // subfolder, so make sure to search there too.
    dst.pop();
    dst.push("lib64");
    println!("cargo:rustc-link-search=native={}", dst.display());

    // On macOS and Linux, we need to explicitly link against the C++ standard
    // library here to avoid getting missing symbol errors from Jolt/JoltC.
    if cfg!(target_os = "macos") {
        println!("cargo:rustc-flags=-l dylib=c++");
    }

    if cfg!(target_os = "linux") {
        println!("cargo:rustc-link-lib=dylib=stdc++");
    }
}

fn link() {
    println!("cargo:rustc-link-lib=Jolt");
    println!("cargo:rustc-link-lib=joltc");
}

/// Generate build flags specifically for generating bindings.
///
/// This is redundant with Jolt and JoltC's CMake files, which do this mapping
/// for us, but we're unable to leverage that config when running bindgen.
fn build_flags() -> Vec<(&'static str, &'static str)> {
    let mut flags = Vec::new();

    // Force the debug renderer on. In the future, we might want to tie this to
    // a crate feature.
    flags.push(("JPH_DEBUG_RENDERER", "ON"));

    // It's important that these flags are never out of sync for Jolt and JoltC.
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
