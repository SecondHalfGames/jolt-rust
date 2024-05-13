use std::ptr;

pub(crate) struct RemoteDrop {
    this: *mut (),
    drop: unsafe fn(*mut ()),
}

impl RemoteDrop {
    pub unsafe fn new<T: 'static>(this: *mut T) -> Self {
        Self {
            this: this.cast(),
            drop: drop_adapter::<T>,
        }
    }
}

impl Drop for RemoteDrop {
    fn drop(&mut self) {
        unsafe { (self.drop)(self.this) }
    }
}

unsafe fn drop_adapter<T>(this: *mut ()) {
    let this = this.cast::<T>().as_mut().unwrap();
    ptr::drop_in_place(this);
}
