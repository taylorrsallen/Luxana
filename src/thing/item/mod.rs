use crate::*;

////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct TankThingItemPlugin;
impl Plugin for TankThingItemPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Equippable>();
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Equippable {
    primary: bool,
    secondary: bool,
}