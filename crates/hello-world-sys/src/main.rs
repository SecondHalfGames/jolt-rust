use std::ffi::{c_uint, c_void, CStr, CString};
use std::ptr;

// Everything prefixed with `JPC_` comes from the joltc_sys crate.
use joltc_sys::*;

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

const BPL: JPC_BroadPhaseLayerInterfaceFns = JPC_BroadPhaseLayerInterfaceFns {
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

const OVB: JPC_ObjectVsBroadPhaseLayerFilterFns = JPC_ObjectVsBroadPhaseLayerFilterFns {
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

const OVO: JPC_ObjectLayerPairFilterFns = JPC_ObjectLayerPairFilterFns {
    ShouldCollide: Some(ovo_should_collide as _),
};

fn vec3(x: f32, y: f32, z: f32) -> JPC_Vec3 {
    JPC_Vec3 { x, y, z, _w: z }
}

fn rvec3(x: Real, y: Real, z: Real) -> JPC_RVec3 {
    JPC_RVec3 { x, y, z, _w: z }
}

fn main() {
    unsafe {
        JPC_RegisterDefaultAllocator();
        JPC_FactoryInit();
        JPC_RegisterTypes();

        let temp_allocator = JPC_TempAllocatorImpl_new(10 * 1024 * 1024);

        let job_system =
            JPC_JobSystemThreadPool_new2(JPC_MAX_PHYSICS_JOBS as _, JPC_MAX_PHYSICS_BARRIERS as _);

        let broad_phase_layer_interface = JPC_BroadPhaseLayerInterface_new(ptr::null(), BPL);

        let object_vs_broad_phase_layer_filter =
            JPC_ObjectVsBroadPhaseLayerFilter_new(ptr::null(), OVB);

        let object_vs_object_layer_filter = JPC_ObjectLayerPairFilter_new(ptr::null(), OVO);

        let physics_system = JPC_PhysicsSystem_new();

        let max_bodies = 1024;
        let num_body_mutexes = 0;
        let max_body_pairs = 1024;
        let max_contact_constraints = 1024;

        JPC_PhysicsSystem_Init(
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

        let body_interface = JPC_PhysicsSystem_GetBodyInterface(physics_system);

        let floor_shape = create_box(&JPC_BoxShapeSettings {
            HalfExtent: vec3(100.0, 1.0, 100.0),
            ..Default::default()
        })
        .unwrap();

        let floor_settings = JPC_BodyCreationSettings {
            Position: rvec3(0.0, -1.0, 0.0),
            MotionType: JPC_MOTION_TYPE_STATIC,
            ObjectLayer: OL_NON_MOVING,
            Shape: floor_shape,
            ..Default::default()
        };

        let floor = JPC_BodyInterface_CreateBody(body_interface, &floor_settings);
        JPC_BodyInterface_AddBody(
            body_interface,
            JPC_Body_GetID(floor),
            JPC_ACTIVATION_DONT_ACTIVATE,
        );

        let sphere_shape = create_sphere(&JPC_SphereShapeSettings {
            Radius: 0.5,
            ..Default::default()
        })
        .unwrap();

        let sphere_settings = JPC_BodyCreationSettings {
            Position: rvec3(0.0, 2.0, 0.0),
            MotionType: JPC_MOTION_TYPE_DYNAMIC,
            ObjectLayer: OL_MOVING,
            Shape: sphere_shape,
            ..Default::default()
        };

        let sphere = JPC_BodyInterface_CreateBody(body_interface, &sphere_settings);
        let sphere_id = JPC_Body_GetID(sphere);
        JPC_BodyInterface_AddBody(body_interface, sphere_id, JPC_ACTIVATION_ACTIVATE);

        JPC_BodyInterface_SetLinearVelocity(body_interface, sphere_id, vec3(0.0, -5.0, 0.0));

        JPC_PhysicsSystem_OptimizeBroadPhase(physics_system);

        let delta_time = 1.0 / 60.0;
        let collision_steps = 1;

        let mut step = 0;
        while JPC_BodyInterface_IsActive(body_interface, sphere_id) {
            step += 1;

            let position = JPC_BodyInterface_GetCenterOfMassPosition(body_interface, sphere_id);
            let velocity = JPC_BodyInterface_GetLinearVelocity(body_interface, sphere_id);
            println!(
                "Step {step}: Position = ({}, {}, {}), Velocity = ({}, {}, {})",
                position.x, position.y, position.z, velocity.x, velocity.y, velocity.z
            );

            JPC_PhysicsSystem_Update(
                physics_system,
                delta_time,
                collision_steps,
                temp_allocator,
                job_system.cast::<JPC_JobSystem>(),
            );
        }

        JPC_BodyInterface_RemoveBody(body_interface, JPC_Body_GetID(floor));
        JPC_BodyInterface_DestroyBody(body_interface, JPC_Body_GetID(floor));

        JPC_BodyInterface_RemoveBody(body_interface, sphere_id);
        JPC_BodyInterface_DestroyBody(body_interface, sphere_id);

        JPC_PhysicsSystem_delete(physics_system);
        JPC_BroadPhaseLayerInterface_delete(broad_phase_layer_interface);
        JPC_ObjectVsBroadPhaseLayerFilter_delete(object_vs_broad_phase_layer_filter);
        JPC_ObjectLayerPairFilter_delete(object_vs_object_layer_filter);

        JPC_JobSystemThreadPool_delete(job_system);
        JPC_TempAllocatorImpl_delete(temp_allocator);

        JPC_UnregisterTypes();
        JPC_FactoryDelete();
    }

    println!("Hello, world!");
}

fn create_box(settings: &JPC_BoxShapeSettings) -> Result<*mut JPC_Shape, CString> {
    let mut shape: *mut JPC_Shape = ptr::null_mut();
    let mut err: *mut JPC_String = ptr::null_mut();

    unsafe {
        if JPC_BoxShapeSettings_Create(settings, &mut shape, &mut err) {
            Ok(shape)
        } else {
            Err(CStr::from_ptr(JPC_String_c_str(err)).to_owned())
        }
    }
}

fn create_sphere(settings: &JPC_SphereShapeSettings) -> Result<*mut JPC_Shape, CString> {
    let mut shape: *mut JPC_Shape = ptr::null_mut();
    let mut err: *mut JPC_String = ptr::null_mut();

    unsafe {
        if JPC_SphereShapeSettings_Create(settings, &mut shape, &mut err) {
            Ok(shape)
        } else {
            Err(CStr::from_ptr(JPC_String_c_str(err)).to_owned())
        }
    }
}

#[test]
fn run_main() {
    main();
}
