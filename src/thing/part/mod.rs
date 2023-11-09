use crate::*;

use bevy::{utils::{hashbrown::Equivalent, HashMap}, gltf::{Gltf, GltfMesh, GltfNode}};

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Component, Default, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
#[reflect(Component, Default)]
pub struct SocketConnection(pub Option<Entity>);

#[derive(Component, Default, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
#[reflect(Component, Default)]
pub enum SocketConnector {
    #[default]
    /// Only rotates along one axis, connects to any Revolute
    Revolute,
    /// Allows free rotation for connected, connects to Female
    SphereMale,
    /// Connected cannot rotate, connects to Female
    FixedMale,
    /// Follows transform of connected, connects to any Male
    Female,
}

#[derive(Default, Clone, Copy, Debug, Reflect)]
pub struct PartSocket {
    pub offset: Vec3,
    pub connector: SocketConnector,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Clone, Debug, Reflect)]
pub struct PartHitbox {
    pub transform: Transform,
    pub mesh: Handle<Mesh>,
}

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
#[derive(Debug, Reflect)]
pub struct PartData {
    pub sockets: Vec<PartSocket>,
    pub primitives: Vec<PartPrimitiveData>,
    pub hitbox: PartHitbox,
}

impl PartData {
    pub fn spawn(&self, transform: Transform, commands: &mut Commands, meshes: &mut ResMut<Assets<Mesh>>) -> Entity {
        commands.spawn(VisibleTransformBundle { transform, ..default() })
            .with_children(|child_builder| {
                for socket in self.sockets.iter() {
                    child_builder.spawn(TransformBundle::from_transform(Transform::from_translation(socket.offset)))
                        .insert(SocketConnection::default())
                        .insert(socket.connector);
                }

                for primitive in self.primitives.iter() {
                    child_builder.spawn(PbrBundle {
                        mesh: primitive.mesh.clone(),
                        material: primitive.material.clone(),
                        ..default()
                    });
                }

                child_builder.spawn(TransformBundle::from_transform(self.hitbox.transform))
                    .insert(VisibilityBundle::default())
                    .insert(RigidBody::KinematicPositionBased)
                    .insert(Collider::from_bevy_mesh(meshes.get(&self.hitbox.mesh).unwrap(), &ComputedColliderShape::TriMesh).unwrap())
                    .insert(Sensor);
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
        gltf_assets: &Res<Assets<Gltf>>,
        gltf_mesh_assets: &Res<Assets<GltfMesh>>,
        gltf_node_assets: &Res<Assets<GltfNode>>,
        packages: &Res<Packages>,
    ) -> Self {
        let gltf_handle = packages.models.fetch_handle("test_character");
        let gltf = gltf_assets.get(gltf_handle).unwrap();
    
        let mut sockets = vec![];
        let mut hitboxes = vec![];
        let mut part_map = HashMap::default();
        
        // Mesh Transforms & Primitives
        for (node_name, node_handle) in gltf.named_nodes.iter() {
            let node = gltf_node_assets.get(node_handle).unwrap();

            if node_name.contains("Socket") { sockets.push((node_name, node)); continue; }
            if node_name.contains("Hitbox") { hitboxes.push((node_name, node)); continue; }
    
            let gltf_mesh = if let Some(mesh) = GltfLoader::try_get_gltf_mesh(node, gltf_mesh_assets) { mesh } else { continue };
            
            let mut primitives = vec![];
            for primitive in gltf_mesh.primitives.iter() {
                let material = if let Some(material) = &primitive.material { material } else { continue };
                primitives.push(PartPrimitiveData::new(primitive.mesh.clone(), material.clone()));
            }
    
            part_map.insert(node_name.clone(), (node.transform, vec![], primitives));
        }

        // Sockets
        for (socket_name, socket_node) in sockets.iter().copied() {
            let socket_str = if let Some(name) = socket_name.strip_prefix("Socket.") { name } else { continue };
            let split: Vec<&str> = socket_str.split(".").collect();
            
            if split.len() < 2 { continue; }

            let socket_offset = socket_node.transform.translation;
            let (part_0_transform, part_0_sockets, _) = if let Some(part) = part_map.get_mut(split[0]) { part } else { continue };
            let offset_0 = part_0_transform.translation;
            
            let socket_0;
            let mut socket_1_connector = SocketConnector::Female;
            if split[1].eq("R") {
                socket_0 = PartSocket { offset: socket_offset - offset_0, connector: SocketConnector::Revolute };
                socket_1_connector = SocketConnector::Revolute;
            } else if split[1].eq("S") {
                socket_0 = PartSocket { offset: socket_offset - offset_0, connector: SocketConnector::SphereMale };
            } else {
                socket_0 = PartSocket { offset: socket_offset - offset_0, connector: SocketConnector::FixedMale };
            }
            
            part_0_sockets.push(socket_0);

            if split.get(2).is_none() { continue; }

            let (part_1_transform, part_1_sockets, _) = if let Some(part) = part_map.get_mut(split[2]) { part } else { continue };
            let offset_1 = part_1_transform.translation;
            part_1_sockets.push(PartSocket { offset: socket_offset - offset_1, connector: socket_1_connector });
        }

        // Hitboxes & Finalization
        let mut part_data_map = PartDataMap::default();
        for (hitbox_name, hitbox_node) in hitboxes.iter() {
            println!("{hitbox_name}");
            let part_name = if let Some(name) = hitbox_name.strip_prefix("Hitbox.") { name } else { continue };
            println!("{part_name}");
            let gltf_mesh = if let Some(mesh) = GltfLoader::try_get_gltf_mesh(hitbox_node, gltf_mesh_assets) { mesh } else { continue };
            
            let (sockets, primitives) = if let Some(part) = part_map.get(part_name) { (part.1.clone(), part.2.clone()) } else { continue };
            let hitbox = PartHitbox { transform: hitbox_node.transform, mesh: gltf_mesh.primitives[0].mesh.clone() };

            println!("{part_name}");
            part_data_map.insert(part_name.into(), PartData { sockets, primitives, hitbox });
        }
    
        part_data_map
    }
}