use super::*;

use bevy::utils::HashMap;

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Component, Reflect)]
pub struct FlatSparseRoot3d<T: Default + Clone + Copy + Sync + Send + 'static> {
    log2dim: u8,
    background: T,
    #[reflect(ignore)] chunks: HashMap<IVec3, Arc<RwLock<Leaf3d<T>>>>,
}

impl<T: Default + Clone + Copy + Sync + Send + 'static> FlatSparseRoot3d<T> {
    pub fn new(log2dim: u8, background: T) -> Self {
        Self {
            log2dim,
            background,
            chunks: HashMap::<IVec3, Arc<RwLock<Leaf3d<T>>>>::default(),
        }
    }

    pub fn adjacent_value_from_index(&self, global_coord: &IVec3, face: usize) -> T {
        self.get_value(&(*global_coord + GRID_3D_DIRECTIONS[face]))
    }

    pub fn get_adjacent_values(&self, global_coord: &IVec3) -> [T; 6] {
        [
            self.adjacent_value_from_index(global_coord, 0),
            self.adjacent_value_from_index(global_coord, 1),
            self.adjacent_value_from_index(global_coord, 2),
            self.adjacent_value_from_index(global_coord, 3),
            self.adjacent_value_from_index(global_coord, 4),
            self.adjacent_value_from_index(global_coord, 5),
        ]
    }

    pub fn get_value(&self, coord: &IVec3) -> T {
        let key = *coord & !CHUNK_3D_MASK;
        if let Some(chunk) = self.chunks.get(&key) {
            chunk.read().unwrap().get_value(self.value_index_from_coord(coord) as usize)
        } else {
            self.background
        }
    }

    pub fn set_value_on(&mut self, coord: &IVec3, value: T) {
        let key = *coord & !CHUNK_3D_MASK;
        if let Some(chunk) = self.chunks.get(&key) {
            chunk.write().unwrap().set_value_on(self.value_index_from_coord(coord) as usize, value);
        } else if !self.is_coord_out_of_bounds(coord) {
            let mut new_chunk = Leaf3d::default();
            new_chunk.set_value_on(self.value_index_from_coord(coord) as usize, value);
            self.chunks.insert(key, Arc::new(RwLock::new(new_chunk)));
        }
    }

    pub fn set_value_off(&mut self, coord: &IVec3) {
        let key = *coord & !CHUNK_3D_MASK;
        if let Some(chunk) = self.chunks.get(&key) {
            chunk.write().unwrap().set_value_off(self.value_index_from_coord(coord) as usize);
        }
    }

    pub fn get_looping_value(&self, coord: &IVec3) -> T {
        let key = *coord & !CHUNK_3D_MASK;
        if let Some(chunk) = self.chunks.get(&key) {
            chunk.read().unwrap().get_value(self.value_index_from_coord(coord) as usize)
        } else {
            self.background
        }
    }

    pub fn set_looping_value_on(&mut self, coord: &IVec3, value: T) {
        let key = *coord & !CHUNK_3D_MASK;
        if let Some(chunk) = self.chunks.get(&key) {
            chunk.write().unwrap().set_value_on(self.value_index_from_coord(coord) as usize, value);
        } else if !self.is_coord_out_of_bounds(coord) {
            let mut new_chunk = Leaf3d::default();
            new_chunk.set_value_on(self.value_index_from_coord(coord) as usize, value);
            self.chunks.insert(key, Arc::new(RwLock::new(new_chunk)));
        }
    }

    pub fn set_looping_value_off(&mut self, coord: &IVec3) {
        let key = *coord & !CHUNK_3D_MASK;
        if let Some(chunk) = self.chunks.get(&key) {
            chunk.write().unwrap().set_value_off(self.value_index_from_coord(coord) as usize);
        }
    }

    #[inline] pub fn log2dim(&self) -> u8 { self.log2dim }
    #[inline] pub fn dim(&self) -> u32 { FlatSparseRoot3dMath::dim(self.log2dim) }
    #[inline] pub fn size(&self) -> u32 { FlatSparseRoot3dMath::size(self.log2dim) }
    #[inline] pub fn background(&self) -> T { self.background }
    #[inline] pub fn chunks(&self) -> &HashMap<IVec3, Arc<RwLock<Leaf3d<T>>>> { &self.chunks }

    #[inline] pub fn is_coord_out_of_bounds(&self, coord: &IVec3) -> bool {
        let half_total_dim = ((self.dim() * CHUNK_3D_DIM) >> 1) as i32;
        let min = -half_total_dim;
        let max = half_total_dim - 1;

        if coord.x < min || coord.x > max || coord.y < min || coord.y > max || coord.z < min || coord.z > max { return true; }
        false
    }

    #[inline] pub fn chunk_from_coord(&self, coord: &IVec3) -> Option<&Arc<RwLock<Leaf3d<T>>>> { self.chunks.get(coord) }

    #[inline] pub fn chunk_coord_from_coord(&self, coord: &IVec3) -> IVec3 { FlatSparseRoot3dMath::chunk_local_coord_from_coord(coord) }
    #[inline] pub fn chunk_coord_from_pos(&self, pos: &Vec3) -> IVec3 { FlatSparseRoot3dMath::chunk_local_coord_from_pos(pos) }

    #[inline] pub fn chunk_looping_index_from_coord(&self, coord: &IVec3) -> u32 { FlatSparseRoot3dMath::chunk_looping_index_from_coord(coord, self.log2dim) }
    #[inline] pub fn chunk_looping_coord_from_index(&self, index: u32) -> IVec3 { FlatSparseRoot3dMath::chunk_looping_coord_from_index(index, self.log2dim) }
    #[inline] pub fn chunk_looping_coord_from_coord(&self, coord: &IVec3) -> IVec3 { FlatSparseRoot3dMath::chunk_looping_coord_from_coord(coord, self.log2dim) }
    #[inline] pub fn chunk_looping_coord_from_pos(&self, pos: &Vec3) -> IVec3 { FlatSparseRoot3dMath::chunk_looping_coord_from_pos(pos, self.log2dim) }

    #[inline] pub fn value_index_from_coord(&self, coord: &IVec3) -> u32 { FlatSparseRoot3dMath::value_index_from_coord(coord) }
    #[inline] pub fn value_local_coord_from_index(&self, index: u32) -> IVec3 { FlatSparseRoot3dMath::local_coord_from_index(index, CHUNK_3D_LOG2DIM as u8) }
}

pub struct FlatSparseRoot3dMath;
impl FlatSparseRoot3dMath {
    #[inline] pub fn dim(log2dim: u8) -> u32 { 1 << log2dim }
    #[inline] pub fn size(log2dim: u8) -> u32 { 1 << log2dim * 3 }

    #[inline] pub fn chunk_local_coord_from_coord(coord: &IVec3) -> IVec3 { *coord & !CHUNK_3D_MASK }
    #[inline] pub fn chunk_local_coord_from_pos(pos: &Vec3) -> IVec3 { pos.as_ivec3() & !CHUNK_3D_MASK }
    
    #[inline]
    pub fn chunk_looping_index_from_coord(coord: &IVec3, root_log2dim: u8) -> u32 {
        (((((coord.z & !CHUNK_3D_MASK) >> CHUNK_3D_LOG2DIM) & ((1 << root_log2dim) - 1)) << root_log2dim * 2) +
         ((((coord.y & !CHUNK_3D_MASK) >> CHUNK_3D_LOG2DIM) & ((1 << root_log2dim) - 1)) << root_log2dim) +
          (((coord.x & !CHUNK_3D_MASK) >> CHUNK_3D_LOG2DIM) & ((1 << root_log2dim) - 1))) as u32
    }

    #[inline]
    pub fn chunk_looping_coord_from_index(index: u32, root_log2dim: u8) -> IVec3 {
        Self::local_coord_from_index(index, root_log2dim) << CHUNK_3D_LOG2DIM
    }

    #[inline]
    pub fn chunk_looping_coord_from_coord(coord: &IVec3, root_log2dim: u8) -> IVec3 {
        Self::chunk_looping_coord_from_index(
            Self::chunk_index_from_coord(coord, root_log2dim),
            root_log2dim,
        )
    }

    #[inline]
    pub fn chunk_looping_coord_from_pos(pos: &Vec3, root_log2dim: u8) -> IVec3 {
        Self::chunk_looping_coord_from_index(
            Self::chunk_looping_index_from_coord(&pos.as_ivec3(), root_log2dim),
            root_log2dim,
        )
    }

    #[inline]
    pub fn chunk_index_from_coord(coord: &IVec3, root_log2dim: u8) -> u32 {
        (((coord.z & ((1 << root_log2dim) - 1)) << root_log2dim * 2) +
         ((coord.y & ((1 << root_log2dim) - 1)) << root_log2dim) +
          (coord.x & ((1 << root_log2dim) - 1))) as u32
    }

    #[inline]
    pub fn value_index_from_coord(coord: &IVec3) -> u32 {
        (((coord.z & CHUNK_3D_MASK) << CHUNK_3D_LOG2DIM * 2) +
         ((coord.y & CHUNK_3D_MASK) << CHUNK_3D_LOG2DIM) +
          (coord.x & CHUNK_3D_MASK)) as u32
    }

    #[inline]
    pub fn local_coord_from_index(index: u32, log2dim: u8) -> IVec3 {
        let z = (index >> (log2dim * 2)) as i32;
        let index = index & ((1 << log2dim * 2) - 1);
        let y = (index >> log2dim) as i32;
        let x = (index & ((1 << log2dim) - 1)) as i32;
        IVec3::new(x, y, z)
    }
}
