use super::*;

use bevy::utils::HashMap;

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Component, Reflect)]
pub struct FlatSparseRoot2d<T: Default + Clone + Copy + Sync + Send + 'static> {
    log2dim: u8,
    background: T,
    #[reflect(ignore)] chunks: HashMap<IVec2, Arc<RwLock<Leaf2d<T>>>>,
}

impl<T: Default + Clone + Copy + Sync + Send + 'static> FlatSparseRoot2d<T> {
    pub fn new(log2dim: u8, background: T) -> Self {
        Self { log2dim, background, chunks: HashMap::<IVec2, Arc<RwLock<Leaf2d<T>>>>::default() }
    }

    #[inline] pub fn log2dim(&self) -> u8 { self.log2dim }
    #[inline] pub fn dim(&self) -> u32 { FlatSparseRoot2dMath::dim(self.log2dim) }
    #[inline] pub fn size(&self) -> u32 { FlatSparseRoot2dMath::size(self.log2dim) }
    #[inline] pub fn background(&self) -> T { self.background }
    #[inline] pub fn chunks(&self) -> &HashMap<IVec2, Arc<RwLock<Leaf2d<T>>>> { &self.chunks }

    #[inline] pub fn total_dim(&self) -> i32 { (self.dim() * CHUNK_2D_DIM) as i32 }
    #[inline] pub fn half_total_dim(&self) -> i32 { self.total_dim() >> 1 }

    #[inline] pub fn chunk_from_coord(&self, coord: IVec2) -> Option<&Arc<RwLock<Leaf2d<T>>>> { self.chunks.get(&coord) }
    
    #[inline] pub fn chunk_index_from_global_coord(&self, coord: IVec2) -> u32 { FlatSparseRoot2dMath::chunk_index_from_coord(coord, self.log2dim) }
    #[inline] pub fn chunk_coord_from_global_coord(&self, coord: IVec2) -> IVec2 { FlatSparseRoot2dMath::chunk_coord_from_global_coord(coord) }
    #[inline] pub fn chunk_coord_from_pos2d(&self, pos: &Vec3) -> IVec2 { FlatSparseRoot2dMath::chunk_coord_from_pos2d(pos) }
    #[inline] pub fn chunk_coord_from_pos3d(&self, pos: &Vec3) -> IVec2 { FlatSparseRoot2dMath::chunk_coord_from_pos3d(pos) }
    #[inline] pub fn chunk_coord_from_index(&self, index: u32) -> IVec2 { FlatSparseRoot2dMath::chunk_coord_from_index(index, self.log2dim) }

    #[inline] pub fn value_index_from_coord(&self, coord: IVec2) -> u32 { FlatSparseRoot2dMath::value_index_from_coord(coord) }
    #[inline] pub fn value_local_coord_from_index(&self, index: u32) -> IVec2 { FlatSparseRoot2dMath::local_coord_from_index(index, CHUNK_2D_LOG2DIM as u8) }

    #[inline] pub fn is_coord_out_of_bounds(&self, coord: IVec2) -> bool {
        let (min, max) = (-self.half_total_dim(), self.half_total_dim() - 1);
        if coord.x < min || coord.x > max || coord.y < min || coord.y > max { true } else { false }
    }

    pub fn adjacent_value_from_direction_index(&self, global_coord: IVec2, face: usize) -> T {
        self.get_value(global_coord + GRID_2D_DIRECTIONS[face])
    }

    pub fn adjacent_value_from_diagonal_index(&self, global_coord: IVec2, face: usize) -> T {
        self.get_value(global_coord + GRID_2D_DIAGONALS[face])
    }

    pub fn get_adjacent_values(&self, global_coord: IVec2) -> [T; 4] {
        [
            self.adjacent_value_from_direction_index(global_coord, 0),
            self.adjacent_value_from_direction_index(global_coord, 1),
            self.adjacent_value_from_direction_index(global_coord, 2),
            self.adjacent_value_from_direction_index(global_coord, 3),
        ]
    }

    pub fn get_diagonal_values(&self, global_coord: IVec2) -> [T; 4] {
        [
            self.adjacent_value_from_diagonal_index(global_coord, 0),
            self.adjacent_value_from_diagonal_index(global_coord, 1),
            self.adjacent_value_from_diagonal_index(global_coord, 2),
            self.adjacent_value_from_diagonal_index(global_coord, 3),
        ]
    }

    pub fn get_value(&self, coord: IVec2) -> T {
        let key = coord & !CHUNK_2D_MASK;
        if let Some(chunk) = self.chunks.get(&key) {
            chunk.read().unwrap().get_value(self.value_index_from_coord(coord) as usize)
        } else {
            self.background
        }
    }

    pub fn set_value_on(&mut self, coord: IVec2, value: T) {
        let key = coord & !CHUNK_2D_MASK;
        if let Some(chunk) = self.chunks.get(&key) {
            chunk.write().unwrap().set_value_on(self.value_index_from_coord(coord) as usize, value);
        } else if !self.is_coord_out_of_bounds(coord) {
            let mut new_chunk = Leaf2d::default();
            new_chunk.set_value_on(self.value_index_from_coord(coord) as usize, value);
            self.chunks.insert(key, Arc::new(RwLock::new(new_chunk)));
        }
    }

    pub fn set_value_off(&mut self, coord: IVec2) {
        let key = coord & !CHUNK_2D_MASK;
        if let Some(chunk) = self.chunks.get(&key) {
            chunk.write().unwrap().set_value_off(self.value_index_from_coord(coord) as usize);
        }
    }
}

pub struct FlatSparseRoot2dMath;
impl FlatSparseRoot2dMath {
    #[inline] pub fn dim(log2dim: u8) -> u32 { 1 << log2dim }
    #[inline] pub fn size(log2dim: u8) -> u32 { 1 << log2dim * 2 }

    #[inline] pub fn chunk_coord_from_global_coord(coord: IVec2) -> IVec2 { coord & !CHUNK_2D_MASK }
    #[inline] pub fn chunk_coord_from_pos2d(pos: &Vec3) -> IVec2 { IVec2::new(pos.x as i32, pos.y as i32) & !CHUNK_2D_MASK }
    #[inline] pub fn chunk_coord_from_pos3d(pos: &Vec3) -> IVec2 { IVec2::new(pos.x as i32, pos.z as i32) & !CHUNK_2D_MASK }
    #[inline] pub fn chunk_coord_from_index(index: u32, root_log2dim: u8) -> IVec2 { Self::local_coord_from_index(index, root_log2dim) * CHUNK_2D_DIM as i32 }

    #[inline]
    pub fn chunk_index_from_coord(coord: IVec2, root_log2dim: u8) -> u32 {
        (((((coord.y & !CHUNK_2D_MASK) >> CHUNK_2D_LOG2DIM) & ((1 << root_log2dim) - 1)) << root_log2dim) +
          (((coord.x & !CHUNK_2D_MASK) >> CHUNK_2D_LOG2DIM) & ((1 << root_log2dim) - 1))) as u32
    }

    #[inline]
    pub fn chunk_local_coord_from_index(index: u32,  root_log2dim: u8) -> IVec2 {
        Self::local_coord_from_index(index, root_log2dim) << CHUNK_2D_LOG2DIM
    }
    
    #[inline]
    pub fn value_index_from_coord(coord: IVec2) -> u32 {
        (((coord.y & CHUNK_2D_MASK) << CHUNK_2D_LOG2DIM) +
          (coord.x & CHUNK_2D_MASK)) as u32
    }

    #[inline]
    pub fn local_coord_from_index(index: u32, data_log2dim: u8) -> IVec2 {
        let index = index & ((1 << data_log2dim * 2) - 1);
        let y = (index >> data_log2dim) as i32;
        let x = (index & ((1 << data_log2dim) - 1)) as i32;
        IVec2::new(x, y)
    }

    /// 2d world, z value is discarded
    pub fn global_coord_from_vec2(pos: Vec2) -> IVec2 {
        let pos_decimal = pos.abs() - pos.abs().floor();
        let coord_offset = IVec2::new(
            if pos_decimal.x > 0.5 { pos.x.signum() as i32 } else { 0 },
            if pos_decimal.y > 0.5 { pos.y.signum() as i32 } else { 0 },
        );

        IVec2::new(pos.x as i32, pos.y as i32) + coord_offset
    }

    /// z value is discarded
    pub fn global_coord_from_pos2d(pos: &Vec3) -> IVec2 { Self::global_coord_from_vec2(Vec2::new(pos.x, pos.y)) }

    /// y value is discarded
    pub fn global_coord_from_pos3d(pos: &Vec3) -> IVec2 { Self::global_coord_from_vec2(Vec2::new(pos.x, pos.z)) }
}