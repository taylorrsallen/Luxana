use crate::*;

mod fixed;
pub use fixed::*;
mod grid;
pub use grid::*;
mod physics;
pub use physics::*;

////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct TankThingMovementPlugin;
impl Plugin for TankThingMovementPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<MoveInput2d>()
            .register_type::<MoveInput3d>()
            .register_type::<RotationInput2d>()
            .register_type::<RotationInput3d>()
            .add_plugins((
                TankThingMovementFixedPlugin,
                TankThingMovementGridPlugin,
                TankThingMovementPhysicsPlugin,
            ));
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct MoveInput2d(pub Vec2);

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct MoveInput3d(pub Vec3);

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct RotationInput2d(pub Vec3);

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct RotationInput3d(pub Vec3);

#[derive(Component, Default, Reflect)]
pub struct CharacterState {
    pub grounded: bool,
}