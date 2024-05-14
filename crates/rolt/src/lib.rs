//! Safe Rust wrapper around [Jolt Physics](github.com/jrouwe/JoltPhysics) using
//! [JoltC](https://github.com/SecondHalfGames/JoltC).
//!
//! These bindings are best-effort and incomplete. The [joltc-sys][joltc_sys]
//! crate contains the unsafe bindings that this crate uses and covers a lot
//! more of Jolt's API.
//!
//! These bindings target Jolt Physics 5.0.0. You can view the C++ documentation
//! for this version of Jolt Physics here:
//!
//! <https://secondhalfgames.github.io/jolt-docs/5.0.0/>

use joltc_sys::*;

mod body;
mod body_interface;
mod conversions;
mod math;
mod narrow_phase;
mod physics_system;
mod reference;
mod remote_drop;
mod simple_types;
mod traits;

pub use crate::body::*;
pub use crate::body_interface::*;
pub use crate::conversions::*;
pub use crate::math::*;
pub use crate::narrow_phase::*;
pub use crate::physics_system::*;
pub use crate::reference::*;
pub use crate::simple_types::*;
pub use crate::traits::*;

/// [`JPH::RegisterDefaultAllocator`](https://secondhalfgames.github.io/jolt-docs/5.0.0/_memory_8h.html#a6ae804b1b68490f6e032ef6e7d9fc93e)
pub fn register_default_allocator() {
    unsafe {
        JPC_RegisterDefaultAllocator();
    }
}

/// Creates a new global factory. Required for initialization and used by Jolt's
/// serialization.
///
/// See also: Jolt's [`Factory`](https://secondhalfgames.github.io/jolt-docs/5.0.0/class_factory.html) class.
pub fn factory_init() {
    unsafe {
        JPC_FactoryInit();
    }
}

/// Deletes the globally registered factory.
///
/// See also: Jolt's [`Factory`](https://secondhalfgames.github.io/jolt-docs/5.0.0/class_factory.html) class.
pub fn factory_delete() {
    unsafe {
        JPC_FactoryDelete();
    }
}

/// [`JPH::RegisterTypes`](https://secondhalfgames.github.io/jolt-docs/5.0.0/_register_types_8h.html#a033e662bc8b7d5a8acd9adcc692b7cb4)
pub fn register_types() {
    unsafe {
        JPC_RegisterTypes();
    }
}

/// [`JPH::UnregisterTypes`](https://secondhalfgames.github.io/jolt-docs/5.0.0/_register_types_8h.html#a1e0db6031789e773039c7fc15ef47057)
pub fn unregister_types() {
    unsafe {
        JPC_UnregisterTypes();
    }
}
