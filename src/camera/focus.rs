use crate::*;

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Component, Default, Reflect)]
pub struct CameraFocus {
    pub target: TransformTargetRef,
    pub offset: Vec3,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
pub fn sys_update_camera_focus(
    mut transform_query: Query<&mut GlobalTransform>,
    mut camera_query: Query<(Entity, &mut Transform, &CameraFocus, &CameraRig)>,
) {
    for (entity, mut transform, focus, rig) in camera_query.iter_mut() {
        let target_pos = if let Some(pos) = focus.target.try_get_pos_mut_query(&transform_query) { pos + focus.offset } else { continue };
        let mut camera_transform = if let Ok(transform) = transform_query.get_mut(entity) { transform } else { continue };
        transform.look_at(target_pos, rig.up);
        *camera_transform = GlobalTransform::from(*transform);
    }
}