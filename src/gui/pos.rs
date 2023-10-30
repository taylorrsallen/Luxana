use crate::*;

////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct LuxanaGuiPosPlugin;
impl Plugin for LuxanaGuiPosPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<GuiPos>()
            .register_type::<GuiVal>()
            .register_type::<GuiZLayer>();
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct GuiPos {
    pub x: GuiVal,
    pub y: GuiVal,
}

impl GuiPos {
    pub fn new(x: GuiVal, y: GuiVal) -> Self { Self { x, y } }
    pub fn from_px(x: f32, y: f32) -> Self { Self { x: GuiVal::Px(x), y: GuiVal::Px(y) } }
    pub fn from_px_vec2(px: Vec2) -> Self { Self { x: GuiVal::Px(px.x), y: GuiVal::Px(px.y) } }
    pub fn from_percent(x: f32, y: f32) -> Self { Self { x: GuiVal::Percent(x), y: GuiVal::Percent(y) } }
    pub fn from_percent_vec2(percent: Vec2) -> Self { Self { x: GuiVal::Percent(percent.x), y: GuiVal::Percent(percent.y) } }

    pub fn as_px(&self, viewport: UVec2) -> [f32; 2] {
        [self.x.as_px(viewport.x), self.y.as_px(viewport.y)]
    }

    pub fn as_px_vec2(&self, viewport: UVec2) -> Vec2 {
        Vec2::new(self.x.as_px(viewport.x), self.y.as_px(viewport.y))
    }

    pub fn as_px_center_offset(&self, viewport: UVec2) -> [f32; 2] {
        let as_px = self.as_px(viewport);
        [as_px[0] - viewport.x as f32 * 0.5, as_px[1] - viewport.y as f32 * 0.5]
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Reflect)]
pub enum GuiVal {
    Px(f32),
    Percent(f32),
}

impl Default for GuiVal {
    fn default() -> Self { Self::Px(0.0) }
}

impl GuiVal {
    pub fn as_px(&self, render_dim: u32) -> f32 {
        match self {
            Self::Px(val) => { *val }
            Self::Percent(val) => { val * render_dim as f32 }
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct GuiZLayer(pub i32);

impl GuiZLayer {
    pub fn new(layer: i32) -> Self { Self { 0: layer } }
    pub fn get(&self) -> i32 { self.0 }
}