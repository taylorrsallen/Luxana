use crate::*;

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Default, Clone, Copy, Debug, Reflect)]
pub struct BodyConnectionData {
    from_male_socket: u8,
    to_body_part: u8,
    in_female_socket: u8,
}

#[derive(Default, Clone, Debug, Reflect)]
pub struct BodyPartData {
    part_id: u16,
    connections: Vec<BodyConnectionData>,
}

#[derive(Default, Clone, Debug, Reflect)]
pub struct BodyData {
    parts: Vec<BodyPartData>,
}

impl BodyData {
    // pub fn spawn(
    //     &self,
    //     transform: Transform,
    //     commands: &mut Commands,
    //     parts_data: &RuntimeDataAssets<PartData>,
    // ) -> Entity {
    //     let body_part = &self.parts[0];
        
    //     let mut children = vec![];
    //     for connection in body_part.connections.iter() {
    //         let male_socket_index = connection.from_male_socket;
    //         let Some(female_socket_entity) = self.spawn_body_part_recursive(*connection, commands, parts_data).1 else { continue };
    //         children.push((male_socket_index, female_socket_entity));
    //     }
        
    //     parts_data.asset_from_id(body_part.part_id as usize).spawn_with_connections(transform, &children, commands).0
    // }

    // fn spawn_body_part_recursive(
    //     &self,
    //     parent_connection: BodyConnectionData,
    //     commands: &mut Commands,
    //     parts_data: &RuntimeDataAssets<PartData>,
    // ) -> (Entity, Option<Entity>) {
    //     let body_part_id = parent_connection.to_body_part;
    //     let body_part = &self.parts[body_part_id as usize];
        
    //     let mut children = vec![];
    //     for connection in body_part.connections.iter() {
    //         let male_socket_index = connection.from_male_socket;
    //         let Some(female_socket_entity) = self.spawn_body_part_recursive(*connection, commands, parts_data).1 else { continue };
    //         children.push((male_socket_index, female_socket_entity));
    //     }

    //     let part_data = parts_data.asset_from_id(body_part.part_id as usize);
    //     if !children.is_empty() {
    //         part_data.spawn_with_connections(Transform::IDENTITY, &children, commands)
    //     } else {
    //         (part_data.spawn(Transform::IDENTITY, commands), None)
    //     }
    // }

    pub fn from_humanoid<S: AsRef<str>>(
        model_name: S,
        parts_data: &RuntimeDataAssets<PartData>,
    ) -> Self {
        let model_prefix = model_name.as_ref().to_owned() + "/";
        let mut body_data = Self::default();

        let pelvis_body_part_id = body_data.parts.len() as u8;
        body_data.parts.push(BodyPartData {
            part_id: parts_data.id_from_name(model_prefix.clone() + "Pelvis"),
            connections: vec![],
        });

        body_data.add_humanoid_limb_data(pelvis_body_part_id, 0, &model_prefix, "LLeg", parts_data);
        body_data.add_humanoid_limb_data(pelvis_body_part_id, 1, &model_prefix, "RLeg", parts_data);

        let stomach_part_id = parts_data.id_from_name(model_prefix.clone() + "Stomach");
        let stomach_part_data = parts_data.get(stomach_part_id as usize);
        let stomach_body_part_id = body_data.parts.len() as u8;
        body_data.parts.push(BodyPartData { part_id: stomach_part_id, connections: vec![] });

        for (i, socket) in stomach_part_data.sockets.iter().enumerate() {
            if socket.connector == SocketConnector::Female {
                body_data.parts[pelvis_body_part_id as usize].connections.push(BodyConnectionData { from_male_socket: 2, to_body_part: stomach_body_part_id, in_female_socket: i as u8 });
            } else {
                body_data.parts[stomach_body_part_id as usize].connections.push(BodyConnectionData { from_male_socket: i as u8, to_body_part: stomach_body_part_id + 1, in_female_socket: 0 });
            }
        }
        
        let chest_part_id = parts_data.id_from_name(model_prefix.clone() + "Chest");
        let chest_part_data = parts_data.get(chest_part_id as usize);
        let chest_body_part_id = body_data.parts.len() as u8;
        body_data.parts.push(BodyPartData { part_id: chest_part_id, connections: vec![] });

        let mut male_socket_count = 0;
        let mut chest_neck_socket = 0;
        for (i, socket) in stomach_part_data.sockets.iter().enumerate() {
            if socket.connector == SocketConnector::Female {
                for connection in body_data.parts[stomach_body_part_id as usize].connections.iter_mut() {
                    if connection.to_body_part == chest_body_part_id { connection.in_female_socket = i as u8; }
                }
            } else {
                match male_socket_count {
                    0 => { body_data.add_humanoid_limb_data(chest_body_part_id, i as u8, &model_prefix, "LArm", parts_data); },
                    1 => { body_data.add_humanoid_limb_data(chest_body_part_id, i as u8, &model_prefix, "RArm", parts_data); },
                    2 => { chest_neck_socket = i as u8; },
                    _ => {},
                }

                male_socket_count += 1;
            }
        }

        // let neck_id = parts_data.id_from_name(model_prefix.clone() + "Neck");
        // let head_id = parts_data.id_from_name(model_prefix.clone() + "Head");

        body_data
    }

    pub fn add_humanoid_limb_data(
        &mut self,
        parent_id: u8,
        parent_socket: u8,
        model_prefix: &String,
        limb_prefix: &'static str,
        parts_data: &RuntimeDataAssets<PartData>,
    ) {
        let next_body_part_id = self.parts.len() as u8;
        let body_part_ids = [next_body_part_id, next_body_part_id + 1, next_body_part_id + 2];

        let part_ids = [parts_data.id_from_name(model_prefix.clone() + limb_prefix + "Upper"),
                        parts_data.id_from_name(model_prefix.clone() + limb_prefix + "Lower"),
                        parts_data.id_from_name(model_prefix.clone() + limb_prefix + "End")];

        let part_data = [parts_data.get(part_ids[0] as usize),
                         parts_data.get(part_ids[1] as usize),
                         parts_data.get(part_ids[2] as usize)];

        // 0: Parent to Upper
        // 1: Upper to Lower
        // 2: Lower to End
        let mut connections = vec![];

        for (i, socket) in part_data[0].sockets.iter().enumerate() {
            if socket.connector == SocketConnector::Female {
                connections.push(BodyConnectionData { from_male_socket: parent_socket, to_body_part: parent_id, in_female_socket: i as u8 });
            } else {
                connections.push(BodyConnectionData { from_male_socket: i as u8, to_body_part: body_part_ids[1], in_female_socket: 0 });
            }
        }

        for (i, socket) in part_data[1].sockets.iter().enumerate() {
            if socket.connector == SocketConnector::Female {
                connections[1].in_female_socket = i as u8;
            } else {
                connections.push(BodyConnectionData { from_male_socket: i as u8, to_body_part: body_part_ids[2], in_female_socket: 0 });
            }
        }

        self.parts[parent_id as usize].connections.push(connections[0].clone());
        self.parts.push(BodyPartData { part_id: part_ids[0], connections: vec![connections[1].clone()] });
        self.parts.push(BodyPartData { part_id: part_ids[1], connections: vec![connections[2].clone()] });
        self.parts.push(BodyPartData { part_id: part_ids[2], ..default() });
    }
}

