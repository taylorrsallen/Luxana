use crate::*;

////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct TankThingInteractorPlugin;
impl Plugin for TankThingInteractorPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Actor>()
            .register_type::<Grabber>();
        
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
/// A Thing with Interactors, on it and/or as children.
#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Actor {
    pub interactors: Vec<Entity>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Reflect)]
pub struct GrabInteraction {
    pub entity: Entity,
    /// From origin of grabbed entity.
    pub offset: Vec3,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
/// An entity that can grab things, LittleBigPlanet style.
/// 
/// Required for equipping [HeldEquippable]s, but does not have the functionality without [EquipSlot].
#[derive(Component, Default, Reflect)]
#[reflect(Component, Default)]
pub struct Grabber {
    interaction: Option<GrabInteraction>,
}

#[derive(Component, Default, Reflect)]
#[reflect(Component, Default)]
pub struct EquipSlot;