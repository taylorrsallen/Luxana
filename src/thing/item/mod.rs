use crate::*;

////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct TankThingItemPlugin;
impl Plugin for TankThingItemPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<HeldEquippable>();
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
/// A Thing equipped to a [Grabber].
#[derive(Component, Default, Reflect)]
#[reflect(Component, Default)]
pub struct HeldEquippable {
    
}