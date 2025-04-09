use std::ops::Deref;

use joltc_sys::*;

/// Rust version of Jolt's [`RefTarget`](https://jrouwe.github.io/JoltPhysicsDocs/5.1.0/class_ref_target.html)
/// CRTP.
///
/// # Safety
///
/// The `value` pointers provided must be live. This trait should only be
/// implemented for C++ types that inherit from `RefTarget<Self>`.
#[allow(clippy::missing_safety_doc)]
pub unsafe trait RefTarget {
    unsafe fn add_ref(value: *const Self);
    unsafe fn release(value: *const Self);
}

unsafe impl RefTarget for JPC_Shape {
    unsafe fn add_ref(value: *const Self) {
        JPC_Shape_AddRef(value);
    }

    unsafe fn release(value: *const Self) {
        JPC_Shape_Release(value);
    }
}

unsafe impl RefTarget for JPC_MutableCompoundShape {
    unsafe fn add_ref(value: *const Self) {
        JPC_Shape_AddRef(value.cast::<JPC_Shape>());
    }

    unsafe fn release(value: *const Self) {
        JPC_Shape_Release(value.cast::<JPC_Shape>());
    }
}

/// Rust equivalent to Jolt's [`RefConst`](https://jrouwe.github.io/JoltPhysicsDocs/5.1.0/class_ref_const.html)
pub struct RefConst<T: RefTarget> {
    ptr: *const T,
}

impl<T: RefTarget> RefConst<T> {
    /// Take ownership over a pointer and start reference counting it.
    ///
    /// # Safety
    ///
    /// `ptr` must be valid.
    pub unsafe fn from_active(ptr: *const T) -> Self {
        T::add_ref(ptr);
        Self { ptr }
    }
}

impl<T: RefTarget> Deref for RefConst<T> {
    type Target = *const T;

    fn deref(&self) -> &Self::Target {
        &self.ptr
    }
}

impl<T: RefTarget> Clone for RefConst<T> {
    fn clone(&self) -> Self {
        unsafe {
            T::add_ref(self.ptr);
        }

        Self { ptr: self.ptr }
    }
}

impl<T: RefTarget> Drop for RefConst<T> {
    fn drop(&mut self) {
        unsafe {
            T::release(self.ptr);
        }
    }
}

/// Rust equivalent to Jolt's [`Ref`](https://jrouwe.github.io/JoltPhysicsDocs/5.1.0/class_ref.html)
pub struct Ref<T: RefTarget> {
    ptr: *mut T,
}

impl<T: RefTarget> Ref<T> {
    /// Take ownership over a pointer and start reference counting it.
    ///
    /// # Safety
    ///
    /// `ptr` must be valid.
    pub unsafe fn from_active(ptr: *mut T) -> Self {
        T::add_ref(ptr);
        Self { ptr }
    }
}

impl<T: RefTarget> Deref for Ref<T> {
    type Target = *mut T;

    fn deref(&self) -> &Self::Target {
        &self.ptr
    }
}

impl<T: RefTarget> Clone for Ref<T> {
    fn clone(&self) -> Self {
        unsafe {
            T::add_ref(self.ptr);
        }

        Self { ptr: self.ptr }
    }
}

impl<T: RefTarget> Drop for Ref<T> {
    fn drop(&mut self) {
        unsafe {
            T::release(self.ptr);
        }
    }
}
