use joltc_sys::*;

/// Represents an object layer, which is internally either a u16 or a u32.
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

/// Represents a broad phase layer.
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

/// An ID that can be used to access a body using [`BodyInterface`][crate::BodyInterface].
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
