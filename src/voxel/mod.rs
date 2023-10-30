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

pub const GRID_2D_DIRECTIONS: [IVec2; 4] = [
    IVec2::new(-1,  0), // Left
    IVec2::new( 1,  0), // Right
    IVec2::new( 0, -1), // Back
    IVec2::new( 0,  1), // Front
];

pub const GRID_2D_DIAGONALS: [IVec2; 4] = [
    IVec2::new(-1, -1), // Left
    IVec2::new( 1, -1), // Right
    IVec2::new(-1,  1), // Back
    IVec2::new( 1,  1), // Front
];

////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct LuxanaVoxelPlugin;
impl Plugin for LuxanaVoxelPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(VoxelFlatPlugin);
    }
}