use crate::*;

////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct TankSpringPhysicsMovementPlugin;
impl Plugin for TankSpringPhysicsMovementPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<SpringPhysicsMover>()
            .add_systems(Update, (
                sys_update_upright_rotation,
                sys_update_upright_force,
                sys_update_physics_movement,
                sys_update_ride_force,
            ).chain());
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Bundle)]
pub struct SpringPhysicsMoverBundle {
    pub spring_physics_mover: SpringPhysicsMover,
    pub move_target: MoveInput3d,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub inherited_visibility: InheritedVisibility,
    pub view_visibility: ViewVisibility,
    pub global_direction_ray: GlobalDirectionRay,
    pub rigid_body: RigidBody,
    pub collider: Collider,
    pub external_force: ExternalForce,
    pub velocity: Velocity,
}

impl Default for SpringPhysicsMoverBundle {
    fn default() -> Self {
        Self {
            transform: Transform::default(),
            global_transform: GlobalTransform::default(),
            visibility: Visibility::default(),
            inherited_visibility: InheritedVisibility::default(),
            view_visibility: ViewVisibility::default(),
            move_target: MoveInput3d::default(),
            global_direction_ray: GlobalDirectionRay { direction: Vec3::NEG_Y, distance: 10.0, ..default() },
            spring_physics_mover: SpringPhysicsMover::default(),
            rigid_body: RigidBody::Dynamic,
            collider: Collider::cuboid(0.5, 0.5, 0.5),
            external_force: ExternalForce::default(),
            velocity: Velocity::default(),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
/// Uses a downward raycast to create a spring force which will float attached Thing at ride_height above the ground.
/// 
/// Uses spring rotation to rotate the Thing towards the direction of movement, supplied by [MoveInput3d].
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct SpringPhysicsMover {
    pub speed: f32,
    pub max_acceleration: f32,
    pub ride_height: f32,
    pub ride_spring_strength: f32,
    pub ride_spring_damper: f32,
    pub upright_rotation: Quat,
    pub upright_spring_strength: f32,
    pub upright_spring_damper: f32,
}

impl Default for SpringPhysicsMover {
    fn default() -> Self {
        Self {
            speed: 5.0,
            max_acceleration: 20.0,
            ride_height: 2.0,
            ride_spring_strength: 25.0,
            ride_spring_damper: 3.0,
            upright_rotation: Quat::IDENTITY,
            upright_spring_strength: 25.0,
            upright_spring_damper: 3.0,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
fn sys_update_physics_movement(
    mut mover_query: Query<(&mut Velocity, &SpringPhysicsMover, &MoveInput3d)>,
    time: Res<Time>,
) {
    for (mut velocity, mover, target) in mover_query.iter_mut() {
        let max_speed_change = mover.max_acceleration * time.delta_seconds();
        velocity.linvel.x = Math::move_towards_f32(velocity.linvel.x, target.0.x * mover.speed, max_speed_change);
        velocity.linvel.z = Math::move_towards_f32(velocity.linvel.z, target.0.z * mover.speed, max_speed_change);
    }
}

fn sys_update_ride_force(
    mut force_query: Query<&mut ExternalForce>,
    mover_query: Query<(Entity, &SpringPhysicsMover, &GlobalDirectionRay)>,
    transform_query: Query<&Transform>,
    velocity_query: Query<&Velocity>,
) {
    for (mover_entity, mover, down_ray) in mover_query.iter() {
        let mut mover_force = force_query.get_mut(mover_entity).unwrap();
        let mover_velocity = velocity_query.get(mover_entity).unwrap().linvel;

        if let Some((other_entity, hit)) = down_ray.hit {
            let other_velocity = if let Ok(velocity) = velocity_query.get(other_entity) { velocity.linvel } else { Vec3::ZERO };

            let ray_direction_velocity = Vec3::dot(Vec3::NEG_Y, mover_velocity);
            let other_direction_velocity = Vec3::dot(Vec3::NEG_Y, other_velocity);
            let relative_velocity = ray_direction_velocity - other_direction_velocity;

            let x = hit.toi - mover.ride_height;
            let spring_force = (x * mover.ride_spring_strength) - (relative_velocity * mover.ride_spring_damper);

            mover_force.force = Vec3::NEG_Y * spring_force;
            if let Ok(mut other_force) = force_query.get_mut(other_entity) {
                let other_center = transform_query.get(other_entity).unwrap().translation;
                let contact_force = ExternalForce::at_point(Vec3::NEG_Y * - spring_force, hit.point, other_center);
                other_force.force += contact_force.force;
                other_force.torque += contact_force.torque;
            }
        } else {
            mover_force.force = Vec3::ZERO;
        }
    }
}

fn sys_update_upright_force(mut mover_query: Query<(&mut ExternalForce, &Transform, &Velocity, &SpringPhysicsMover)>) {
    for (mut force, transform, velocity, mover) in mover_query.iter_mut() {
        let current_rotation = transform.rotation;
        let to_goal: Quat = Math::shortest_rotation(mover.upright_rotation, current_rotation);

        let (mut rotation_axis, radians) = to_goal.to_axis_angle();
        rotation_axis = rotation_axis.normalize();
        
        force.torque = (rotation_axis * (radians * mover.upright_spring_strength)) - (velocity.angvel * mover.upright_spring_damper);
    }
}

fn sys_update_upright_rotation(
    mut mover_query: Query<(&mut SpringPhysicsMover, &Transform, &MoveInput3d)>,
) {
    for (mut mover, transform, target) in mover_query.iter_mut() {
        let mut look_transform = Transform::from_translation(Vec3::ZERO);

        if target.0 == Vec3::ZERO {
            let mut forward = transform.forward();
            forward.y = 0.0;
            forward = forward.normalize();

            look_transform.look_at(forward, Vec3::Y);
        } else if target.0.x == 0.0 && target.0.z == 0.0 {
            let mut forward = transform.forward();
            forward.y = 0.0;
            forward = forward.normalize();

            if target.0.y > 0.0 {
                look_transform.look_at(Vec3::Y * 0.2 + forward, (Vec3::Y - forward).normalize());
            } else {
                look_transform.look_at(Vec3::NEG_Y * 0.2 + forward, (Vec3::Y - forward).normalize());
            }
        } else {
            let target_normalized = target.0.normalize();
            let look_at = Vec3::new(target_normalized.x, target_normalized.y - 0.1 * target.0.length(), target_normalized.z);
            look_transform.look_at(look_at, Vec3::Y);
        }

        mover.upright_rotation = look_transform.rotation;
    }
}