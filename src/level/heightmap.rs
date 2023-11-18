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
/// For tracking changed chunks. Just use mark_change with global coord any time you edit a chunk.
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
/// Put this on a [HeightmapRoot] along with [HeightmapRootChanges], and the heightmap will be meshed.
/// 
/// You must mark any changes made using [HeightmapRootChanges], or the mesh will not update.
/// 
/// If a [Handle<StandardMaterial>] is on the entity, it will be used as the material for the mesh.
#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct HeightmapRootMesher {
    meshes: HashMap<IVec2, Entity>,
}

fn sys_update_heightmap_meshes(
    mut commands: Commands,
    mut heightmap_query: Query<(Entity, &HeightmapRoot, &mut HeightmapRootMesher, &mut HeightmapRootChanges), Changed<HeightmapRootChanges>>,
    material_query: Query<&Handle<StandardMaterial>, With<HeightmapRootMesher>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (root_entity, root, mut root_mesher, mut root_changes) in heightmap_query.iter_mut() {
        let mut new_mesh_entities = vec![];
        for key in root_changes.iter() {
            let Some(new_mesh) = try_get_heightmap_mesh(key, root) else { continue };

            if let Some(old_mesh_entity) = root_mesher.meshes.get(&key) { commands.entity(*old_mesh_entity).despawn_recursive(); }

            let mesh = meshes.add(new_mesh.clone());
            let material = if let Ok(material) = material_query.get(root_entity) { material.clone() } else { materials.add(StandardMaterial { base_color: Color::rgb(0.3, 0.9, 0.6), perceptual_roughness: 0.9, ..default() }) };
            let transform = Transform::from_translation(Vec3::new(key.x as f32, 0.0, key.y as f32));
            let new_mesh_entity = commands.spawn(PbrBundle { mesh, material, transform, ..default() })
                .insert(Collider::from_bevy_mesh(&new_mesh, &ComputedColliderShape::TriMesh).unwrap())
                .id();

            root_mesher.meshes.insert(key, new_mesh_entity);
            new_mesh_entities.push(new_mesh_entity);
        }

        commands.entity(root_entity).push_children(&new_mesh_entities);
        root_changes.clear();
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
fn try_get_heightmap_mesh(
    key: IVec2,
    root: &HeightmapRoot,
) -> Option<Mesh> {
    let chunk = if let Some(chunk) = root.chunk_from_coord(key) { chunk.read().unwrap() } else { return None };

    let mut neighbor_flags: u32 = 0;
    let chunk_r = if let Some(chunk_r) = root.chunk_from_coord(IVec2::new(key.x + CHUNK_2D_DIM as i32, key.y)) { neighbor_flags |= 1; Some(chunk_r.read().unwrap()) } else { None };
    let chunk_f = if let Some(chunk_f) = root.chunk_from_coord(IVec2::new(key.x, key.y + CHUNK_2D_DIM as i32)) { neighbor_flags |= 2; Some(chunk_f.read().unwrap()) } else { None };
    let chunk_rf = if let Some(chunk_rf) = root.chunk_from_coord(IVec2::new(key.x + CHUNK_2D_DIM as i32, key.y + CHUNK_2D_DIM as i32)) { neighbor_flags |= 4; Some(chunk_rf.read().unwrap()) } else { None };

    Some(match neighbor_flags {
        // R neighbor
        1 => { MeshGen::from_square_heightmap_with_r_neighbor(chunk.data(), chunk_r.unwrap().data(), CHUNK_2D_DIM) },
        // F neighbor
        2 => { MeshGen::from_square_heightmap_with_f_neighbor(chunk.data(), chunk_f.unwrap().data(), CHUNK_2D_DIM) },
        // R & F neighbors
        3 => { MeshGen::from_square_heightmap_with_r_f_neighbors(chunk.data(), chunk_r.unwrap().data(), chunk_f.unwrap().data(), CHUNK_2D_DIM) }
        // Only RF neighbor: impossible case which is ignored
        4 => { MeshGen::from_square_heightmap(chunk.data(), CHUNK_2D_DIM) }
        // TODO: R & RF neighbors
        5 => { MeshGen::from_square_heightmap(chunk.data(), CHUNK_2D_DIM) }
        // TODO: F & RF neighbors
        6 => { MeshGen::from_square_heightmap(chunk.data(), CHUNK_2D_DIM) }
        // R & F & RF neighbors
        7 => { MeshGen::from_square_heightmap_with_r_f_rf_neighbors(chunk.data(), chunk_r.unwrap().data(), chunk_f.unwrap().data(), chunk_rf.unwrap().data(), CHUNK_2D_DIM) }
        // No neighbors
        _ => { MeshGen::from_square_heightmap(chunk.data(), CHUNK_2D_DIM) }
    })
}