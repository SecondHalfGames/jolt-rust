use std::marker::PhantomData;

use glam::Quat;
use joltc_sys::*;

use crate::{Body, BodyId, IntoJolt, IntoRolt, ObjectLayer, RVec3, Vec3};

/// See also: Jolt's [`BodyInterface`](https://jrouwe.github.io/JoltPhysicsDocs/5.1.0/class_body_interface.html) class.
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
    pub unsafe fn create_body(&self, settings: &JPC_BodyCreationSettings) -> Option<Body> {
        let raw = JPC_BodyInterface_CreateBody(self.raw, settings);

        if raw.is_null() {
            None
        } else {
            Some(Body::<'physics_system>::new(raw))
        }
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

    /// # Safety
    /// `shape` must be a valid shape.
    pub unsafe fn set_shape(
        &self,
        body_id: BodyId,
        shape: *const JPC_Shape,
        update_mass_properties: bool,
        activation: JPC_Activation,
    ) {
        JPC_BodyInterface_SetShape(
            self.raw,
            body_id.raw(),
            shape,
            update_mass_properties,
            activation,
        )
    }

    pub fn is_active(&self, body_id: BodyId) -> bool {
        unsafe { JPC_BodyInterface_IsActive(self.raw, body_id.raw()) }
    }

    pub fn user_data(&self, body_id: BodyId) -> u64 {
        unsafe { JPC_BodyInterface_GetUserData(self.raw, body_id.raw()) }
    }

    pub fn set_user_data(&self, body_id: BodyId, user_data: u64) {
        unsafe { JPC_BodyInterface_SetUserData(self.raw, body_id.raw(), user_data) }
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

    pub fn set_object_layer(&self, body_id: BodyId, object_layer: ObjectLayer) {
        unsafe { JPC_BodyInterface_SetObjectLayer(self.raw, body_id.raw(), object_layer.raw()) }
    }

    pub fn notify_shape_changed(
        &self,
        body_id: BodyId,
        old_com: Vec3,
        update_mass_properties: bool,
        activation: JPC_Activation,
    ) {
        unsafe {
            JPC_BodyInterface_NotifyShapeChanged(
                self.raw,
                body_id.raw(),
                old_com.into_jolt(),
                update_mass_properties,
                activation,
            )
        }
    }

    pub fn set_position_and_rotation_when_changed(
        &self,
        body_id: BodyId,
        pos: RVec3,
        rot: Quat,
        activation: JPC_Activation,
    ) {
        unsafe {
            JPC_BodyInterface_SetPositionAndRotationWhenChanged(
                self.raw,
                body_id.raw(),
                pos.into_jolt(),
                rot.into_jolt(),
                activation,
            )
        }
    }

    pub fn as_raw(&self) -> *mut JPC_BodyInterface {
        self.raw
    }
}
