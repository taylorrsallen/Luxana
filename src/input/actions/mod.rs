use crate::*;

use bevy::utils::hashbrown::HashMap;

mod binding;
pub use binding::*;

#[derive(Component)]
pub struct InputActions(HashMap<u8, f32>);

impl Default for InputActions {
    fn default() -> Self { Self { 0: HashMap::new() } }
}

impl InputActions {
    pub fn value<T: Into<u8>>(&self, action: T) -> f32 {
        if let Some(value) = self.0.get(&action.into()) { *value } else { 0.0 }
    }

    pub fn is_active<T: Into<u8>>(&self, action: T) -> bool {
        if let Some(value) = self.0.get(&action.into()) { *value == 1.0 } else { false }
    }
}

pub fn sys_update_input_actions(mut player_query: Query<(&mut InputActions, &InputActionBindings, &RawButtonInput, &RawAxisInput)>) {
    for (mut input_actions, input_action_bindings, raw_button_input, raw_axis_input) in player_query.iter_mut() {
        input_actions.0.clear();
        for input_action_binding in input_action_bindings.iter() {
            let mut value = input_action_binding.binding().value(raw_button_input, raw_axis_input);
            if let Some(existing_value) = input_actions.0.get(&input_action_binding.action()) { value += existing_value; }
            
            input_actions.0.insert(input_action_binding.action(), value);
        }
    }
}