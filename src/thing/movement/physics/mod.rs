use crate::*;

mod spring;
pub use spring::*;
mod thrust;
pub use thrust::*;

////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct TankThingMovementPhysicsPlugin;
impl Plugin for TankThingMovementPhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            TankSpringPhysicsMovementPlugin,
            TankThrustPhysicsMovementPlugin,
        ));
    }
}