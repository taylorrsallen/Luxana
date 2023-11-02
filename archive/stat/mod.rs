use crate::*;

////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct TankThingStatPlugin;
impl Plugin for TankThingStatPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<CurrentHealth>()
            .register_type::<MaxHealth>()
            .register_type::<PredictedHealth>()
            .register_type::<Killable>()
            .add_systems(Update, (
                sys_update_max_health,
                sys_update_health,
            ).chain());
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct CurrentHealth(pub f32);

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct MaxHealth(pub f32);

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct PredictedHealth(pub f32);

/// Put this on any Thing you want to be destroyed if its [CurrentHealth] drops to 0.
/// 
/// [Dying] will be added when the time comes. It's up to YOU to decide what that means.
/// 
/// `Query<&YourDeathComponent, Added<Dying>>` to have unique deaths per Thing.
#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Killable;

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Dying;

////////////////////////////////////////////////////////////////////////////////////////////////////
fn sys_update_health(
    mut commands: Commands,
    mut health_query: Query<(Entity, &mut CurrentHealth), Changed<CurrentHealth>>,
    killable_query: Query<&Killable>,
    max_health_query: Query<&MaxHealth>,
) {
    for (entity, mut health) in health_query.iter_mut() {
        if let Ok(max_health) = max_health_query.get(entity) {
            if health.0 > max_health.0 { health.0 = max_health.0; }
        }

        if health.0 <= 0.0 && killable_query.contains(entity) {
            commands.entity(entity).insert(Dying);
        }
    }
}

fn sys_update_max_health(
    mut health_query: Query<(&mut CurrentHealth, &MaxHealth), Changed<MaxHealth>>,
) {
    for (mut health, max_health) in health_query.iter_mut() {
        if health.0 > max_health.0 { health.0 = max_health.0; }
    }
}