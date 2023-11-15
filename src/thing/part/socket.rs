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