use std::ops::Deref;

/// Rust version of Jolt's [`RefTarget`](https://jrouwe.github.io/JoltPhysics/class_ref_target.html)
/// CRTP.
pub trait RefTarget {
    fn add_ref(value: *const Self);
    fn release(value: *const Self);
}

/// Rust equivalent to Jolt's [`RefConst`](https://jrouwe.github.io/JoltPhysics/class_ref_const.html)
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
        T::add_ref(self.ptr);
        Self { ptr: self.ptr }
    }
}

impl<T: RefTarget> Drop for Ref<T> {
    fn drop(&mut self) {
        T::release(self.ptr)
    }
}
