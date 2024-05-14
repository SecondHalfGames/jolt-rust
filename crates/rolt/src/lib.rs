use joltc_sys::*;

mod body;
mod body_interface;
mod math;
mod physics_system;
mod reference;
mod remote_drop;
mod simple_types;
mod traits;

pub use crate::body::*;
pub use crate::body_interface::*;
pub use crate::math::*;
pub use crate::physics_system::*;
pub use crate::reference::*;
pub use crate::simple_types::*;
pub use crate::traits::*;

pub fn register_default_allocator() {
    unsafe {
        JPC_RegisterDefaultAllocator();
    }
}

pub fn factory_init() {
    unsafe {
        JPC_FactoryInit();
    }
}

pub fn factory_delete() {
    unsafe {
        JPC_FactoryDelete();
    }
}

pub fn register_types() {
    unsafe {
        JPC_RegisterTypes();
    }
}

pub fn unregister_types() {
    unsafe {
        JPC_UnregisterTypes();
    }
}
