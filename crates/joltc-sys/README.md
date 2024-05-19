# `joltc-sys` — Jolt bindings via [JoltC]
This crate contains unsafe bindings to JoltC.

## Build Requirements
- CMake 3.16 or newer
- `libclang`, see the [bindgen guide](https://rust-lang.github.io/rust-bindgen/requirements.html) for installation steps.

## Features
- `double-precision`: Enable higher precision simulation using doubles instead of floats.
- `object-layer-u32`: Changes the ObjectLayer type to use 32 bits instead of 16 bits.