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
        let mut data_assets = DataAssets::<T>::new(&self.type_name);
        data_assets.init_directory();
        data_assets.load();
        
        app.register_type::<DataAssets<T>>()
            .insert_resource(data_assets);
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Default, Clone, Serialize, Deserialize, Debug, Reflect)]
#[reflect(Default)]
pub struct DataAsset<T: Default + Clone + Serialize + std::fmt::Debug + TypePath + FromReflect + Sync + Send + 'static> {
    pub name: String,
    pub data: T,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Resource, Default, Debug, Reflect)]
#[reflect(Resource, Default)]
pub struct DataAssets<T: Default + Clone + Serialize + for<'a> Deserialize<'a> + std::fmt::Debug + TypePath + FromReflect + Sync + Send + 'static> {
    pub type_name: String,
    pub data: Vec<DataAsset<T>>,
}

impl<T: Default + Clone + Serialize + for<'a> Deserialize<'a> + std::fmt::Debug + TypePath + FromReflect + Sync + Send + 'static> DataAssets<T> {
    pub fn new<S: AsRef<str>>(type_name: S) -> Self {
        Self { type_name: type_name.as_ref().to_owned(), ..default() }
    }

    pub fn full_path(&self) -> String { DATA_ASSET_DIR.to_string() + "/" + &self.type_name }
    pub fn asset_path(&self) -> String { "data/".to_string() + &self.type_name }

    pub fn init_directory(&self) {
        Serial::create_directory_path(self.full_path());
        self.create_example_assets();
    }

    fn create_example_assets(&self) {
        let example_data = DataAsset::<T> { name: "example".into(), ..default() };
        let multi_example_data = vec![example_data.clone(); 2];
        Serial::save_type_to_ron(&example_data, self.full_path(), "example", 2);
        Serial::save_type_to_ron(&multi_example_data, self.full_path(), "multi_example", 3);
    }

    /// Loads all data asset .ron files from `assets/data/{name}/*`
    pub fn load(&mut self) {
        let full_path = self.full_path();
        let files = Serial::file_paths_from_directory_recursive(&full_path, "");
        let mut asset_map = HashMap::default();

        for file in files.iter() {
            let Some(file) = file.strip_prefix("/") else { continue };
            let Some(contents) = Serial::load_string_from_ron(&full_path, file) else { continue };
            if let Ok(asset) = ron::from_str::<DataAsset<T>>(&contents) {
                asset_map.insert(asset.name, asset.data);
            } else if let Ok(assets) = ron::from_str::<Vec<DataAsset<T>>>(&contents) {
                asset_map.extend(assets.iter().map(|asset| { (asset.name.clone(), asset.data.clone()) }));
            }
        }

        asset_map.remove("example");

        for (name, asset) in asset_map.iter() {
            println!("{name}: {:?}", asset);
        }
    }
}