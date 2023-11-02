use crate::*;

mod movement;
pub use movement::*;
mod turret;
pub use turret::*;

////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct TankActorPlugin;
impl Plugin for TankActorPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
                TankActorMovementPlugin,
                TankActorTurretPlugin,
            ));   
    }
}