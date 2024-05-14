use std::marker::PhantomData;

use joltc_sys::*;

use crate::{Body, BodyId, IntoJolt, IntoRolt, RVec3, Vec3};

/// See also: Jolt's [`BodyInterface`](https://secondhalfgames.github.io/jolt-docs/5.0.0/class_body_interface.html) class.
pub struct BodyInterface<'physics_system> {
    raw: *mut JPC_BodyInterface,
    _phantom: PhantomData<&'physics_system ()>,
}

impl<'physics_system> BodyInterface<'physics_system> {
    pub(crate) fn new(raw: *mut JPC_BodyInterface) -> Self {
        Self {
            raw,
            _phantom: PhantomData,
        }
    }

    /// # Safety
    /// `settings` must be initialized and valid, with a valid `Shape` pointer.
    pub unsafe fn create_body(&self, settings: &JPC_BodyCreationSettings) -> Body {
        let raw = JPC_BodyInterface_CreateBody(self.raw, settings);

        Body::<'physics_system>::new(raw)
    }

    pub fn add_body(&self, body_id: BodyId, activation_mode: JPC_Activation) {
        unsafe {
            JPC_BodyInterface_AddBody(self.raw, body_id.raw(), activation_mode);
        }
    }

    pub fn remove_body(&self, body_id: BodyId) {
        unsafe { JPC_BodyInterface_RemoveBody(self.raw, body_id.raw()) }
    }

    pub fn destroy_body(&self, body_id: BodyId) {
        unsafe { JPC_BodyInterface_DestroyBody(self.raw, body_id.raw()) }
    }

    pub fn is_active(&self, body_id: BodyId) -> bool {
        unsafe { JPC_BodyInterface_IsActive(self.raw, body_id.raw()) }
    }

    pub fn center_of_mass_position(&self, body_id: BodyId) -> RVec3 {
        unsafe { JPC_BodyInterface_GetCenterOfMassPosition(self.raw, body_id.raw()).into_rolt() }
    }

    pub fn linear_velocity(&self, body_id: BodyId) -> Vec3 {
        unsafe { JPC_BodyInterface_GetLinearVelocity(self.raw, body_id.raw()).into_rolt() }
    }

    pub fn set_linear_velocity(&self, body_id: BodyId, velocity: Vec3) {
        unsafe {
            JPC_BodyInterface_SetLinearVelocity(self.raw, body_id.raw(), velocity.into_jolt());
        }
    }

    pub fn as_raw(&self) -> *mut JPC_BodyInterface {
        self.raw
    }
}
