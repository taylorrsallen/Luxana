use crate::*;

use bevy::{utils::{hashbrown::Equivalent, HashMap}, gltf::{Gltf, GltfMesh, GltfNode}, transform::TransformSystem};

mod body;
pub use body::*;
mod hitbox;
pub use hitbox::*;
mod socket;
pub use socket::*;

////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct TankThingPartPlugin;
impl Plugin for TankThingPartPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<SocketConnection>()
            .register_type::<SocketConnector>()
            .add_plugins(RuntimeDataAssetPlugin::<BodyData>::new("bodies"))
            .add_plugins(RuntimeDataAssetPlugin::<PartData>::new("parts"))
            .add_systems(PostUpdate, sys_update_socket_connections.before(TransformSystem::TransformPropagate));
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Component)]
pub struct PartMarker;

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Default, Clone, Debug, Reflect)]
pub struct PartData {
    pub sockets: Vec<PartSocket>,
    pub hitbox: Option<PartHitbox>,
    pub rotation: Quat,
    #[reflect(ignore)] pub primitives: Vec<(Handle<Mesh>, Handle<StandardMaterial>)>,
}

impl PartData {
    pub fn female_socket_index(&self) -> Option<usize> {
        for (i, socket) in self.sockets.iter().enumerate() {
            if socket.connector == SocketConnector::Female { return Some(i); }
        }

        None
    }

    pub fn spawn(&self, transform: Transform, commands: &mut Commands) -> Entity {
        commands.spawn(SpatialBundle { transform, ..default() })
            .insert(PartMarker)
            .with_children(|child_builder| {
                for (mesh, material) in self.primitives.iter().cloned() {
                    child_builder.spawn(PbrBundle { mesh, material, transform: Transform::from_rotation(self.rotation.clone()), ..default() });
                    // child_builder.spawn(SpatialBundle::default()).insert(InstancedObject);
                }

                for socket in self.sockets.iter() { child_builder.spawn(SocketBundle::new(socket, &self.rotation)); }
                if let Some(hitbox) = &self.hitbox { child_builder.spawn(PartHitboxBundle::new(hitbox, &self.rotation)); }
            })
            .id()
    }

    /// `connections: &[(MaleSocketIndex, FemaleSocketEntity)]`
    /// 
    /// Returns (PartEntity, PartFemaleSocketEntity)
    pub fn spawn_with_connections(&self, transform: Transform, connections: &[(u8, Entity)], commands: &mut Commands) -> (Entity, Option<Entity>) {
        let mut sockets: Vec<Entity> = vec![];
        let part_entity = commands.spawn(SpatialBundle::default())
            .insert(PartMarker)
            .with_children(|child_builder| {
                for (mesh, material) in self.primitives.iter().cloned() {
                    child_builder.spawn(PbrBundle { mesh, material, transform: Transform::from_rotation(self.rotation.clone()), ..default() });
                }

                sockets = self.sockets.iter().map(|socket| child_builder.spawn(SocketBundle::new(socket, &self.rotation)).id()).collect();
                if let Some(hitbox) = &self.hitbox { child_builder.spawn(PartHitboxBundle::new(hitbox, &self.rotation)); }
            })
            .id();

        for (male_socket_index, female_socket_entity) in connections.iter().copied() {
            commands.entity(sockets[male_socket_index as usize]).insert(SocketConnection(Some(female_socket_entity)));
        }

        let female_socket_index = self.sockets.iter().enumerate()
            .filter(|(_, socket)| socket.connector == SocketConnector::Female)
            .map(|(i, _)| i)
            .last();

        let female_socket_entity = if let Some(index) = female_socket_index { Some(sockets[index]) } else { None };

        (part_entity, female_socket_entity)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct PartLoader;
impl PartLoader {
    pub fn from_gltf<S: AsRef<str>>(
        model_name: S,
        parts_data: &mut ResMut<RuntimeDataAssets<PartData>>,
        gltf_assets: &Res<Assets<Gltf>>,
        gltf_mesh_assets: &Res<Assets<GltfMesh>>,
        gltf_node_assets: &Res<Assets<GltfNode>>,
        packages: &Res<Packages>,
    ) {
        let gltf_handle = packages.models.fetch_handle(model_name.as_ref());
        let gltf = gltf_assets.get(gltf_handle).unwrap();
    
        let mut socket_nodes = vec![];
        let mut bone_nodes = vec![];
        let mut hitbox_nodes = vec![];
        let mut part_node_map = HashMap::default();
        
        // Primitives
        for (node_name, node_handle) in gltf.named_nodes.iter() {
            let node = gltf_node_assets.get(node_handle).unwrap();

            if node_name.contains("Bone") { bone_nodes.push((node_name, node)); continue; }
            if node_name.contains("Socket") { socket_nodes.push((node_name, node)); continue; }
            if node_name.contains("Hitbox") { hitbox_nodes.push((node_name, node)); continue; }
    
            let Some(gltf_mesh) = GltfLoader::try_get_gltf_mesh(node, gltf_mesh_assets) else { continue };
            
            let mut primitives = vec![];
            for primitive in gltf_mesh.primitives.iter() {
                let Some(material) = primitive.material.clone() else { continue };
                primitives.push((primitive.mesh.clone(), material));
            }
    
            part_node_map.insert(node_name.clone(), (node.transform, PartData { primitives, ..default() }));
        }

        // Bones
        for (bone_name, bone_node) in bone_nodes {
            println!("{bone_name}:");
            println!("   Transform: {:?}", bone_node.transform);
            for (i, child_node) in bone_node.children.iter().enumerate() {
                println!("   Child Transform [{i}]: {:?}", child_node.transform);
            }
        }

        // Sockets
        for (socket_name, socket_node) in socket_nodes {
            let Some(socket_str) = socket_name.strip_prefix("Socket.") else { continue };
            let split: Vec<&str> = socket_str.split(".").collect();
            
            if split.len() < 2 { continue; }
            let socket_0_part_name = split[0];
            let socket_0_name = split[1];

            let Some((part_0_transform, part_0_data)) = part_node_map.get_mut(socket_0_part_name) else { continue };
            part_0_data.sockets.push(PartSocket::from_primary_socket_node(socket_0_name, socket_node.transform, part_0_transform.translation));

            let socket_1_part_name = if let Some(name) = split.get(2) { *name } else { continue };
            let Some((part_1_transform, part_1_data)) = part_node_map.get_mut(socket_1_part_name) else { continue };
            let offset_1 = part_1_transform.translation;
            part_1_data.sockets.push(PartSocket::from_secondary_socket_node(socket_node.transform, part_1_transform.translation));
            part_1_data.rotation = socket_node.transform.rotation.inverse();
        }

        // Hitboxes
        for (hitbox_name, hitbox_node) in hitbox_nodes {
            let Some((part_name, hitbox_shape)) = PartHitbox::part_name_and_hitbox_shape_from_hitbox_name(hitbox_name) else { continue };
            let Some(gltf_mesh) = GltfLoader::try_get_gltf_mesh(hitbox_node, gltf_mesh_assets) else { continue };
            let Some((_, part_data)) = part_node_map.get_mut(part_name) else { continue };
            part_data.hitbox = Some(PartHitbox { transform: hitbox_node.transform, shape: hitbox_shape });
        }

        for (name, (_, data)) in part_node_map.iter() {
            parts_data.add(model_name.as_ref().to_owned() + "/" + name, data);
        }
    }
}