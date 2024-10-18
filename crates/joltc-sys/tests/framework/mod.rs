use std::ffi::{c_uint, c_void, CStr, CString};
use std::ptr;

// Everything prefixed with `JPC_` comes from the joltc_sys crate.
use joltc_sys::*;

pub const OL_NON_MOVING: JPC_ObjectLayer = 0;
pub const OL_MOVING: JPC_ObjectLayer = 1;

pub const BPL_NON_MOVING: JPC_BroadPhaseLayer = 0;
pub const BPL_MOVING: JPC_BroadPhaseLayer = 1;
pub const BPL_COUNT: JPC_BroadPhaseLayer = 2;

#[allow(unused_variables)]
pub trait SmokeTest {
    unsafe fn setup(system: *mut JPC_PhysicsSystem) -> Self;

    unsafe fn post_update(&mut self, system: *mut JPC_PhysicsSystem) -> bool {
        false
    }

    unsafe fn teardown(&mut self, system: *mut JPC_PhysicsSystem) {}
}

pub fn create_box(settings: &JPC_BoxShapeSettings) -> Result<*mut JPC_Shape, CString> {
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

pub fn create_sphere(settings: &JPC_SphereShapeSettings) -> Result<*mut JPC_Shape, CString> {
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

pub fn create_convex_hull(
    settings: &JPC_ConvexHullShapeSettings,
) -> Result<*mut JPC_Shape, CString> {
    let mut shape: *mut JPC_Shape = ptr::null_mut();
    let mut err: *mut JPC_String = ptr::null_mut();

    unsafe {
        if JPC_ConvexHullShapeSettings_Create(settings, &mut shape, &mut err) {
            Ok(shape)
        } else {
            Err(CStr::from_ptr(JPC_String_c_str(err)).to_owned())
        }
    }
}

pub fn vec3(x: f32, y: f32, z: f32) -> JPC_Vec3 {
    JPC_Vec3 { x, y, z, _w: z }
}

pub fn rvec3(x: Real, y: Real, z: Real) -> JPC_RVec3 {
    JPC_RVec3 { x, y, z, _w: z }
}

pub fn vec4(x: f32, y: f32, z: f32, w: f32) -> JPC_Vec4 {
    JPC_Vec4 { x, y, z, w }
}

// If 'double-precision' is set, there is padding in this struct
#[allow(clippy::needless_update)]
pub fn rmat44_identity() -> JPC_RMat44 {
    unsafe {
        JPC_RMat44 {
            col: [
                vec4(1.0, 0.0, 0.0, 0.0),
                vec4(0.0, 1.0, 0.0, 0.0),
                vec4(0.0, 0.0, 1.0, 0.0),
            ],
            col3: rvec3(0.0, 0.0, 0.0),
            ..std::mem::zeroed()
        }
    }
}

pub fn rmat44_translation(col3: JPC_RVec3) -> JPC_RMat44 {
    JPC_RMat44 {
        col3,
        ..rmat44_identity()
    }
}

fn global_init() {
    use std::sync::OnceLock;

    static INITIALIZED: OnceLock<()> = OnceLock::new();

    INITIALIZED.get_or_init(|| unsafe {
        JPC_RegisterDefaultAllocator();
        JPC_FactoryInit();
        JPC_RegisterTypes();
    });
}

pub fn run_test<S: SmokeTest>() {
    global_init();

    unsafe {
        let temp_allocator = JPC_TempAllocatorImpl_new(10 * 1024 * 1024);

        let job_system =
            JPC_JobSystemThreadPool_new2(JPC_MAX_PHYSICS_JOBS as _, JPC_MAX_PHYSICS_BARRIERS as _);

        let broad_phase_layer_interface = JPC_BroadPhaseLayerInterface_new(ptr::null(), BPL);

        let object_vs_broad_phase_layer_filter =
            JPC_ObjectVsBroadPhaseLayerFilter_new(ptr::null_mut(), OVB);

        let object_vs_object_layer_filter = JPC_ObjectLayerPairFilter_new(ptr::null_mut(), OVO);

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

        let mut test = S::setup(physics_system);

        // TODO: register body activation listener
        // TODO: register contact listener

        // TODO: PhysicsSystem::OptimizeBroadPhase

        let delta_time = 1.0 / 60.0;
        let collision_steps = 1;

        loop {
            JPC_PhysicsSystem_Update(
                physics_system,
                delta_time,
                collision_steps,
                temp_allocator,
                job_system,
            );

            if !test.post_update(physics_system) {
                break;
            }
        }

        test.teardown(physics_system);

        JPC_PhysicsSystem_delete(physics_system);
        JPC_BroadPhaseLayerInterface_delete(broad_phase_layer_interface);
        JPC_ObjectVsBroadPhaseLayerFilter_delete(object_vs_broad_phase_layer_filter);
        JPC_ObjectLayerPairFilter_delete(object_vs_object_layer_filter);

        JPC_JobSystemThreadPool_delete(job_system);
        JPC_TempAllocatorImpl_delete(temp_allocator);
    }
}

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
