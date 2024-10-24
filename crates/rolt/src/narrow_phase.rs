use std::marker::PhantomData;
use std::mem;

use joltc_sys::*;

use crate::{
    BodyFilterImpl, BodyId, BroadPhaseLayerFilterImpl, CastShapeBase, CastShapeCollector,
    CastShapeCollectorImpl, FromJolt, IntoJolt, ObjectLayerFilterImpl, RVec3, Vec3,
};

/// See also: Jolt's [`NarrowPhaseQuery`](https://jrouwe.github.io/JoltPhysicsDocs/5.1.0/class_narrow_phase_query.html) class.
pub struct NarrowPhaseQuery<'physics_system> {
    raw: *const JPC_NarrowPhaseQuery,
    _phantom: PhantomData<&'physics_system ()>,
}

/// See also: Jolt's [`RRayCast`](https://jrouwe.github.io/JoltPhysicsDocs/5.1.0/struct_r_ray_cast.html) class.
#[derive(Debug, Default, Clone, Copy)]
pub struct RRayCast {
    /// Origin of the ray.
    pub origin: RVec3,

    /// Direction and length of the ray. Anything beyond this length will not be
    /// reported as a hit.
    pub direction: Vec3,
}

impl RRayCast {
    pub fn as_raw(&self) -> JPC_RRayCast {
        JPC_RRayCast {
            Origin: self.origin.into_jolt(),
            Direction: self.direction.into_jolt(),
        }
    }
}

/// Arguments for [`NarrowPhaseQuery::cast_ray`].
#[derive(Default)]
pub struct RayCastArgs {
    pub ray: RRayCast,
    pub broad_phase_layer_filter: Option<BroadPhaseLayerFilterImpl<'static>>,
    pub object_layer_filter: Option<ObjectLayerFilterImpl<'static>>,
    pub body_filter: Option<BodyFilterImpl<'static>>,
}

/// The result of calling [`NarrowPhaseQuery::cast_ray`].
#[derive(Debug, Clone, Copy)]
pub struct RayCastResult {
    pub body_id: BodyId,
    pub fraction: f32,
    pub sub_shape_id: JPC_SubShapeID,
}

impl FromJolt for RayCastResult {
    type Jolt = JPC_RayCastResult;

    fn from_jolt(value: Self::Jolt) -> Self {
        Self {
            body_id: BodyId::new(value.BodyID),
            fraction: value.Fraction,
            sub_shape_id: value.SubShapeID2,
        }
    }
}

pub struct RShapeCast {
    pub shape: *const JPC_Shape,
    pub scale: Vec3,
    pub center_of_mass_start: JPC_RMat44,
    pub direction: Vec3,
    // const JPC_AABox ShapeWorldBounds;
}

pub struct CastShapeArgs<'a> {
    pub shapecast: RShapeCast,
    pub base_offset: RVec3,
    pub settings: JPC_ShapeCastSettings,
    pub collector: Option<CastShapeCollectorImpl<'a>>,
    pub broad_phase_layer_filter: Option<BroadPhaseLayerFilterImpl<'static>>,
    pub object_layer_filter: Option<ObjectLayerFilterImpl<'static>>,
    pub body_filter: Option<BodyFilterImpl<'static>>,
    // const JPC_ShapeFilter *ShapeFilter;
}

#[non_exhaustive]
#[derive(Default)]
pub struct ClosestHitCastShapeCollector {
    pub result: Option<JPC_ShapeCastResult>,
}

impl ClosestHitCastShapeCollector {
    pub fn new() -> Self {
        Self { result: None }
    }
}

impl CastShapeCollector for ClosestHitCastShapeCollector {
    fn reset(&mut self) {
        self.result = None;
    }

    fn add_hit(&mut self, base: &mut CastShapeBase, result: &JPC_ShapeCastResult) {
        fn early_out_fraction(result: &JPC_ShapeCastResult) -> f32 {
            if result.Fraction > 0.0 {
                result.Fraction
            } else {
                -result.PenetrationDepth
            }
        }

        let early_out = early_out_fraction(result);

        let set = self
            .result
            .map(|old| early_out < early_out_fraction(&old))
            .unwrap_or(true);

        if set {
            base.update_early_out_fraction(early_out);
            self.result = Some(*result);
        }
    }
}

impl<'physics_system> NarrowPhaseQuery<'physics_system> {
    pub(crate) fn new(raw: *const JPC_NarrowPhaseQuery) -> Self {
        Self {
            raw,
            _phantom: PhantomData,
        }
    }

    pub fn cast_ray(&self, args: RayCastArgs) -> Option<RayCastResult> {
        let mut raw_args = JPC_NarrowPhaseQuery_CastRayArgs {
            Ray: args.ray.as_raw(),
            Result: unsafe { mem::zeroed() },
            BroadPhaseLayerFilter: args.broad_phase_layer_filter.as_ref().into_jolt(),
            ObjectLayerFilter: args.object_layer_filter.as_ref().into_jolt(),
            BodyFilter: args.body_filter.as_ref().into_jolt(),
        };

        let hit = unsafe { JPC_NarrowPhaseQuery_CastRay(self.raw, &mut raw_args) };

        if hit {
            Some(RayCastResult::from_jolt(raw_args.Result))
        } else {
            None
        }
    }

    pub unsafe fn cast_shape(&self, args: CastShapeArgs<'_>) {
        let mut raw_args = JPC_NarrowPhaseQuery_CastShapeArgs {
            ShapeCast: JPC_RShapeCast {
                Shape: args.shapecast.shape,
                Scale: args.shapecast.scale.into_jolt(),
                CenterOfMassStart: args.shapecast.center_of_mass_start,
                Direction: args.shapecast.direction.into_jolt(),
                // const JPC_AABox ShapeWorldBounds;
                ..mem::zeroed()
            },
            Settings: args.settings,
            BaseOffset: args.base_offset.into_jolt(),
            Collector: args.collector.as_ref().into_jolt(),
            BroadPhaseLayerFilter: args.broad_phase_layer_filter.as_ref().into_jolt(),
            ObjectLayerFilter: args.object_layer_filter.as_ref().into_jolt(),
            BodyFilter: args.body_filter.as_ref().into_jolt(),
            // const JPC_ShapeFilter *ShapeFilter;
            ..mem::zeroed()
        };

        JPC_NarrowPhaseQuery_CastShape(self.raw, &mut raw_args);
    }

    pub fn as_raw(&self) -> *const JPC_NarrowPhaseQuery {
        self.raw
    }
}
