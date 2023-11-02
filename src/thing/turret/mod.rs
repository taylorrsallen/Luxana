use bevy::utils::FloatOrd;
use bevy_rapier3d::na::ComplexField;

use crate::*;

////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct TankThingTurretPlugin;
impl Plugin for TankThingTurretPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<TurretYawPivot>()
            .register_type::<TurretPitchPivot>()
            .register_type::<TurretBase>()
            .register_type::<TurretRange>()
            .register_type::<TurretTarget>()
            .add_systems(Update, (
                sys_update_turret_target,
                sys_update_turret_movement,
            ));
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct TurretYawPivot {
    pub speed: f32,
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct TurretPitchPivot {
    pub speed: f32,
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct TurretBase {
    pub target_group: Group,
    pub projectile: u32,
    pub yaw_pivot: Option<Entity>,
    pub pitch_pivot: Option<Entity>,
    pub emitter: Option<Entity>,
    #[reflect(ignore)]
    pub target: Option<Entity>,
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct TurretRange {
    pub range: f32,
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct TurretTarget {
    
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub enum TurretTargetingMode {
    #[default]
    ClosestToSelf,
    ClosestToTarget(TransformTargetRef),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
fn sys_update_turret_target(
    mut gizmos: Gizmos,
    mut turret_query: Query<(Entity, &mut TurretBase, &TurretRange)>,
    transform_query: Query<&GlobalTransform>,
    rapier_context: Res<RapierContext>,
) {
    for (turret_entity, mut turret_base, turret_range) in turret_query.iter_mut() {
        let turret_transform = if let Ok(transform) = transform_query.get(turret_entity) { transform } else { continue };
        gizmos.sphere(turret_transform.translation(), Quat::IDENTITY, turret_range.range, Color::YELLOW);

        let sensor_shape_pos = turret_transform.translation();
        let sensor_shape = Collider::ball(turret_range.range);
        let mut sensor_filter = QueryFilter::exclude_fixed();
        sensor_filter.groups = Some(CollisionGroups { memberships: Group::GROUP_4, filters: turret_base.target_group });

        let mut possible_targets = vec![];
        rapier_context.intersections_with_shape(sensor_shape_pos, Quat::IDENTITY, &sensor_shape, sensor_filter, |target_entity| {
            let target_translation = if let Ok(transform) = transform_query.get(target_entity) { transform.translation() } else { return true };
            possible_targets.push((target_entity, target_translation));
            gizmos.line(turret_transform.translation(), target_translation, Color::GRAY);
            true
        });

        turret_base.target = possible_targets.iter()
            .min_by_key(|(_, target_translation)| { FloatOrd(Vec3::distance(*target_translation, turret_transform.translation())) })
            .map(|(target_entity, _)| { *target_entity });
    }
}

fn sys_update_turret_movement(
    mut gizmos: Gizmos,
    mut transform_query: Query<&mut Transform>,
    turret_query: Query<(Entity, &TurretBase)>,
    yaw_query: Query<&TurretYawPivot>,
    pitch_query: Query<&TurretPitchPivot>,
    global_transform_query: Query<&GlobalTransform>,
    time: Res<Time>,
) {
    for (turret_entity, turret) in turret_query.iter() {
        let target_entity = if let Some(entity) = turret.target { entity } else { continue };
        let target_transform = if let Ok(transform) = global_transform_query.get(target_entity) { transform } else { continue };

        let turret_transform = if let Ok(transform) = global_transform_query.get(turret_entity) { transform } else { continue };
        gizmos.line(turret_transform.translation(), target_transform.translation(), Color::RED);
        gizmos.sphere(target_transform.translation(), Quat::IDENTITY, 0.5, Color::RED);

        if let Some(yaw_entity) = turret.yaw_pivot {
            if let Ok(yaw_pivot) = yaw_query.get(yaw_entity) {
                if let Ok(yaw_global_transform) = global_transform_query.get(yaw_entity) {
                    if let Ok(mut yaw_transform) = transform_query.get_mut(yaw_entity) {
                        let mut forward = yaw_global_transform.forward();
                        forward.y = 0.0;
                        forward.normalize();

                        let mut right = yaw_global_transform.right();
                        right.y = 0.0;
                        right.normalize();

                        let mut look_at = target_transform.translation() - yaw_global_transform.translation();
                        look_at.y = 0.0;

                        let direction = -right.dot(look_at).signum();

                        let angle_between = forward.angle_between(look_at);

                        let max_rotation = yaw_pivot.speed * time.delta_seconds();
                        let mut rotation = angle_between;
                        if rotation > max_rotation { rotation = max_rotation; }
                        rotation *= direction;
                        
                        yaw_transform.rotate_axis(yaw_global_transform.up(), rotation);
                    }
                }
            }
        }

        if let Some(pitch_entity) = turret.pitch_pivot {
            if let Ok(pitch_pivot) = pitch_query.get(pitch_entity) {
                if let Ok(pitch_global_transform) = global_transform_query.get(pitch_entity) {
                    if let Ok(mut pitch_transform) = transform_query.get_mut(pitch_entity) {
                        let aim_vector = target_transform.translation() - pitch_global_transform.translation();
                        let aim_distance = Vec3::new(pitch_global_transform.translation().x, 0.0, pitch_global_transform.translation().z).distance(Vec3::new(target_transform.translation().x, 0.0, target_transform.translation().z));
                        
                        let check_vector = pitch_global_transform.forward() * aim_distance + pitch_global_transform.up() * aim_vector.y;
                        let angle_between = pitch_global_transform.forward().angle_between(check_vector);
                        
                        let direction = pitch_global_transform.up().dot(aim_vector).signum();

                        let max_rotation = pitch_pivot.speed * time.delta_seconds();
                        let rotation = if angle_between > max_rotation { max_rotation } else { angle_between } * direction;
                        
                        let right = pitch_transform.right();
                        pitch_transform.rotate_axis(right, rotation);
                    }
                }
            }
        }
    }


        // let pitch_entity = if let Some(entity) = turret_base.pitch_pivot { entity } else { continue };
        // let pitch_transform = if let Ok(transform) = transform_query.get(pitch_entity) { transform } else { continue };
}