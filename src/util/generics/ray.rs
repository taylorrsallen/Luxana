use crate::*;

////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct TankRayPlugin;
impl Plugin for TankRayPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
                sys_update_global_direction_ray,
                sys_update_local_direction_ray,
            ));
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
/// Casts a ray in a global direction from entity center + offset.
#[derive(Component, Default, Reflect)]
pub struct GlobalDirectionRay {
    pub offset: Vec3,
    pub direction: Vec3,
    pub distance: f32,
    #[reflect(ignore)]
    pub hit: Option<(Entity, RayIntersection)>,
}

fn sys_update_global_direction_ray(
    mut ray_query: Query<(Entity, &mut GlobalDirectionRay, &Transform)>,
    rapier_context: Res<RapierContext>,
) {
    for (entity, mut ray, transform) in ray_query.iter_mut() {
        let filter = QueryFilter::new().exclude_collider(entity).exclude_sensors();
        let ray_origin = transform.translation + ray.offset;
        ray.hit = rapier_context.cast_ray_and_get_normal(ray_origin, ray.direction, ray.distance, true, filter);
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
/// Casts a ray in a localized direction from entity center + offset.
/// 
/// Ex: a direction of (0.0, 0.0, 1.0) would emit a ray along the forward vector of the entity.
#[derive(Component, Default, Reflect)]
pub struct LocalDirectionRay {
    pub offset: Vec3,
    pub direction: Vec3,
    pub distance: f32,
    #[reflect(ignore)]
    pub hit: Option<(Entity, RayIntersection)>,
}

fn sys_update_local_direction_ray(
    mut ray_query: Query<(Entity, &mut LocalDirectionRay, &Transform)>,
    rapier_context: Res<RapierContext>,
) {
    for (entity, mut ray, transform) in ray_query.iter_mut() {
        let filter = QueryFilter::new().exclude_collider(entity).exclude_sensors();
        let ray_origin = transform.translation + ray.offset;
        let ray_direction = transform.right() * ray.direction.x + transform.up() * ray.direction.y + transform.forward() * ray.direction.z;
        ray.hit = rapier_context.cast_ray_and_get_normal(ray_origin, ray_direction, ray.distance, true, filter);
    }
}