use crate::*;

mod spring;
pub use spring::*;
mod thrust;
pub use thrust::*;

////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct LuxanaPhysicsMovementPlugin;
impl Plugin for LuxanaPhysicsMovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            LuxanaSpringPhysicsMovementPlugin,
            LuxanaThrustPhysicsMovementPlugin,
        ));
    }
}