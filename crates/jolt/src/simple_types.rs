use jolt_sys::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ObjectLayer(JPC_ObjectLayer);

impl ObjectLayer {
    pub const fn new(value: JPC_ObjectLayer) -> Self {
        Self(value)
    }

    pub const fn raw(self) -> JPC_ObjectLayer {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BroadPhaseLayer(JPC_BroadPhaseLayer);

impl BroadPhaseLayer {
    pub const fn new(value: JPC_BroadPhaseLayer) -> Self {
        Self(value)
    }

    pub const fn raw(self) -> JPC_BroadPhaseLayer {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BodyId(JPC_BodyID);

impl BodyId {
    pub const fn new(value: JPC_BodyID) -> Self {
        Self(value)
    }

    pub const fn raw(self) -> JPC_BodyID {
        self.0
    }
}
