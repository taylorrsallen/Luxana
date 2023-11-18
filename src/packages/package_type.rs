use crate::*;
use super::*;

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Default)]
pub struct PackageType<T: Asset> {
    name: &'static str,
    extension: &'static str,
    ids: HashMap<String, u32>,
    assets: Vec<Handle<T>>,
}

impl<T: Asset> PackageType<T> {
    pub fn new(name: &'static str, extension: &'static str) -> Self {
        Self { name, extension, ids: HashMap::default(), assets: vec![] }
    }

    pub fn handle(&self, id: u32) -> &Handle<T> { &self.assets[id as usize] }
    pub fn fetch_handle(&self, asset: &str) -> &Handle<T> { self.handle(self.fetch_id(asset)) }

    pub fn fetch_id(&self, asset: &str) -> u32 {
        if let Some(id) = self.ids.get(&(self.name.to_owned() + "/" + asset)) {
            *id
        } else {
            println!("Could not find asset id for: [{asset}]");
            0
        }
    }

    pub fn load(&mut self, asset_server: &Res<AssetServer>) {
        let files = Serial::file_paths_from_directory_recursive("assets", self.name);

        for asset_file in files.iter() {
            self.ids.insert(asset_file.clone(), self.assets.len() as u32);
            self.assets.push(asset_server.load(asset_file.clone() + "." + &self.extension));
        }
    }

    pub fn get_load_state(&self, asset_server: &Res<AssetServer>) -> LoadState {
        for id in self.assets.iter().map(|handle| handle.id()) {
            let load_state = if let Some(state) = asset_server.get_load_state(id) { state } else { continue };
            match load_state {
                LoadState::Loaded => { continue; },
                _ => { return load_state; }
            }
        }

        LoadState::Loaded
    }
}