use crate::*;

use std::sync::{RwLock, Arc};
use bevy::{utils::{HashMap, HashSet}, pbr::wireframe::Wireframe};

////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct TankLevelHeightmapPlugin;
impl Plugin for TankLevelHeightmapPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<HeightmapRootChanges>()
            .register_type::<HeightmapRootMesher>()
            .add_systems(PostUpdate, (
                sys_update_heightmap_meshes,
            ));
    }
}

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
#[derive(Reflect)]
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
pub struct HeightmapRootChanges(HashSet<IVec2>);

impl HeightmapRootChanges {
    pub fn mark_change(&mut self, coord: IVec2) {
        let chunk_coord = coord & !CHUNK_2D_MASK;
        self.0.insert(chunk_coord);

        let local_coord = HeightmapChunk::local_coord_from_global_coord(coord);
        if local_coord.x == 0 {
            self.0.insert(IVec2::new(chunk_coord.x - CHUNK_2D_DIM as i32, chunk_coord.y));
            if local_coord.y == 0 {
                self.0.insert(IVec2::new(chunk_coord.x - CHUNK_2D_DIM as i32, chunk_coord.y - CHUNK_2D_DIM as i32));
                self.0.insert(IVec2::new(chunk_coord.x, chunk_coord.y - CHUNK_2D_DIM as i32));
            }
        } else if local_coord.y == 0 {
            self.0.insert(IVec2::new(chunk_coord.x, chunk_coord.y - CHUNK_2D_DIM as i32));
        }
    }

    pub fn clear(&mut self) { self.0.clear() }
    pub fn iter(&self) -> impl Iterator<Item = IVec2> + '_ { self.0.iter().copied() }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct HeightmapRootMesher {
    meshes: HashMap<IVec2, Entity>,
}

fn sys_update_heightmap_meshes(
    mut commands: Commands,
    mut heightmap_query: Query<(Entity, &HeightmapRoot, &mut HeightmapRootMesher, &mut HeightmapRootChanges), Changed<HeightmapRootChanges>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (root_entity, root, mut root_mesher, mut root_changes) in heightmap_query.iter_mut() {
        let mut new_mesh_entities = vec![];
        for key in root_changes.iter() {
            let chunk = if let Some(chunk) = root.chunk_from_coord(key) { chunk.read().unwrap() } else { continue };

            // I hate this
            let new_mesh;
            if let Some(chunk_r) = root.chunk_from_coord(IVec2::new(key.x + CHUNK_2D_DIM as i32, key.y)) {
                if let Some(chunk_f) = root.chunk_from_coord(IVec2::new(key.x, key.y + CHUNK_2D_DIM as i32)) {
                    if let Some(chunk_rf) = root.chunk_from_coord(IVec2::new(key.x + CHUNK_2D_DIM as i32, key.y + CHUNK_2D_DIM as i32)) {
                        new_mesh = MeshGen::from_square_heightmap_with_r_f_rf_neighbors(
                            chunk.data(),
                            chunk_r.read().unwrap().data(),
                            chunk_f.read().unwrap().data(),
                            chunk_rf.read().unwrap().data(),
                            CHUNK_2D_DIM);
                    } else {
                        new_mesh = MeshGen::from_square_heightmap_with_r_f_neighbors(
                            chunk.data(),
                            chunk_r.read().unwrap().data(),
                            chunk_f.read().unwrap().data(),
                            CHUNK_2D_DIM);
                    }
                } else {
                    if let Some(chunk_rf) = root.chunk_from_coord(IVec2::new(key.x + CHUNK_2D_DIM as i32, key.y + CHUNK_2D_DIM as i32)) {
                        // TODO: Right, Right & Front neighbors
                        new_mesh = MeshGen::from_square_heightmap(chunk.data(), CHUNK_2D_DIM);
                    } else {
                        new_mesh = MeshGen::from_square_heightmap_with_r_neighbor(
                            chunk.data(),
                            chunk_r.read().unwrap().data(),
                            CHUNK_2D_DIM);
                    }
                }
            } else if let Some(chunk_f) = root.chunk_from_coord(IVec2::new(key.x, key.y + CHUNK_2D_DIM as i32)) {
                if let Some(chunk_rf) = root.chunk_from_coord(IVec2::new(key.x + CHUNK_2D_DIM as i32, key.y + CHUNK_2D_DIM as i32)) {
                    // TODO: Front, Right & Front neighbors
                    new_mesh = MeshGen::from_square_heightmap(chunk.data(), CHUNK_2D_DIM);
                } else {
                    new_mesh = MeshGen::from_square_heightmap_with_f_neighbor(
                        chunk.data(),
                        chunk_f.read().unwrap().data(),
                        CHUNK_2D_DIM);
                }
            } else {
                // Alone in the world
                // Only having a Right & Front neighbor is an ILLEGAL state which we ignore
                new_mesh = MeshGen::from_square_heightmap(chunk.data(), CHUNK_2D_DIM);
            }

            if let Some(old_mesh_entity) = root_mesher.meshes.get(&key) { commands.entity(*old_mesh_entity).despawn_recursive(); }
            let new_mesh_entity = commands.spawn(PbrBundle {
                    mesh: meshes.add(new_mesh.clone()),
                    material: materials.add(StandardMaterial { base_color: Color::rgb(0.3, 0.9, 0.6), unlit: true, ..default() }),
                    transform: Transform::from_translation(Vec3::new(key.x as f32, 0.0, key.y as f32)),
                    ..default()
                })
                .insert(Collider::from_bevy_mesh(&new_mesh, &ComputedColliderShape::TriMesh).unwrap())
                .id();

            root_mesher.meshes.insert(key, new_mesh_entity);
            new_mesh_entities.push(new_mesh_entity);
        }

        commands.entity(root_entity).push_children(&new_mesh_entities);
        root_changes.clear();
    }
}