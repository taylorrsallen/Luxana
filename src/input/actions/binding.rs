use crate::*;

use std::slice::Iter;
use serde::*;

////////////////////////////////////////////////////////////////////////////////////////////////////
pub enum InputButtonState {
    Pressed,
    Released,
    Held,
    None,
}

pub struct InputButton {
    state: u8,
    button: u8,
}

impl InputButton {
    pub fn key(state: InputButtonState, key: KeyCode) -> Self { Self { state: state as u8, button: key as u8 } }
    pub fn mouse_button(state: InputButtonState, mouse_button: MouseButton) -> Self { Self { state: state as u8, button: RawButtonInput::mouse_button_bit(mouse_button) as u8 } }
    pub fn gamepad_button(state: InputButtonState, gamepad_button: GamepadButtonType) -> Self { Self { state: state as u8, button: RawButtonInput::gamepad_button_bit(gamepad_button) as u8 } }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
pub enum InputAxisState {
    Positive,
    Negative,
}

pub enum MouseAxis {
    MotionX,
    MotionY,
    ScrollX,
    ScrollY,
}

pub enum InputAxis {
    Mouse(MouseAxis),
    Gamepad(GamepadAxisType),
}


////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum Binding {
    Button { state: u8, input: u8 }, // active = 1.0, inactive = 0.0
    ButtonAxis { states: u8, inputs: [u8; 2] }, // [0] active = 1.0, [1] active = -1.0, both or none = 0.0
    ButtonCombo { states: u8, inputs: [u8; 4] }, // all active = 1.0, all not active = 0.0
    Axis { state: u8, axis: u8 }, // copy value from axis: state 0 = -1.0..=1.0 range, state 1 = 0..=1.0 range, state 2 = -1.0..=0.0 range
}

impl Binding {
    pub fn value(&self, raw_button_input: &RawButtonInput, raw_axis_input: &RawAxisInput) -> f32 {
        let mut value = 0.0;
        self.set_value(&mut value, raw_button_input, raw_axis_input);
        value
    }

    fn set_value(&self, value: &mut f32, raw_button_input: &RawButtonInput, raw_axis_input: &RawAxisInput) {
        match *self {
            Binding::Button { state, input } => { Binding::set_button_value(state, input, value, raw_button_input); }
            Binding::ButtonAxis { states, inputs } => { Binding::set_button_axis_value(states, inputs, value, raw_button_input); }
            Binding::ButtonCombo { states, inputs } => { Binding::set_button_combo_value(states, inputs, value, raw_button_input); }
            Binding::Axis { state, axis } => { Binding::set_axis_value(state, axis, value, raw_axis_input); }
        }
    }

    fn set_button_value(state: u8, input: u8, value: &mut f32, raw_button_input: &RawButtonInput) {
        if raw_button_input.get(state as usize).is_bit_on(input as usize) { *value = 1.0; }
    }

    fn set_button_axis_value(states: u8, inputs: [u8; 2], value: &mut f32, raw_button_input: &RawButtonInput) {
        for i in 0..inputs.len() {
            let state = states >> (i * 2) & 3;
            if raw_button_input.get(state as usize).is_bit_on(inputs[i] as usize) { *value += (i as f32 * -2.0) + 1.0; }
        }
    }

    fn set_button_combo_value(states: u8, inputs: [u8; 4], value: &mut f32, raw_button_input: &RawButtonInput) {
        let mut inputs_satisfied: u8 = 0;
        for i in 0..inputs.len() {
            let state = states >> (i * 2) & 3;
            if state == 3 {
                inputs_satisfied |= 1 << i;
            } else if raw_button_input.get(state as usize).is_bit_on(inputs[i] as usize) {
                inputs_satisfied |= 1 << i;
            }
        }

        if inputs_satisfied == 15 { *value = 1.0; }
    }

    fn set_axis_value(state: u8, axis: u8, value: &mut f32, raw_axis_input: &RawAxisInput) {
        *value = raw_axis_input.get(axis as u32);
        match state {
            0 => { *value = raw_axis_input.get(axis as u32).max(0.0); }
            1 => { *value = raw_axis_input.get(axis as u32).min(0.0) * -1.0; }
            _ => { panic!("Invalid axis state value"); }
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct InputActionBinding {
    action: u8,
    binding: Binding,
}

impl InputActionBinding {
    pub fn action(&self) -> u8 { self.action }
    pub fn binding(&self) -> &Binding { &self.binding }

    pub fn button<T: Into<u8>>(action: T, input_button: InputButton) -> Self {
        Self { action: action.into(), binding: Binding::Button { state: input_button.state, input: input_button.button } }
    }

    pub fn button_axis<T: Into<u8>>(action: T, positive_button: InputButton, negative_button: InputButton) -> Self {
        let mut states = 0;
        let mut inputs = [0; 2];
        let input_buttons = [positive_button, negative_button];
        for i in 0..2 {
            states |= input_buttons[i].state << (i * 2);
            inputs[i] = input_buttons[i].button;
        }

        Self { action: action.into(), binding: Binding::ButtonAxis { states, inputs } }
    }

    pub fn button_combo<T: Into<u8>>(action: T, input_buttons: &[InputButton]) -> Self {
        let mut states = 0;
        let mut inputs = [0; 4];
        for (i, input_button) in input_buttons.iter().enumerate() {
            states |= input_button.state << (i * 2);
            inputs[i] = input_button.button;
        }

        Self { action: action.into(), binding: Binding::ButtonCombo { states, inputs } }
    }
    
    pub fn axis<T: Into<u8>>(action: T, state: InputAxisState, input_axis: InputAxis) -> Self {
        let axis = match input_axis {
                InputAxis::Mouse(mouse_axis) => { mouse_axis as u8 }
                InputAxis::Gamepad(gamepad_axis) => { RawAxisInput::gamepad_axis_bit(gamepad_axis) as u8 }
            };

        Self { action: action.into(), binding: Binding::Axis { state: state as u8, axis } }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Component, Serialize, Deserialize)]
pub struct InputActionBindings(Vec<InputActionBinding>);

impl Default for InputActionBindings {
    fn default() -> Self { Self { 0: vec![] } }
}

impl InputActionBindings {
    pub fn new(bindings: &[InputActionBinding]) -> Self { Self { 0: bindings.into() } }
    pub fn iter(&self) -> Iter<'_, InputActionBinding> { self.0.iter() }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// pub enum ActionType {
//     Button,

// }

// pub struct InputActionBindingsBuilder<T: Bitflag>(PhantomData<T>);

// impl<T: Bitflag> InputActionBindingsBuilder<T> {
//     pub fn new() -> InputActionBindings {
//         let mut input_action_bindings = InputActionBindings::new(bindings)
//     }
// }