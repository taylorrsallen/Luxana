use crate::*;

use bevy::{
    render::{
        camera::{RenderTarget, Viewport, camera_system},
        view::VisibilitySystems
    },
    window::{WindowRef, PrimaryWindow},
    transform::TransformSystem,
    pbr::{
        ScreenSpaceAmbientOcclusionBundle,
        ScreenSpaceAmbientOcclusionSettings,
        ScreenSpaceAmbientOcclusionQualityLevel
    },
    core_pipeline::{clear_color::ClearColorConfig, tonemapping::Tonemapping}
};

mod anchor;
pub use anchor::*;
mod focus;
pub use focus::*;
mod orbit;
pub use orbit::*;
mod zoom;
pub use zoom::*;

////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct TankCameraPlugin;
impl Plugin for TankCameraPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<CameraRig>()
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
pub struct Cameras;
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
}

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Bundle)]
pub struct GuiCameraBundle {
    pub camera_2d: Camera2dBundle,
}

impl Default for GuiCameraBundle {
    fn default() -> Self {
        Self {
            camera_2d: Camera2dBundle {
                camera_2d: Camera2d { clear_color: ClearColorConfig::None },
                tonemapping: Tonemapping::AcesFitted,
                ..default()
            },
        }
    }
}

#[derive(Bundle)]
pub struct MainCameraBundle {
    pub camera_3d: Camera3dBundle,
    pub audio_receiver: AudioReceiver,
    pub fog_settings: FogSettings,
    // pub ssao_bundle: ScreenSpaceAmbientOcclusionBundle,
}

impl Default for MainCameraBundle {
    fn default() -> Self {
        Self {
            camera_3d: Camera3dBundle {
                camera_3d: Camera3d { clear_color: ClearColorConfig::None, ..default() },
                projection: Projection::Perspective(PerspectiveProjection { fov: 60.0 * 0.01745329, ..default() }),
                tonemapping: Tonemapping::AcesFitted,
                ..default()
            },
            audio_receiver: AudioReceiver,
            fog_settings: FogSettings {
                color: Color::rgba(0.1, 0.2, 0.4, 1.0),
                directional_light_color: Color::rgba(1.0, 0.95, 0.75, 0.5),
                directional_light_exponent: 500.0,
                falloff: FogFalloff::from_visibility_colors(20.0, Color::rgb(0.35, 0.5, 0.66), Color::rgb(0.8, 0.844, 1.0))
            },
            // ssao_bundle: ScreenSpaceAmbientOcclusionBundle::default(),
        }
    }
}