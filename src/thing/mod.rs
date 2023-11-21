use crate::*;

mod actor;
pub use actor::*;
mod emitter;
pub use emitter::*;
mod item;
pub use item::*;
mod movement;
pub use movement::*;
mod part;
pub use part::*;
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
                TankThingActorPlugin,
                TankThingEmitterPlugin,
                TankThingItemPlugin,
                TankThingMovementPlugin,
                TankThingPartPlugin,
                TankThingProjectilePlugin,
                TankThingStatPlugin,
                TankThingTurretPlugin,
            ));   
    }
}