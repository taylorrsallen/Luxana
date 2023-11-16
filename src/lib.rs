#![allow(ambiguous_glob_reexports)]
#![allow(unused)]

use std::marker::PhantomData;
use winit::window::Icon;

pub use bevy::prelude::*;
use bevy::{
    render::{
        RenderPlugin,
        settings::{WgpuSettings, WgpuFeatures, RenderCreation}
    },
    pbr::wireframe::WireframePlugin,
    winit::WinitWindows,
    window::PrimaryWindow
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

pub use bevy_rapier3d::prelude::*;
pub use bevy_egui::*;

mod ai;
pub use ai::*;
mod audio;
pub use audio::*;
mod camera;
pub use camera::*;
mod input;
pub use input::*;
mod level;
pub use level::*;
mod gui;
pub use gui::*;
mod packages;
pub use packages::*;
mod player;
pub use player::*;
mod render;
pub use render::*;
mod state;
pub use state::*;
mod thing;
pub use thing::*;
mod util;
pub use util::*;
mod voxel;
pub use voxel::*;

////////////////////////////////////////////////////////////////////////////////////////////////////
pub const ASSET_DATA_DIR: &'static str = "assets/data";
pub const SAVE_DATA_DIR: &'static str = "data/save";
pub const APP_DATA_DIR: &'static str = "data/app";

#[derive(Default)]
pub struct TankPlugin {
    pub game_name: String,
    pub msaa: Msaa,
}

impl Plugin for TankPlugin {
    fn build(&self, app: &mut App) {
        // Bevy Defaults
        app.add_plugins(DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: self.game_name.clone(),
                        resolution: (640.0, 480.0).into(),
                        resizable: true,
                        ..default()
                    }),
                    ..default()
                })
                .set(RenderPlugin {
                    render_creation: RenderCreation::Automatic(WgpuSettings {
                        features: WgpuFeatures::POLYGON_MODE_LINE,
                        ..default()
                    })
                })
                .set(ImagePlugin::default_nearest())
            )
            .add_systems(Startup, sys_init_window_icon)
            .insert_resource(self.msaa)
            
            // Dev + Debug
            .add_plugins(WireframePlugin)
            .add_plugins(WorldInspectorPlugin::default())

            // UI
            // .add_plugins(EguiPlugin)

            // Physics
            .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())

            // Tank
            .add_plugins((
                TankAIPlugin,
                TankAudioPlugin,
                TankCameraPlugin,
                TankInputPlugin,
                TankLevelPlugin,
                TankGuiPlugin,
                TankPackagesPlugin,
                TankPlayerPlugin,
                TankRenderPlugin,
                TankStatePlugin,
                TankThingPlugin,
                TankUtilPlugin,
                TankVoxelPlugin,
            ));
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
/// Winit must be added separately in Cargo.toml AND be the same version as is used by Bevy for this to work.
fn sys_init_window_icon(
    primary_window_query: Query<Entity, With<PrimaryWindow>>,
    windows: NonSend<WinitWindows>,
) {
    let winit_window = if let Ok(entity) = primary_window_query.get_single() { windows.get_window(entity).unwrap() } else { panic!("Help! No Primary Window!") };
    
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open("assets/images/icon.png").expect("Failed to open icon path").into_rgba8();
        let (width, height) = image.dimensions();
        (image.into_raw(), width, height)
    };

    let icon = Icon::from_rgba(icon_rgba, icon_width, icon_height).unwrap();

    winit_window.set_window_icon(Some(icon));
}