mod generated;

pub use generated::*;

#[cfg(feature = "double-precision")]
pub type Real = f64;

#[cfg(not(feature = "double-precision"))]
pub type Real = f32;

macro_rules! ffi_default {
    ($($c_struct:ident -> $default_fn:ident,)*) => {
        $(
            impl Default for $c_struct {
                fn default() -> Self {
                    unsafe {
                        let mut settings = std::mem::MaybeUninit::<$c_struct>::zeroed();
                        $default_fn(settings.as_mut_ptr());
                        settings.assume_init()
                    }
                }
            }
        )*
    };
}

ffi_default! {
    JPC_BodyCreationSettings -> JPC_BodyCreationSettings_default,

    // All of the ShapeSettings types
    JPC_TriangleShapeSettings -> JPC_TriangleShapeSettings_default,
    JPC_BoxShapeSettings -> JPC_BoxShapeSettings_default,
    JPC_SphereShapeSettings -> JPC_SphereShapeSettings_default,
    JPC_CapsuleShapeSettings -> JPC_CapsuleShapeSettings_default,
    JPC_CylinderShapeSettings -> JPC_CylinderShapeSettings_default,
    JPC_ConvexHullShapeSettings -> JPC_ConvexHullShapeSettings_default,
    JPC_SubShapeSettings -> JPC_SubShapeSettings_default,
    JPC_StaticCompoundShapeSettings -> JPC_StaticCompoundShapeSettings_default,
    JPC_MutableCompoundShapeSettings -> JPC_MutableCompoundShapeSettings_default,
}
