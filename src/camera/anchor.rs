use crate::*;

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Component, Default, Reflect)]
pub struct CameraAnchor {
    pub target: TransformTargetRef,
    pub offset: Vec3,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
pub fn sys_update_camera_anchor(
    mut transform_query: Query<&mut GlobalTransform>,
    mut camera_query: Query<(Entity, &mut Transform, &CameraAnchor, &CameraRig)>,
) {
    for (entity, mut transform, anchor, rig) in camera_query.iter_mut() {
        let target_pos = if let Some(pos) = anchor.target.try_get_pos_mut_query(&transform_query) {
            pos + rig.right * anchor.offset.x + rig.up * anchor.offset.y + rig.forward * anchor.offset.z
        } else { continue };
        let mut camera_transform = if let Ok(transform) = transform_query.get_mut(entity) { transform } else { continue };
        transform.translation = target_pos;
        *camera_transform = GlobalTransform::from_translation(target_pos);
    }
}