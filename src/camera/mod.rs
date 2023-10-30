use crate::*;

use bevy::{render::{camera::{RenderTarget, Viewport, camera_system}, view::VisibilitySystems}, window::{WindowRef, PrimaryWindow}, transform::TransformSystem};

mod anchor;
pub use anchor::*;
mod focus;
pub use focus::*;
mod orbit;
pub use orbit::*;
mod zoom;
pub use zoom::*;

////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct LuxanaCameraPlugin;
impl Plugin for LuxanaCameraPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<TransformTargetRef>()
            .register_type::<CameraRig>()
            .register_type::<CameraAnchor>()
            .register_type::<CameraOrbit>()
            .register_type::<CameraZoom>()
            .add_systems(PostUpdate, (
                sys_update_camera_up,
                sys_update_camera_orientation,
                sys_update_camera_anchor,
                sys_update_camera_focus,
                sys_update_camera_orbit,
                sys_update_camera_zoom,
            ).chain()
                .after(TransformSystem::TransformPropagate)
                .before(VisibilitySystems::CalculateBounds)
                .before(VisibilitySystems::UpdateOrthographicFrusta)
                .before(VisibilitySystems::UpdatePerspectiveFrusta)
                .before(VisibilitySystems::UpdateProjectionFrusta)
                .before(VisibilitySystems::VisibilityPropagate)
                .before(VisibilitySystems::CheckVisibility)
            );
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Component, Reflect)]
pub struct CameraRig {
    pub right: Vec3,
    pub up: Vec3,
    pub forward: Vec3,
}

impl Default for CameraRig {
    fn default() -> Self { Self { right: Vec3::X, up: Vec3::Y, forward: Vec3::NEG_Z } }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct CameraMatchTargetOrientation(pub TransformTargetRef);

fn sys_update_camera_orientation(
    mut camera_query: Query<(&mut CameraRig, &CameraMatchTargetOrientation)>,
    transform_query: Query<&GlobalTransform, Changed<GlobalTransform>>,
) {
    for (mut rig, match_target) in camera_query.iter_mut() {
        let target_entity = if let Some(entity) = match_target.0.try_get_entity() { entity } else { continue };
        let target_transform = if let Ok(transform) = transform_query.get(target_entity) { transform } else { continue };
        rig.right = target_transform.right();
        rig.up = target_transform.up();
        rig.forward = target_transform.forward();
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct CameraMatchTargetUp(pub TransformTargetRef);

fn sys_update_camera_up(
    mut camera_query: Query<(&mut CameraRig, &CameraMatchTargetUp), Without<CameraMatchTargetOrientation>>,
    transform_query: Query<&GlobalTransform, Changed<GlobalTransform>>,
) {
    for (mut rig, match_target) in camera_query.iter_mut() {
        let target_entity = if let Some(entity) = match_target.0.try_get_entity() { entity } else { continue };
        let target_transform = if let Ok(transform) = transform_query.get(target_entity) { transform } else { continue };
        rig.up = target_transform.up();
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Resource, Default)]
pub struct Cameras(Vec<Entity>);

impl Cameras {
    pub fn window_entity_from_camera(camera: &Camera, primary_window_query: &Query<Entity, With<PrimaryWindow>>) -> Entity {
        Self::window_entity_from_render_target(&camera.target, primary_window_query)
    }

    pub fn window_entity_from_render_target(render_target: &RenderTarget, primary_window_query: &Query<Entity, With<PrimaryWindow>>) -> Entity {
        match render_target {
            RenderTarget::Window(window_ref) => { Self::window_entity_from_ref(window_ref, primary_window_query) }
            RenderTarget::Image(_) => { panic!("RenderTarget::Image unsupported"); }
            RenderTarget::TextureView(_) => { panic!("RenderTarget::TextureView unsupported"); }
        }
    }

    pub fn window_entity_from_ref(window_ref: &WindowRef, primary_window_query: &Query<Entity, With<PrimaryWindow>>) -> Entity {
        match window_ref {
            WindowRef::Primary => { primary_window_query.get_single().unwrap() }
            WindowRef::Entity(entity) => { *entity }
        }
    }

    // fn splitscreen_viewport_position(camera_count: usize, order: usize, resolution_width: u32, resolution_height: u32) -> UVec2 {
    //     match camera_count {
    //         1 => { UVec2::new(0, 0) }
    //         2 => { UVec2::new((resolution_width / 2) * order as u32, 0) }
    //         3 => {
    //             if order == 0 {
    //                 UVec2::new(0, 0)
    //             } else {
    //                 UVec2::new((resolution_width / 2) * (order as u32 - 1), resolution_height / 2)
    //             }
    //         }
    //         4 => { UVec2::new((resolution_width / 2) * (order as u32 % 2), if order < 2 { 0 } else { resolution_height / 2 }) }
    //         _ => { panic!("Requested viewport position for invalid camera_count!") }
    //     }
    // }

    // fn splitscreen_viewport_size(camera_count: usize, order: usize, resolution_width: u32, resolution_height: u32) -> UVec2 {
    //     match camera_count {
    //         1 => { UVec2::new(resolution_width, resolution_height) }
    //         2 => { UVec2::new(resolution_width / 2, resolution_height) }
    //         3 => {
    //             if order == 0 {
    //                 UVec2::new(resolution_width, resolution_height / 2)
    //             } else {
    //                 UVec2::new(resolution_width / 2, resolution_height / 2)
    //             }
    //         }
    //         4 => { UVec2::new(resolution_width / 2, resolution_height / 2) }
    //         _ => { panic!("Requested viewport size for invalid camera_count!") }
    //     }
    // }

    // fn splitscreen_viewport(camera_count: usize, order: usize, resolution_width: u32, resolution_height: u32) -> Viewport {
    //     Viewport {
    //         physical_position: Self::splitscreen_viewport_position(camera_count, order, resolution_width, resolution_height),
    //         physical_size: Self::splitscreen_viewport_size(camera_count, order, resolution_width, resolution_height),
    //         ..default()
    //     }
    // }
}