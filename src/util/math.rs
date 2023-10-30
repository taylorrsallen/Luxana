use crate::*;

////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct Math;
impl Math {
    pub fn deg2rad(deg: f32) -> f32 { deg * 0.01745329 }

    pub fn shortest_rotation(from: Quat, to: Quat) -> Quat {
        if from.dot(to) < 0.0 {
            from * (to * -1.0).inverse()
        } else {
            from * to.inverse()
        }
    }

    pub fn move_towards_f32(current: f32, target: f32, rate: f32) -> f32 {
        if current < target {
            target.min(current + rate)
        } else if current > target {
            target.max(current - rate)
        } else {
            current
        }
    }

    pub fn move_towards_vec3(current: Vec3, target: Vec3, rate: f32) -> Vec3 {
        Vec3::new(
            Self::move_towards_f32(current.x, target.x, rate),
            Self::move_towards_f32(current.y, target.y, rate),
            Self::move_towards_f32(current.z, target.z, rate),
        )
    }

    pub fn lerp(from: f32, to: f32, percent: f32) -> f32 {
        (1.0 - percent) * from + to * percent
    }

    pub fn inverse_lerp(a: f32, b: f32, v: f32) -> f32{
        (v - a) / (b - a)
    }

    pub fn taxicab_distance_2d(a: IVec2, b: IVec2) -> u32 {
        let distance_coord = (a - b).abs();
        (distance_coord.x + distance_coord.y) as u32
    }

    pub fn taxicab_distance_3d(a: IVec3, b: IVec3) -> u32 {
        let distance_coord = (a - b).abs();
        (distance_coord.x + distance_coord.y + distance_coord.z) as u32
    }

    pub fn nearest_coord_2d(from: &[IVec2], to: IVec2) -> IVec2 {
        let mut nearest_coord = to;
        let mut nearest_distance = u32::MAX;
        for coord in from.iter() {
            let distance = Self::taxicab_distance_2d(*coord, to);
            if distance < nearest_distance {
                nearest_distance = distance;
                nearest_coord = *coord;
            }
        }

        nearest_coord
    }
}