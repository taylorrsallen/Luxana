use crate::*;

use bevy::render::view::RenderLayers;

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Bundle)]
pub struct VisibleTransformBundle {
    visibility_bundle: VisibilityBundle,
    transform_bundle: TransformBundle,
}

impl Default for VisibleTransformBundle {
    fn default() -> Self {
        Self {
            visibility_bundle: VisibilityBundle::default(),
            transform_bundle: TransformBundle::default(),
        }
    }
}

impl VisibleTransformBundle {
    pub fn new(transform: Transform) -> Self {
        Self {
            visibility_bundle: VisibilityBundle::default(),
            transform_bundle: TransformBundle::from_transform(transform),
        }
    }
}