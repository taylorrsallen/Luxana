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
    pub uvs: Vec<[f32; 2]>,
    pub colors: Vec<[f32; 4]>,
}

impl Default for MeshData {
    fn default() -> Self {
        Self { verts: vec![], indices: vec![], uvs: vec![], colors: vec![] }
    }
}

impl MeshData {
    pub fn is_empty(&self) -> bool { self.verts.is_empty() }

    pub fn mesh(&self) -> Mesh {
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, self.verts.clone());
        mesh.set_indices(Some(Indices::U32(self.indices.clone())));
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, self.uvs.clone());
        if !self.colors.is_empty() { mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, self.colors.clone()); }
        mesh
    }

    pub fn add_triangle(&mut self, a: u32, b: u32, c: u32) {
        self.indices.push(a);
        self.indices.push(b);
        self.indices.push(c);
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct NormalsMeshData {
    pub base: MeshData,
    pub normals: Vec<[f32; 3]>,
}

impl Default for NormalsMeshData {
    fn default() -> Self {
        Self { base: MeshData::default(), normals: vec![] }
    }
}

impl NormalsMeshData {
    pub fn is_empty(&self) -> bool { self.base.verts.is_empty() }

    pub fn mesh(&self) -> Mesh {
        let mut mesh = self.base.mesh();
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, self.normals.clone());
        mesh
    }

    pub fn add_triangle(&mut self, a: u32, b: u32, c: u32) { self.base.add_triangle(a, b, c); }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct MeshGen;
impl MeshGen {
    pub fn from_flat_chunk_2d_height_tables<T: Default + NumCast + Clone + Copy + PartialEq + Eq + Sync + Send + 'static>(
        c0_all: &[T],
        c1_side: &[T],
        c2_bottom: &[T],
        c3_corner: T,
        c0_lod: u8,
        c1_lod: u8,
        c2_lod: u8,
        chunk_dim: u32,
        height_multiplier: f32,
    ) -> Mesh {
        let c0_dim = chunk_dim >> c0_lod;
        let _c1_dim = chunk_dim >> c1_lod;
        let _c2_dim = chunk_dim >> c2_lod;

        let mesh_dim = c0_dim + 1;
        let mesh_half_dim = mesh_dim as f32 * 0.5;
        let mesh_size = mesh_dim * mesh_dim;
        let mut mesh_data = MeshData::default();
        mesh_data.verts = Vec::<[f32; 3]>::with_capacity(mesh_size as usize);

        // Verts
        for y in 0..c0_dim {
            for x in 0..c0_dim {
                // Verts Internal
                mesh_data.verts.push([
                    x as f32 - mesh_half_dim,
                    cast::<T, f32>(c0_all[(y * chunk_dim + x) as usize]).unwrap() * height_multiplier,
                    y as f32 - mesh_half_dim,
                ]);
            }

            // Verts Side
            mesh_data.verts.push([
                c0_dim as f32 - mesh_half_dim,
                cast::<T, f32>(c1_side[y as usize]).unwrap() * height_multiplier,
                y as f32 - mesh_half_dim,
            ]);
        }

        // Verts Top
        for x in 0..c0_dim {
            mesh_data.verts.push([
                x as f32 - mesh_half_dim,
                cast::<T, f32>(c2_bottom[x as usize]).unwrap() * height_multiplier,
                c0_dim as f32 - mesh_half_dim,
            ]);
        }

        // Verts Corner
        mesh_data.verts.push([
            c0_dim as f32 - mesh_half_dim,
            cast::<T, f32>(c3_corner).unwrap() * height_multiplier,
            c0_dim as f32 - mesh_half_dim,
        ]);
        
        // Triangles & UVs
        for y in 0..mesh_dim { for x in 0..mesh_dim {
            if x < c0_dim && y < c0_dim {
                let i = y * mesh_dim + x;
                mesh_data.add_triangle(i + 1 + mesh_dim, i + 1, i);
                mesh_data.add_triangle(i + mesh_dim, i + 1 + mesh_dim, i);
            }
    
            mesh_data.uvs.push([x as f32 / mesh_dim as f32, y as f32 / mesh_dim as f32]);
        }}

        // Mesh
        mesh_data.mesh()
    }

    pub fn from_flat_sparse_chunk_3d<T: Default + Clone + Copy + Sync + Send + 'static>(
        chunk_coord: &IVec3,
        root: &FlatSparseRoot3d<T>,
    ) -> Option<Mesh> {
        let mut mesh_data = NormalsMeshData::default();
        let chunk = if let Some(chunk) = root.chunk_from_coord(&chunk_coord) { chunk.read().unwrap() } else { return None };

        for i in OnMaskIter::new(0, chunk.active_mask()) {
            let voxel_coord = root.value_local_coord_from_index(i);
            VoxelMesher::add_cube_with_normals(&mut mesh_data, &voxel_coord, chunk_coord, root);
        }

        if mesh_data.is_empty() { return None; }
        Some(mesh_data.mesh())
    }

    pub fn from_flat_sparse_chunk_2d_terrain_data<T: HeightData + ShapeData + Default + Clone + Copy + Sync + Send + 'static>(
        chunk_coord: &IVec2,
        root: &FlatSparseRoot2d<T>,
    ) -> Option<Mesh> {
        let mut mesh_data = NormalsMeshData::default();
        let chunk = if let Some(chunk) = root.chunk_from_coord(*chunk_coord) { chunk.read().unwrap() } else { return None };

        for i in 0..CHUNK_2D_SIZE {
            let value_coord = root.value_local_coord_from_index(i as u32);
            let global_coord = value_coord + *chunk_coord;
            let value = root.get_value(global_coord);
            Tile3dMesher::add_tile(&mut mesh_data, &value_coord, &global_coord, &value, root);
        }

        if mesh_data.is_empty() { return None; }
        Some(mesh_data.mesh())
    }
}