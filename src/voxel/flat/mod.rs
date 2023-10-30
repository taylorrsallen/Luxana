use crate::*;

use std::sync::{Arc, RwLock};

mod chunk;
pub use chunk::*;
mod sparse;
pub use sparse::*;

////////////////////////////////////////////////////////////////////////////////////////////////////
pub const CHUNK_2D_LOG2DIM: u32 = 4;
pub const CHUNK_2D_DIM: u32 = 1 << CHUNK_2D_LOG2DIM;
pub const CHUNK_2D_SIZE: usize = (1 << CHUNK_2D_LOG2DIM * 2) as usize;
pub const CHUNK_2D_WORD_NUM: usize = CHUNK_2D_SIZE >> 6;
pub const CHUNK_2D_MASK: i32 = CHUNK_2D_DIM as i32 - 1;

pub const CHUNK_3D_LOG2DIM: u32 = 3;
pub const CHUNK_3D_DIM: u32 = 1 << CHUNK_3D_LOG2DIM;
pub const CHUNK_3D_DIM_USIZE: usize = 1 << CHUNK_3D_LOG2DIM;
pub const CHUNK_3D_SIZE: usize = (1 << CHUNK_3D_LOG2DIM * 3) as usize;
pub const CHUNK_3D_WORD_NUM: usize = CHUNK_3D_SIZE >> 6;
pub const CHUNK_3D_MASK: i32 = CHUNK_3D_DIM as i32 - 1;

pub const MAX_WORLD_LOG2DIM: u8 = 8;

pub struct VoxelFlatPlugin;
impl Plugin for VoxelFlatPlugin {
    fn build(&self, app: &mut App) {
        
    }
}