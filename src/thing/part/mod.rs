use crate::*;

use bevy::{utils::{hashbrown::Equivalent, HashMap}, gltf::{Gltf, GltfMesh, GltfNode}};

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Default)]
pub struct PartData {
    pub sockets: Vec<Vec3>,
    pub meshes: Vec<Handle<Mesh>>,
    pub materials: Vec<Handle<StandardMaterial>>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct PartsDataMap(HashMap<String, PartData>);

impl Default for PartsDataMap {
    fn default() -> Self { Self { 0: HashMap::default() } }
}

impl PartsDataMap {
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
    
        let mut parts_data_map = PartsDataMap::default();
        // let mut primitives_map = HashMap::default();
        for (node_name, node_handle) in gltf.named_nodes.iter() {
            println!("{}", node_name);
            if node_name.contains("Hit") { continue; }
    
            let node = gltf_node_assets.get(node_handle).unwrap();
            let gltf_mesh = if let Some(mesh_handle) = &node.mesh {
                    if let Some(mesh) = gltf_mesh_assets.get(mesh_handle) { mesh } else { continue }
                } else { continue };
    
            for child_node in node.children.iter() {
                println!("Child detected");
                if let Some(extras) = &child_node.extras { println!("{}", extras.value); }
            }
    
    
            // primitives_map.insert(node_name.clone(), (node.transform, mesh, material));
            parts_data_map.insert(node_name.clone(), PartData {
                sockets: vec![],
                meshes: gltf_mesh.primitives.iter().map(|primitive| { primitive.mesh.clone() }).collect(),
                materials: gltf_mesh.primitives.iter().map(|primitive| { primitive.material.as_ref().unwrap().clone() }).collect(),
            });
        }
    
        parts_data_map
    }

    pub fn from_primitives(
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
    ) -> Self {
        let mut parts_data_map = Self::default();
    
        let pelvis_mesh = meshes.add(Mesh::from(shape::Box::new(1.0, 1.0, 1.0)));
        let pelvis_material = materials.add(StandardMaterial { base_color: Color::RED.into(), ..default() });
    
        let stomach_mesh = meshes.add(Mesh::from(shape::Box::new(1.0, 1.0, 1.0)));
        let stomach_material = materials.add(StandardMaterial { base_color: Color::ORANGE.into(), ..default() });
    
        let chest_mesh = meshes.add(Mesh::from(shape::Box::new(1.0, 1.0, 1.0)));
        let chest_material = materials.add(StandardMaterial { base_color: Color::YELLOW.into(), ..default() });
    
        let neck_mesh = meshes.add(Mesh::from(shape::Box::new(1.0, 1.0, 1.0)));
        let neck_material = materials.add(StandardMaterial { base_color: Color::GREEN.into(), ..default() });
    
        let head_mesh = meshes.add(Mesh::from(shape::Box::new(1.0, 1.0, 1.0)));
        let head_material = materials.add(StandardMaterial { base_color: Color::BLUE.into(), ..default() });
    
        let upper_arm_mesh = meshes.add(Mesh::from(shape::Box::new(0.125, 0.125, 0.5)));
        let lower_arm_mesh = meshes.add(Mesh::from(shape::Box::new(0.1, 0.1, 0.5)));
        let hand_mesh = meshes.add(Mesh::try_from(shape::Icosphere { radius: 0.1, subdivisions: 5 }).unwrap());
        let upper_arm_material = materials.add(StandardMaterial { base_color: Color::rgb(0.2, 0.2, 0.2).into(), ..default() });
        let lower_arm_material = materials.add(StandardMaterial { base_color: Color::rgb(0.35, 0.35, 0.35).into(), ..default() });
        let hand_material = materials.add(StandardMaterial { base_color: Color::rgb(0.7, 0.7, 0.7).into(), ..default() });
        
        let upper_leg_mesh = meshes.add(Mesh::from(shape::Box::new(0.125, 0.125, 0.5)));
        let lower_leg_mesh = meshes.add(Mesh::from(shape::Box::new(0.1, 0.1, 0.5)));
        let foot_mesh = meshes.add(Mesh::from(shape::Box::new(0.1, 0.075, 0.2)));
        let upper_leg_material = materials.add(StandardMaterial { base_color: Color::rgb(0.2, 0.2, 0.2).into(), ..default() });
        let lower_leg_material = materials.add(StandardMaterial { base_color: Color::rgb(0.35, 0.35, 0.35).into(), ..default() });
        let foot_material = materials.add(StandardMaterial { base_color: Color::rgb(0.7, 0.7, 0.7).into(), ..default() });
        
        parts_data_map.insert("Pelvis".into(), PartData { meshes: vec![pelvis_mesh], materials: vec![pelvis_material], ..default() });
        parts_data_map.insert("Stomach".into(), PartData { meshes: vec![stomach_mesh], materials: vec![stomach_material], ..default() });
        parts_data_map.insert("Chest".into(), PartData { meshes: vec![chest_mesh], materials: vec![chest_material], ..default() });
        parts_data_map.insert("Neck".into(), PartData { meshes: vec![neck_mesh], materials: vec![neck_material], ..default() });
        parts_data_map.insert("Head".into(), PartData { meshes: vec![head_mesh], materials: vec![head_material], ..default() });
    
        parts_data_map.insert("LArmUpper".into(), PartData { meshes: vec![upper_arm_mesh.clone()], materials: vec![upper_arm_material.clone()], ..default() });
        parts_data_map.insert("LArmLower".into(), PartData { meshes: vec![lower_arm_mesh.clone()], materials: vec![lower_arm_material.clone()], ..default() });
        parts_data_map.insert("LHand".into(), PartData { meshes: vec![hand_mesh.clone()], materials: vec![hand_material.clone()], ..default() });
        parts_data_map.insert("RArmUpper".into(), PartData { meshes: vec![upper_arm_mesh], materials: vec![upper_arm_material], ..default() });
        parts_data_map.insert("RArmLower".into(), PartData { meshes: vec![lower_arm_mesh], materials: vec![lower_arm_material], ..default() });
        parts_data_map.insert("RHand".into(), PartData { meshes: vec![hand_mesh], materials: vec![hand_material], ..default() });
    
        parts_data_map.insert("LLegUpper".into(), PartData { meshes: vec![upper_leg_mesh.clone()], materials: vec![upper_leg_material.clone()], ..default() });
        parts_data_map.insert("LLegLower".into(), PartData { meshes: vec![lower_leg_mesh.clone()], materials: vec![lower_leg_material.clone()], ..default() });
        parts_data_map.insert("LFoot".into(), PartData { meshes: vec![foot_mesh.clone()], materials: vec![foot_material.clone()], ..default() });
        parts_data_map.insert("RLegUpper".into(), PartData { meshes: vec![upper_leg_mesh], materials: vec![upper_leg_material], ..default() });
        parts_data_map.insert("RLegLower".into(), PartData { meshes: vec![lower_leg_mesh], materials: vec![lower_leg_material], ..default() });
        parts_data_map.insert("RFoot".into(), PartData { meshes: vec![foot_mesh], materials: vec![foot_material], ..default() });
    
        parts_data_map
    }
}