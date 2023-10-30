use crate::*;

////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct VoxelMesher;
impl VoxelMesher {
    pub fn add_cube<T: Default + Clone + Copy + Sync + Send + 'static>(
        mesh_data: &mut MeshData,
        voxel_coord: &IVec3,
        chunk_coord: &IVec3,
        root: &FlatSparseRoot3d<T>,
    ) {
        let global_coord = *voxel_coord + *chunk_coord;
        let neighbors = root.get_adjacent_values(&global_coord);
    
        for face in 0..6 {
            // if neighbors[face as usize] != 0 { continue; } // Opaque Test

            let vert_count = mesh_data.verts.len();
    
            for vert_index in 0..4 {
                let vert = CUBE_VERTS[CUBE_QUAD_VERTS[face as usize][vert_index]];
                mesh_data.verts.push([
                    vert[0] + voxel_coord.x as f32,
                    vert[1] + voxel_coord.y as f32,
                    vert[2] + voxel_coord.z as f32,
                ]);
    
                let uv = CUBE_UVS[vert_index];
                let uv_offset = TEX_ATLAS_UV_DIM * 0.0; // voxel.face_texture_id(face, defs) as f32;
                let uv_offset_floor = uv_offset.floor() as f32;
                mesh_data.uvs.push([
                    uv[0] + uv_offset - uv_offset_floor,
                    uv[1] + uv_offset_floor * TEX_ATLAS_UV_DIM as f32,
                ]);
            }
    
            for tri_index in 0..6 { mesh_data.indices.push(CUBE_QUAD_INDICES[tri_index] + vert_count as u32); }
        }
    }

    pub fn add_cube_with_normals<T: Default + Clone + Copy + Sync + Send + 'static>(
        mesh_data: &mut NormalsMeshData,
        voxel_coord: &IVec3,
        chunk_coord: &IVec3,
        root: &FlatSparseRoot3d<T>,
    ) {
        Self::add_cube(&mut mesh_data.base, voxel_coord, chunk_coord, root);
        for face in 0..6 { mesh_data.normals.extend(vec![CUBE_NORMALS[face as usize]; 4]); }
    }
}