use crate::*;

mod animation;
pub use animation::*;
mod ray;
pub use ray::*;
mod lifetime;
pub use lifetime::*;
mod validate;
pub use validate::*;

////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct LuxanaUtilGenericsPlugin;
impl Plugin for LuxanaUtilGenericsPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<LevelOfDetail>()
            .register_type::<RngSeed>()
            .register_type::<Id>()
            .add_plugins((
                LuxanaAnimationPlugin,
                LuxanaRayPlugin,
                LuxanaLifetimePlugin,
            ));
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Component, Default, Clone, Copy, Reflect)]
pub struct LevelOfDetail(u8);

impl LevelOfDetail {
    pub fn get(&self) -> u8 { self.0 }
    pub fn set(&mut self, value: u8) { self.0 = value }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Component, Default, Clone, Copy, Reflect)]
pub struct RngSeed(u32);

impl RngSeed {
    pub fn new(seed: u32) -> Self { Self { 0: seed } }
    pub fn get(&self) -> u32 { self.0 }
    pub fn set(&mut self, seed: u32) { self.0 = seed }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Component, Default, Clone, Copy, Reflect)]
pub struct Id(u32);

impl Id {
    pub fn new(id: u32) -> Self { Self { 0: id } }
    pub fn get(&self) -> u32 { self.0 }
    pub fn set(&mut self, id: u32) { self.0 = id }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Reflect)]
pub enum TransformTargetRef {
    Position(Vec3),
    Entity(Entity),
}

impl Default for TransformTargetRef {
    fn default() -> Self { Self::Position(Vec3::ZERO) }
}

impl TransformTargetRef {
    pub fn try_get_pos(&self, transform_query: &Query<&GlobalTransform>) -> Option<Vec3> {
        match self {
            Self::Position(position) => { Some(*position) }
            Self::Entity(entity) => {
                if let Ok(target) = transform_query.get(*entity) { Some(target.translation()) } else { None }
            }
        }
    }

    pub fn try_get_pos_mut_query(&self, transform_query: &Query<&mut GlobalTransform>) -> Option<Vec3> {
        match self {
            Self::Position(position) => { Some(*position) }
            Self::Entity(entity) => {
                if let Ok(target) = transform_query.get(*entity) { Some(target.translation()) } else { None }
            }
        }
    }

    pub fn try_get_entity(&self) -> Option<Entity> {
        match self {
            Self::Entity(entity) => { Some(*entity) }
            _ => { None }
        }
    }
}