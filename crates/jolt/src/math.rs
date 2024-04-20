use jolt_sys::{JPC_Color, JPC_DVec3, JPC_Quat, JPC_Vec3};

pub use jolt_sys::Real;

#[cfg(feature = "double-precision")]
pub type RVec3 = DVec3;

#[cfg(not(feature = "double-precision"))]
pub type RVec3 = Vec3;

#[repr(C, align(16))]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}

impl From<Vec3> for JPC_Vec3 {
    fn from(value: Vec3) -> Self {
        Self {
            x: value.x,
            y: value.y,
            z: value.z,
            _w: value.z,
        }
    }
}

impl From<JPC_Vec3> for Vec3 {
    fn from(value: JPC_Vec3) -> Self {
        Self {
            x: value.x,
            y: value.y,
            z: value.z,
        }
    }
}

#[repr(C, align(16))]
pub struct DVec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl DVec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
}

impl From<DVec3> for JPC_DVec3 {
    fn from(value: DVec3) -> Self {
        Self {
            x: value.x,
            y: value.y,
            z: value.z,
            _w: value.z,
        }
    }
}

impl From<JPC_DVec3> for DVec3 {
    fn from(value: JPC_DVec3) -> Self {
        Self {
            x: value.x,
            y: value.y,
            z: value.z,
        }
    }
}

#[repr(C)]
pub struct Quat {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Quat {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }

    pub fn identity() -> Self {
        Self::new(0.0, 0.0, 0.0, 1.0)
    }
}

impl From<Quat> for JPC_Quat {
    fn from(value: Quat) -> Self {
        Self {
            x: value.x,
            y: value.y,
            z: value.z,
            w: value.w,
        }
    }
}

impl From<JPC_Quat> for Quat {
    fn from(value: JPC_Quat) -> Self {
        Self {
            x: value.x,
            y: value.y,
            z: value.z,
            w: value.w,
        }
    }
}

#[repr(C)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
}

impl From<Color> for JPC_Color {
    fn from(value: Color) -> Self {
        Self {
            r: value.r,
            g: value.g,
            b: value.b,
            a: value.a,
        }
    }
}

impl From<JPC_Color> for Color {
    fn from(value: JPC_Color) -> Self {
        Self {
            r: value.r,
            g: value.g,
            b: value.b,
            a: value.a,
        }
    }
}
