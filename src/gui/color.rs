use crate::*;

////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct LuxanaGuiColorPlugin;
impl Plugin for LuxanaGuiColorPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<GuiColor>()
            .add_systems(Update, (
                sys_update_gui_linear_color_animation,
                sys_update_gui_color,
            ).chain());
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Component, Default, Reflect)]
pub struct GuiColor(Color);

impl GuiColor {
    pub fn new(color: Color) -> Self { Self { 0: color } }
    pub fn get(&self) -> Color { self.0 }
    pub fn set(&mut self, color: Color) { self.0 = color; }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
fn sys_update_gui_color(
    mut color_query: Query<(&mut Sprite, &GuiColor), Changed<GuiColor>>,
) {
    for (mut sprite, gui_color) in color_query.iter_mut() {
        sprite.color = gui_color.get();
    }
}

fn sys_update_gui_linear_color_animation(
    mut color_query: Query<(&mut GuiColor, &LinearColorAnimation), Changed<LinearColorAnimation>>,
) {
    for (mut gui_color, animation) in color_query.iter_mut() {
        gui_color.set(animation.color());
    }
}