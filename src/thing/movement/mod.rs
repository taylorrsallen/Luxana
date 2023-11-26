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
#[reflect(Component, Default)]
pub struct MoveInput2d(pub Vec2);

#[derive(Component, Default, Reflect)]
#[reflect(Component, Default)]
pub struct MoveInput3d(pub Vec3);

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Component, Default, Reflect)]
#[reflect(Component, Default)]
pub struct RotationInput2d(pub Vec3);

#[derive(Component, Default, Reflect)]
#[reflect(Component, Default)]
pub struct RotationInput3d(pub Vec3);

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Default, Clone, Copy, PartialEq, Eq, Debug, Reflect)]
pub enum MoverStateFlags {
    #[default]
    /// Should movers apply forces that lock the Thing to the ground?
    Grounded = 1,
    Swimming = 2,
    Sliding  = 4,
    Tumbling = 8,
}

#[derive(Component, Default, Debug, Reflect)]
#[reflect(Component, Default)]
pub struct MoverState(u32);

impl MoverState {
    pub fn flags(&self) -> u32 { self.0 }
    
    pub fn is_grounded(&self) -> bool { self.0 & MoverStateFlags::Grounded as u32 == MoverStateFlags::Grounded as u32 }
    pub fn set_grounded(&mut self, active: bool) { self.set_flags(MoverStateFlags::Grounded as u32, active); }
    pub fn set_grounded_on(&mut self) { self.set_flags_on(MoverStateFlags::Grounded as u32); }
    pub fn set_grounded_off(&mut self) { self.set_flags_off(MoverStateFlags::Grounded as u32); }

    pub fn set_flags(&mut self, flags: u32, active: bool) { if active { self.set_flags_on(flags) } else { self.set_flags_off(flags) } }
    pub fn set_flags_on(&mut self, flags: u32) { self.0 |= flags; }
    pub fn set_flags_off(&mut self, flags: u32) { self.0 &= !flags; }
}