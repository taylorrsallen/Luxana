use crate::*;

use std::fs::*;
use bevy::{utils::HashMap, asset::{Asset, LoadState}, gltf::Gltf};

mod package_type;
pub use package_type::*;

////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct TankPackagesPlugin;
impl Plugin for TankPackagesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, stsys_init_packages)
            .add_systems(Update, sys_update_load_state.run_if(in_state(AppState::EngineInit)));
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Resource)]
pub struct Packages {
    pub fonts: PackageType<Font>,
    pub images: PackageType<Image>,
    pub models: PackageType<Gltf>,
    pub sounds: PackageType<bevy_kira_audio::AudioSource>,
}

impl Default for Packages {
    fn default() -> Self {
        Self {
            fonts: PackageType::<Font>::new("fonts", "ttf"),
            images: PackageType::<Image>::new("images", "png"),
            models: PackageType::<Gltf>::new("models", "glb"),
            sounds: PackageType::<bevy_kira_audio::AudioSource>::new("sounds", "ogg"),
        }
    }
}

impl Packages {
    pub fn load(asset_server: &Res<AssetServer>) -> Self {
        let mut packages = Self::default();
        packages.fonts.load(asset_server);
        packages.images.load(asset_server);
        packages.models.load(asset_server);
        packages.sounds.load(asset_server);
        packages
    }

    pub fn get_load_state(&self, asset_server: &Res<AssetServer>) -> LoadState {
        let states = vec![
            self.fonts.get_load_state(asset_server),
            self.images.get_load_state(asset_server),
            self.models.get_load_state(asset_server),
            self.sounds.get_load_state(asset_server),
        ];

        for state in states {
            match state {
                LoadState::Loaded => continue,
                _ => return state,
            }
        }

        LoadState::Loaded
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
fn stsys_init_packages(mut commands: Commands, asset_server: Res<AssetServer>, time: Res<Time>) {
    println!("[{}] Started loading packages", time.elapsed_seconds());
    let packages = Packages::load(&asset_server);
    commands.insert_resource(packages);
}

fn sys_update_load_state(
    mut next_state: ResMut<NextState<AppState>>,
    asset_server: Res<AssetServer>,
    packages: Res<Packages>,
    time: Res<Time>,
) {
    if packages.get_load_state(&asset_server) == LoadState::Loaded {
        println!("[{}] Finished loading packages", time.elapsed_seconds());
        next_state.0 = Some(AppState::GameInit);
    }
}