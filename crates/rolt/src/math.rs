use joltc_sys::{JPC_Color, JPC_DVec3, JPC_Mat44, JPC_Quat, JPC_Vec3, JPC_Vec4};

/// The type used for representing world space values.
///
/// Either `f32` (default) or `f64` (`double-precision` feature).
pub use joltc_sys::Real;

pub use glam::{DVec3, Mat4, Quat, Vec3, Vec4};

use crate::{FromJolt, IntoJolt};

/// Represents a world-space vector, which can use either `f32` or `f64`.
///
/// Because the `double-precision` feature is enabled, this uses `f64`.
#[cfg(feature = "double-precision")]
pub type RVec3 = DVec3;

/// Represents a world-space vector, which can use either `f32` or `f64`.
///
/// Because the `double-precision` feature is NOT enabled, this uses `f32`.
#[cfg(not(feature = "double-precision"))]
pub type RVec3 = Vec3;

impl IntoJolt for Vec3 {
    type Jolt = JPC_Vec3;

    fn into_jolt(self) -> Self::Jolt {
        JPC_Vec3 {
            x: self.x,
            y: self.y,
            z: self.z,
            _w: self.z,
        }
    }
}

impl FromJolt for Vec3 {
    type Jolt = JPC_Vec3;

    fn from_jolt(value: Self::Jolt) -> Self {
        Vec3::new(value.x, value.y, value.z)
    }
}

impl IntoJolt for Vec4 {
    type Jolt = JPC_Vec4;

    fn into_jolt(self) -> Self::Jolt {
        JPC_Vec4 {
            x: self.x,
            y: self.y,
            z: self.z,
            w: self.w,
        }
    }
}

impl FromJolt for Vec4 {
    type Jolt = JPC_Vec4;

    fn from_jolt(value: Self::Jolt) -> Self {
        Vec4::new(value.x, value.y, value.z, value.w)
    }
}

impl IntoJolt for DVec3 {
    type Jolt = JPC_DVec3;

    fn into_jolt(self) -> Self::Jolt {
        JPC_DVec3 {
            x: self.x,
            y: self.y,
            z: self.z,
            _w: self.z,
        }
    }
}

impl FromJolt for DVec3 {
    type Jolt = JPC_DVec3;

    fn from_jolt(value: Self::Jolt) -> Self {
        DVec3::new(value.x, value.y, value.z)
    }
}

impl IntoJolt for Quat {
    type Jolt = JPC_Quat;

    fn into_jolt(self) -> Self::Jolt {
        JPC_Quat {
            x: self.x,
            y: self.y,
            z: self.z,
            w: self.w,
        }
    }
}

impl FromJolt for Quat {
    type Jolt = JPC_Quat;

    fn from_jolt(value: Self::Jolt) -> Self {
        Quat::from_xyzw(value.x, value.y, value.z, value.w)
    }
}

impl IntoJolt for Mat4 {
    type Jolt = JPC_Mat44;

    fn into_jolt(self) -> Self::Jolt {
        JPC_Mat44 {
            matrix: [
                self.x_axis.into_jolt(),
                self.y_axis.into_jolt(),
                self.z_axis.into_jolt(),
                self.w_axis.into_jolt(),
            ],
        }
    }
}

impl FromJolt for Mat4 {
    type Jolt = JPC_Mat44;

    fn from_jolt(value: Self::Jolt) -> Self {
        Mat4::from_cols(
            Vec4::from_jolt(value.matrix[0]),
            Vec4::from_jolt(value.matrix[1]),
            Vec4::from_jolt(value.matrix[2]),
            Vec4::from_jolt(value.matrix[3]),
        )
    }
}

/// Represents an sRGB color with alpha.
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

impl IntoJolt for Color {
    type Jolt = JPC_Color;

    fn into_jolt(self) -> Self::Jolt {
        JPC_Color {
            r: self.r,
            g: self.g,
            b: self.b,
            a: self.a,
        }
    }
}

impl FromJolt for Color {
    type Jolt = JPC_Color;

    fn from_jolt(value: Self::Jolt) -> Self {
        Self {
            r: value.r,
            g: value.g,
            b: value.b,
            a: value.a,
        }
    }
}
