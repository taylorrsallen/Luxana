use crate::*;

use num::{cast, NumCast};
use bevy::render::mesh::*;

mod gltf;
pub use gltf::*;
mod tile;
pub use tile::*;
mod voxel;
pub use voxel::*;

////////////////////////////////////////////////////////////////////////////////////////////////////
pub const TEX_ATLAS_DIM: usize = 32;
pub const TEX_ATLAS_UV_DIM: f32 = 1.0 / TEX_ATLAS_DIM as f32;
pub const CUBE_HALF_DIM: f32 = 0.5;

pub const CUBE_VERTS: [[f32; 3]; 8] = [
    [-CUBE_HALF_DIM, -CUBE_HALF_DIM, -CUBE_HALF_DIM], // 0 Left,  Bottom, Back
    [ CUBE_HALF_DIM, -CUBE_HALF_DIM, -CUBE_HALF_DIM], // 1 Right, Bottom, Back
    [-CUBE_HALF_DIM,  CUBE_HALF_DIM, -CUBE_HALF_DIM], // 2 Left,  Top,    Back,
    [ CUBE_HALF_DIM,  CUBE_HALF_DIM, -CUBE_HALF_DIM], // 3 Right, Top,    Back,
    [-CUBE_HALF_DIM, -CUBE_HALF_DIM,  CUBE_HALF_DIM], // 4 Left,  Bottom, Front,
    [ CUBE_HALF_DIM, -CUBE_HALF_DIM,  CUBE_HALF_DIM], // 5 Right, Bottom, Front,
    [-CUBE_HALF_DIM,  CUBE_HALF_DIM,  CUBE_HALF_DIM], // 6 Left,  Top,    Front,
    [ CUBE_HALF_DIM,  CUBE_HALF_DIM,  CUBE_HALF_DIM], // 7 Right, Top,    Front,
];

pub const CUBE_NORMALS: [[f32; 3]; 6] = [
    [-1.0,  0.0,  0.0], // Left face
    [ 1.0,  0.0,  0.0], // Right face
    [ 0.0, -1.0,  0.0], // Bottom face
    [ 0.0,  1.0,  0.0], // Top face
    [ 0.0,  0.0, -1.0], // Back face
    [ 0.0,  0.0,  1.0], // Front face
];

pub const CUBE_UVS: [[f32; 2]; 4] = [
    [TEX_ATLAS_UV_DIM - TEX_ATLAS_UV_DIM * 0.001, TEX_ATLAS_UV_DIM - TEX_ATLAS_UV_DIM * 0.001],
    [TEX_ATLAS_UV_DIM * 0.001                   , TEX_ATLAS_UV_DIM - TEX_ATLAS_UV_DIM * 0.001],
    [TEX_ATLAS_UV_DIM - TEX_ATLAS_UV_DIM * 0.001, TEX_ATLAS_UV_DIM * 0.001                   ],
    [TEX_ATLAS_UV_DIM * 0.001                   , TEX_ATLAS_UV_DIM * 0.001                   ],
];

pub const CUBE_QUAD_VERTS: [[usize; 4]; 6] = [ // Read as: 0, 1, 2, 2, 1, 3
    [4, 0, 6, 2], // Left face
    [1, 5, 3, 7], // Right face
    [4, 5, 0, 1], // Bottom face
    [2, 3, 6, 7], // Top face
    [0, 1, 2, 3], // Back face
    [5, 4, 7, 6], // Front face
];

pub const CUBE_QUAD_INDICES: [u32; 6] = [0, 2, 1, 1, 2, 3];

////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct MeshData {
    pub verts: Vec<[f32; 3]>,
    pub indices: Vec<u32>,
    pub normals: Vec<[f32; 3]>,
    pub uvs: Vec<[f32; 2]>,
    pub colors: Vec<[f32; 4]>,
}

impl Default for MeshData {
    fn default() -> Self {
        Self { verts: vec![], indices: vec![], normals: vec![], uvs: vec![], colors: vec![] }
    }
}

impl MeshData {
    pub fn is_empty(&self) -> bool { self.verts.is_empty() }

    pub fn mesh(&self) -> Mesh {
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, self.verts.clone());
        mesh.set_indices(Some(Indices::U32(self.indices.clone())));
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, self.normals.clone());
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, self.uvs.clone());
        if !self.colors.is_empty() { mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, self.colors.clone()); }
        mesh
    }

    pub fn add_triangle(&mut self, a: u32, b: u32, c: u32) {
        self.indices.push(a);
        self.indices.push(b);
        self.indices.push(c);
    }

    pub fn calculate_normals(&mut self) {
        let mut normals = vec![Vec3::ZERO; self.verts.len()];

        let tri_count = self.indices.len() / 3;
        for i in 0..tri_count {
            let normal_tri_index = i * 3;
            let vert_index_a = self.indices[normal_tri_index] as usize;
            let vert_index_b = self.indices[normal_tri_index + 1] as usize;
            let vert_index_c = self.indices[normal_tri_index + 2] as usize;

            let tri_normal = self.surface_normals_from_indices(vert_index_a, vert_index_b, vert_index_c);
            normals[vert_index_a] += tri_normal;
            normals[vert_index_b] += tri_normal;
            normals[vert_index_c] += tri_normal;
        }

        self.normals = normals.iter().map(|normal| { normal.normalize().to_array() }).collect();
    }

    pub fn surface_normals_from_indices(&self, index_a: usize, index_b: usize, index_c: usize) -> Vec3 {
        let point_a = Vec3::from_array(self.verts[index_a]);
        let side_ab = Vec3::from_array(self.verts[index_b]) - point_a;
        let side_ac = Vec3::from_array(self.verts[index_c]) - point_a;
        side_ab.cross(side_ac).normalize()
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct MeshGen;
impl MeshGen {
    /// - data: The chunk being meshed
    /// - data_r: Right neighbor
    /// - data_f: Front neighbor
    /// - data_rf: Right & Front neighbor
    pub fn from_square_heightmap_with_r_f_rf_neighbors(
        data: &[f32],
        data_r: &[f32],
        data_f: &[f32],
        data_rf: &[f32],
        dim: u32,
    ) -> Mesh {
        let mut mesh_data = MeshData::default();
        let mesh_dim = dim + 1;

        // Verts
        for z in 0..dim {
            for x in 0..dim {
                // Verts from internal
                let i = z * dim + x;
                mesh_data.verts.push([x as f32, data[i as usize], z as f32]);
            }

            // Verts from left side of right neighbor
            mesh_data.verts.push([dim as f32, data_r[(z * dim) as usize], z as f32]);
        }

        // Verts from bottom of front neighbor
        for x in 0..dim { mesh_data.verts.push([x as f32, data_f[x as usize], dim as f32]); }

        // Vert from bottom left corner of right & front neighbor
        mesh_data.verts.push([dim as f32, data_rf[0], dim as f32]);

        // UVs
        for z in 0..mesh_dim { for x in 0..mesh_dim {
            mesh_data.uvs.push([x as f32 / mesh_dim as f32, z as f32 / mesh_dim as f32]);
        }}

        for z in 0..dim { for x in 0..dim {
            let i = z * mesh_dim + x;
            mesh_data.add_triangle(i, i + mesh_dim, i + 1);
            mesh_data.add_triangle(i + mesh_dim, i + mesh_dim + 1, i + 1);
        }}

        mesh_data.calculate_normals();
        mesh_data.mesh()
    }

    /// - data: The chunk being meshed
    /// - data_r: Right neighbor
    /// - data_f: Front neighbor
    pub fn from_square_heightmap_with_r_f_neighbors(
        data: &[f32],
        data_r: &[f32],
        data_f: &[f32],
        dim: u32,
    ) -> Mesh {
        let mut mesh_data = MeshData::default();
        let mesh_dim = dim + 1;

        // Verts
        for z in 0..dim {
            for x in 0..dim {
                // Verts from internal
                let i = z * dim + x;
                mesh_data.verts.push([x as f32, data[i as usize], z as f32]);
            }

            // Verts from left side of right neighbor
            mesh_data.verts.push([dim as f32, data_r[(z * dim) as usize], z as f32]);
        }

        // Verts from bottom of front neighbor
        for x in 0..dim { mesh_data.verts.push([x as f32, data_f[x as usize], dim as f32]); }

        // UVs
        for z in 0..mesh_dim { for x in 0..mesh_dim {
            mesh_data.uvs.push([x as f32 / mesh_dim as f32, z as f32 / mesh_dim as f32]);
        }}

        // Get rid of last UV because vert doesn't exist
        mesh_data.uvs.pop();

        // All triangles except rightmost column
        for z in 0..dim { for x in 0..dim-1 {
            let i = z * mesh_dim + x;
            mesh_data.add_triangle(i, i + mesh_dim, i + 1);
            mesh_data.add_triangle(i + mesh_dim, i + mesh_dim + 1, i + 1);
        }}

        // Triangles in rightmost column except top right corner
        for z in 0..dim-1 {
            let i = z * mesh_dim + dim-1;
            mesh_data.add_triangle(i, i + mesh_dim, i + 1);
            mesh_data.add_triangle(i + mesh_dim, i + mesh_dim + 1, i + 1);
        }

        // Triangle in top right corner
        let i = (dim-1) * mesh_dim + dim-1;
        mesh_data.add_triangle(i, i + mesh_dim, i + 1);

        mesh_data.calculate_normals();
        mesh_data.mesh()
    }

    /// - data: The chunk being meshed
    /// - data_r: Right neighbor
    pub fn from_square_heightmap_with_r_neighbor(
        data: &[f32],
        data_r: &[f32],
        dim: u32,
    ) -> Mesh {
        let mut mesh_data = MeshData::default();
        let mesh_dims = UVec2::new(dim+1, dim);

        // Verts
        for z in 0..dim {
            for x in 0..dim {
                // Verts from internal
                let i = z * dim + x;
                mesh_data.verts.push([x as f32, data[i as usize], z as f32]);
            }

            // Verts from left side of right neighbor
            mesh_data.verts.push([dim as f32, data_r[(z * dim) as usize], z as f32]);
        }

        // UVs
        for z in 0..mesh_dims.y { for x in 0..mesh_dims.x {
            mesh_data.uvs.push([x as f32 / mesh_dims.x as f32, z as f32 / mesh_dims.y as f32]);
        }}

        // Triangles
        for z in 0..mesh_dims.y-1 { for x in 0..mesh_dims.x-1 {
            let i = z * mesh_dims.x + x;
            mesh_data.add_triangle(i, i + mesh_dims.x, i + 1);
            mesh_data.add_triangle(i + mesh_dims.x, i + mesh_dims.x + 1, i + 1);
        }}

        mesh_data.calculate_normals();
        mesh_data.mesh()
    }

    /// - data: The chunk being meshed
    /// - data_f: Front neighbor
    pub fn from_square_heightmap_with_f_neighbor(
        data: &[f32],
        data_f: &[f32],
        dim: u32,
    ) -> Mesh {
        let mut mesh_data = MeshData::default();
        let mesh_dims = UVec2::new(dim, dim+1);

        // Verts
        for z in 0..dim { for x in 0..dim {
            // Verts from internal
            let i = z * dim + x;
            mesh_data.verts.push([x as f32, data[i as usize], z as f32]);
        }}

        // Verts from bottom of front neighbor
        for x in 0..dim { mesh_data.verts.push([x as f32, data_f[x as usize], dim as f32]); }

        // UVs
        for z in 0..mesh_dims.y { for x in 0..mesh_dims.x {
            mesh_data.uvs.push([x as f32 / mesh_dims.x as f32, z as f32 / mesh_dims.y as f32]);
        }}

        // Triangles
        for z in 0..mesh_dims.y-1 { for x in 0..mesh_dims.x-1 {
            let i = z * mesh_dims.x + x;
            mesh_data.add_triangle(i, i + mesh_dims.x, i + 1);
            mesh_data.add_triangle(i + mesh_dims.x, i + mesh_dims.x + 1, i + 1);
        }}

        mesh_data.calculate_normals();
        mesh_data.mesh()
    }

    pub fn from_square_heightmap(data: &[f32], dim: u32) -> Mesh {
        let mut mesh_data = MeshData::default();

        // Verts & UVs
        for z in 0..dim { for x in 0..dim {
            let i = z * dim + x;
            mesh_data.verts.push([x as f32, data[i as usize], z as f32]);
            mesh_data.uvs.push([x as f32 / dim as f32, z as f32 / dim as f32]);
        }}

        // Triangles
        for z in 0..dim-1 { for x in 0..dim-1 {
            let i = z * dim + x;
            mesh_data.add_triangle(i, i + dim, i + 1);
            mesh_data.add_triangle(i + dim, i + dim + 1, i + 1);
        }}

        mesh_data.calculate_normals();
        mesh_data.mesh()
    }

    // pub fn from_flat_sparse_chunk_3d<T: Default + Clone + Copy + Sync + Send + 'static>(
    //     chunk_coord: &IVec3,
    //     root: &FlatSparseRoot3d<T>,
    // ) -> Option<Mesh> {
    //     let mut mesh_data = MeshData::default();
    //     let chunk = if let Some(chunk) = root.chunk_from_coord(&chunk_coord) { chunk.read().unwrap() } else { return None };

    //     for i in OnMaskIter::new(0, chunk.active_mask()) {
    //         let voxel_coord = root.value_local_coord_from_index(i);
    //         VoxelMesher::add_cube_with_normals(&mut mesh_data, &voxel_coord, chunk_coord, root);
    //     }

    //     if mesh_data.is_empty() { return None; }
    //     Some(mesh_data.mesh())
    // }

    // pub fn from_flat_sparse_chunk_2d_terrain_data<T: HeightData + ShapeData + Default + Clone + Copy + Sync + Send + 'static>(
    //     chunk_coord: &IVec2,
    //     root: &FlatSparseRoot2d<T>,
    // ) -> Option<Mesh> {
    //     let mut mesh_data = MeshData::default();
    //     let chunk = if let Some(chunk) = root.chunk_from_coord(*chunk_coord) { chunk.read().unwrap() } else { return None };

    //     for i in 0..CHUNK_2D_SIZE {
    //         let value_coord = root.value_local_coord_from_index(i as u32);
    //         let global_coord = value_coord + *chunk_coord;
    //         let value = root.get_value(global_coord);
    //         Tile3dMesher::add_tile(&mut mesh_data, &value_coord, &global_coord, &value, root);
    //     }

    //     if mesh_data.is_empty() { return None; }
    //     Some(mesh_data.mesh())
    // }
}