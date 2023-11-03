use crate::*;

mod flat;
pub use flat::*;
mod traits;
pub use traits::*;

////////////////////////////////////////////////////////////////////////////////////////////////////
pub const GRID_3D_DIRECTIONS: [IVec3; 6] = [
    IVec3::new(-1,  0,  0), // Left
    IVec3::new( 1,  0,  0), // Right
    IVec3::new( 0, -1,  0), // Bottom
    IVec3::new( 0,  1,  0), // Top
    IVec3::new( 0,  0, -1), // Back
    IVec3::new( 0,  0,  1), // Front
];

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

////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct TankVoxelPlugin;
impl Plugin for TankVoxelPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TankVoxelFlatPlugin);
    }
}