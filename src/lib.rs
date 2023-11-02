#![allow(ambiguous_glob_reexports)]
#![allow(unused)]

use std::marker::PhantomData;

pub use bevy::prelude::*;
use bevy::{render::{RenderPlugin, settings::{WgpuSettings, WgpuFeatures}}, pbr::wireframe::WireframePlugin, winit::WinitWindows, window::PrimaryWindow};
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
mod gui;
pub use gui::*;
mod networking;
pub use networking::*;
mod packages;
pub use packages::*;
mod player;
pub use player::*;
mod state;
pub use state::*;
mod thing;
pub use thing::*;
mod util;
pub use util::*;
mod voxel;
pub use voxel::*;
use winit::window::Icon;

////////////////////////////////////////////////////////////////////////////////////////////////////
pub const ASSET_DATA_DIR: &'static str = "assets/data";
pub const SAVE_DATA_DIR: &'static str = "data/save";
pub const APP_DATA_DIR: &'static str = "data/app";

pub struct TankPlugin {
    pub game_name: String,
}

impl Plugin for TankPlugin {
    fn build(&self, app: &mut App) {
        // Bevy Defaults
        app.insert_resource(Msaa::Off)
            .add_plugins(DefaultPlugins
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
                    wgpu_settings: WgpuSettings {
                        features: WgpuFeatures::POLYGON_MODE_LINE,
                        ..default()
                    }
                })
                .set(ImagePlugin::default_nearest())
            )
            .add_plugins(WireframePlugin)
            .add_systems(Startup, sys_init_window_icon)
            
            // Bevy Dev + Debug
            .add_plugins(WorldInspectorPlugin::default())

            // Bevy UI
            // .add_plugins(EguiPlugin)

            // Bevy Physics
            .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())

            // Tank Plugins
            .add_plugins((
                TankActorPlugin,
                TankAIPlugin,
                TankAudioPlugin,
                TankCameraPlugin,
                TankInputPlugin,
                TankGuiPlugin,
                // networking
                TankPackagesPlugin,
                TankPlayerPlugin,
                TankStatePlugin,
                TankUtilPlugin,
                TankVoxelPlugin,
            ));
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
fn sys_init_window_icon(
    primary_window_query: Query<Entity, With<PrimaryWindow>>,
    windows: NonSend<WinitWindows>,
) {
    let winit_window = if let Ok(entity) = primary_window_query.get_single() { windows.get_window(entity).unwrap() } else { panic!("Help! No Primary Window!") };
    
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open("assets/images/icon.png").expect("Failed to open icon path").into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };

    let icon = Icon::from_rgba(icon_rgba, icon_width, icon_height).unwrap();

    winit_window.set_window_icon(Some(icon));
}