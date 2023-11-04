use crate::*;

mod heightmap;
pub use heightmap::*;
mod overworld;
pub use overworld::*;

////////////////////////////////////////////////////////////////////////////////////////////////////
/// 0. Left
/// 1. Right
/// 2. Back
/// 3. Front
/// 
/// ```
/// x --- 3 --- x
/// |     |     |
/// 0 --- o --- 1
/// |     |     |
/// x --- 2 --- x
/// ```
pub const GRID_2D_DIRECTIONS: [IVec2; 4] = [
    IVec2::new(-1,  0), // Left
    IVec2::new( 1,  0), // Right
    IVec2::new( 0, -1), // Back
    IVec2::new( 0,  1), // Front
];

/// 0. Left  Back
/// 1. Right Back
/// 2. Left  Front
/// 3. Right Front
/// 
/// ```
/// 2 --- x --- 3
/// |     |     |
/// x --- o --- x
/// |     |     |
/// 0 --- x --- 1
/// ```
pub const GRID_2D_DIAGONALS: [IVec2; 4] = [
    IVec2::new(-1, -1), // Left  Back
    IVec2::new( 1, -1), // Right Back
    IVec2::new(-1,  1), // Left  Front
    IVec2::new( 1,  1), // Right Front
];

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
                TankLevelOverworldPlugin,
            ));
    }
}