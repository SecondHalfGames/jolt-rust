use std::ptr;

use joltc_sys::*;

use crate::{
    BodyInterface, BroadPhaseLayerInterfaceImpl, NarrowPhaseQuery, ObjectLayerPairFilterImpl,
    ObjectVsBroadPhaseLayerFilterImpl,
};

/// The root of everything for a physics simulation.
///
/// See also: Jolt's [`PhysicsSystem`](https://jrouwe.github.io/JoltPhysicsDocs/5.1.0/class_physics_system.html) class.
pub struct PhysicsSystem {
    raw: *mut JPC_PhysicsSystem,
    broad_phase_layer_interface: Option<BroadPhaseLayerInterfaceImpl>,
    object_vs_broad_phase_layer_filter: Option<ObjectVsBroadPhaseLayerFilterImpl>,
    object_layer_pair_filter: Option<ObjectLayerPairFilterImpl>,
}

impl PhysicsSystem {
    pub fn new() -> Self {
        unsafe {
            Self {
                raw: JPC_PhysicsSystem_new(),
                broad_phase_layer_interface: None,
                object_vs_broad_phase_layer_filter: None,
                object_layer_pair_filter: None,
            }
        }
    }

    pub fn init(
        &mut self,
        max_bodies: u32,
        num_body_mutexes: u32,
        max_body_pairs: u32,
        max_contact_constraints: u32,
        broad_phase_layer_interface: impl Into<BroadPhaseLayerInterfaceImpl>,
        object_vs_broad_phase_layer_filter: impl Into<ObjectVsBroadPhaseLayerFilterImpl>,
        object_layer_pair_filter: impl Into<ObjectLayerPairFilterImpl>,
    ) {
        let bpli = broad_phase_layer_interface.into();
        let bpli_raw = bpli.as_raw();
        self.broad_phase_layer_interface = Some(bpli);

        let ovbplf = object_vs_broad_phase_layer_filter.into();
        let ovbplf_raw = ovbplf.as_raw();
        self.object_vs_broad_phase_layer_filter = Some(ovbplf);

        let olpf = object_layer_pair_filter.into();
        let olpf_raw = olpf.as_raw();
        self.object_layer_pair_filter = Some(olpf);

        unsafe {
            JPC_PhysicsSystem_Init(
                self.raw,
                max_bodies,
                num_body_mutexes,
                max_body_pairs,
                max_contact_constraints,
                bpli_raw,
                ovbplf_raw,
                olpf_raw,
            );
        }
    }

    pub fn optimize_broad_phase(&self) {
        unsafe {
            JPC_PhysicsSystem_OptimizeBroadPhase(self.raw);
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
                self.raw,
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
            JPC_PhysicsSystem_DrawBodies(self.raw, settings, renderer, ptr::null());
        }
    }

    pub fn body_interface(&self) -> BodyInterface<'_> {
        unsafe {
            let raw = JPC_PhysicsSystem_GetBodyInterface(self.raw);
            BodyInterface::new(raw)
        }
    }

    pub fn narrow_phase_query(&self) -> NarrowPhaseQuery<'_> {
        unsafe {
            let raw = JPC_PhysicsSystem_GetNarrowPhaseQuery(self.raw);
            NarrowPhaseQuery::new(raw)
        }
    }

    pub fn as_raw(&self) -> *mut JPC_PhysicsSystem {
        self.raw
    }
}

impl Drop for PhysicsSystem {
    fn drop(&mut self) {
        unsafe {
            JPC_PhysicsSystem_delete(self.raw);
        }
    }
}
