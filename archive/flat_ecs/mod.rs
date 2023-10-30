use crate::*;

mod chunk;
mod root2d;
mod root3d;
pub use chunk::*;
pub use root2d::*;
pub use root3d::*;

////////////////////////////////////////////////////////////////////////////////////////////////////
pub const CHUNK2D_LOG2DIM: u32 = 4;
pub const CHUNK2D_DIM: u32 = 1 << CHUNK2D_LOG2DIM;
pub const CHUNK2D_SIZE: usize = (1 << CHUNK2D_LOG2DIM * 2) as usize;
pub const CHUNK2D_WORD_NUM: usize = CHUNK2D_SIZE >> 6;
pub const CHUNK2D_MASK: i32 = (1 << CHUNK2D_LOG2DIM) - 1;

pub const CHUNK3D_LOG2DIM: u32 = 3;
pub const CHUNK3D_DIM: u32 = 1 << CHUNK3D_LOG2DIM;
pub const CHUNK3D_DIM_USIZE: usize = 1 << CHUNK3D_LOG2DIM;
pub const CHUNK3D_SIZE: usize = (1 << CHUNK3D_LOG2DIM * 3) as usize;
pub const CHUNK3D_WORD_NUM: usize = CHUNK3D_SIZE >> 6;
pub const CHUNK3D_MASK: i32 = (1 << CHUNK3D_LOG2DIM) - 1;

pub const MAX_WORLD_LOG2DIM: u8 = 8;

pub struct VoxelFlatPlugin;
impl Plugin for VoxelFlatPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<FlatRoot2d>()
            .register_type::<FlatRoot3d>();
    }
}