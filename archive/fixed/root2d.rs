use super::*;

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Component, Reflect)]
pub struct FlatFixedRoot2d {
    log2dim: u8,
    #[reflect(ignore)] chunks: Vec<Arc<RwLock<Leaf2d>>>,
}

impl FlatFixedRoot2d {
    pub fn new(log2dim: u8) -> Self {
        Self {
            log2dim,
            chunks: vec![Leaf2d::default_arc_rwlock(); FlatFixedRoot2dMath::size(log2dim) as usize],
        }
    }

    #[inline] pub fn log2dim(&self) -> u8 { self.log2dim }
    #[inline] pub fn dim(&self) -> u32 { FlatFixedRoot2dMath::dim(self.log2dim) }
    #[inline] pub fn size(&self) -> u32 { FlatFixedRoot2dMath::size(self.log2dim) }
    
    #[inline] pub fn chunk_from_coord(&self, coord: IVec2) -> Arc<RwLock<Leaf2d>> { self.chunk_from_index(self.chunk_index_from_coord(coord)) }
    #[inline] pub unsafe fn chunk_from_coord_unchecked(&self, coord: IVec2) -> Arc<RwLock<Leaf2d>> { self.chunk_from_index_unchecked(self.chunk_index_from_coord(coord)) }
    #[inline] pub fn chunk_from_index(&self, index: u32) -> Arc<RwLock<Leaf2d>> { self.chunks[index as usize].clone() }
    #[inline] pub unsafe fn chunk_from_index_unchecked(&self, index: u32) -> Arc<RwLock<Leaf2d>> { self.chunks.get_unchecked(index as usize).clone() }

    #[inline] pub fn chunk_index_from_coord(&self, coord: IVec2) -> u32 { FlatFixedRoot2dMath::chunk_index_from_coord(coord, self.log2dim) }
    #[inline] pub fn chunk_local_coord_from_index(&self, index: u32) -> IVec2 { FlatFixedRoot2dMath::chunk_local_coord_from_index(index, self.log2dim) }
    #[inline] pub fn chunk_local_coord_from_position(&self, position: Vec3) -> IVec2 { FlatFixedRoot2dMath::chunk_local_coord_from_position(position, self.log2dim) }

    #[inline] pub fn value_index_from_coord(&self, coord: IVec2) -> u32 { FlatFixedRoot2dMath::value_index_from_coord(coord) }
    #[inline] pub fn value_local_coord_from_index(&self, index: u32) -> IVec2 { FlatFixedRoot2dMath::value_local_coord_from_index(index, CHUNK2D_LOG2DIM as u8) }
}

pub struct FlatFixedRoot2dMath;
impl FlatFixedRoot2dMath {
    #[inline] pub fn dim(log2dim: u8) -> u32 { 1 << log2dim }
    #[inline] pub fn size(log2dim: u8) -> u32 { 1 << log2dim * 2 }

    #[inline]
    pub fn chunk_index_from_coord(coord: IVec2, world_log2dim: u8) -> u32 {
        (((((coord.y & !CHUNK2D_MASK) >> CHUNK2D_LOG2DIM) & ((1 << world_log2dim) - 1)) << world_log2dim) +
          (((coord.x & !CHUNK2D_MASK) >> CHUNK2D_LOG2DIM) & ((1 << world_log2dim) - 1))) as u32
    }

    #[inline]
    pub fn chunk_local_coord_from_index(index: u32,  world_log2dim: u8) -> IVec2 {
        Self::value_local_coord_from_index(index, world_log2dim) << CHUNK2D_LOG2DIM
    }

    #[inline]
    pub fn chunk_local_coord_from_position(position: Vec3, world_log2dim: u8) -> IVec2 {
        Self::chunk_local_coord_from_index(
            Self::chunk_index_from_coord(IVec2::new(position.x as i32, position.z as i32), world_log2dim),
            world_log2dim,
        )
    }
    
    #[inline]
    pub fn value_index_from_coord(coord: IVec2) -> u32 {
        (((coord.y & CHUNK2D_MASK) << CHUNK2D_LOG2DIM) +
          (coord.x & CHUNK2D_MASK)) as u32
    }

    #[inline]
    pub fn value_local_coord_from_index(index: u32, data_log2dim: u8) -> IVec2 {
        let index = index & ((1 << data_log2dim * 2) - 1);
        let y = (index >> data_log2dim) as i32;
        let x = (index & ((1 << data_log2dim) - 1)) as i32;
        IVec2::new(x, y)
    }
}
