# `rolt` — aspirationally safe Rust Jolt bindings
This crate contains a higher-level wrapper around JoltC, providing ergonomics comparable to using Jolt from C++.

The safety of this crate is currently provided on a best-effort basis.

For more complete and unsafe bindings, see [joltc-sys](https://crates.io/crates/joltc-sys).

## Build Requirements
- CMake 3.16 or newer
- `libclang`, see the [bindgen guide](https://rust-lang.github.io/rust-bindgen/requirements.html) for installation steps.

## Features
- `double-precision`: Forwards to `joltc-sys/double-precision`
- `object-layer-u32`: Forwards to `joltc-sys/object-layer-u32`