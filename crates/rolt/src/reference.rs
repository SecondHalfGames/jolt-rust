use std::ops::Deref;

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

unsafe impl RefTarget for joltc_sys::JPC_Shape {
    unsafe fn add_ref(value: *const Self) {
        joltc_sys::JPC_Shape_AddRef(value);
    }

    unsafe fn release(value: *const Self) {
        joltc_sys::JPC_Shape_Release(value);
    }
}

/// Rust equivalent to Jolt's [`RefConst`](https://jrouwe.github.io/JoltPhysicsDocs/5.1.0/class_ref_const.html)
pub struct Ref<T: RefTarget> {
    ptr: *const T,
}

impl<T: RefTarget> Ref<T> {
    /// Take ownership over a pointer and start reference counting it.
    ///
    /// # Safety
    ///
    /// `ptr` must be valid and have an extra ref already added. More
    /// specifically, the object's refcount must be equal to the number of
    /// existing Ref types (C++ and Rust) plus one.
    pub unsafe fn from_active(ptr: *const T) -> Self {
        Self { ptr }
    }
}

impl<T: RefTarget> Deref for Ref<T> {
    type Target = *const T;

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
