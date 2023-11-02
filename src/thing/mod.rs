use crate::*;

mod interactor;
pub use interactor::*;
mod movement;
pub use movement::*;
mod projectile;
pub use projectile::*;
mod stat;
pub use stat::*;
mod turret;
pub use turret::*;

////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct TankThingPlugin;
impl Plugin for TankThingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
                TankThingInteractorPlugin,
                TankThingMovementPlugin,
                TankThingProjectilePlugin,
                TankThingStatPlugin,
                TankThingTurretPlugin,
            ));   
    }
}