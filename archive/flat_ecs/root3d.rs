use crate::*;
use super::*;

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Component, Reflect)]
pub struct FlatRoot3d {
    log2dim: u8,
    chunks: Vec<Entity>,
}

impl FlatRoot3d {
    pub fn new(log2dim: u8, chunks: &[Entity]) -> Self { Self { log2dim, chunks: chunks.into() } }

    pub fn spawn(seed: u32, log2dim: u8, commands: &mut Commands) -> Entity {
        let world_dim = 1 << log2dim;
        
        let mut chunks = vec![];
        for z in 0..world_dim { for y in 0..world_dim { for x in 0..world_dim {
            chunks.push(commands.spawn(VoxelChunk)
                .insert(VisibleObjectBundle::new(Transform::from_translation(Vec3::new(
                    (x * CHUNK3D_DIM) as f32,
                    (y * CHUNK3D_DIM) as f32,
                    (z * CHUNK3D_DIM) as f32,
                ))))
                .id());
        }}}
        
        commands.spawn(Self::new(log2dim, &chunks))
            .insert(RngSeed::new(seed))
            .id()
    }

    #[inline] pub fn dim(&self) -> u32 { 1 << self.log2dim }
    #[inline] pub fn size(&self) -> u32 { 1 << self.log2dim * 3 }
    #[inline] pub fn log2dim(&self) -> u8 { self.log2dim }
    #[inline] pub fn chunks(&self) -> &[Entity] { &self.chunks }
    #[inline] pub fn chunks_mut(&mut self) -> &mut [Entity] { &mut self.chunks }
    
    #[inline] pub fn chunk_from_coord(&self, coord: IVec3) -> Entity { self.chunk_from_index(self.chunk_index_from_coord(coord) as usize) }
    #[inline] pub unsafe fn chunk_from_coord_unchecked(&self, coord: IVec3) -> Entity { self.chunk_from_index_unchecked(self.chunk_index_from_coord(coord) as usize) }
    #[inline] pub fn chunk_from_index(&self, index: usize) -> Entity { self.chunks[index] }
    #[inline] pub unsafe fn chunk_from_index_unchecked(&self, index: usize) -> Entity { *self.chunks.get_unchecked(index) }

    #[inline] pub fn chunk_index_from_coord(&self, coord: IVec3) -> u32 { FlatRoot3dMath::chunk_index_from_coord(coord, self.log2dim) }
    #[inline] pub fn chunk_local_coord_from_index(&self, index: usize) -> IVec3 { FlatRoot3dMath::chunk_local_coord_from_index(index, self.log2dim) }
    #[inline] pub fn chunk_local_coord_from_position(&self, position: Vec3) -> IVec3 { FlatRoot3dMath::chunk_local_coord_from_position(position, self.log2dim) }

    #[inline] pub fn data_index_from_coord(&self, coord: IVec3) -> u32 { FlatRoot3dMath::data_index_from_coord(coord) }
    #[inline] pub fn data_local_coord_from_index(&self, index: usize) -> IVec3 { FlatRoot3dMath::data_local_coord_from_index(index, CHUNK3D_LOG2DIM as u8) }
}

pub struct FlatRoot3dMath;
impl FlatRoot3dMath {
    #[inline]
    pub fn chunk_index_from_coord(coord: IVec3, world_log2dim: u8) -> u32 {
        (((((coord.z & !CHUNK3D_MASK) >> CHUNK3D_LOG2DIM) & ((1 << world_log2dim) - 1)) << world_log2dim * 2) +
         ((((coord.y & !CHUNK3D_MASK) >> CHUNK3D_LOG2DIM) & ((1 << world_log2dim) - 1)) << world_log2dim) +
          (((coord.x & !CHUNK3D_MASK) >> CHUNK3D_LOG2DIM) & ((1 << world_log2dim) - 1))) as u32
    }

    #[inline]
    pub fn chunk_local_coord_from_index(index: usize, world_log2dim: u8) -> IVec3 {
        Self::data_local_coord_from_index(index, world_log2dim) << CHUNK3D_LOG2DIM
    }

    #[inline]
    pub fn chunk_local_coord_from_position(position: Vec3, world_log2dim: u8) -> IVec3 {
        Self::chunk_local_coord_from_index(
            Self::chunk_index_from_coord(IVec3::new(position.x as i32, position.y as i32, position.z as i32), world_log2dim) as usize,
            world_log2dim,
        )
    }
    
    #[inline]
    pub fn data_index_from_coord(coord: IVec3) -> u32 {
        (((coord.z & CHUNK3D_MASK) << CHUNK3D_LOG2DIM * 2) +
         ((coord.y & CHUNK3D_MASK) << CHUNK3D_LOG2DIM) +
          (coord.x & CHUNK3D_MASK)) as u32
    }

    #[inline]
    pub fn data_local_coord_from_index(index: usize, data_log2dim: u8) -> IVec3 {
        let z = (index >> (data_log2dim * 2)) as i32;
        let index = index & ((1 << data_log2dim * 2) - 1);
        let y = (index >> data_log2dim) as i32;
        let x = (index & ((1 << data_log2dim) - 1)) as i32;
        IVec3::new(x, y, z)
    }
}
