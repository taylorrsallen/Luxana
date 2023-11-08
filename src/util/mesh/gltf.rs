use crate::*;

use bevy::gltf::*;

////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct GltfLoader;
impl GltfLoader {
    pub fn spawn_gltf(
        gltf: &Gltf,
        commands: &mut Commands,
        gltf_mesh_assets: &Res<Assets<GltfMesh>>,
        gltf_node_assets: &Res<Assets<GltfNode>>,
    ) -> Vec<Entity> {
        let mut entities = vec![];
        for gltf_node_handle in gltf.nodes.iter() {
            let gltf_node = gltf_node_assets.get(gltf_node_handle).unwrap();
            if let Some(gltf_mesh_handle) = &gltf_node.mesh {
                let gltf_mesh = gltf_mesh_assets.get(&gltf_mesh_handle).unwrap();
                entities.push(commands.spawn(PbrBundle {
                        mesh: gltf_mesh.primitives[0].mesh.clone(),
                        material: gltf_mesh.primitives[0].material.clone().unwrap(),
                        transform: gltf_node.transform,
                        ..default()
                    })
                    .id());
            }
        }
    
        entities
    }

    #[inline]
    pub fn try_get_gltf_mesh<'a>(
        node: &GltfNode,
        gltf_mesh_assets: &'a Res<Assets<GltfMesh>>,
    ) -> Option<&'a GltfMesh> {
        if let Some(mesh_handle) = &node.mesh {
            if let Some(mesh) = gltf_mesh_assets.get(mesh_handle) { Some(&mesh) } else { None }
        } else { None }
    }
}