use crate::*;

mod spring;
pub use spring::*;
mod thrust;
pub use thrust::*;

////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct TankActorMovementPhysicsPlugin;
impl Plugin for TankActorMovementPhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            TankSpringPhysicsMovementPlugin,
            TankThrustPhysicsMovementPlugin,
        ));
    }
}