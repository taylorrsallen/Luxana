#![allow(ambiguous_glob_reexports)]
#![allow(unused)]

use std::marker::PhantomData;

pub use bevy::prelude::*;
use bevy::{render::{RenderPlugin, settings::{WgpuSettings, WgpuFeatures}}, pbr::wireframe::WireframePlugin};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

pub use bevy_rapier3d::prelude::*;
pub use bevy_egui::*;

mod ai;
pub use ai::*;
mod audio;
pub use audio::*;
mod camera;
pub use camera::*;
mod character;
pub use character::*;
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
mod util;
pub use util::*;
mod voxel;
pub use voxel::*;

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
            
            // Bevy Dev + Debug
            .add_plugins(WorldInspectorPlugin::default())

            // Bevy UI
            // .add_plugins(EguiPlugin)

            // Bevy Physics
            .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())

            // Tank Plugins
            .add_plugins((
                TankAIPlugin,
                TankAudioPlugin,
                TankCameraPlugin,
                TankCharacterPlugin,
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