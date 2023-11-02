use crate::*;

////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct TankThingEmitterPlugin;
impl Plugin for TankThingEmitterPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Emitter>();
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Emitter {
    active: bool,
    /// From local rotation
    direction: Vec3,
    strength: f32,
    /// How much force is applied to the emitter
    force: f32,
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct EmitterSound {
    /// From local rotation
    sound_id: u32,
    volume: f32,
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct EmitterCooldown(Timer);

impl Default for EmitterCooldown {
    fn default() -> Self { Self { 0: Timer::from_seconds(1.0, TimerMode::Repeating) } }
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub enum EmitterTrigger {
    #[default]
    Once,
    Semi(u32),
    Auto,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
fn sys_update_emitters(
    emitter_query: Query<(&Emitter, &EmitterCooldown, &EmitterTrigger)>,
    emitter_sound_query: Query<&EmitterSound>,
    time: Res<Time>,
) {

}