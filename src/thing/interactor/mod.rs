use crate::*;

mod grabber;
pub use grabber::*;

////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct TankThingInteractorPlugin;
impl Plugin for TankThingInteractorPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
                TankThingInteractorGrabberPlugin,
            ));   
    }
}