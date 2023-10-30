use crate::*;

////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct TankGuiCursorPlugin;
impl Plugin for TankGuiCursorPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<GuiCursor>();
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Default, PartialEq, Eq, Clone, Copy, Reflect)]
pub enum GuiCursorState {
    #[default]
    Idle,
    Pressed,
    Released,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct GuiCursor {
    pub gui_root: Option<Entity>,
    pub pos: Option<Vec2>,
    pub active_button: Option<Entity>,
    previous_state: GuiCursorState,
    state: GuiCursorState,
}

impl GuiCursor {
    pub fn new(gui_root: Option<Entity>) -> Self { Self { gui_root, ..default() } }

    pub fn state(&self) -> GuiCursorState { self.state }
    pub fn previous_state(&self) -> GuiCursorState { self.previous_state }

    pub fn set_state(&mut self, state: GuiCursorState) {
        self.previous_state = self.state;
        self.state = state;
    }
}