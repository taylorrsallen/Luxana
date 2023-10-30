use crate::*;

mod generics;
pub use generics::*;

mod bitmask;
pub use bitmask::*;
mod database;
pub use database::*;
mod image;
pub use self::image::*;
mod math;
pub use math::*;
mod mesh;
pub use mesh::*;
mod noise;
pub use self::noise::*;
mod serial;
pub use serial::*;
mod thread;
pub use thread::*;

mod bundle;
pub use bundle::*;

pub struct LuxanaUtilPlugin;
impl Plugin for LuxanaUtilPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            LuxanaUtilGenericsPlugin,
        ));
    }
}