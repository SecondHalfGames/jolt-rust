# jolt-rust
Rust bindings for [Jolt Physics](https://github.com/jrouwe/JoltPhysics) 5.1.0 using [JoltC].

This project is an early work in progress. Watch for exposed nails.

## Goals
1. `joltc-sys`: Functioning, up-to-date unsafe bindings to Jolt Physics
2. `rolt`: Ergonomic, safe bindings to Jolt Physics

## Crates

### `joltc-sys` — Jolt bindings via [JoltC]
This crate contains unsafe bindings to JoltC.

```toml
joltc-sys = "0.2.0"
```

Features:
- `double-precision`: Enable higher precision simulation using doubles instead of floats.
- `object-layer-u32`: Changes the ObjectLayer type to use 32 bits instead of 16 bits.

### `rolt` — aspirationally safe Rust Jolt bindings
This crate contains a higher-level wrapper around JoltC, providing ergonomics comparable to using Jolt from C++.

The safety of this crate is currently provided on a best-effort basis.

```toml
rolt = "0.2.0"
```

Features:
- `double-precision`: Forwards to `joltc-sys/double-precision`
- `object-layer-u32`: Forwards to `joltc-sys/object-layer-u32`

### `hello-world-sys` — HelloWorld using `joltc-sys`
This is a port of Jolt's [HelloWorld] example to Rust using `jolt-sys`. It isn't pretty nor safe, but it does have identical behavior.

### `hello-world` — HelloWorld using `rolt`
This is a port of Jolt's [HelloWorld] example to Rust using the `jolt` crate. The goal of this example is to replicate the behavior of the original example entirely in safe Rust.

## Submodules
This repository uses Git submodules. Make sure to initialize submodules recursively so that JoltC and Jolt are both referenced correctly in your checkout:

```bash
git submodule update --init --recursive
```

## License
Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution
Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

[JoltC]: https://github.com/SecondHalfGames/JoltC
[HelloWorld]: https://github.com/jrouwe/JoltPhysics/blob/master/HelloWorld/HelloWorld.cpp
