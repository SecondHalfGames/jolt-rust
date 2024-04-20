# jolt-rust
Rust bindings for [Jolt Physics](https://github.com/jrouwe/JoltPhysics) 5.0.0 using [JoltC].

This project is an early work in progress. Watch for exposed nails.

## Goals
1. `jolt-sys`: Functioning, up-to-date unsafe bindings to Jolt Physics
2. `jolt`: Ergonomic, safe bindings to Jolt Physics

## Crates

### `jolt-sys` — Jolt bindings via [JoltC]
This crate contains unsafe bindings to JoltC.

### `jolt` — aspirationally safe Rust Jolt bindings
This crate contains a higher-level wrapper around JoltC, providing ergonomics comparable to using Jolt from C++.

The safety of this crate is currently provided on a best-effort basis.

### `hello-world-sys` — HelloWorld using `jolt-sys`
This is a port of Jolt's [HelloWorld] example to Rust using `jolt-sys`. It isn't pretty nor safe, but it does have identical behavior.

### `hello-world` — HelloWorld using `jolt`
This is a port of Jolt's [HelloWorld] example to Rust using the `jolt` crate. The goal of this example is to replicate the behavior of the original example entirely in safe Rust.

## License
Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution
Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

[JoltC]: https://github.com/SecondHalfGames/JoltC
[HelloWorld]: https://github.com/jrouwe/JoltPhysics/blob/master/HelloWorld/HelloWorld.cpp