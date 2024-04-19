// someday:
// #![forbid(unsafe_code)]

use std::ffi::c_void;
use std::ptr;

use jolt::{BroadPhaseLayer, BroadPhaseLayerInterface, ObjectLayer};
use jolt_sys::{JPC_BroadPhaseLayer, JPC_ObjectLayer};

const OL_NON_MOVING: JPC_ObjectLayer = 0;
const OL_MOVING: JPC_ObjectLayer = 1;

const BPL_NON_MOVING: JPC_BroadPhaseLayer = 0;
const BPL_MOVING: JPC_BroadPhaseLayer = 1;
const BPL_COUNT: JPC_BroadPhaseLayer = 2;

struct BroadPhaseLayers;

impl BroadPhaseLayerInterface for BroadPhaseLayers {
    fn get_num_broad_phase_layers(&self) -> u32 {
        BPL_COUNT as u32
    }

    fn get_broad_phase_layer(&self, layer: ObjectLayer) -> BroadPhaseLayer {
        match layer.raw() {
            OL_NON_MOVING => BroadPhaseLayer::new(BPL_NON_MOVING),
            OL_MOVING => BroadPhaseLayer::new(BPL_MOVING),
            _ => unreachable!(),
        }
    }
}

unsafe extern "C" fn ovb_should_collide(
    _this: *const c_void,
    layer1: JPC_ObjectLayer,
    layer2: JPC_BroadPhaseLayer,
) -> bool {
    match layer1 {
        OL_NON_MOVING => layer2 == BPL_MOVING,
        OL_MOVING => true,
        _ => unreachable!(),
    }
}

const OVB: jolt_sys::JPC_ObjectVsBroadPhaseLayerFilterFns =
    jolt_sys::JPC_ObjectVsBroadPhaseLayerFilterFns {
        ShouldCollide: Some(ovb_should_collide as _),
    };

unsafe extern "C" fn ovo_should_collide(
    _this: *const c_void,
    layer1: JPC_ObjectLayer,
    layer2: JPC_ObjectLayer,
) -> bool {
    match layer1 {
        OL_NON_MOVING => layer2 == OL_MOVING,
        OL_MOVING => true,
        _ => unreachable!(),
    }
}

const OVO: jolt_sys::JPC_ObjectLayerPairFilterFns = jolt_sys::JPC_ObjectLayerPairFilterFns {
    ShouldCollide: Some(ovo_should_collide as _),
};

fn main() {
    unsafe {
        jolt_sys::JPC_RegisterDefaultAllocator();
        jolt_sys::JPC_FactoryInit();
        jolt_sys::JPC_RegisterTypes();

        let temp_allocator = jolt_sys::JPC_TempAllocatorImpl_new(10 * 1024 * 1024);

        let job_system = jolt_sys::JPC_JobSystemThreadPool_new2(
            jolt_sys::JPC_MAX_PHYSICS_JOBS as _,
            jolt_sys::JPC_MAX_PHYSICS_BARRIERS as _,
        );

        let broad_phase_layer_interface = BroadPhaseLayers.leak_raw();

        let object_vs_broad_phase_layer_filter = jolt_sys::JPC_ObjectVsBroadPhaseLayerFilter {
            self_: ptr::null_mut(),
            fns: OVB,
        };

        let object_vs_object_layer_filter = jolt_sys::JPC_ObjectLayerPairFilter {
            self_: ptr::null_mut(),
            fns: OVO,
        };

        let physics_system = jolt_sys::JPC_PhysicsSystem_new();

        let max_bodies = 1024;
        let num_body_mutexes = 0;
        let max_body_pairs = 1024;
        let max_contact_constraints = 1024;

        jolt_sys::JPC_PhysicsSystem_Init(
            physics_system,
            max_bodies,
            num_body_mutexes,
            max_body_pairs,
            max_contact_constraints,
            broad_phase_layer_interface,
            object_vs_broad_phase_layer_filter,
            object_vs_object_layer_filter,
        );

        // TODO: register body activation listener
        // TODO: register contact listener
        // TODO: body interface
        // TODO: creating bodies
        // TODO: PhysicsSystem::OptimizeBroadPhase

        let delta_time = 1.0 / 60.0;
        let collision_steps = 1;

        // TODO: Update loop
        for _i in 0..100 {
            jolt_sys::JPC_PhysicsSystem_Update(
                physics_system,
                delta_time,
                collision_steps,
                temp_allocator,
                job_system,
            );
        }

        // TODO: RemoveBody and DestroyBody

        jolt_sys::JPC_PhysicsSystem_delete(physics_system);
        jolt_sys::JPC_JobSystemThreadPool_delete(job_system);
        jolt_sys::JPC_TempAllocatorImpl_delete(temp_allocator);

        jolt_sys::JPC_UnregisterTypes();
        jolt_sys::JPC_FactoryDelete();
    }

    println!("Hello, world!");
}

#[test]
fn run_main() {
    main();
}
