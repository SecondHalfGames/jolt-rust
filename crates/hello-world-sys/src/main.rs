use std::ffi::{c_uint, c_void, CStr};
use std::mem::MaybeUninit;
use std::ptr;

use jolt_sys::{
    JPC_BodyInterface_AddBody, JPC_BodyInterface_CreateBody,
    JPC_BodyInterface_GetCenterOfMassPosition, JPC_BodyInterface_GetLinearVelocity, JPC_Body_GetID,
    JPC_BroadPhaseLayer, JPC_ObjectLayer, Real, JPC_ACTIVATION_ACTIVATE,
    JPC_ACTIVATION_DONT_ACTIVATE, JPC_MOTION_TYPE_DYNAMIC, JPC_MOTION_TYPE_STATIC,
};

const OL_NON_MOVING: JPC_ObjectLayer = 0;
const OL_MOVING: JPC_ObjectLayer = 1;

const BPL_NON_MOVING: JPC_BroadPhaseLayer = 0;
const BPL_MOVING: JPC_BroadPhaseLayer = 1;
const BPL_COUNT: JPC_BroadPhaseLayer = 2;

unsafe extern "C" fn bpl_get_num_broad_phase_layers(_this: *const c_void) -> c_uint {
    BPL_COUNT as _
}

unsafe extern "C" fn bpl_get_broad_phase_layer(
    _this: *const c_void,
    layer: JPC_ObjectLayer,
) -> JPC_BroadPhaseLayer {
    match layer {
        OL_NON_MOVING => BPL_NON_MOVING,
        OL_MOVING => BPL_MOVING,
        _ => unreachable!(),
    }
}

const BPL: jolt_sys::JPC_BroadPhaseLayerInterfaceFns = jolt_sys::JPC_BroadPhaseLayerInterfaceFns {
    GetNumBroadPhaseLayers: Some(bpl_get_num_broad_phase_layers as _),
    GetBroadPhaseLayer: Some(bpl_get_broad_phase_layer as _),
};

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

fn vec3(x: f32, y: f32, z: f32) -> jolt_sys::JPC_Vec3 {
    jolt_sys::JPC_Vec3 { x, y, z, _w: z }
}

fn rvec3(x: Real, y: Real, z: Real) -> jolt_sys::JPC_RVec3 {
    jolt_sys::JPC_RVec3 { x, y, z, _w: z }
}

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

        let broad_phase_layer_interface =
            jolt_sys::JPC_BroadPhaseLayerInterface_new(ptr::null(), BPL);

        let object_vs_broad_phase_layer_filter =
            jolt_sys::JPC_ObjectVsBroadPhaseLayerFilter_new(ptr::null_mut(), OVB);

        let object_vs_object_layer_filter =
            jolt_sys::JPC_ObjectLayerPairFilter_new(ptr::null_mut(), OVO);

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

        let body_interface = jolt_sys::JPC_PhysicsSystem_GetBodyInterface(physics_system);

        let floor_shape_settings = jolt_sys::JPC_BoxShapeSettings_new(vec3(100.0, 1.0, 100.0));

        let mut floor_shape: *mut jolt_sys::JPC_Shape = ptr::null_mut();
        let mut err: *mut jolt_sys::JPC_String = ptr::null_mut();
        if !jolt_sys::JPC_ShapeSettings_Create(
            floor_shape_settings.cast::<jolt_sys::JPC_ShapeSettings>(),
            &mut floor_shape,
            &mut err,
        ) {
            panic!(
                "Fatal error: {:?}",
                CStr::from_ptr(jolt_sys::JPC_String_c_str(err))
            );
        }

        let mut floor_settings = MaybeUninit::<jolt_sys::JPC_BodyCreationSettings>::zeroed();
        jolt_sys::JPC_BodyCreationSettings_default(floor_settings.as_mut_ptr());
        let mut floor_settings = floor_settings.assume_init();
        floor_settings.Position = rvec3(0.0, -1.0, 0.0);
        floor_settings.MotionType = JPC_MOTION_TYPE_STATIC;
        floor_settings.ObjectLayer = OL_NON_MOVING;
        floor_settings.Shape = floor_shape; // FIXME: Should be const

        let floor = JPC_BodyInterface_CreateBody(body_interface, &mut floor_settings);
        JPC_BodyInterface_AddBody(
            body_interface,
            JPC_Body_GetID(floor),
            JPC_ACTIVATION_DONT_ACTIVATE,
        );

        let sphere_shape_settings = jolt_sys::JPC_SphereShapeSettings_new(0.5);

        let mut sphere_shape: *mut jolt_sys::JPC_Shape = ptr::null_mut();
        let mut err: *mut jolt_sys::JPC_String = ptr::null_mut();
        if !jolt_sys::JPC_ShapeSettings_Create(
            sphere_shape_settings.cast::<jolt_sys::JPC_ShapeSettings>(),
            &mut sphere_shape,
            &mut err,
        ) {
            panic!(
                "Fatal error: {:?}",
                CStr::from_ptr(jolt_sys::JPC_String_c_str(err))
            );
        }

        let mut sphere_settings = MaybeUninit::<jolt_sys::JPC_BodyCreationSettings>::zeroed();
        jolt_sys::JPC_BodyCreationSettings_default(sphere_settings.as_mut_ptr());
        let mut sphere_settings = sphere_settings.assume_init();
        sphere_settings.Position = rvec3(0.0, 2.0, 0.0);
        sphere_settings.MotionType = JPC_MOTION_TYPE_DYNAMIC;
        sphere_settings.ObjectLayer = OL_MOVING;
        sphere_settings.Shape = sphere_shape; // FIXME: Should be const

        let sphere = JPC_BodyInterface_CreateBody(body_interface, &mut sphere_settings);
        let sphere_id = JPC_Body_GetID(sphere);
        JPC_BodyInterface_AddBody(body_interface, sphere_id, JPC_ACTIVATION_ACTIVATE);

        // TODO: PhysicsSystem::OptimizeBroadPhase

        let delta_time = 1.0 / 60.0;
        let collision_steps = 1;

        // TODO: Update loop
        for i in 0..35 {
            let position = JPC_BodyInterface_GetCenterOfMassPosition(body_interface, sphere_id);
            let velocity = JPC_BodyInterface_GetLinearVelocity(body_interface, sphere_id);
            println!(
                "Step {i}: Position = ({}, {}, {}), Velocity = ({}, {}, {})",
                position.x, position.y, position.z, velocity.x, velocity.y, velocity.z
            );

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
        jolt_sys::JPC_BroadPhaseLayerInterface_delete(broad_phase_layer_interface);
        jolt_sys::JPC_ObjectVsBroadPhaseLayerFilter_delete(object_vs_broad_phase_layer_filter);
        jolt_sys::JPC_ObjectLayerPairFilter_delete(object_vs_object_layer_filter);

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
