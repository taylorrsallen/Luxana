use crate::*;

use serde::*;

mod dev;
pub use dev::*;

////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct TankStatePlugin;
impl Plugin for TankStatePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<AppDebugSettings>()
            .add_state::<AppState>()
            .add_state::<AppDebugState>()
            .add_state::<AppDevState>()
            .add_systems(Update, sys_toggle_integrated_states)
            .add_systems(Update, (
                sys_log_app_state.run_if(resource_changed::<State<AppState>>()),
                sys_log_debug_state.run_if(resource_changed::<State<AppDebugState>>()),
                sys_log_dev_state.run_if(resource_changed::<State<AppDevState>>()),
            ).run_if(in_state(AppDevState::Enabled)))
            .add_systems(OnExit(AppDevState::Enabled), sys_log_dev_state)
            .add_systems(OnEnter(AppDebugState::Enabled), onsys_init_debug_settings)
            .add_systems(Update,
                sys_update_debug_settings
                    .run_if(in_state(AppDebugState::Enabled)
                    .and_then(resource_exists::<AppDebugSettings>())
                    .and_then(resource_changed::<AppDebugSettings>())))
            .add_systems(OnExit(AppDebugState::Enabled), onsys_remove_debug_settings);
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Default, States, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AppState {
    #[default]
    Load,
    Menu,
    Game,
}

fn sys_log_app_state(state: Res<State<AppState>>, time: Res<Time>) {
    println!("[{}] AppState::{:?}", time.elapsed_seconds(), state.get());
}

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Default, States, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AppDebugState {
    #[default]
    Disabled,
    Enabled,
}

fn sys_log_debug_state(state: Res<State<AppDebugState>>, time: Res<Time>) {
    println!("[{}] AppDebugState::{:?}", time.elapsed_seconds(), state.get());
}

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Default, States, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AppDevState {
    #[default]
    Disabled,
    Enabled,
}

fn sys_log_dev_state(state: Res<State<AppDevState>>, time: Res<Time>) {
    println!("[{}] AppDevState::{:?}", time.elapsed_seconds(), state.get());
}

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Resource, Serialize, Deserialize, Reflect)]
#[reflect(Resource)]
pub struct AppDebugSettings {
    // util
    // state
    // package
    // settings
    // input
    // camera
    // gui
    // profile
    pub player: bool,
    // character
    pub world: bool,
}

impl Default for AppDebugSettings {
    fn default() -> Self {
        Self {
            player: false,
            world: false,
        }
    }
}

impl AppDebugSettings {
    fn save(&self) {
        Serial::save_ron_file_to_path(self, APP_DATA_DIR, "debug", 1);
    }

    fn load_or_create_default() -> Self {
        Serial::load_ron_file_from_path_or_create_default(APP_DATA_DIR, "debug", 1)
    }
}

fn onsys_init_debug_settings(mut commands: Commands) {
    commands.insert_resource(AppDebugSettings::load_or_create_default());
}

fn sys_update_debug_settings(settings: Res<AppDebugSettings>) {
    settings.save();
}

fn onsys_remove_debug_settings(mut commands: Commands) {
    commands.remove_resource::<AppDebugSettings>();
}

////////////////////////////////////////////////////////////////////////////////////////////////////
fn sys_toggle_integrated_states(
    mut next_debug_state: ResMut<NextState<AppDebugState>>,
    mut next_dev_state: ResMut<NextState<AppDevState>>,
    debug_state: Res<State<AppDebugState>>,
    dev_state: Res<State<AppDevState>>,
    keys: Res<Input<KeyCode>>,
) {
    if keys.pressed(KeyCode::AltLeft) && keys.pressed(KeyCode::ShiftLeft) {
        if keys.just_pressed(KeyCode::F9) {
            match debug_state.get() {
                AppDebugState::Disabled => { next_debug_state.set(AppDebugState::Enabled); }
                AppDebugState::Enabled => { next_debug_state.set(AppDebugState::Disabled); }
            }
        } else if keys.just_pressed(KeyCode::F10) {
            match dev_state.get() {
                AppDevState::Disabled => { next_dev_state.set(AppDevState::Enabled); }
                AppDevState::Enabled => { next_dev_state.set(AppDevState::Disabled); }
            }
        }
    }
}