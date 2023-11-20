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
/// TODO: If it becomes a memory problem, take asset_id_map out of here and put it in its own resource. Once the game is finished setting up, delete that resource.
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
        Serial::save_type_to_ron_file(&T::default(), self.full_path(), "example", 1);
        self.load_all();
    }

    /// Saves based on asset names in `asset_id_map`. Will overwrite existing files.
    pub fn save_all(&self) {
        let full_path = self.full_path();

        for (name, id) in self.asset_id_map.iter() {
            let asset = self.data[*id as usize].clone();
            Serial::save_type_to_ron_file(&asset, &full_path, name, 1);
        }
    }

    /// Loads all .ron files from `assets/data/{name}/*`
    pub fn load_all(&mut self) {
        let full_path = self.full_path();
        let files = Serial::file_paths_from_directory_recursive(&full_path, "");
        let mut data_asset_map = HashMap::default();

        for file in files.iter() {
            let Some(file_name) = file.strip_prefix("/") else { continue };
            let Some(asset) = Serial::load_type_from_ron_file::<&String, &str, T>(&full_path, file_name) else { continue };
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

    /// Will fail & return u16::MAX if `asset_name` already exists.
    /// 
    /// Saves new asset file on success.
    /// 
    /// Returns id of added asset.
    pub fn add<S: AsRef<str>>(&mut self, asset_name: S, asset: &T) -> u16 {
        if self.asset_id_map.contains_key(asset_name.as_ref()) { return u16::MAX; }

        let id = self.data.len() as u16;
        self.asset_id_map.insert(asset_name.as_ref().to_owned(), id);
        self.data.push(asset.clone());
        Serial::save_type_to_ron_file(&asset, &self.full_path(), asset_name, 1);
        id
    }

    /// Will overwrite `asset_name` if it already exists.
    /// 
    /// Overwrites existing asset file on success.
    /// 
    /// Returns id of inserted asset.
    pub fn insert<S: AsRef<str>>(&mut self, asset_name: S, asset: &T) -> u16 {
        let id = if let Some(id) = self.asset_id_map.get(asset_name.as_ref()) {
                *id as usize
            } else {
                let id = self.data.len();
                self.asset_id_map.insert(asset_name.as_ref().to_owned(), id as u16);
                id
            };

        Serial::save_type_to_ron_file(&asset, &self.full_path(), asset_name, 1);
        self.data.insert(id, asset.clone());
        id as u16
    }

    /// Will fail & return u16::MAX if `asset_id` does not exist.
    /// 
    /// Otherwise, will overwrite asset with given id, maintaining the name of that asset, and overwriting the existing asset file.
    /// 
    /// Returns name of inserted asset.
    pub fn insert_with_id<S: AsRef<str>>(&mut self, asset_name: S, asset: &T) -> u16 {
        let id = if let Some(id) = self.asset_id_map.get(asset_name.as_ref()) {
                *id as usize
            } else {
                let id = self.data.len();
                self.asset_id_map.insert(asset_name.as_ref().to_owned(), id as u16);
                id
            };

        Serial::save_type_to_ron_file(&asset, &self.full_path(), asset_name, 1);
        self.data.insert(id, asset.clone());
        id as u16
    }

    /// Removing assets will require the game to be reinitialized, as it will corrupt all cached asset ids.
    /// 
    /// Deletes the associated asset file on success.
    /// 
    /// This is intended for use in setup or asset editor tools.
    pub fn remove_with_id(&mut self, id: usize) {
        let mut remove_name = None;
        for (asset_name, asset_id) in self.asset_id_map.iter() {
            if *asset_id == id as u16 { remove_name = Some(asset_name.to_owned()); break; }
        }

        let Some(asset_name) = remove_name else { return };
        self.remove(&asset_name, id);
    }

    /// Removing assets will require the game to be reinitialized, as it will corrupt all cached asset ids.
    /// 
    /// Deletes the associated asset file on success.
    /// 
    /// This is intended for use in setup or asset editor tools.
    pub fn remove_with_name<S: AsRef<str>>(&mut self, asset_name: S) {
        let Some(id) = self.asset_id_map.get(asset_name.as_ref()).cloned() else { return };
        self.remove(asset_name, id as usize);
    }

    fn remove<S: AsRef<str>>(&mut self, asset_name: S, asset_id: usize) {
        swap_last_asset_id(asset_id, self.data.len(), &mut self.asset_id_map);
        Serial::remove_ron_file(&self.full_path(), asset_name.as_ref());
        self.asset_id_map.remove(asset_name.as_ref());
        self.data.swap_remove(asset_id);
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
/// DataAssets that will never be saved or loaded.
pub struct RuntimeDataAssetPlugin<T: Default + Clone + TypePath + FromReflect + Sync + Send + 'static> {
    type_name: String,
    phantom_data: PhantomData<T>,
}

impl<T: Default + Clone + TypePath + FromReflect + Sync + Send + 'static> RuntimeDataAssetPlugin<T> {
    pub fn new<S: AsRef<str>>(type_name: S) -> Self {
        Self { type_name: type_name.as_ref().to_owned(), phantom_data: PhantomData }
    }
}

impl<T: Default + Clone + TypePath + FromReflect + Sync + Send + 'static> Plugin for RuntimeDataAssetPlugin<T> {
    fn build(&self, app: &mut App) {
        app.register_type::<RuntimeDataAssets<T>>()
            .insert_resource(RuntimeDataAssets::<T>::new(&self.type_name));
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Resource, Default, Debug, Reflect)]
#[reflect(Resource, Default)]
pub struct RuntimeDataAssets<T: Default + Clone + TypePath + FromReflect + Sync + Send + 'static> {
    type_name: String,
    data: Vec<T>,
    asset_id_map: HashMap::<String, u16>,
}

impl<T: Default + Clone + TypePath + FromReflect + Sync + Send + 'static> RuntimeDataAssets<T> {
    pub fn new<S: AsRef<str>>(type_name: S) -> Self {
        Self { type_name: type_name.as_ref().to_owned(), data: vec![], asset_id_map: HashMap::default() }
    }

    pub fn type_name(&self) -> &String { &self.type_name }
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

    /// Will fail & return u16::MAX if `asset_name` already exists.
    /// 
    /// Returns id of added asset.
    pub fn add<S: AsRef<str>>(&mut self, asset_name: S, asset: &T) -> u16 {
        if self.asset_id_map.contains_key(asset_name.as_ref()) { return u16::MAX; }
        
        let id = self.data.len() as u16;
        self.asset_id_map.insert(asset_name.as_ref().to_owned(), id);
        self.data.push(asset.clone());
        id
    }

    /// Will overwrite `asset_name` if it already exists.
    /// 
    /// Returns id of inserted asset.
    pub fn insert<S: AsRef<str>>(&mut self, asset_name: S, asset: &T) -> u16 {
        let id = if let Some(id) = self.asset_id_map.get(asset_name.as_ref()) {
                *id as usize
            } else {
                let id = self.data.len();
                self.asset_id_map.insert(asset_name.as_ref().to_owned(), id as u16);
                id
            };

        self.data.insert(id, asset.clone());
        id as u16
    }

    /// Will fail & return u16::MAX if `asset_id` does not exist.
    /// 
    /// Otherwise, will overwrite asset with given id, maintaining the name of that asset.
    /// 
    /// Returns name of inserted asset.
    pub fn insert_with_id<S: AsRef<str>>(&mut self, asset_name: S, asset: &T) -> u16 {
        let id = if let Some(id) = self.asset_id_map.get(asset_name.as_ref()) {
                *id as usize
            } else {
                let id = self.data.len();
                self.asset_id_map.insert(asset_name.as_ref().to_owned(), id as u16);
                id
            };

        self.data.insert(id, asset.clone());
        id as u16
    }

    /// Removing assets will require the game to be reinitialized, as it will corrupt all cached asset ids.
    /// 
    /// This is intended for use in setup or asset editor tools.
    pub fn remove_with_id(&mut self, id: usize) {
        let mut remove_name = None;
        for (asset_name, asset_id) in self.asset_id_map.iter() {
            if *asset_id == id as u16 { remove_name = Some(asset_name.to_owned()); break; }
        }

        let Some(asset_name) = remove_name else { return };
        self.remove(&asset_name, id);
    }

    /// Removing assets will require the game to be reinitialized, as it will corrupt all cached asset ids.
    /// 
    /// This is intended for use in setup or asset editor tools.
    pub fn remove_with_name<S: AsRef<str>>(&mut self, asset_name: S) {
        let Some(id) = self.asset_id_map.get(asset_name.as_ref()).cloned() else { return };
        self.remove(asset_name, id as usize);
    }

    fn remove<S: AsRef<str>>(&mut self, asset_name: S, asset_id: usize) {
        swap_last_asset_id(asset_id, self.data.len(), &mut self.asset_id_map);
        self.asset_id_map.remove(asset_name.as_ref());
        self.data.swap_remove(asset_id);
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
/// The `asset_name` associated with the last `asset_id` is swapped to associate with `remove_id` instead.
fn swap_last_asset_id(remove_id: usize, data_len: usize, asset_id_map: &mut HashMap<String, u16>) {
    let last_id = data_len - 1;

    // If the id of the asset being removed is the last id, then there is no work to be done
    if last_id == remove_id { return; }

    // The name associated with the last_id needs to have its id set to remove_id
    let mut last_name = None;
    for (existing_name, existing_id) in asset_id_map.iter() {
        if *existing_id == last_id as u16 { last_name = Some(existing_name.to_owned()); break; }
    }

    // If there is no name associated with the last id, the database is corrupt and we should panic.
    // Otherwise, replace the id of the last asset with the id of the asset being removed.
    asset_id_map.insert(last_name.unwrap(), remove_id as u16);
}