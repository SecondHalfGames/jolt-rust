#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(feature = "double-precision")]
pub type Real = f64;

#[cfg(not(feature = "double-precision"))]
pub type Real = f32;
