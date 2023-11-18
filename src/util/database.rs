use crate::*;

use serde::*;

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Default)]
pub struct DatabasePlugin<DataType: Default + Serialize + for<'a> Deserialize<'a> + Sync + Send + 'static>(&'static str, PhantomData<DataType>);

impl<DataType: Default + Serialize + for<'a> Deserialize<'a> + Sync + Send + 'static> Plugin for DatabasePlugin<DataType> {
    fn build(&self, app: &mut App) {
        app.insert_resource(Database::<DataType>::new(self.0).load());
    }
}

impl<DataType: Default + Serialize + for<'a> Deserialize<'a> + Sync + Send + 'static> DatabasePlugin<DataType> {
    pub fn new(name: &'static str) -> Self { Self(name, PhantomData::default()) }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Resource)]
pub struct Database<DataType: Default + Serialize + for<'a> Deserialize<'a> + Sync + Send + 'static> {
    name: &'static str,
    data: Vec<DataType>,
}

impl<DataType: Default + Serialize + for<'a> Deserialize<'a> + Sync + Send + 'static> Database<DataType> {    
    fn new(name: &'static str) -> Self { Self { name, data: vec![] } }

    pub fn load(mut self) -> Self {
        self.data = if let Some(data) = Serial::load_type_from_ron(DATA_ASSET_DIR, self.name.to_owned()) { data } else { vec![DataType::default()] };
        self.save();
        self
    }

    pub fn save(&self) { Serial::save_type_to_ron(&self.data, DATA_ASSET_DIR, self.name, 2); }
    pub fn get(&self, index: usize) -> Option<&DataType> { self.data.get(index) }
}

pub trait Data<T: SaveData> {
    fn serialize(&self) -> T;
}

pub trait SaveData {
    fn save(&self);
    fn load() -> Self;
}