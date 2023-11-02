use crate::*;

mod grabber;
pub use grabber::*;

////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct TankThingInteractorPlugin;
impl Plugin for TankThingInteractorPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Actor>()
            .add_plugins((
                TankThingInteractorGrabberPlugin,
            ));   
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
/// A Thing with Interactors, on it and/or as children.
#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Actor {
    pub interactors: Vec<Entity>,
}