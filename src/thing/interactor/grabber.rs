use crate::*;

////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct TankThingInteractorGrabberPlugin;
impl Plugin for TankThingInteractorGrabberPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Grabber>();
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Reflect)]
pub struct GrabInteraction {
    pub entity: Entity,
    /// From origin of grabbed entity.
    pub offset: Vec3,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
/// If you grab an equippable, it gets equipped. That means it doesn't drop when you release.
/// 
/// Because you can't grab and equip at the same time.
#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Grabber {
    interaction: Option<GrabInteraction>,
}