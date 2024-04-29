use std::marker::PhantomData;

use joltc_sys::*;

use crate::BodyId;

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

    pub fn as_raw(&self) -> *mut JPC_Body {
        self.inner
    }
}
