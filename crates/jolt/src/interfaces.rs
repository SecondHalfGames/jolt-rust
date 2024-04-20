use std::ffi::{c_uint, c_void};
use std::marker::PhantomData;
use std::ptr;

use jolt_sys::*;

use crate::{BroadPhaseLayer, ObjectLayer};

pub trait IntoBroadPhaseLayerInterface {
    fn as_raw(&self) -> *mut JPC_BroadPhaseLayerInterface;
}

pub trait BroadPhaseLayerInterface: Sized {
    fn get_num_broad_phase_layers(&self) -> u32;
    fn get_broad_phase_layer(&self, layer: ObjectLayer) -> BroadPhaseLayer;

    // FIXME: This is more like 'create_raw' because it allocates an object that
    // should be later deallocated.
    fn as_raw(&self) -> *mut JPC_BroadPhaseLayerInterface {
        type Bridge<T> = BroadPhaseLayerInterfaceBridge<T>;

        let fns = JPC_BroadPhaseLayerInterfaceFns {
            GetNumBroadPhaseLayers: Some(Bridge::<Self>::GetNumBroadPhaseLayers as _),
            GetBroadPhaseLayer: Some(Bridge::<Self>::GetBroadPhaseLayer as _),
        };

        unsafe { JPC_BroadPhaseLayerInterface_new(ptr::from_ref(self).cast::<c_void>(), fns) }
    }
}

impl IntoBroadPhaseLayerInterface for *mut JPC_BroadPhaseLayerInterface {
    fn as_raw(&self) -> *mut JPC_BroadPhaseLayerInterface {
        *self
    }
}

impl<T> IntoBroadPhaseLayerInterface for T
where
    T: BroadPhaseLayerInterface,
{
    fn as_raw(&self) -> *mut JPC_BroadPhaseLayerInterface {
        <Self as BroadPhaseLayerInterface>::as_raw(self)
    }
}

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
