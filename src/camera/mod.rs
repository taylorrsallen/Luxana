use crate::*;

use bevy::{
    render::{
        camera::{RenderTarget, Viewport, camera_system},
        view::VisibilitySystems, RenderSet
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

////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct TankCameraPlugin;
impl Plugin for TankCameraPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<CameraRig>()
            .register_type::<CameraMatchTargetUp>()
            .register_type::<CameraMatchTargetOrientation>()
            .register_type::<CameraAnchor>()
            .register_type::<CameraOrbit>()
            .register_type::<CameraZoom>()
            .add_systems(PostUpdate, sys_update_camera_rig
                .after(TransformSystem::TransformPropagate)
                .before(RenderSet::ManageViews)
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

#[derive(Component, Default, Debug, Reflect)]
#[reflect(Component, Default)]
pub struct CameraMatchTargetUp(pub TransformTargetRef);

#[derive(Component, Default, Debug, Reflect)]
#[reflect(Component, Default)]
pub struct CameraMatchTargetOrientation(pub TransformTargetRef);

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Component, Default, Debug, Reflect)]
#[reflect(Component, Default)]
pub struct CameraAnchor {
    pub target: TransformTargetRef,
    pub offset: Vec3,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Component, Default, Debug, Reflect)]
#[reflect(Component, Default)]
pub struct CameraFocus {
    pub target: TransformTargetRef,
    pub offset: Vec3,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Component, Default, Debug, Reflect)]
#[reflect(Component, Default)]
pub struct CameraOrbit {
    pub offset: Vec2,
    pub rotation: Vec2,
}

impl CameraOrbit {
    pub fn new(offset: Vec2) -> Self { Self { offset, rotation: Vec2::ZERO } }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Component, Default, Debug, Reflect)]
#[reflect(Component, Default)]
pub struct CameraZoom(pub f32);

impl CameraZoom {
    pub fn new(zoom: f32) -> Self { Self { 0: zoom } }
    pub fn get(&self) -> f32 { self.0 }
    pub fn get_mut(&mut self) -> &mut f32 { &mut self.0 }
    pub fn set(&mut self, zoom: f32) { self.0 = zoom }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
fn sys_update_camera_rig(
    mut rig_query: Query<(Entity, &mut CameraRig), With<Camera3d>>,
    mut global_transform_query: Query<&mut GlobalTransform>,
    mut transform_query: Query<&mut Transform>,
    anchor_query: Query<&CameraAnchor>,
    focus_query: Query<&CameraFocus>,
    orbit_query: Query<&CameraOrbit>,
    zoom_query: Query<&CameraZoom>,
    match_up_query: Query<&CameraMatchTargetUp>,
    match_orientation_query: Query<&CameraMatchTargetOrientation>,
) {
    for(camera_entity, mut camera_rig) in rig_query.iter_mut() {
        try_match_target_up(camera_entity, &mut camera_rig, &match_up_query, &global_transform_query);
        try_match_target_orientation(camera_entity, &mut camera_rig, &match_orientation_query, &global_transform_query);

        let mut new_camera_transform = Transform::IDENTITY;
        if let Ok(camera_anchor) = anchor_query.get(camera_entity) { try_update_camera_anchor(&mut new_camera_transform, &camera_rig, camera_anchor, &global_transform_query); }
        if let Ok(camera_focus) = focus_query.get(camera_entity) { try_update_camera_focus(&mut new_camera_transform, &camera_rig, camera_focus, &global_transform_query); }
        if let Ok(camera_orbit) = orbit_query.get(camera_entity) { update_camera_orbit(&mut new_camera_transform, camera_orbit); }
        if let Ok(camera_zoom) = zoom_query.get(camera_entity) { update_camera_zoom(&mut new_camera_transform, camera_zoom); }

        let mut camera_global_transform = global_transform_query.get_mut(camera_entity).unwrap();
        let mut camera_transform = transform_query.get_mut(camera_entity).unwrap();
        *camera_global_transform = GlobalTransform::from(new_camera_transform.clone());
        *camera_transform = new_camera_transform;
    }

    fn try_match_target_up(
        camera_entity: Entity,
        camera_rig: &mut CameraRig,
        match_up_query: &Query<&CameraMatchTargetUp>,
        transform_query: &Query<&mut GlobalTransform>,
    ) {
        let Ok(match_up) = match_up_query.get(camera_entity) else { return };
        let Some(target_entity) = match_up.0.try_get_entity() else { return };
        let Ok(target_transform) = transform_query.get(target_entity) else { return; };
        camera_rig.up = target_transform.up();
    }

    fn try_match_target_orientation(
        camera_entity: Entity,
        camera_rig: &mut CameraRig,
        match_orientation_query: &Query<&CameraMatchTargetOrientation>,
        transform_query: &Query<&mut GlobalTransform>,
    ) {
        let Ok(match_orientation) = match_orientation_query.get(camera_entity) else { return };
        let Some(target_entity) = match_orientation.0.try_get_entity() else { return };
        let Ok(target_transform) = transform_query.get(target_entity) else { return; };
        camera_rig.right = target_transform.right();
        camera_rig.up = target_transform.up();
        camera_rig.forward = target_transform.forward();
    }

    fn try_update_camera_anchor(
        new_camera_transform: &mut Transform,
        camera_rig: &CameraRig,
        camera_anchor: &CameraAnchor,
        transform_query: &Query<&mut GlobalTransform>,
    ) {
        new_camera_transform.translation = if let Some(pos) = camera_anchor.target.try_get_pos_mut_query(&transform_query) {
            pos + camera_rig.right * camera_anchor.offset.x + camera_rig.up * camera_anchor.offset.y + camera_rig.forward * camera_anchor.offset.z
        } else {
            return
        };
    }

    fn try_update_camera_focus(
        new_camera_transform: &mut Transform,
        camera_rig: &CameraRig,
        camera_focus: &CameraFocus,
        transform_query: &Query<&mut GlobalTransform>,
    ) {
        let target_pos = if let Some(pos) = camera_focus.target.try_get_pos_mut_query(&transform_query) { pos + camera_focus.offset } else { return };
        new_camera_transform.look_at(target_pos, camera_rig.up);
    }

    fn update_camera_orbit(
        new_camera_transform: &mut Transform,
        camera_orbit: &CameraOrbit,
    ) {
        new_camera_transform.rotate_y(-camera_orbit.rotation.x);
        let yaw_right = new_camera_transform.right().normalize() * camera_orbit.offset.x;
        let yaw_up = new_camera_transform.up().normalize() * camera_orbit.offset.y;
        new_camera_transform.rotate_local_x(-camera_orbit.rotation.y);
        new_camera_transform.translation += yaw_right + yaw_up;
    }

    fn update_camera_zoom(
        new_camera_transform: &mut Transform,
        camera_zoom: &CameraZoom,
    ) {
        let zoom_offset = new_camera_transform.forward() * camera_zoom.get();
        new_camera_transform.translation -= zoom_offset;
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
                falloff: FogFalloff::from_visibility_colors(40.0, Color::rgb(0.35, 0.5, 0.66), Color::rgb(0.8, 0.844, 1.0))
            },
            // ssao_bundle: ScreenSpaceAmbientOcclusionBundle::default(),
        }
    }
}