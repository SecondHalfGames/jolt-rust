use std::marker::PhantomData;

use joltc_sys::*;

use crate::BodyId;

/// See also: Jolt's [`Body`](https://jrouwe.github.io/JoltPhysicsDocs/5.1.0/class_body.html) class.
pub struct Body<'interface> {
    inner: *mut JPC_Body,
    _phantom: PhantomData<&'interface ()>,
}

impl<'interface> Body<'interface> {
    pub(crate) fn new(inner: *mut JPC_Body) -> Self {
        Self {
            inner,
            _phantom: PhantomData,
        }
    }

    pub fn id(&self) -> BodyId {
        let raw = unsafe { JPC_Body_GetID(self.inner) };
        BodyId::new(raw)
    }

    pub fn user_data(&self) -> u64 {
        let raw = unsafe { JPC_Body_GetUserData(self.inner) };
        raw
    }

    pub fn as_raw(&self) -> *mut JPC_Body {
        self.inner
    }
}
