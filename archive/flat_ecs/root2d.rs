use crate::*;
use super::*;

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Component, Reflect)]
pub struct FlatRoot2d {
    log2dim: u8,
    chunks: Vec<Entity>,
}

impl FlatRoot2d {
    pub fn new(log2dim: u8, chunks: &[Entity]) -> Self { Self { log2dim, chunks: chunks.into() } }

    pub fn spawn(seed: u32, log2dim: u8, commands: &mut Commands) {
        let world_dim = 1 << log2dim;
        
        let mut chunks = vec![];
        for y in 0..world_dim { for x in 0..world_dim {
            chunks.push(commands.spawn(VoxelChunk)
                .insert(VisibleObjectBundle::new(Transform::from_translation(Vec3::new(
                    (x * CHUNK2D_DIM) as f32,
                    0.0,
                    (y * CHUNK2D_DIM) as f32,
                ))))
                .id());
        }}
        
        commands.spawn(Self::new(log2dim, &chunks))
            .insert(RngSeed::new(seed));
    }

    #[inline] pub fn dim(&self) -> u32 { 1 << self.log2dim }
    #[inline] pub fn size(&self) -> u32 { 1 << self.log2dim * 2 }
    #[inline] pub fn log2dim(&self) -> u8 { self.log2dim }
    #[inline] pub fn chunks(&self) -> &[Entity] { &self.chunks }
    #[inline] pub fn chunks_mut(&mut self) -> &mut [Entity] { &mut self.chunks }
    
    #[inline] pub fn chunk_from_coord(&self, coord: IVec2) -> Entity { self.chunk_from_index(self.chunk_index_from_coord(coord) as usize) }
    #[inline] pub unsafe fn chunk_from_coord_unchecked(&self, coord: IVec2) -> Entity { self.chunk_from_index_unchecked(self.chunk_index_from_coord(coord) as usize) }
    #[inline] pub fn chunk_from_index(&self, index: usize) -> Entity { self.chunks[index] }
    #[inline] pub unsafe fn chunk_from_index_unchecked(&self, index: usize) -> Entity { *self.chunks.get_unchecked(index) }

    #[inline] pub fn chunk_index_from_coord(&self, coord: IVec2) -> u32 { FlatRoot2dMath::chunk_index_from_coord(coord, self.log2dim) }
    #[inline] pub fn chunk_local_coord_from_index(&self, index: usize) -> IVec2 { FlatRoot2dMath::chunk_local_coord_from_index(index, self.log2dim) }
    #[inline] pub fn chunk_local_coord_from_position(&self, position: Vec3) -> IVec2 { FlatRoot2dMath::chunk_local_coord_from_position(position, self.log2dim) }

    #[inline] pub fn data_index_from_coord(&self, coord: IVec2) -> u32 { FlatRoot2dMath::data_index_from_coord(coord) }
    #[inline] pub fn data_local_coord_from_index(&self, index: usize) -> IVec2 { FlatRoot2dMath::data_local_coord_from_index(index, CHUNK2D_LOG2DIM as u8) }
}

pub struct FlatRoot2dMath;
impl FlatRoot2dMath {
    #[inline]
    pub fn chunk_index_from_coord(coord: IVec2, world_log2dim: u8) -> u32 {
        (((((coord.y & !CHUNK2D_MASK) >> CHUNK2D_LOG2DIM) & ((1 << world_log2dim) - 1)) << world_log2dim) +
          (((coord.x & !CHUNK2D_MASK) >> CHUNK2D_LOG2DIM) & ((1 << world_log2dim) - 1))) as u32
    }

    #[inline]
    pub fn chunk_local_coord_from_index(index: usize,  world_log2dim: u8) -> IVec2 {
        Self::data_local_coord_from_index(index, world_log2dim) << CHUNK2D_LOG2DIM
    }

    #[inline]
    pub fn chunk_local_coord_from_position(position: Vec3, world_log2dim: u8) -> IVec2 {
        Self::chunk_local_coord_from_index(
            Self::chunk_index_from_coord(IVec2::new(position.x as i32, position.z as i32), world_log2dim) as usize,
            world_log2dim,
        )
    }
    
    #[inline]
    pub fn data_index_from_coord(coord: IVec2) -> u32 {
        (((coord.y & CHUNK2D_MASK) << CHUNK2D_LOG2DIM) +
          (coord.x & CHUNK2D_MASK)) as u32
    }

    #[inline]
    pub fn data_local_coord_from_index(index: usize, data_log2dim: u8) -> IVec2 {
        let index = index & ((1 << data_log2dim * 2) - 1);
        let y = (index >> data_log2dim) as i32;
        let x = (index & ((1 << data_log2dim) - 1)) as i32;
        IVec2::new(x, y)
    }
}
