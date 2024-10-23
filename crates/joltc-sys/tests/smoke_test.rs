mod framework;

use std::f32::consts::{PI, TAU};
use std::ffi::c_void;
use std::mem;
use std::ptr::addr_of_mut;

use joltc_sys::*;
use rand::Rng;

use crate::framework::*;

struct SetupTeardown;

impl SmokeTest for SetupTeardown {
    unsafe fn setup(_system: *mut JPC_PhysicsSystem) -> Self {
        Self
    }
}

#[test]
fn setup_teardown() {
    run_test::<SetupTeardown>();
}

struct HelloShapes {
    floor: JPC_BodyID,
    sphere: JPC_BodyID,
}

impl SmokeTest for HelloShapes {
    unsafe fn setup(system: *mut JPC_PhysicsSystem) -> Self {
        let body_interface = JPC_PhysicsSystem_GetBodyInterface(system);

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
        let floor_id = JPC_Body_GetID(floor);
        JPC_BodyInterface_AddBody(body_interface, floor_id, JPC_ACTIVATION_DONT_ACTIVATE);

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

        Self {
            sphere: sphere_id,
            floor: floor_id,
        }
    }

    unsafe fn post_update(&mut self, system: *mut JPC_PhysicsSystem) -> bool {
        let body_interface = JPC_PhysicsSystem_GetBodyInterface(system);
        JPC_BodyInterface_IsActive(body_interface, self.sphere)
    }

    unsafe fn teardown(&mut self, system: *mut JPC_PhysicsSystem) {
        let body_interface = JPC_PhysicsSystem_GetBodyInterface(system);

        JPC_BodyInterface_RemoveBody(body_interface, self.floor);
        JPC_BodyInterface_DestroyBody(body_interface, self.floor);

        JPC_BodyInterface_RemoveBody(body_interface, self.sphere);
        JPC_BodyInterface_DestroyBody(body_interface, self.sphere);
    }
}

#[test]
fn hello_shapes() {
    run_test::<HelloShapes>();
}

struct HelloConvexHull {
    floor: JPC_BodyID,
    hull: JPC_BodyID,
}

impl SmokeTest for HelloConvexHull {
    unsafe fn setup(system: *mut JPC_PhysicsSystem) -> Self {
        let body_interface = JPC_PhysicsSystem_GetBodyInterface(system);

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
        let floor_id = JPC_Body_GetID(floor);
        JPC_BodyInterface_AddBody(body_interface, floor_id, JPC_ACTIVATION_DONT_ACTIVATE);

        let mut rng = rand::thread_rng();
        let mut points = Vec::with_capacity(200);
        for _ in 0..200 {
            let theta = rng.gen_range(0.0..PI);
            let phi = rng.gen_range(0.0..TAU);
            points.push(vec3(
                theta.sin() * phi.cos(),
                theta.sin() * phi.sin(),
                theta.cos(),
            ));
        }

        let hull_shape = create_convex_hull(&JPC_ConvexHullShapeSettings {
            Points: points.as_ptr(),
            PointsLen: points.len(),
            ..Default::default()
        })
        .unwrap();

        let hull_settings = JPC_BodyCreationSettings {
            Position: rvec3(0.0, 2.0, 0.0),
            MotionType: JPC_MOTION_TYPE_DYNAMIC,
            ObjectLayer: OL_MOVING,
            Shape: hull_shape,
            ..Default::default()
        };

        let hull = JPC_BodyInterface_CreateBody(body_interface, &hull_settings);
        let hull_id = JPC_Body_GetID(hull);
        JPC_BodyInterface_AddBody(body_interface, hull_id, JPC_ACTIVATION_ACTIVATE);

        JPC_BodyInterface_SetLinearVelocity(body_interface, hull_id, vec3(0.0, -5.0, 0.0));

        Self {
            hull: hull_id,
            floor: floor_id,
        }
    }

    unsafe fn post_update(&mut self, system: *mut JPC_PhysicsSystem) -> bool {
        let body_interface = JPC_PhysicsSystem_GetBodyInterface(system);
        JPC_BodyInterface_IsActive(body_interface, self.hull)
    }

    unsafe fn teardown(&mut self, system: *mut JPC_PhysicsSystem) {
        let body_interface = JPC_PhysicsSystem_GetBodyInterface(system);

        JPC_BodyInterface_RemoveBody(body_interface, self.floor);
        JPC_BodyInterface_DestroyBody(body_interface, self.floor);

        JPC_BodyInterface_RemoveBody(body_interface, self.hull);
        JPC_BodyInterface_DestroyBody(body_interface, self.hull);
    }
}

#[test]
fn hello_convex_hull() {
    run_test::<HelloConvexHull>();
}

struct NarrowPhaseRayCast {
    sphere: JPC_BodyID,
}

impl SmokeTest for NarrowPhaseRayCast {
    unsafe fn setup(system: *mut JPC_PhysicsSystem) -> Self {
        let body_interface = JPC_PhysicsSystem_GetBodyInterface(system);

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

        let query = JPC_PhysicsSystem_GetNarrowPhaseQuery(system);

        let mut args = JPC_NarrowPhaseQuery_CastRayArgs {
            Ray: JPC_RRayCast {
                Origin: rvec3(1.0, 2.0, 0.0),
                Direction: vec3(-2.0, 0.0, 0.0),
            },
            ..mem::zeroed()
        };
        let hit = JPC_NarrowPhaseQuery_CastRay(query, &mut args);

        assert!(hit, "ray should hit the sphere");
        assert!(
            (args.Result.Fraction - 0.25).abs() < 0.01,
            "ray should hit at around 0.25 fraction"
        );

        Self { sphere: sphere_id }
    }

    unsafe fn post_update(&mut self, _system: *mut JPC_PhysicsSystem) -> bool {
        false
    }

    unsafe fn teardown(&mut self, system: *mut JPC_PhysicsSystem) {
        let body_interface = JPC_PhysicsSystem_GetBodyInterface(system);

        JPC_BodyInterface_RemoveBody(body_interface, self.sphere);
        JPC_BodyInterface_DestroyBody(body_interface, self.sphere);
    }
}

#[test]
fn narrow_phase_ray_cast() {
    run_test::<NarrowPhaseRayCast>();
}

struct NarrowPhaseShapeCast {
    sphere: JPC_BodyID,
}

impl SmokeTest for NarrowPhaseShapeCast {
    unsafe fn setup(system: *mut JPC_PhysicsSystem) -> Self {
        let body_interface = JPC_PhysicsSystem_GetBodyInterface(system);

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

        let query = JPC_PhysicsSystem_GetNarrowPhaseQuery(system);

        #[derive(Default)]
        struct CollectorState {
            result: Option<JPC_ShapeCastResult>,
        }

        unsafe extern "C" fn reset(this: *mut c_void) {
            let this = this.cast::<CollectorState>();
            (*this).result = None;
        }

        unsafe extern "C" fn add_hit(this: *mut c_void, result: *const JPC_ShapeCastResult) {
            let this = this.cast::<CollectorState>();
            (*this).result = Some(*result);
        }

        let collector_fns = JPC_CastShapeCollectorFns {
            Reset: Some(reset as _),
            AddHit: Some(add_hit as _),
        };

        let mut collector_state = CollectorState::default();
        let collector = JPC_CastShapeCollector_new(
            addr_of_mut!(collector_state).cast::<c_void>(),
            collector_fns,
        );

        let mut args = JPC_NarrowPhaseQuery_CastShapeArgs {
            ShapeCast: JPC_RShapeCast {
                Shape: sphere_shape,
                Scale: vec3(1.0, 1.0, 1.0),
                CenterOfMassStart: rmat44_translation(rvec3(-5.0, 2.0, 0.0)),
                Direction: vec3(5.0, 0.0, 0.0),
                ..mem::zeroed()
            },
            Settings: Default::default(),
            BaseOffset: rvec3(0.0, 0.0, 0.0),
            Collector: collector,
            ..mem::zeroed()
        };
        JPC_NarrowPhaseQuery_CastShape(query, &mut args);

        JPC_CastShapeCollector_delete(collector);

        let result = collector_state
            .result
            .expect("sphere should hit the other sphere");

        assert!(
            (result.Fraction - 0.8).abs() < 0.01,
            "ray should hit at around 0.8 fraction"
        );

        Self { sphere: sphere_id }
    }

    unsafe fn post_update(&mut self, _system: *mut JPC_PhysicsSystem) -> bool {
        false
    }

    unsafe fn teardown(&mut self, system: *mut JPC_PhysicsSystem) {
        let body_interface = JPC_PhysicsSystem_GetBodyInterface(system);

        JPC_BodyInterface_RemoveBody(body_interface, self.sphere);
        JPC_BodyInterface_DestroyBody(body_interface, self.sphere);
    }
}

#[test]
fn narrow_phase_shape_cast() {
    run_test::<NarrowPhaseShapeCast>();
}
