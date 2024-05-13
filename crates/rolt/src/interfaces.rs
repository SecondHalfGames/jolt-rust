use std::ffi::{c_uint, c_void};
use std::marker::PhantomData;
use std::ptr;

use joltc_sys::*;

use crate::remote_drop::RemoteDrop;
use crate::{BroadPhaseLayer, ObjectLayer};

#[allow(dead_code)]
pub struct BroadPhaseLayerInterfaceImpl {
    raw: *mut JPC_BroadPhaseLayerInterface,
    remote_this: Option<RemoteDrop>,
}

impl BroadPhaseLayerInterfaceImpl {
    pub fn new<T: BroadPhaseLayerInterface + 'static>(value: T) -> Self {
        type Bridge<T> = BroadPhaseLayerInterfaceBridge<T>;

        let fns = JPC_BroadPhaseLayerInterfaceFns {
            GetNumBroadPhaseLayers: Some(Bridge::<T>::GetNumBroadPhaseLayers as _),
            GetBroadPhaseLayer: Some(Bridge::<T>::GetBroadPhaseLayer as _),
        };

        let this = Box::into_raw(Box::new(value));

        let raw = unsafe { JPC_BroadPhaseLayerInterface_new(this.cast::<c_void>(), fns) };
        let remote_this = unsafe { RemoteDrop::new(this) };

        Self {
            raw,
            remote_this: Some(remote_this),
        }
    }

    pub unsafe fn from_raw(this: *const c_void, fns: JPC_BroadPhaseLayerInterfaceFns) -> Self {
        let raw = unsafe { JPC_BroadPhaseLayerInterface_new(this, fns) };

        Self {
            raw,
            remote_this: None,
        }
    }

    pub fn as_raw(&self) -> *mut JPC_BroadPhaseLayerInterface {
        self.raw
    }
}

impl Drop for BroadPhaseLayerInterfaceImpl {
    fn drop(&mut self) {
        unsafe {
            JPC_BroadPhaseLayerInterface_delete(self.raw);
        }
    }
}

impl<T> From<T> for BroadPhaseLayerInterfaceImpl
where
    T: BroadPhaseLayerInterface + 'static,
{
    fn from(value: T) -> Self {
        Self::new(value)
    }
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
