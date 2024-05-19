# Jolt-Rust Changelog

## [v0.3.0](https://github.com/SecondHalfGames/jolt-rust/compare/rolt-v0.2.0..rolt-v0.3.0)

### joltc-sys
- Several improvements to API coverage.
- Added several name shape types, including convex hulls and compound shapes.
- Shapes are now created by filling out a struct and creating shapes directly from that instead of creating an intermediate opaque settings type.
- `JPC_ShapeSettings_Create` has been replaced by methods for each specific shape type, like `JPC_SphereShapeSettings_Create`.
- Fixed builds with lld on Windows

### rolt
- Added safe wrappers for most of the interfaces that JoltC exposes, including `BodyFilter`.
- Changed math types like `Vec3` to be re-exported from [glam](https://crates.io/crates/glam) instead of having unique types.
- Added `FromJolt`/`IntoJolt`/`IntoRolt` helper traits to make converting to and from -sys types easier.
- Improved documentation across the board, including links to Jolt's new [official multi-versioned documentation](https://jrouwe.github.io/JoltPhysicsDocs/).

## [v0.2.0](https://github.com/SecondHalfGames/jolt-rust/compare/rolt-v0.1.0..rolt-v0.2.0)

### joltc-sys
- Significantly higher API coverage, especially for Body and BodyInterface.

## v0.1.1
- Attempt to fix docs.rs builds from differing build environments.

## v0.1.0
- Initial release of `joltc-sys` and `rolt`