use crate::*;

////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct LuxanaAnimationPlugin;
impl Plugin for LuxanaAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<LinearColorAnimation>()
            .add_systems(Update, sys_update_color_animation);
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Component, Reflect)]
pub struct LinearColorAnimation {
    pub from_color: Color,
    pub to_color: Color,
    pub progress: Timer,
}

impl LinearColorAnimation {
    pub fn color(&self) -> Color {
        let progress_percent = self.progress.elapsed_secs() / self.progress.duration().as_secs_f32();
        Color::rgba(
            Math::lerp(self.from_color.r(), self.to_color.r(), progress_percent),
            Math::lerp(self.from_color.g(), self.to_color.g(), progress_percent),
            Math::lerp(self.from_color.b(), self.to_color.b(), progress_percent),
            Math::lerp(self.from_color.a(), self.to_color.a(), progress_percent),
        )
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
fn sys_update_color_animation(
    mut animation_query: Query<&mut LinearColorAnimation>,
    time: Res<Time>,
) {
    for mut animation in animation_query.iter_mut() {
        animation.progress.tick(time.delta());
    }
}