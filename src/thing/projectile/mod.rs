use crate::*;

////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct TankThingProjectilePlugin;
impl Plugin for TankThingProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<PhysicsProjectile>()
            .add_systems(Update, (
                sys_update_physics_projectiles,
            ));
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct PhysicsProjectile {
    damage: f32,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
fn sys_update_physics_projectiles(
    mut commands: Commands,
    mut health_query: Query<&mut CurrentHealth>,
    projectile_query: Query<(Entity, &PhysicsProjectile, &CollidingEntities), Changed<CollidingEntities>>,
) {
    for (projectile_entity, projectile, colliding_entities) in projectile_query.iter() {
        for colliding_entity in colliding_entities.iter() {
            let mut health = if let Ok(health) = health_query.get_mut(colliding_entity) { health } else { continue };
            health.0 -= projectile.damage;
            commands.entity(projectile_entity).despawn_recursive();
            break;
        }
    }
}