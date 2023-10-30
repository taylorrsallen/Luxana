use crate::*;

mod spring;
pub use spring::*;
mod thrust;
pub use thrust::*;

////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct TankPhysicsMovementPlugin;
impl Plugin for TankPhysicsMovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            TankSpringPhysicsMovementPlugin,
            TankThrustPhysicsMovementPlugin,
        ));
    }
}