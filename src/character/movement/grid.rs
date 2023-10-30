use crate::*;

////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct TankGridMovementPlugin;
impl Plugin for TankGridMovementPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<GridMover>()
            .add_systems(Update, sys_update_grid_mover);
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Bundle, Default)]
pub struct GridMoverBundle {
    move_target: MoveInput2d,
    grid_mover: GridMover,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Component, Reflect)]
pub struct GridMover {
    speed: f32,
}

impl Default for GridMover {
    fn default() -> Self {
        Self { speed: 5.0 }
    }
}

fn sys_update_grid_mover(
    mut mover_query: Query<(&mut Transform, &GridMover, &MoveInput2d)>,
    time: Res<Time>,
) {
    // for (mut transform, mover, target) in mover_query.iter_mut() {
    //     transform.translation += target.0 * mover.speed * time.delta_seconds();
    // }
}