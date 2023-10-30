use crate::*;

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct CameraOrbit {
    pub offset: Vec2,
    pub rotation: Vec2,
}

impl CameraOrbit {
    pub fn new(offset: Vec2) -> Self { Self { offset, rotation: Vec2::ZERO } }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
pub fn sys_update_camera_orbit(
    mut orbit_query: Query<(Entity, &mut Transform, &CameraOrbit)>,
    mut transform_query: Query<&mut GlobalTransform>,
) {
    for (entity, mut transform, orbit) in orbit_query.iter_mut() {
        let mut camera_transform = if let Ok(transform) = transform_query.get_mut(entity) { transform } else { continue };

        transform.rotation = Quat::IDENTITY;
        transform.rotate_y(-orbit.rotation.x);
        let yaw_right = transform.right().normalize() * orbit.offset.x;
        let yaw_up = transform.up().normalize() * orbit.offset.y;
        transform.rotate_local_x(-orbit.rotation.y);
        let anchored_pos = transform.translation;
        transform.translation = anchored_pos + yaw_right + yaw_up;
        *camera_transform = GlobalTransform::from(*transform);
    }
}