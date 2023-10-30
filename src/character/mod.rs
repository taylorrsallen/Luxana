use crate::*;

mod movement;
pub use movement::*;
mod turret;
pub use turret::*;

////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct LuxanaCharacterPlugin;
impl Plugin for LuxanaCharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
                LuxanaCharacterMovementPlugin,
                LuxanaCharacterTurretPlugin,
            ));   
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct CharacterGroundUp(pub Vec3);