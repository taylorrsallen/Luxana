use crate::*;

use bevy::utils::HashMap;

////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct TankLevelHeightmapPlugin;
impl Plugin for TankLevelHeightmapPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<HeightmapRoot>()
            .register_type::<HeightmapRoot>()
            .register_type::<HeightmapChunkMesh>();
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////


////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct HeightmapRoot {
    chunks: HashMap<IVec2, Entity>,
}

impl HeightmapRoot {
    pub fn new() -> Self { Self { chunks: HashMap::<IVec2, Entity>::default() } }
    
    #[inline] pub fn chunks(&self) -> &HashMap<IVec2, Entity> { &self.chunks }

    #[inline] pub fn chunk_from_coord(&self, coord: IVec2) -> Option<&Entity> { self.chunks.get(&(coord & !CHUNK_2D_MASK)) }

    pub fn get_value_at_coord(&self, coord: IVec2) -> f32 {
        0.0
    }

    pub fn add_chunk(&mut self, coord: IVec2) {
        if let Some(chunk_entity) = self.chunk_from_coord(coord) { return; }
        
    }

    pub fn set_value_at_coord(
        &self,
        value: f32,
        coord: IVec2,
        chunk_query: &mut Query<&mut HeightmapChunk>,
    ) {
        let chunk_entity = if let Some(entity) = self.chunk_from_coord(coord) { *entity } else { return };
        let mut chunk = if let Ok(chunk) = chunk_query.get_mut(chunk_entity) { chunk } else { return };
        chunk.set_value_at_coord(value, coord);
    }

    pub fn modify_value_at_coord(
        &self,
        modifier: f32,
        coord: IVec2,
        chunk_query: &mut Query<&mut HeightmapChunk>,
    ) {
        let chunk_entity = if let Some(entity) = self.chunk_from_coord(coord) { *entity } else { return };
        let mut chunk = if let Ok(chunk) = chunk_query.get_mut(chunk_entity) { chunk } else { return };
        chunk.modify_value_at_coord(modifier, coord);
    }

    pub fn modify_values_at_pos(&mut self, modifier: f32, pos: Vec2) {

    }

    pub fn modify_values_in_radius(&mut self, modifier: f32, origin: Vec2, radius: f32) {

    }

    pub fn modify_values_with_radial_falloff(&mut self, modifier: f32, origin: Vec2, radius: f32) {

    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct HeightmapChunk {
    dims: UVec2,
    data: Vec<f32>,
}

impl HeightmapChunk {
    pub fn dims(&self) -> UVec2 { self.dims }
    pub fn data(&self) -> &Vec<f32> { &self.data }
    pub fn data_mut(&mut self) -> &mut Vec<f32> { &mut self.data }

    #[inline] pub fn local_coord_from_global_coord(coord: IVec2) -> UVec2 {
        (coord & CHUNK_2D_MASK).abs().as_uvec2()
    }

    #[inline] pub fn value_index_from_global_coord(coord: IVec2) -> usize {
        let local_coord = Self::local_coord_from_global_coord(coord);
        (local_coord.y * CHUNK_2D_DIM + local_coord.x) as usize
    }

    pub fn zero_init(&mut self) {
        self.data = vec![0.0; (self.dims.x * self.dims.y) as usize];
    }

    pub fn set_value_at_coord(&mut self, value: f32, coord: IVec2) {
        self.data[Self::value_index_from_global_coord(coord)] = value;
    }

    pub fn modify_value_at_coord(&mut self, modifier: f32, coord: IVec2) {
        self.data[Self::value_index_from_global_coord(coord)] += modifier;
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct HeightmapChunkMesh;

fn sys_update_heightmap_chunk_meshes(
    mut commands: Commands,
    heightmap_query: Query<(Entity, &HeightmapChunk, &Parent), (With<HeightmapChunkMesh>, Changed<HeightmapChunk>)>
) {

}