use crate::*;

////////////////////////////////////////////////////////////////////////////////////////////////////
pub const TILE_VERTS: [[f32; 2]; 4] = [
    [-CUBE_HALF_DIM, -CUBE_HALF_DIM], // 0 Left,  Back
    [ CUBE_HALF_DIM, -CUBE_HALF_DIM], // 1 Right, Back
    [-CUBE_HALF_DIM,  CUBE_HALF_DIM], // 2 Left,  Front
    [ CUBE_HALF_DIM,  CUBE_HALF_DIM], // 3 Right, Front
];

pub const TILE_HEIGHT_VALUES: [f32; 3] = [0.0, CUBE_HALF_DIM, CUBE_HALF_DIM * 2.0];

const TILE_HEIGHT_INDICES: [[u8; 4]; 17] = [
    [0, 0, 0, 0], // 0  Cube
    [1, 0, 1, 0], // 1  Left  Ramp
    [0, 1, 0, 1], // 2  Right Ramp
    [1, 1, 0, 0], // 3  Back  Ramp
    [0, 0, 1, 1], // 4  Front Ramp
    [0, 0, 0, 1], // 5  Left  Back  External Corner
    [0, 0, 1, 0], // 6  Right Back  External Corner
    [0, 1, 0, 0], // 7  Left  Front External Corner
    [1, 0, 0, 0], // 8  Right Front External Corner
    [0, 1, 1, 1], // 9  Left  Back  Internal Corner
    [1, 0, 1, 1], // 10 Left  Back  Internal Corner
    [1, 1, 0, 1], // 11 Left  Back  Internal Corner
    [1, 1, 1, 0], // 12 Left  Back  Internal Corner
    [0, 1, 1, 2], // 13 Left  Back  Rhombus
    [1, 0, 2, 1], // 14 Right Back  Rhombus
    [1, 2, 0, 1], // 15 Left  Front Rhombus
    [2, 1, 1, 0], // 16 Right Front Rhombus
];

const TILE_QUAD_TRI_INDICES: [[u8; 6]; 2] = [ // Ramp type &= 1 to get the index
    [0, 2, 1, 1, 2, 3], // Right Back Slope & Left Front Slope
    [2, 3, 0, 0, 3, 1], // Left Back Slope & Right Front Slope
];

const TILE_TEMP_TEXTURE_IDS: [u8; 17] = [
    0,
    4, 2, 4, 4,
    1, 3, 3, 5,
    1, 3, 3, 5,
    1, 3, 3, 5,
];

////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct Tile3dMesher;
impl Tile3dMesher {
    pub fn add_tile<T: HeightData + ShapeData + Default + Clone + Copy + Sync + Send + 'static>(
        mesh_data: &mut NormalsMeshData,
        value_coord: &IVec2,
        global_coord: &IVec2,
        value: &T,
        root: &FlatSparseRoot2d<T>,
    ) {
        let shape = value.shape() as usize;
        let vert_count = mesh_data.base.verts.len();

        for vert_index in 0..4 {
            let vert = TILE_VERTS[vert_index];
            mesh_data.base.verts.push([
                vert[0] + value_coord.x as f32,
                (value.height() as f32 - 127.0) * 0.5 + TILE_HEIGHT_VALUES[TILE_HEIGHT_INDICES[shape][vert_index] as usize],
                vert[1] + value_coord.y as f32,
            ]);

            let uv = CUBE_UVS[vert_index];
            let uv_offset = TEX_ATLAS_UV_DIM * 4.0; // voxel.face_texture_id(face, defs) as f32;
            let uv_offset_floor = uv_offset.floor() as f32;
            mesh_data.base.uvs.push([uv[0] + uv_offset - uv_offset_floor, uv[1] + uv_offset_floor * TEX_ATLAS_UV_DIM as f32]);
        }

        mesh_data.normals.extend(vec![CUBE_NORMALS[3]; 4]);

        for tri_index in 0..6 { mesh_data.base.indices.push(CUBE_QUAD_INDICES[tri_index] + vert_count as u32); }
    }
}

// pub struct Tile2dMesher;
// impl Tile2dMesher {
//     pub fn add_tile<T: HeightData + ShapeData + Default + Clone + Copy + Sync + Send + 'static>(
//         mesh_data: &mut NormalsMeshData,
//         value_coord: &IVec2,
//         global_coord: &IVec2,
//         value: &T,
//         root: &FlatSparseRoot2d<T>,
//     ) {
//         let shape = value.shape() as usize;
//         let vert_count = mesh_data.base.verts.len();

//         for vert_index in 0..4 {
//             let vert = TILE_VERTS[vert_index];
//             mesh_data.base.verts.push([
//                 vert[0] + value_coord.x as f32,
//                 (value.height() as f32 - 127.0) * 0.5 + TILE_HEIGHT_VALUES[TILE_HEIGHT_INDICES[shape][vert_index] as usize],
//                 vert[1] + value_coord.y as f32,
//             ]);

//             let uv = CUBE_UVS[vert_index];
//             let uv_offset = TEX_ATLAS_UV_DIM * TILE_TEMP_TEXTURE_IDS[shape] as f32; // voxel.face_texture_id(face, defs) as f32;
//             let uv_offset_floor = uv_offset.floor() as f32;
//             mesh_data.base.uvs.push([uv[0] + uv_offset - uv_offset_floor, uv[1] + uv_offset_floor * TEX_ATLAS_UV_DIM as f32]);
//         }

//         mesh_data.normals.extend(vec![CUBE_NORMALS[3]; 4]);

//         for tri_index in 0..6 { mesh_data.base.indices.push(CUBE_QUAD_INDICES[tri_index] + vert_count as u32); }
//     }
// }