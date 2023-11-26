use crate::*;

use bevy::window::PrimaryWindow;
use bevy_egui::egui::{Image, FontDefinitions, FontData, FontId, FontFamily, RichText, Align2, Color32, TextStyle, TextureId};

mod button;
pub use button::*;
mod color;
pub use color::*;
mod cursor;
pub use cursor::*;
mod follow;
pub use follow::*;
mod image;
pub use image::*;
mod collider;
pub use collider::*;
mod label;
pub use label::*;
mod layout;
pub use layout::*;
mod menu;
pub use menu::*;
mod pos;
pub use pos::*;

////////////////////////////////////////////////////////////////////////////////////////////////////
/// TODO: Make this a super plugin and un-plugin all the mini plugins so control flow is more controlled.
pub struct TankGuiPlugin;
impl Plugin for TankGuiPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<GuiData>()
            .add_plugins((
                TankGuiButtonPlugin,
                TankGuiCursorPlugin,
                TankGuiColorPlugin,
                TankGuiFollowPlugin,
                TankGuiHitboxPlugin,
                TankGuiLabelPlugin,
                TankGuiMenuPlugin,
                TankGuiPosPlugin,
            ))
            .add_systems(OnEnter(AppState::EngineInit), onsys_init_egui)
            .add_systems(Update, sys_update_gui_views);
            // .add_systems(PreUpdate, (
            //     sys_update_gui,
            // ).chain());
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Component, Default, Debug, Reflect)]
#[reflect(Component, Default)]
pub struct GuiData;

////////////////////////////////////////////////////////////////////////////////////////////////////
fn sys_update_gui_views(
    mut egui_context_query: Query<&mut EguiContext>,
    player_query: Query<(&PlayerGuiViewer, &PlayerMainCameraRef)>,
    primary_window_query: Query<Entity, With<PrimaryWindow>>,
    window_query: Query<&Window>,
    camera_query: Query<&Camera, With<Camera3d>>,
    children_query: Query<&Children>,
    mut egui_user_textures: ResMut<EguiUserTextures>,
    packages: Res<Packages>,
) {
    let mut egui_area_id = 0;
    for (gui_viewer, camera_ref) in player_query.iter() {
        let Some(camera_entity) = camera_ref.try_get().clone() else { continue };
        let Ok(camera) = camera_query.get(camera_entity) else { continue };
        let window_entity = Cameras::window_entity_from_camera(camera, &primary_window_query);
        let Ok(mut egui_context) = egui_context_query.get_mut(window_entity) else { continue };

        let viewport = if let Some(camera_viewport) = &camera.viewport {
                UVec2::new(camera_viewport.physical_size.x, camera_viewport.physical_size.y)
            } else {
                let Ok(window) = window_query.get(window_entity) else { continue };
                UVec2::new(window.physical_width(), window.physical_height())
            };

        let texture_id = egui_user_textures.add_image(packages.images.fetch_handle("icon").clone_weak());

        for gui_data_entity in gui_viewer.iter().copied() {
            egui::Area::new(egui_area_id.to_string())
                .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
                .show(egui_context.get_mut(), |ui| {
                    ui.add(Image::new(egui::load::SizedTexture::new(
                        texture_id, [512.0, 512.0],
                    )));
                });
        }
    }

    fn draw_gui_recursive(
        entity: Entity,
        children_query: Query<&Children>,
        egui_context: &mut EguiContext,
    ) {
        
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// fn sys_update_gui(
//     mut transform_query: Query<&mut Transform>,
//     mut egui_context_query: Query<&mut EguiContext, With<Window>>,
//     root_query: Query<(&GuiRoot, &GuiViewport, &Children)>,
//     pos_query: Query<&GuiPos, With<Parent>>,
//     z_layer_query: Query<&GuiZLayer, With<Parent>>,
//     color_query: Query<&GuiColor, With<Parent>>,
//     label_query: Query<&GuiLabel, With<Parent>>,
//     primary_window_query: Query<Entity, With<PrimaryWindow>>,
//     camera_query: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
// ) {
//     let mut egui_area_id = 0;
//     for (gui_root, gui_viewport, gui_children) in root_query.iter() {
//         let camera_entity = if let Some(camera) = gui_root.camera { camera } else { continue };
//         let (camera, camera_transform) = if let Ok(camera) = camera_query.get(camera_entity) { camera } else { continue };
//         let window_entity = Cameras::window_entity_from_camera(camera, &primary_window_query);

//         for child_entity in gui_children.iter() {
//             let gui_pos = if let Ok(gui_pos) = pos_query.get(*child_entity) { gui_pos } else { continue };

//             if let Ok(mut transform) = transform_query.get_mut(*child_entity) {
//                 let mut translation = if let Some(xy) = camera.viewport_to_world_2d(camera_transform, gui_pos.as_px_vec2(gui_viewport.get())) { Vec3::new(xy.x, xy.y, transform.translation.z) } else { continue };
//                 if let Ok(z_layer) = z_layer_query.get(*child_entity) { translation.z = z_layer.get() as f32; }
//                 transform.translation = translation;

//                 // Scale?
//             }

//             if let Ok(label) = label_query.get(*child_entity) {
//                 let mut ctx = if let Ok(ctx) = egui_context_query.get_mut(window_entity) { ctx } else { continue };
//                 let color = if let Ok(color) = color_query.get(*child_entity) { color.get().as_rgba_u8() } else { [255 as u8; 4] };

//                 egui::Area::new(egui_area_id.to_string())
//                     .anchor(Align2::CENTER_CENTER, gui_pos.as_px_center_offset(gui_viewport.get()))
//                     .show(ctx.get_mut(), |ui| {
//                         ui.add(egui::Label::new(RichText::new(&label.content)
//                             .font(FontId { size: label.size, family: FontFamily::Name("hack_regular".into()) })
//                             .color(Color32::from_rgba_unmultiplied(color[0], color[1], color[2], color[3]))
//                         ).wrap(false));
//                     });

//                 egui_area_id += 1;
//             }
//         }
//     }
// }

////////////////////////////////////////////////////////////////////////////////////////////////////
fn onsys_init_egui(
    mut ctx_query: Query<&mut EguiContext>,
    fonts: Res<Assets<Font>>,
    packages: Res<Packages>,
) {

    // let mut font_definitions = FontDefinitions::default();

    // font_definitions.font_data.insert(
    //         "hack_regular".to_owned(),
    //         FontData::from_owned(Serial::try_get_bytes_from_path("assets/fonts/hack/regular.ttf").unwrap()),
    //     );

    // font_definitions.families.insert(
    //         FontFamily::Name("hack_regular".into()),
    //         vec![
    //             "hack_regular".to_owned(),
    //         ],
    //     );

    // for mut ctx in ctx_query.iter_mut() { ctx.get_mut().set_fonts(font_definitions.clone()); }
}