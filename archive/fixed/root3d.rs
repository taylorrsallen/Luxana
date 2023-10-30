use super::*;

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Component, Reflect)]
pub struct FlatFixedRoot3d {
    log2dim: u8,
    #[reflect(ignore)] chunks: Vec<Arc<RwLock<Leaf3d>>>,
}

impl FlatFixedRoot3d {
    pub fn new(log2dim: u8) -> Self {
        Self {
            log2dim,
            chunks: vec![Leaf3d::default_arc_rwlock(); FlatFixedRoot3dMath::size(log2dim) as usize],
        }
    }

    #[inline] pub fn log2dim(&self) -> u8 { self.log2dim }
    #[inline] pub fn dim(&self) -> u32 { FlatFixedRoot3dMath::dim(self.log2dim) }
    #[inline] pub fn size(&self) -> u32 { FlatFixedRoot3dMath::size(self.log2dim) }
    
    #[inline] pub fn chunk_from_coord(&self, coord: IVec3) -> Arc<RwLock<Leaf3d>> { self.chunk_from_index(self.chunk_index_from_coord(coord)) }
    #[inline] pub unsafe fn chunk_from_coord_unchecked(&self, coord: IVec3) -> Arc<RwLock<Leaf3d>> { self.chunk_from_index_unchecked(self.chunk_index_from_coord(coord)) }
    #[inline] pub fn chunk_from_index(&self, index: u32) -> Arc<RwLock<Leaf3d>> { self.chunks[index as usize].clone() }
    #[inline] pub unsafe fn chunk_from_index_unchecked(&self, index: u32) -> Arc<RwLock<Leaf3d>> { self.chunks.get_unchecked(index as usize).clone() }

    #[inline] pub fn chunk_index_from_coord(&self, coord: IVec3) -> u32 { FlatFixedRoot3dMath::chunk_index_from_coord(coord, self.log2dim) }
    #[inline] pub fn chunk_local_coord_from_index(&self, index: u32) -> IVec3 { FlatFixedRoot3dMath::chunk_local_coord_from_index(index, self.log2dim) }
    #[inline] pub fn chunk_local_coord_from_pos(&self, pos: Vec3) -> IVec3 { FlatFixedRoot3dMath::chunk_local_coord_from_position(pos, self.log2dim) }

    #[inline] pub fn value_index_from_coord(&self, coord: IVec3) -> u32 { FlatFixedRoot3dMath::value_index_from_coord(coord) }
    #[inline] pub fn value_local_coord_from_index(&self, index: u32) -> IVec3 { FlatFixedRoot3dMath::value_local_coord_from_index(index, CHUNK2D_LOG2DIM as u8) }
}

pub struct FlatFixedRoot3dMath;
impl FlatFixedRoot3dMath {
    #[inline] pub fn dim(log2dim: u8) -> u32 { 1 << log2dim }
    #[inline] pub fn size(log2dim: u8) -> u32 { 1 << log2dim * 3 }

    #[inline]
    pub fn chunk_index_from_coord(coord: IVec3, world_log2dim: u8) -> u32 {
        (((((coord.z & !CHUNK3D_MASK) >> CHUNK3D_LOG2DIM) & ((1 << world_log2dim) - 1)) << world_log2dim * 2) +
         ((((coord.y & !CHUNK3D_MASK) >> CHUNK3D_LOG2DIM) & ((1 << world_log2dim) - 1)) << world_log2dim) +
          (((coord.x & !CHUNK3D_MASK) >> CHUNK3D_LOG2DIM) & ((1 << world_log2dim) - 1))) as u32
    }

    #[inline]
    pub fn chunk_local_coord_from_index(index: u32, world_log2dim: u8) -> IVec3 {
        Self::value_local_coord_from_index(index, world_log2dim) << CHUNK3D_LOG2DIM
    }

    #[inline]
    pub fn chunk_local_coord_from_position(position: Vec3, world_log2dim: u8) -> IVec3 {
        Self::chunk_local_coord_from_index(
            Self::chunk_index_from_coord(IVec3::new(position.x as i32, position.y as i32, position.z as i32), world_log2dim),
            world_log2dim,
        )
    }
    
    #[inline]
    pub fn value_index_from_coord(coord: IVec3) -> u32 {
        (((coord.z & CHUNK3D_MASK) << CHUNK3D_LOG2DIM * 2) +
         ((coord.y & CHUNK3D_MASK) << CHUNK3D_LOG2DIM) +
          (coord.x & CHUNK3D_MASK)) as u32
    }

    #[inline]
    pub fn value_local_coord_from_index(index: u32, data_log2dim: u8) -> IVec3 {
        let z = (index >> (data_log2dim * 2)) as i32;
        let index = index & ((1 << data_log2dim * 2) - 1);
        let y = (index >> data_log2dim) as i32;
        let x = (index & ((1 << data_log2dim) - 1)) as i32;
        IVec3::new(x, y, z)
    }
}
