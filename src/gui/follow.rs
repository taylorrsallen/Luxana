use crate::*;

use bevy::window::PrimaryWindow;
use bevy_egui::egui::{RichText, FontId, Align2, Color32, TextStyle, FontFamily};

////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct LuxanaGuiFollowPlugin;
impl Plugin for LuxanaGuiFollowPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<GuiFollow>()
            .add_systems(Update, sys_update_gui_follow);
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Component, Default, Reflect)]
pub struct GuiFollow {
    pub pos: Vec3,
    pub camera: Option<Entity>,
}

impl GuiFollow {
    pub fn from_camera(camera: Entity) -> Self {
        Self { camera: Some(camera), ..default() }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
fn sys_update_gui_follow(
    mut follow_query: Query<(&mut GuiPos, &GuiFollow)>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
) {
    for (mut pos, follow) in follow_query.iter_mut() {
        let camera_entity = if let Some(camera_entity) = follow.camera { camera_entity } else { continue };
        let (camera, camera_transform) = if let Ok(camera) = camera_query.get(camera_entity) { camera } else { continue };
        let viewport_pos = if let Some(viewport_pos) = camera.world_to_viewport(camera_transform, follow.pos) { viewport_pos } else { continue };
        *pos = GuiPos::from_px_vec2(viewport_pos);
    }
}