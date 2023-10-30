use crate::*;

mod movement;
pub use movement::*;
mod turret;
pub use turret::*;

////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct TankCharacterPlugin;
impl Plugin for TankCharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
                TankCharacterMovementPlugin,
                TankCharacterTurretPlugin,
            ));   
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct CharacterGroundUp(pub Vec3);