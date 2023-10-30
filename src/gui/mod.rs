use crate::*;

use bevy::window::PrimaryWindow;
use bevy_egui::egui::{FontDefinitions, FontData, FontId, FontFamily, RichText, Align2, Color32, TextStyle};

mod button;
pub use button::*;
mod color;
pub use color::*;
mod cursor;
pub use cursor::*;
mod follow;
pub use follow::*;
mod collider;
pub use collider::*;
mod label;
pub use label::*;
mod menu;
pub use menu::*;
mod pos;
pub use pos::*;

////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct TankGuiPlugin;
impl Plugin for TankGuiPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<GuiRoot>()
            .register_type::<GuiViewport>()
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
            .add_systems(OnEnter(AppState::EngineInit), onsys_init_egui_fonts)
            .add_systems(PreUpdate, (
                sys_update_gui_window_changed,
                sys_update_gui_camera_changed,
                sys_update_gui_viewport,
                sys_update_gui,
            ).chain());
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Bundle, Default)]
pub struct GuiRootBundle {
    pub root: GuiRoot,
    pub viewport: GuiViewport,
    pub transform: TransformBundle,
    pub visibility: VisibilityBundle,
}

impl GuiRootBundle {
    pub fn new(gui_camera: Option<Entity>) -> Self {
        Self { root: GuiRoot { camera: gui_camera }, ..default() }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Resource, Default, Reflect)]
#[reflect(Resource)]
pub struct GuiContext {
    
}

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct GuiRoot {
    pub camera: Option<Entity>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct GuiViewport(UVec2);

impl GuiViewport {
    pub fn get(&self) -> UVec2 { self.0 }
    pub fn set(&mut self, viewport: UVec2) { self.0 = viewport; }
    pub fn clear(&mut self) { self.0 = UVec2::ZERO; }
}

fn sys_update_gui_window_changed(
    mut root_query: Query<&mut GuiRoot>,
    primary_window_query: Query<Entity, With<PrimaryWindow>>,
    changed_window_query: Query<Entity, Changed<Window>>,
    camera_query: Query<&Camera, With<Camera2d>>,
) {
    for changed_window_entity in changed_window_query.iter() {
        for mut gui_root in root_query.iter_mut() {
            let root_camera_entity = if let Some(camera) = gui_root.camera { camera } else { continue };
            let root_camera = if let Ok(camera) = camera_query.get(root_camera_entity) { camera } else { continue };
            let root_window_entity = Cameras::window_entity_from_camera(root_camera, &primary_window_query);
            if changed_window_entity != root_window_entity { continue; }
            gui_root.set_changed();
        }
    }
}

fn sys_update_gui_camera_changed(
    mut root_query: Query<&mut GuiRoot>,
    changed_camera_query: Query<Entity, (Changed<Camera>, With<Camera2d>)>,
) {
    for changed_camera_entity in changed_camera_query.iter() {
        for mut gui_root in root_query.iter_mut() {
            let root_camera_entity = if let Some(camera) = gui_root.camera { camera } else { continue };
            if changed_camera_entity != root_camera_entity { continue; }
            gui_root.set_changed();
        }
    }
}

fn sys_update_gui_viewport(
    mut root_query: Query<(&mut GuiViewport, &GuiRoot), Changed<GuiRoot>>,
    primary_window_query: Query<Entity, With<PrimaryWindow>>,
    window_query: Query<&Window>,
    camera_query: Query<&Camera, With<Camera2d>>,
) {
    for (mut gui_viewport, gui_root) in root_query.iter_mut() {
        let camera_entity = if let Some(camera) = gui_root.camera { camera } else { gui_viewport.clear(); continue };
        let camera = if let Ok(camera) = camera_query.get(camera_entity) { camera } else { gui_viewport.clear(); continue };
        let window_entity = Cameras::window_entity_from_camera(camera, &primary_window_query);

        let new_gui_viewport: UVec2;
        if let Some(camera_viewport) = &camera.viewport {
            new_gui_viewport = UVec2::new(camera_viewport.physical_size.x, camera_viewport.physical_size.y);
        } else {
            let window = if let Ok(window) = window_query.get(window_entity) { window } else { gui_viewport.clear(); continue };
            new_gui_viewport = UVec2::new(window.physical_width(), window.physical_height());
        }

        gui_viewport.set(new_gui_viewport);
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
fn sys_update_gui(
    mut transform_query: Query<&mut Transform>,
    mut egui_context_query: Query<&mut EguiContext, With<Window>>,
    root_query: Query<(&GuiRoot, &GuiViewport, &Children)>,
    pos_query: Query<&GuiPos, With<Parent>>,
    z_layer_query: Query<&GuiZLayer, With<Parent>>,
    color_query: Query<&GuiColor, With<Parent>>,
    label_query: Query<&GuiLabel, With<Parent>>,
    primary_window_query: Query<Entity, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
) {
    let mut egui_area_id = 0;
    for (gui_root, gui_viewport, gui_children) in root_query.iter() {
        let camera_entity = if let Some(camera) = gui_root.camera { camera } else { continue };
        let (camera, camera_transform) = if let Ok(camera) = camera_query.get(camera_entity) { camera } else { continue };
        let window_entity = Cameras::window_entity_from_camera(camera, &primary_window_query);

        for child_entity in gui_children.iter() {
            let gui_pos = if let Ok(gui_pos) = pos_query.get(*child_entity) { gui_pos } else { continue };

            if let Ok(mut transform) = transform_query.get_mut(*child_entity) {
                let mut translation = if let Some(xy) = camera.viewport_to_world_2d(camera_transform, gui_pos.as_px_vec2(gui_viewport.get())) { Vec3::new(xy.x, xy.y, transform.translation.z) } else { continue };
                if let Ok(z_layer) = z_layer_query.get(*child_entity) { translation.z = z_layer.get() as f32; }
                transform.translation = translation;

                // Scale?
            }

            if let Ok(label) = label_query.get(*child_entity) {
                let mut ctx = if let Ok(ctx) = egui_context_query.get_mut(window_entity) { ctx } else { continue };
                let color = if let Ok(color) = color_query.get(*child_entity) { color.get().as_rgba_u8() } else { [255 as u8; 4] };

                egui::Area::new(egui_area_id.to_string())
                    .anchor(Align2::CENTER_CENTER, gui_pos.as_px_center_offset(gui_viewport.get()))
                    .show(ctx.get_mut(), |ui| {
                        ui.add(egui::Label::new(RichText::new(&label.content)
                            .font(FontId { size: label.size, family: FontFamily::Name("hack_regular".into()) })
                            .color(Color32::from_rgba_unmultiplied(color[0], color[1], color[2], color[3]))
                        ).wrap(false));
                    });

                egui_area_id += 1;
            }
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
fn onsys_init_egui_fonts(mut ctx_query: Query<&mut EguiContext>, fonts: Res<Assets<Font>>, packages: Res<Packages>) {
    let mut font_definitions = FontDefinitions::default();

    font_definitions.font_data.insert(
            "hack_regular".to_owned(),
            FontData::from_owned(Serial::file_to_bytes("assets/fonts/hack/regular.ttf").unwrap()),
        );

    font_definitions.families.insert(
            FontFamily::Name("hack_regular".into()),
            vec![
                "hack_regular".to_owned(),
            ],
        );

    for mut ctx in ctx_query.iter_mut() { ctx.get_mut().set_fonts(font_definitions.clone()); }
}