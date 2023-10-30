use crate::*;

use bevy::render::view::RenderLayers;

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Bundle)]
pub struct VisibleObjectBundle {
    visibility_bundle: VisibilityBundle,
    transform_bundle: TransformBundle,
    render_layers: RenderLayers,
}

impl Default for VisibleObjectBundle {
    fn default() -> Self {
        Self {
            visibility_bundle: VisibilityBundle::default(),
            transform_bundle: TransformBundle::default(),
            render_layers: RenderLayers::default(),
        }
    }
}

impl VisibleObjectBundle {
    pub fn new(transform: Transform) -> Self {
        Self {
            visibility_bundle: VisibilityBundle::default(),
            transform_bundle: TransformBundle::from_transform(transform),
            render_layers: RenderLayers::default(),
        }
    }
}