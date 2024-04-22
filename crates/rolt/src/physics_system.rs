use std::ptr;

use joltc_sys::*;

use crate::{BodyInterface, IntoBroadPhaseLayerInterface};

pub struct PhysicsSystem(*mut JPC_PhysicsSystem);

impl PhysicsSystem {
    pub fn new() -> Self {
        unsafe { Self(JPC_PhysicsSystem_new()) }
    }

    /// # Safety
    /// not really
    pub unsafe fn init(
        &self,
        max_bodies: u32,
        num_body_mutexes: u32,
        max_body_pairs: u32,
        max_contact_constraints: u32,
        broad_phase_layer_interface: impl IntoBroadPhaseLayerInterface,
        object_vs_broad_phase_layer_interface: *mut JPC_ObjectVsBroadPhaseLayerFilter,
        object_layer_pair_filter: *mut JPC_ObjectLayerPairFilter,
    ) {
        unsafe {
            JPC_PhysicsSystem_Init(
                self.0,
                max_bodies,
                num_body_mutexes,
                max_body_pairs,
                max_contact_constraints,
                broad_phase_layer_interface.as_raw(),
                object_vs_broad_phase_layer_interface,
                object_layer_pair_filter,
            );
        }
    }

    /// # Safety
    /// definitely not
    pub unsafe fn update(
        &self,
        delta_time: f32,
        collision_steps: i32,
        temp_allocator: *mut JPC_TempAllocatorImpl,
        job_system: *mut JPC_JobSystemThreadPool,
    ) {
        unsafe {
            JPC_PhysicsSystem_Update(
                self.0,
                delta_time,
                collision_steps,
                temp_allocator,
                job_system,
            );
        }
    }

    /// # Safety
    /// `renderer` must be valid and non-null.
    pub unsafe fn draw_bodies(
        &self,
        settings: &mut JPC_BodyManager_DrawSettings,
        renderer: *mut JPC_DebugRendererSimple,
    ) {
        unsafe {
            JPC_PhysicsSystem_DrawBodies(self.0, settings, renderer, ptr::null());
        }
    }

    pub fn body_interface(&self) -> BodyInterface<'_> {
        unsafe {
            let raw = JPC_PhysicsSystem_GetBodyInterface(self.0);
            BodyInterface::new(raw)
        }
    }

    pub fn as_raw(&self) -> *mut JPC_PhysicsSystem {
        self.0
    }
}

impl Drop for PhysicsSystem {
    fn drop(&mut self) {
        unsafe {
            JPC_PhysicsSystem_delete(self.0);
        }
    }
}
