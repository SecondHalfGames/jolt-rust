use jolt_sys::*;

pub struct BodyInterface(*mut JPC_BodyInterface);

impl BodyInterface {
    pub(crate) fn new(inner: *mut JPC_BodyInterface) -> Self {
        Self(inner)
    }

    pub fn as_raw(&self) -> *mut JPC_BodyInterface {
        self.0
    }
}
