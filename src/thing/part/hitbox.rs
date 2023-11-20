use crate::*;

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Default, Clone, Debug, Reflect)]
pub enum HitboxShape {
    #[default]
    Cube,
    Sphere,
    Capsule,
}

impl HitboxShape {
    pub fn from_shape_name(name: &str) -> Self {
        match name {
            "Cube" => Self::Cube,
            "Sphere" => Self::Sphere,
            "Capsule" => Self::Capsule,
            _ => Self::default(),
        }
    }

    pub fn collider(&self) -> Collider {
        match self {
            Self::Cube => { Collider::cuboid(1.0, 1.0, 1.0) },
            Self::Sphere => { Collider::ball(0.5) }
            Self::Capsule => { Collider::capsule_y(1.0, 0.5) }
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Default, Clone, Debug, Reflect)]
pub struct PartHitbox {
    pub transform: Transform,
    pub shape: HitboxShape,
}

impl PartHitbox {
    pub fn collider(&self) -> Collider {
        self.shape.collider()
    }

    pub fn part_name_and_hitbox_shape_from_hitbox_name(name: &String) -> Option<(&str, HitboxShape)> {
        if let Some(hitbox_str) = name.strip_prefix("Hitbox.") {
            let split: Vec<&str> = hitbox_str.split(".").collect();
            let part_name = if let Some(part_name) = split.get(0) { *part_name } else { return None };
            let hitbox_shape = if let Some(shape_name) = split.get(1) { HitboxShape::from_shape_name(shape_name) } else { HitboxShape::default() };
            Some((part_name, hitbox_shape))
        } else { None }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Bundle, Default)]
pub struct PartHitboxBundle {
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub collision_groups: CollisionGroups,
    pub collider: Collider,
    pub sensor: Sensor,
}

impl PartHitboxBundle {
    pub fn new(hitbox: &PartHitbox, part_rotation: &Quat) -> Self {
        let mut transform = hitbox.transform;
        let rotated_translation = part_rotation.mul_vec3(transform.translation);
        transform.translation = rotated_translation;
        transform.rotate(*part_rotation);

        let mut collision_groups = CollisionGroups::new(COLLISION_GROUP_PART, Group::all());
        collision_groups.filters.remove(COLLISION_GROUP_PART);
        
        Self { transform, collision_groups, collider: hitbox.collider(), ..default() }
    }
}