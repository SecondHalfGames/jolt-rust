#![forbid(unsafe_code)]

use jolt::{BroadPhaseLayer, BroadPhaseLayerInterface, ObjectLayer};

const OL_NON_MOVING: ObjectLayer = ObjectLayer::new(0);
const OL_MOVING: ObjectLayer = ObjectLayer::new(1);

const BPL_NON_MOVING: BroadPhaseLayer = BroadPhaseLayer::new(0);
const BPL_MOVING: BroadPhaseLayer = BroadPhaseLayer::new(1);

struct BroadPhaseLayers;

impl BroadPhaseLayerInterface for BroadPhaseLayers {
    fn get_num_broad_phase_layers(&self) -> u32 {
        2
    }

    fn get_broad_phase_layer(&self, layer: jolt::ObjectLayer) -> jolt::BroadPhaseLayer {
        match layer {
            OL_NON_MOVING => BPL_NON_MOVING,
            OL_MOVING => BPL_MOVING,
            _ => unreachable!(),
        }
    }
}

fn main() {
    let _bpl = BroadPhaseLayers.leak_raw();

    println!("Hello, world!");
}
