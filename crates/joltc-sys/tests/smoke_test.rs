mod framework;

use joltc_sys::*;

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

        let floor_shape_settings = JPC_BoxShapeSettings_new(vec3(100.0, 1.0, 100.0));
        let floor_shape = create_shape(floor_shape_settings.cast()).unwrap();

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

        let sphere_shape_settings = JPC_SphereShapeSettings_new(0.5);
        let sphere_shape = create_shape(sphere_shape_settings.cast()).unwrap();

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
