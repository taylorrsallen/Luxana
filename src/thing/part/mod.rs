use crate::*;

mod hitbox;
pub use hitbox::*;
mod socket;
pub use socket::*;

use bevy::{utils::{hashbrown::Equivalent, HashMap}, gltf::{Gltf, GltfMesh, GltfNode}};

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Clone, Debug, Reflect)]
pub struct PartPrimitiveData {
    pub mesh: Handle<Mesh>,
    pub material: Handle<StandardMaterial>,
}

impl PartPrimitiveData {
    pub fn new(mesh: Handle<Mesh>, material: Handle<StandardMaterial>) -> Self {
        Self { mesh, material }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Default, Debug, Reflect)]
pub struct PartData {
    pub sockets: Vec<PartSocket>,
    pub primitives: Vec<PartPrimitiveData>,
    pub hitbox: Option<PartHitbox>,
}

impl PartData {
    pub fn spawn(&self, transform: Transform, commands: &mut Commands, meshes: &mut ResMut<Assets<Mesh>>) -> Entity {
        commands.spawn(VisibleTransformBundle { transform, ..default() })
            .with_children(|child_builder| {
                for socket in self.sockets.iter() { child_builder.spawn(SocketBundle::new(socket)); }
                if let Some(hitbox) = &self.hitbox { child_builder.spawn(PartHitboxBundle::new(hitbox)); }

                for primitive in self.primitives.iter() {
                    child_builder.spawn(PbrBundle { mesh: primitive.mesh.clone(), material: primitive.material.clone(), ..default() });
                    // child_builder.spawn(SpatialBundle::default())
                    //     .insert(InstancedObject);
                }
            })
            .id()
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Default)]
pub struct PartDataMap(HashMap<String, PartData>);

impl PartDataMap {
    pub fn get<Q: ?Sized>(&self, key: &Q) -> Option<&PartData>
    where
        Q: std::hash::Hash + Equivalent<String>,
    {
        self.0.get(key)
    }

    pub fn insert(&mut self, key: String, value: PartData) {
        self.0.insert(key, value);
    }

    pub fn from_gltf(
        gltf_handle: &Handle<Gltf>,
        gltf_assets: &Res<Assets<Gltf>>,
        gltf_mesh_assets: &Res<Assets<GltfMesh>>,
        gltf_node_assets: &Res<Assets<GltfNode>>,
    ) -> Self {
        let gltf = gltf_assets.get(gltf_handle).unwrap();
    
        let mut socket_nodes = vec![];
        let mut hitbox_nodes = vec![];
        let mut part_map = HashMap::default();
        
        // Mesh Transforms & Primitives
        for (node_name, node_handle) in gltf.named_nodes.iter() {
            let node = gltf_node_assets.get(node_handle).unwrap();

            if node_name.contains("Socket") { socket_nodes.push((node_name, node)); continue; }
            if node_name.contains("Hitbox") { hitbox_nodes.push((node_name, node)); continue; }
    
            let Some(gltf_mesh) = GltfLoader::try_get_gltf_mesh(node, gltf_mesh_assets) else { continue };
            
            let mut primitives = vec![];
            for primitive in gltf_mesh.primitives.iter() {
                let Some(material) = &primitive.material else { continue };
                primitives.push(PartPrimitiveData::new(primitive.mesh.clone(), material.clone()));
            }
    
            part_map.insert(node_name.clone(), (node.transform, PartData { primitives, ..default() }));
        }

        // Sockets
        for (socket_name, socket_node) in socket_nodes.iter().copied() {
            let Some(socket_str) = socket_name.strip_prefix("Socket.") else { continue };
            let split: Vec<&str> = socket_str.split(".").collect();
            
            if split.len() < 2 { continue; }
            let socket_0_part_name = split[0];
            let socket_0_name = split[1];

            let Some((part_0_transform, part_0_data)) = part_map.get_mut(socket_0_part_name) else { continue };
            part_0_data.sockets.push(PartSocket::from_primary_socket_node(socket_0_name, socket_node.transform, part_0_transform.translation));

            let socket_1_part_name = if let Some(name) = split.get(2) { *name } else { continue };
            let Some((part_1_transform, part_1_data)) = part_map.get_mut(socket_1_part_name) else { continue };
            let offset_1 = part_1_transform.translation;
            part_1_data.sockets.push(PartSocket::from_secondary_socket_node(socket_node.transform, part_1_transform.translation));
        }

        // Hitboxes & Finalization
        let mut part_data_map = PartDataMap::default();
        for (hitbox_name, hitbox_node) in hitbox_nodes.iter().copied() {
            let Some((part_name, hitbox_shape)) = PartHitbox::part_name_and_hitbox_shape_from_hitbox_name(hitbox_name) else { continue };
            let Some(gltf_mesh) = GltfLoader::try_get_gltf_mesh(hitbox_node, gltf_mesh_assets) else { continue };
            let Some((_, part_data)) = part_map.get_mut(part_name) else { continue };

            part_data.hitbox = Some(PartHitbox { transform: hitbox_node.transform, shape: hitbox_shape });
        }
    
        part_data_map
    }
}