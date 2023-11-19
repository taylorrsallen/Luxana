use crate::*;

use std::fs::read_dir;
use bevy::utils::HashMap;
use serde::{Deserialize, Serialize};

////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct DataAssetPlugin<T: Default + Clone + Serialize + for<'a> Deserialize<'a> + std::fmt::Debug + TypePath + FromReflect + Sync + Send + 'static> {
    type_name: String,
    phantom_data: PhantomData<T>,
}

impl<T: Default + Clone + Serialize + for<'a> Deserialize<'a> + std::fmt::Debug + TypePath + FromReflect + Sync + Send + 'static> DataAssetPlugin<T> {
    pub fn new<S: AsRef<str>>(type_name: S) -> Self {
        Self { type_name: type_name.as_ref().to_owned(), phantom_data: PhantomData }
    }
}

impl<T: Default + Clone + Serialize + for<'a> Deserialize<'a> + std::fmt::Debug + TypePath + FromReflect + Sync + Send + 'static> Plugin for DataAssetPlugin<T> {
    fn build(&self, app: &mut App) {
        app.register_type::<DataAssets<T>>()
            .insert_resource(DataAssets::<T>::new(&self.type_name));
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Resource, Default, Debug, Reflect)]
#[reflect(Resource, Default)]
pub struct DataAssets<T: Default + Clone + Serialize + for<'a> Deserialize<'a> + std::fmt::Debug + TypePath + FromReflect + Sync + Send + 'static> {
    type_name: String,
    data: Vec<T>,
    asset_id_map: HashMap::<String, u16>,
}

impl<T: Default + Clone + Serialize + for<'a> Deserialize<'a> + std::fmt::Debug + TypePath + FromReflect + Sync + Send + 'static> DataAssets<T> {
    pub fn new<S: AsRef<str>>(type_name: S) -> Self {
        let mut data_assets = Self { type_name: type_name.as_ref().to_owned(), ..default() };
        data_assets.init();
        data_assets
    }

    pub fn type_name(&self) -> &String { &self.type_name }
    pub fn full_path(&self) -> String { DATA_ASSET_DIR.to_string() + "/" + &self.type_name }
    pub fn asset_path(&self) -> String { "data/".to_string() + &self.type_name }
    pub fn data(&self) -> &[T] { &self.data }

    /// It is intended that you use this to cache the ID of the desired asset, and then get the asset when needed using `asset_from_id`.
    pub fn id_from_name<S: AsRef<str>>(&self, asset: S) -> u16 {
        if let Some(id) = self.asset_id_map.get(asset.as_ref()) { *id } else { 0 }
    }

    pub fn asset_from_id(&self, id: usize) -> &T { &self.data[id] }

    /// Don't use this in performance critical areas
    pub fn asset_from_name<S: AsRef<str>>(&self, asset: S) -> &T {
        self.asset_from_id(self.id_from_name(asset) as usize)
    }

    pub fn init(&mut self) {
        Serial::create_directory_path(self.full_path());
        self.create_example_assets();
        self.load_all();
    }

    fn create_example_assets(&self) {
        Serial::save_type_to_ron(&T::default(), self.full_path(), "example", 1);
    }

    /// Saves based on asset names in `asset_id_map`. Will overwrite existing files.
    pub fn save_all(&self) {
        let full_path = self.full_path();

        for (name, id) in self.asset_id_map.iter() {
            let asset = self.data[*id as usize].clone();
            Serial::save_type_to_ron(&asset, &full_path, name, 1);
        }
    }

    /// Loads all data asset .ron files from `assets/data/{name}/*`
    pub fn load_all(&mut self) {
        let full_path = self.full_path();
        let files = Serial::file_paths_from_directory_recursive(&full_path, "");
        let mut data_asset_map = HashMap::default();

        for file in files.iter() {
            let Some(file_name) = file.strip_prefix("/") else { continue };
            let Some(asset) = Serial::load_type_from_ron::<&String, &str, T>(&full_path, file_name) else { continue };
            data_asset_map.insert(file_name.to_string(), asset);
        }

        data_asset_map.remove("example");

        let mut id = 0;
        for (name, asset) in data_asset_map.iter() {
            println!("[{id}] {name}: {:?}", asset);
            self.asset_id_map.insert(name.clone(), id);
            self.data.push(asset.clone());
            id += 1;
        }
    }
}