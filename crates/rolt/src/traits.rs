#![allow(non_snake_case)]

use std::ffi::{c_uint, c_void};
use std::marker::PhantomData;

use joltc_sys::*;
use paste::paste;

use crate::remote_drop::RemoteDrop;
use crate::{Body, BodyId, BroadPhaseLayer, ObjectLayer};

macro_rules! define_impl_struct {
    (
        $base_name:ident {
            $($method:ident),* $(,)?
        }
    ) => {
        paste! {
            #[allow(dead_code)]
            #[doc = "Holds an implementation of the [" $base_name "] trait or the manual vtable equivalent."]
            pub struct [<$base_name Impl>] {
                raw: *mut [<JPC_ $base_name >],
                remote_this: Option<RemoteDrop>,
            }

            impl [<$base_name Impl>] {
                pub fn new<T: $base_name + 'static>(value: T) -> Self {
                    type Bridge<T> = [< $base_name Bridge >]<T>;

                    let fns = [<JPC_ $base_name Fns>] {
                        $(
                            $method: Some(Bridge::<T>::$method as _),
                        )*
                    };

                    let this = Box::into_raw(Box::new(value));

                    let raw = unsafe { [<JPC_ $base_name _new>](this.cast::<c_void>(), fns) };
                    let remote_this = unsafe { RemoteDrop::new(this) };

                    Self {
                        raw,
                        remote_this: Some(remote_this),
                    }
                }

                pub unsafe fn from_raw(this: *const c_void, fns: [<JPC_ $base_name Fns>]) -> Self {
                    let raw = unsafe { [<JPC_ $base_name _new>](this, fns) };

                    Self {
                        raw,
                        remote_this: None,
                    }
                }

                pub unsafe fn new_existing(raw: *mut [<JPC_ $base_name>]) -> Self {
                    Self {
                        raw,
                        remote_this: None,
                    }
                }

                pub fn as_raw(&self) -> *mut [<JPC_ $base_name>] {
                    self.raw
                }
            }

            impl Drop for [<$base_name Impl>] {
                fn drop(&mut self) {
                    unsafe {
                        [<JPC_ $base_name _delete>](self.raw);
                    }
                }
            }

            impl<T> From<T> for [<$base_name Impl>]
            where
                T: $base_name + 'static,
            {
                fn from(value: T) -> Self {
                    Self::new(value)
                }
            }
        }
    };
}

/// See also: Jolt's [`BroadPhaseLayerInterface`](https://secondhalfgames.github.io/jolt-docs/5.0.0/class_broad_phase_layer_interface.html) class.
pub trait BroadPhaseLayerInterface {
    fn get_num_broad_phase_layers(&self) -> u32;
    fn get_broad_phase_layer(&self, layer: ObjectLayer) -> BroadPhaseLayer;
}

define_impl_struct!(BroadPhaseLayerInterface {
    GetNumBroadPhaseLayers,
    GetBroadPhaseLayer,
});

struct BroadPhaseLayerInterfaceBridge<T> {
    _phantom: PhantomData<T>,
}

impl<T: BroadPhaseLayerInterface> BroadPhaseLayerInterfaceBridge<T> {
    unsafe extern "C" fn GetNumBroadPhaseLayers(this: *const c_void) -> c_uint {
        let this = this.cast::<T>().as_ref().unwrap();

        this.get_num_broad_phase_layers()
    }

    unsafe extern "C" fn GetBroadPhaseLayer(
        this: *const c_void,
        layer: JPC_ObjectLayer,
    ) -> JPC_BroadPhaseLayer {
        let this = this.cast::<T>().as_ref().unwrap();
        let layer = ObjectLayer::new(layer);

        this.get_broad_phase_layer(layer).raw()
    }
}

/// See also: Jolt's [`ObjectVsBroadPhaseLayerFilter`](https://secondhalfgames.github.io/jolt-docs/5.0.0/class_object_vs_broad_phase_layer_filter.html) class.
pub trait ObjectVsBroadPhaseLayerFilter {
    fn should_collide(&self, layer1: ObjectLayer, layer2: BroadPhaseLayer) -> bool;
}

define_impl_struct!(ObjectVsBroadPhaseLayerFilter { ShouldCollide });

struct ObjectVsBroadPhaseLayerFilterBridge<T> {
    _phantom: PhantomData<T>,
}

impl<T: ObjectVsBroadPhaseLayerFilter> ObjectVsBroadPhaseLayerFilterBridge<T> {
    unsafe extern "C" fn ShouldCollide(
        this: *const c_void,
        layer1: JPC_ObjectLayer,
        layer2: JPC_BroadPhaseLayer,
    ) -> bool {
        let this = this.cast::<T>().as_ref().unwrap();
        let layer1 = ObjectLayer::new(layer1);
        let layer2 = BroadPhaseLayer::new(layer2);

        this.should_collide(layer1, layer2)
    }
}

/// See also: Jolt's [`ObjectLayerPairFilter`](https://secondhalfgames.github.io/jolt-docs/5.0.0/class_object_layer_pair_filter.html) class.
pub trait ObjectLayerPairFilter {
    fn should_collide(&self, layer1: ObjectLayer, layer2: ObjectLayer) -> bool;
}

define_impl_struct!(ObjectLayerPairFilter { ShouldCollide });

struct ObjectLayerPairFilterBridge<T> {
    _phantom: PhantomData<T>,
}

impl<T: ObjectLayerPairFilter> ObjectLayerPairFilterBridge<T> {
    unsafe extern "C" fn ShouldCollide(
        this: *const c_void,
        layer1: JPC_ObjectLayer,
        layer2: JPC_ObjectLayer,
    ) -> bool {
        let this = this.cast::<T>().as_ref().unwrap();
        let layer1 = ObjectLayer::new(layer1);
        let layer2 = ObjectLayer::new(layer2);

        this.should_collide(layer1, layer2)
    }
}

/// See also: Jolt's [`BroadPhaseLayerFilter`](https://secondhalfgames.github.io/jolt-docs/5.0.0/class_broad_phase_layer_filter.html) class.
pub trait BroadPhaseLayerFilter {
    fn should_collide(&self, layer: BroadPhaseLayer) -> bool;
}

define_impl_struct!(BroadPhaseLayerFilter { ShouldCollide });

struct BroadPhaseLayerFilterBridge<T> {
    _phantom: PhantomData<T>,
}

impl<T: BroadPhaseLayerFilter> BroadPhaseLayerFilterBridge<T> {
    unsafe extern "C" fn ShouldCollide(this: *const c_void, layer: JPC_BroadPhaseLayer) -> bool {
        let this = this.cast::<T>().as_ref().unwrap();
        let layer = BroadPhaseLayer::new(layer);

        this.should_collide(layer)
    }
}

/// See also: Jolt's [`ObjectLayerFilter`](https://secondhalfgames.github.io/jolt-docs/5.0.0/class_object_layer_filter.html) class.
pub trait ObjectLayerFilter {
    fn should_collide(&self, layer: ObjectLayer) -> bool;
}

define_impl_struct!(ObjectLayerFilter { ShouldCollide });

struct ObjectLayerFilterBridge<T> {
    _phantom: PhantomData<T>,
}

impl<T: ObjectLayerFilter> ObjectLayerFilterBridge<T> {
    unsafe extern "C" fn ShouldCollide(this: *const c_void, layer: JPC_ObjectLayer) -> bool {
        let this = this.cast::<T>().as_ref().unwrap();
        let layer = ObjectLayer::new(layer);

        this.should_collide(layer)
    }
}

/// See also: Jolt's [`BodyFilter`](https://secondhalfgames.github.io/jolt-docs/5.0.0/class_body_filter.html) class.
pub trait BodyFilter {
    fn should_collide(&self, body_id: BodyId) -> bool;
    fn should_collide_locked(&self, body: &mut Body) -> bool;
}

define_impl_struct!(BodyFilter {
    ShouldCollide,
    ShouldCollideLocked
});

struct BodyFilterBridge<T> {
    _phantom: PhantomData<T>,
}

impl<T: BodyFilter> BodyFilterBridge<T> {
    unsafe extern "C" fn ShouldCollide(this: *const c_void, body_id: JPC_BodyID) -> bool {
        let this = this.cast::<T>().as_ref().unwrap();
        let body_id = BodyId::new(body_id);

        this.should_collide(body_id)
    }

    unsafe extern "C" fn ShouldCollideLocked(this: *const c_void, body: *const JPC_Body) -> bool {
        let this = this.cast::<T>().as_ref().unwrap();

        // FIXME: cast_mut should not be required here
        let mut body = Body::new(body.cast_mut());

        this.should_collide_locked(&mut body)
    }
}
