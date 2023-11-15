use crate::*;

////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct TankThingMovementFixedPlugin;
impl Plugin for TankThingMovementFixedPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<FixedMover>()
            .add_systems(Update, sys_update_fixed_mover);
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Bundle, Default)]
pub struct FixedMoverBundle {
    pub fixed_mover: FixedMover,
    pub move_target: MoveInput3d,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub inherited_visibility: InheritedVisibility,
    pub view_visibility: ViewVisibility,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Component, Reflect)]
pub struct FixedMover {
    pub speed: f32,
}

impl Default for FixedMover {
    fn default() -> Self { Self { speed: 10.0 } }
}

fn sys_update_fixed_mover(
    mut mover_query: Query<(&mut Transform, &FixedMover, &MoveInput3d)>,
    time: Res<Time>,
) {
    for (mut transform, mover, target) in mover_query.iter_mut() {
        transform.translation += target.0 * mover.speed * time.delta_seconds();
    }
}