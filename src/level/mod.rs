use crate::*;

mod heightmap;
pub use heightmap::*;

////////////////////////////////////////////////////////////////////////////////////////////////////
const CHUNK_2D_LOG2DIM: u32 = 5;
const CHUNK_2D_DIM: u32 = 1 << CHUNK_2D_LOG2DIM;
const CHUNK_2D_SIZE: usize = (1 << CHUNK_2D_LOG2DIM * 2) as usize;
const CHUNK_2D_WORD_NUM: usize = CHUNK_2D_SIZE >> 6;
const CHUNK_2D_MASK: i32 = CHUNK_2D_DIM as i32 - 1;

////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct TankLevelPlugin;
impl Plugin for TankLevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
                TankLevelHeightmapPlugin,
            ));
    }
}