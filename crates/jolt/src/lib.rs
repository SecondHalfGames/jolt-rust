use std::ffi::{c_uint, c_void};
use std::marker::PhantomData;
use std::ptr;

use jolt_sys::{
    JPC_BroadPhaseLayer, JPC_BroadPhaseLayerInterface, JPC_BroadPhaseLayerInterfaceFns,
    JPC_ObjectLayer,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ObjectLayer(JPC_ObjectLayer);

impl ObjectLayer {
    pub const fn new(value: u16) -> Self {
        Self(value)
    }

    pub const fn raw(self) -> JPC_ObjectLayer {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BroadPhaseLayer(JPC_BroadPhaseLayer);

impl BroadPhaseLayer {
    pub const fn new(value: u8) -> Self {
        Self(value)
    }

    pub const fn raw(self) -> JPC_BroadPhaseLayer {
        self.0
    }
}

pub trait BroadPhaseLayerInterface: Sized {
    fn get_num_broad_phase_layers(&self) -> u32;
    fn get_broad_phase_layer(&self, layer: ObjectLayer) -> BroadPhaseLayer;

    fn leak_raw(self) -> JPC_BroadPhaseLayerInterface {
        // FIXME: cursed
        let this = Box::leak(Box::new(self));
        jpc_bpli(this)
    }
}

struct BroadPhaseLayerInterfaceBridge<T> {
    _phantom: PhantomData<T>,
}

#[allow(non_snake_case)]
impl<T: BroadPhaseLayerInterface> BroadPhaseLayerInterfaceBridge<T> {
    unsafe extern "C" fn GetNumBroadPhaseLayers(this: *mut c_void) -> c_uint {
        let this = this.cast::<T>().as_mut().unwrap();

        this.get_num_broad_phase_layers()
    }

    unsafe extern "C" fn GetBroadPhaseLayer(
        this: *mut c_void,
        layer: JPC_ObjectLayer,
    ) -> JPC_BroadPhaseLayer {
        let this = this.cast::<T>().as_mut().unwrap();
        let layer = ObjectLayer(layer);

        this.get_broad_phase_layer(layer).raw()
    }
}

fn jpc_bpli<T>(input: &mut T) -> JPC_BroadPhaseLayerInterface
where
    T: BroadPhaseLayerInterface,
{
    type Bridge<T> = BroadPhaseLayerInterfaceBridge<T>;

    let fns = JPC_BroadPhaseLayerInterfaceFns {
        GetNumBroadPhaseLayers: Some(Bridge::<T>::GetNumBroadPhaseLayers as _),
        GetBroadPhaseLayer: Some(Bridge::<T>::GetBroadPhaseLayer as _),
    };

    JPC_BroadPhaseLayerInterface {
        self_: ptr::from_mut(input).cast::<c_void>(),
        fns,
    }
}
