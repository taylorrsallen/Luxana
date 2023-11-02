use crate::*;

////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct TankStatePlugin;
impl Plugin for TankStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<AppState>();
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(States, Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AppState {
    #[default]
    EngineInit,
    GameInit,
    Main,
}