use crate::*;

use bevy::utils::FloatOrd;

////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct TankAIPlugin;
impl Plugin for TankAIPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<BasicAI>()
            .add_systems(Update, (
                sys_update_basic_ai_goals,
                sys_update_basic_ai_actions,
            ).chain());
    }
}

// Assign behaviors with priorities
// AI tries to act out behaviors

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Reflect)]
pub enum AIMacroGoal {
    Wait,
    DestroyPlayers,
}

impl Default for AIMacroGoal {
    fn default() -> Self { Self::Wait }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Reflect)]
pub enum AIMicroGoal {
    Wait,
    /// Find best way to destroy target
    /// * What is my target?
    /// * What options do I have? (Projectile, melee, environmental hazards)
    /// * MoveTo necessary position
    /// * Carry out optimal action
    Destroy(Entity),
}

impl Default for AIMicroGoal {
    fn default() -> Self { Self::Wait }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Reflect)]
pub enum AIAction {
    Wait,
    MoveTo(TransformTargetRef),
}

impl Default for AIAction {
    fn default() -> Self { Self::Wait }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct BasicAI {
    macro_goal: AIMacroGoal,
    micro_goal: AIMicroGoal,
    action: AIAction,
}

impl BasicAI {
    pub fn crazy_dave() -> Self {
        Self { macro_goal: AIMacroGoal::DestroyPlayers, ..default() }
    }

    fn update_action(&mut self) {

    }
}

fn sys_update_basic_ai_goals(
    mut ai_query: Query<(Entity, &mut BasicAI)>,
    transform_query: Query<&GlobalTransform>,
    player_query: Query<&PlayerController>,
) {
    let player_controlled_entities: Vec<(Entity, &GlobalTransform)> = player_query.iter()
        .filter(|player_controller| {
            if let Some(entity) = player_controller.controlled_entity { transform_query.contains(entity) } else { false }
        })
        .map(|player_controller| {
            let controlled_entity = player_controller.controlled_entity.unwrap();
            unsafe { (controlled_entity, transform_query.get_unchecked(controlled_entity).unwrap()) }
        })
        .collect();

    for (ai_entity, mut ai) in ai_query.iter_mut() {
        if player_controlled_entities.is_empty() { continue; }
        let ai_transform = if let Ok(transform) = transform_query.get(ai_entity) { transform } else { continue };
        let closest_player_controlled = player_controlled_entities.iter()
            .min_by_key(|(_, transform)| { FloatOrd(transform.translation().distance(ai_transform.translation())) })
            .unwrap();

        ai.micro_goal = AIMicroGoal::Destroy(closest_player_controlled.0);
        if ai_transform.translation().distance(closest_player_controlled.1.translation()) > 2.0 {
            ai.action = AIAction::MoveTo(TransformTargetRef::Entity(closest_player_controlled.0));
        } else {
            ai.action = AIAction::Wait;
        }
    }
}

fn sys_update_basic_ai_actions(
    mut ai_query: Query<(Entity, &BasicAI, &mut MoveInput3d)>,
    transform_query: Query<&GlobalTransform>,
) {
    for (ai_entity, ai, mut move_input) in ai_query.iter_mut() {
        let ai_transform = if let Ok(transform) = transform_query.get(ai_entity) { transform } else { continue };
        match &ai.action {
            AIAction::Wait => {
                move_input.0 = Vec3::ZERO;
            }
            AIAction::MoveTo(transform_target_ref) => {
                let target_pos = if let Some(pos) = transform_target_ref.try_get_pos(&transform_query) { pos } else { continue };
                let mut move_direction = (target_pos - ai_transform.translation());
                let y_value = if move_direction.y >= 1.0 { 1.0 } else { 0.0 };
                move_direction.y = 0.0;
                move_direction = move_direction.normalize();

                move_input.0 = Vec3::new(move_direction.x, y_value, move_direction.z);
            }
        }
        
    }
}