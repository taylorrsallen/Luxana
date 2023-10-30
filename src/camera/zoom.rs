use crate::*;

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct CameraZoom(pub f32);

impl CameraZoom {
    pub fn new(zoom: f32) -> Self { Self { 0: zoom } }
    pub fn get(&self) -> f32 { self.0 }
    pub fn get_mut(&mut self) -> &mut f32 { &mut self.0 }
    pub fn set(&mut self, zoom: f32) { self.0 = zoom }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
pub fn sys_update_camera_zoom(
    mut camera_query: Query<(Entity, &mut Transform, &CameraZoom)>,
    mut transform_query: Query<&mut GlobalTransform>,
) {
    for (entity, mut transform, zoom) in camera_query.iter_mut() {
        let mut camera_transform = if let Ok(transform) = transform_query.get_mut(entity) { transform } else { continue };
        let zoom_offset = transform.forward() * zoom.get();
        transform.translation -= zoom_offset;
        *camera_transform = GlobalTransform::from(*transform);
    }
}