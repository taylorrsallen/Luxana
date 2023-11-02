use crate::*;

////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct TankThrustPhysicsMovementPlugin;
impl Plugin for TankThrustPhysicsMovementPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<ThrustMover3d>()
            .add_systems(Update, sys_update_thrust_mover_3d);
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Bundle, Default)]
pub struct ThrustMover3dBundle {
    move_input_3d: MoveInput3d,
    rotation_input_3d: RotationInput3d,
    thrust_mover_3d: ThrustMover3d,
    external_impulse: ExternalImpulse,
    velocity: Velocity,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
/// A simplified movement model for a spaceship with multiple thrusters at various positions and angles.
/// 
/// TODO: Thrust & Torque strength will be calculated from sum of thruster positions and torque, and need to be
/// updated if a thruster changes.
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct ThrustMover3d {
    pub thrust_strength: Vec3,
    pub torque_strength: Vec3,
}

impl Default for ThrustMover3d {
    fn default() -> Self {
        Self {
            thrust_strength: Vec3::ONE * 0.5,
            torque_strength: Vec3::ONE * 0.01,
        }
    }
}

fn sys_update_thrust_mover_3d(
    mut mover_query: Query<(&mut ExternalImpulse, &ThrustMover3d, &MoveInput3d, &RotationInput3d)>,
    time: Res<Time>,
) {
    for (mut impulse, mover, move_input, rotation_input) in mover_query.iter_mut() {
        impulse.impulse = mover.thrust_strength * move_input.0;
        impulse.torque_impulse = mover.torque_strength * rotation_input.0;
    }
}