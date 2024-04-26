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
    JPC_BoxShapeSettings2 -> JPC_BoxShapeSettings2_default,
}
