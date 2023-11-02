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
/// Dying will be marked true when the time comes. It's up to YOU to decide what that means.
/// 
/// `Query<(&Killable, &YourDeathComponent), Changed<Killable>>` to have unique deaths per Thing.
/// 
/// Should Dying just be a marker component instead of a bool? I don't know. Maybe.
/// It didn't feel right to me so I archived that version for now.
#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Killable {
    dying: bool,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
fn sys_update_health(
    mut killable_query: Query<&mut Killable>,
    mut health_query: Query<(Entity, &mut CurrentHealth), Changed<CurrentHealth>>,
    max_health_query: Query<&MaxHealth>,
) {
    for (entity, mut health) in health_query.iter_mut() {
        if let Ok(max_health) = max_health_query.get(entity) {
            if health.0 > max_health.0 { health.0 = max_health.0; }
        }

        if let Ok(mut killable) = killable_query.get_mut(entity) {
            if health.0 <= 0.0 { killable.dying = true; }
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