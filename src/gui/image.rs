use crate::*;

use bevy_egui::egui::TextureId;

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Component, Debug)]
pub struct GuiImage {
    id: TextureId,
    dims: Vec2,
}

#[derive(Component, Default, Debug, Reflect)]
#[reflect(Component, Default)]
pub struct SerializedGuiImage {
    name: String,
    dims: Vec2,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
pub fn sys_deserialize_gui_image(

) {

}