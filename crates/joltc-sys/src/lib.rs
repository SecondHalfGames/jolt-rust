mod generated;

pub use generated::*;

#[cfg(feature = "double-precision")]
pub type Real = f64;

#[cfg(not(feature = "double-precision"))]
pub type Real = f32;

impl Default for JPC_BodyCreationSettings {
    fn default() -> Self {
        unsafe {
            let mut settings = std::mem::MaybeUninit::<JPC_BodyCreationSettings>::zeroed();
            JPC_BodyCreationSettings_default(settings.as_mut_ptr());
            settings.assume_init()
        }
    }
}
