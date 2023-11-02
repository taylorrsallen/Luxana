use crate::*;

mod movement;
pub use movement::*;
mod turret;
pub use turret::*;

////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct TankThingPlugin;
impl Plugin for TankThingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
                TankActorMovementPlugin,
                TankActorTurretPlugin,
            ));   
    }
}