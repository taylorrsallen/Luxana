use crate::*;

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Component, Default, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
#[reflect(Component, Default)]
pub struct SocketConnection(pub Option<Entity>);

////////////////////////////////////////////////////////////////////////////////////////////////////
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

impl SocketConnector {
    pub fn from_socket_name(name: &str) -> Self {
        match name {
            "R" => Self::Revolute,
            "S" => Self::SphereMale,
            "F" => Self::FixedMale,
            _ => Self::default(),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Default, Clone, Copy, Debug, Reflect)]
pub struct PartSocket {
    pub offset: Vec3,
    pub rotation: Quat,
    pub connector: SocketConnector,
}

impl PartSocket {
    pub fn from_primary_socket_node(name: &str, transform: Transform, part_offset: Vec3) -> Self {
        Self {
            offset: transform.translation - part_offset,
            rotation: transform.rotation,
            connector: SocketConnector::from_socket_name(name),
        }
    }

    pub fn from_secondary_socket_node(transform: Transform, part_offset: Vec3) -> Self {
        Self {
            offset: transform.translation - part_offset,
            rotation: transform.rotation,
            connector: SocketConnector::Female,
        }
    }

    pub fn transform(&self) -> Transform {
        Transform::from_translation(self.offset).with_rotation(self.rotation)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
pub fn sys_update_socket_connections(
    mut commands: Commands,
    mut transform_query: Query<&mut Transform>,
    changed_socket_query: Query<Entity, Changed<SocketConnection>>,
    socket_query: Query<(&SocketConnection, &Parent)>,
    part_query: Query<&Children, With<PartMarker>>,
) {
    // fn update_socket_connection_recursive(
    //     base_entity: Entity,
    //     commands: &mut Commands,
    //     transform_query: &mut Query<&mut Transform>,
    //     socket_query: &Query<(&SocketConnection, &Parent)>,
    //     part_query: &Query<&Children, With<PartMarker>>,
    // ) {

    // }

    changed_socket_query.for_each(|base_entity| {
        let Ok((base_connection, base_parent)) = socket_query.get(base_entity) else { return };
        let Some(connected_entity) = base_connection.0.clone() else { return };
        let Ok((_, connected_parent)) = socket_query.get(connected_entity) else { return };

        if base_parent.get() == connected_parent.get() { return; }
        if !part_query.contains(base_parent.get()) || !part_query.contains(connected_parent.get()) { return; }

        let Ok(mut connected_parent_transform) = transform_query.get_mut(connected_parent.get()) else { return };
        connected_parent_transform.translation = Vec3::ZERO;
        commands.entity(connected_parent.get()).set_parent(base_entity);
    });
}

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Bundle, Default)]
pub struct SocketBundle {
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub connection: SocketConnection,
    pub connector: SocketConnector,
}

impl SocketBundle {
    pub fn new(socket: &PartSocket) -> Self {
        Self {
            transform: socket.transform(),
            connector: socket.connector,
            ..default()
        }
    }
}