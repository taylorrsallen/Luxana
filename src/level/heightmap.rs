use crate::*;

use std::sync::{RwLock, Arc};
use bevy::utils::{HashMap, HashSet};

////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct TankLevelHeightmapPlugin;
impl Plugin for TankLevelHeightmapPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<HeightmapRootMesher>();
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////


////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Component)]
pub struct HeightmapRoot {
    chunks: HashMap<IVec2, Arc<RwLock<HeightmapChunk>>>,
}

impl Default for HeightmapRoot {
    fn default() -> Self { Self { chunks: HashMap::<IVec2, Arc<RwLock<HeightmapChunk>>>::default() } }
}

impl HeightmapRoot {
    #[inline] pub fn chunks(&self) -> &HashMap<IVec2, Arc<RwLock<HeightmapChunk>>> { &self.chunks }

    #[inline] pub fn chunk_from_coord(&self, coord: IVec2) -> Option<&Arc<RwLock<HeightmapChunk>>> { self.chunks.get(&(coord & !CHUNK_2D_MASK)) }

    pub fn get_value_at_coord(&self, coord: IVec2) -> f32 {
        let key = coord & !CHUNK_2D_MASK;
        if let Some(chunk) = self.chunks.get(&key) {
            chunk.read().unwrap().get_value_at_coord(coord)
        } else {
            0.0
        }
    }

    pub fn set_value_at_coord(&mut self, coord: IVec2, value: f32) {
        let key = coord & !CHUNK_2D_MASK;
        if let Some(chunk) = self.chunks.get(&key) {
            chunk.write().unwrap().set_value_at_coord(coord, value);
        } else {
            let mut chunk = HeightmapChunk::default();
            chunk.set_value_at_coord(coord, value);
            self.chunks.insert(key, Arc::new(RwLock::new(chunk)));
        }
    }

    pub fn modify_value_at_coord(&mut self, coord: IVec2, modifier: f32) {
        let key = coord & !CHUNK_2D_MASK;
        if let Some(chunk) = self.chunks.get(&key) {
            chunk.write().unwrap().modify_value_at_coord(coord, modifier);
        } else {
            let mut chunk = HeightmapChunk::default();
            chunk.modify_value_at_coord(coord, modifier);
            self.chunks.insert(key, Arc::new(RwLock::new(chunk)));
        }
    }

    pub fn modify_values_at_pos(&mut self, modifier: f32, pos: Vec2) {

    }

    pub fn modify_values_in_radius(&mut self, modifier: f32, origin: Vec2, radius: f32) {

    }

    pub fn modify_values_with_radial_falloff(&mut self, modifier: f32, origin: Vec2, radius: f32) {

    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct HeightmapChunk {
    data: [f32; CHUNK_2D_SIZE],
}

impl Default for HeightmapChunk {
    fn default() -> Self { Self { data: [0.0; CHUNK_2D_SIZE] } }
}

impl HeightmapChunk {
    pub fn data(&self) -> &[f32; CHUNK_2D_SIZE] { &self.data }
    pub fn data_mut(&mut self) -> &mut [f32; CHUNK_2D_SIZE] { &mut self.data }

    #[inline] pub fn local_coord_from_global_coord(coord: IVec2) -> UVec2 {
        (coord & CHUNK_2D_MASK).abs().as_uvec2()
    }

    #[inline] pub fn value_index_from_global_coord(coord: IVec2) -> usize {
        let local_coord = Self::local_coord_from_global_coord(coord);
        (local_coord.y * CHUNK_2D_DIM + local_coord.x) as usize
    }

    pub fn get_value_at_coord(&self, coord: IVec2) -> f32 {
        self.data[Self::value_index_from_global_coord(coord)]
    }

    pub fn set_value_at_coord(&mut self, coord: IVec2, value: f32) {
        self.data[Self::value_index_from_global_coord(coord)] = value;
    }

    pub fn modify_value_at_coord(&mut self, coord: IVec2, modifier: f32) {
        self.data[Self::value_index_from_global_coord(coord)] += modifier;
    }

    pub fn modify_value_at_coord_with_range(&mut self, coord: IVec2, modifier: f32, min: f32, max: f32) {
        let mut value = self.data[Self::value_index_from_global_coord(coord)];
        if value + modifier > max { value = max; } else if value + modifier < min { value = min; } else { value += modifier; }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct HeightmapRootChanges {
    pub changed_keys: HashSet<IVec2>,
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct HeightmapRootMesher {
    meshes: HashMap<IVec2, Entity>,
}

fn sys_update_heightmap_meshes(
    mut commands: Commands,
    mut heightmap_query: Query<(&HeightmapRoot, &mut HeightmapRootMesher, &mut HeightmapRootChanges), Changed<HeightmapRootChanges>>
) {
    for (root, mut root_mesher, mut root_changes) in heightmap_query.iter_mut() {
        for key in root_changes.changed_keys.iter() {
            let chunk = if let Some(chunk) = root.chunk_from_coord(*key) { chunk.read().unwrap() } else { continue };

            // Create mesh from chunk data... magical process

            if let Some(old_mesh_entity) = root_mesher.meshes.get(key) { commands.entity(*old_mesh_entity).despawn_recursive(); }
            root_mesher.meshes.insert(*key, commands.spawn(PbrBundle {
                    // >>> Mesh goes here <<<
                    transform: Transform::from_translation(Vec3::new(key.x as f32, 0.0, key.y as f32)),
                    ..default()
                }).id());

        }

        root_changes.changed_keys.clear();
    }
}