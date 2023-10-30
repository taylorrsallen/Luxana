use crate::*;

////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct TankLifetimePlugin;
impl Plugin for TankLifetimePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Lifetime>()
            .add_systems(Update, sys_update_lifetime);
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Component, Reflect)]
pub struct Lifetime(pub Timer);

impl Default for Lifetime {
    fn default() -> Self {
        Self { 0: Timer::from_seconds(5.0, TimerMode::Once) }
    }
}

impl Lifetime {
    pub fn new(duration: f32) -> Self {
        Self { 0: Timer::from_seconds(duration, TimerMode::Once) }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
fn sys_update_lifetime(
    mut commands: Commands,
    mut lifetime_query: Query<(Entity, &mut Lifetime)>,
    time: Res<Time>,
) {
    for (entity, mut lifetime) in lifetime_query.iter_mut() {
        lifetime.0.tick(time.delta());
        if lifetime.0.just_finished() { commands.entity(entity).despawn_recursive(); }
    }
}