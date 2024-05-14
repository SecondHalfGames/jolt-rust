use std::ffi::{c_uint, c_void};
use std::marker::PhantomData;

use joltc_sys::*;
use paste::paste;

use crate::remote_drop::RemoteDrop;
use crate::{BroadPhaseLayer, ObjectLayer};

macro_rules! define_impl_struct {
    (
        $base_name:ident {
            $($method:ident,)*
        }
    ) => {
        paste! {
            #[allow(dead_code)]
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

#[allow(non_snake_case)]
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
