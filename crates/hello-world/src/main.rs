// someday:
// #![forbid(unsafe_code)]

use std::ffi::{c_void, CStr, CString};
use std::ptr;

// Everything prefixed with `JPC_` comes from the joltc_sys crate.
use joltc_sys::*;

use rolt::{BodyId, BroadPhaseLayer, BroadPhaseLayerInterface, ObjectLayer, Vec3};

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
    rolt::register_default_allocator();
    rolt::factory_init();
    rolt::register_types();

    unsafe {
        let temp_allocator = JPC_TempAllocatorImpl_new(10 * 1024 * 1024);

        let job_system =
            JPC_JobSystemThreadPool_new2(JPC_MAX_PHYSICS_JOBS as _, JPC_MAX_PHYSICS_BARRIERS as _);

        let broad_phase_layer_interface = BroadPhaseLayers;

        let object_vs_broad_phase_layer_filter =
            JPC_ObjectVsBroadPhaseLayerFilter_new(ptr::null(), OVB);

        let object_vs_object_layer_filter = JPC_ObjectLayerPairFilter_new(ptr::null(), OVO);

        let physics_system = rolt::PhysicsSystem::new();

        let max_bodies = 1024;
        let num_body_mutexes = 0;
        let max_body_pairs = 1024;
        let max_contact_constraints = 1024;

        physics_system.init(
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

        let body_interface = physics_system.body_interface();

        let floor_shape_settings = JPC_BoxShapeSettings_new(vec3(100.0, 1.0, 100.0));
        let floor_shape = create_shape(floor_shape_settings.cast()).unwrap();

        let floor = body_interface.create_body(&JPC_BodyCreationSettings {
            Position: rvec3(0.0, -1.0, 0.0),
            MotionType: JPC_MOTION_TYPE_STATIC,
            ObjectLayer: OL_NON_MOVING,
            Shape: floor_shape,
            ..Default::default()
        });
        let floor_id = BodyId::new(JPC_Body_GetID(floor));
        body_interface.add_body(floor_id, JPC_ACTIVATION_DONT_ACTIVATE);

        let sphere_shape_settings = JPC_SphereShapeSettings_new(0.5);
        let sphere_shape = create_shape(sphere_shape_settings.cast()).unwrap();

        let sphere = body_interface.create_body(&JPC_BodyCreationSettings {
            Position: rvec3(0.0, 2.0, 0.0),
            MotionType: JPC_MOTION_TYPE_DYNAMIC,
            ObjectLayer: OL_MOVING,
            Shape: sphere_shape,
            ..Default::default()
        });
        let sphere_id = BodyId::new(JPC_Body_GetID(sphere));

        body_interface.add_body(sphere_id, JPC_ACTIVATION_ACTIVATE);
        body_interface.set_linear_velocity(sphere_id, Vec3::new(0.0, -5.0, 0.0));

        physics_system.optimize_broad_phase();

        let delta_time = 1.0 / 60.0;
        let collision_steps = 1;

        let mut step = 0;
        while body_interface.is_active(sphere_id) {
            step += 1;

            let position = body_interface.center_of_mass_position(sphere_id);
            let velocity = body_interface.linear_velocity(sphere_id);
            println!(
                "Step {step}: Position = ({}, {}, {}), Velocity = ({}, {}, {})",
                position.x, position.y, position.z, velocity.x, velocity.y, velocity.z
            );

            physics_system.update(delta_time, collision_steps, temp_allocator, job_system);
        }

        body_interface.remove_body(floor_id);
        body_interface.destroy_body(floor_id);

        body_interface.remove_body(sphere_id);
        body_interface.destroy_body(sphere_id);

        drop(physics_system);

        JPC_ObjectVsBroadPhaseLayerFilter_delete(object_vs_broad_phase_layer_filter);
        JPC_ObjectLayerPairFilter_delete(object_vs_object_layer_filter);

        JPC_JobSystemThreadPool_delete(job_system);
        JPC_TempAllocatorImpl_delete(temp_allocator);
    }

    rolt::unregister_types();
    rolt::factory_delete();

    println!("Hello, world!");
}

unsafe fn create_shape(settings: *const JPC_ShapeSettings) -> Result<*mut JPC_Shape, CString> {
    let mut shape: *mut JPC_Shape = ptr::null_mut();
    let mut err: *mut JPC_String = ptr::null_mut();

    if JPC_ShapeSettings_Create(settings, &mut shape, &mut err) {
        Ok(shape)
    } else {
        Err(CStr::from_ptr(JPC_String_c_str(err)).to_owned())
    }
}

#[test]
fn run_main() {
    main();
}
