use crate::*;

use bevy::window::PrimaryWindow;

////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct TankGuiLabelPlugin;
impl Plugin for TankGuiLabelPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<GuiLabel>();
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Component, Reflect)]
pub struct GuiLabel {
    pub content: String,
    pub size: f32,
}