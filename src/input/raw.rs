use crate::*;
use super::*;

#[derive(Component, Clone)]
pub struct RawButtonInput([Bitmask<4>; 3]);

impl Default for RawButtonInput {
    fn default() -> Self { Self { 0: [Bitmask::new(false); 3] } }
}

impl RawButtonInput {
    pub fn get(&self, state: usize) -> &Bitmask<4> { &self.0[state] }

    fn press(&mut self, input: u32) {
        self.0[0].set_bit_on(input as usize);
        self.0[2].set_bit_on(input as usize);
    }

    pub fn press_key(&mut self, input: KeyCode) { self.press(input as u32); }
    pub fn press_mouse_button(&mut self, input: MouseButton) { self.press(Self::mouse_button_bit(input)); }
    pub fn press_gamepad_button(&mut self, input: GamepadButtonType) { self.press(Self::gamepad_button_bit(input)); }

    pub fn is_pressed(&self, input: u32) -> bool { self.0[0].is_bit_on(input as usize) }
    pub fn is_slice_pressed(&self, inputs: &[u32]) -> bool { inputs.into_iter().any(|input| self.is_pressed(*input)) }
    pub fn is_any_pressed(&self) -> bool { !self.0[0].is_off() }

    pub fn consume_pressed(&mut self, input: u32) -> bool {
        let pressed = self.0[0].is_bit_on(input as usize);
        self.0[0].set_bit_off(input as usize);
        pressed
    }

    fn release(&mut self, input: u32) {
        self.0[0].set_bit_off(input as usize);
        self.0[1].set_bit_on(input as usize);
        self.0[2].set_bit_off(input as usize);
    }

    pub fn release_key(&mut self, input: KeyCode) { self.release(input as u32); }
    pub fn release_mouse_button(&mut self, input: MouseButton) { self.release(Self::mouse_button_bit(input)); }
    pub fn release_gamepad_button(&mut self, input: GamepadButtonType) { self.release(Self::gamepad_button_bit(input)); }

    pub fn is_released(&self, input: u32) -> bool { self.0[1].is_bit_on(input as usize) }
    pub fn is_slice_released(&self, inputs: &[u32]) -> bool { inputs.into_iter().any(|input| self.is_released(*input)) }
    pub fn is_any_released(&self) -> bool { !self.0[1].is_off() }

    pub fn consume_released(&mut self, input: u32) -> bool {
        let released = self.0[1].is_bit_on(input as usize);
        self.0[1].set_bit_off(input as usize);
        released
    }


    pub fn is_held(&self, input: u32) -> bool { self.0[2].is_bit_on(input as usize) }
    pub fn is_slice_held(&self, inputs: &[u32]) -> bool { inputs.into_iter().any(|input| self.is_held(*input)) }
    pub fn is_any_held(&self) -> bool { !self.0[2].is_off() }

    pub fn consume_held(&mut self, input: u32) -> bool {
        let held = self.0[2].is_bit_on(input as usize);
        self.0[2].set_bit_off(input as usize);
        held
    }

    // pub fn release_all(&mut self) {
    //     self.just_released.extend(self.pressed.drain());
    // }

    pub fn clear(&mut self, input: u32) {
        self.0[0].set_bit_off(input as usize);
        self.0[1].set_bit_off(input as usize);
    }

    pub fn clear_all(&mut self) {
        self.0[0].set_off();
        self.0[1].set_off();
    }

    // pub fn get_pressed(&self) -> impl ExactSizeIterator<Item = &T> {
    //     self.pressed.iter()
    // }

    // pub fn get_released(&self) -> impl ExactSizeIterator<Item = &T> {
    //     self.just_released.iter()
    // }

    pub fn mouse_button_bit(input: MouseButton) -> u32 {
        return match input {
            MouseButton::Left => { 0 }
            MouseButton::Right => { 1 }
            MouseButton::Middle => { 2 }
            MouseButton::Other(value) => { (value as u32).min(MOUSE_VARIABLE_INPUTS) + MOUSE_INPUTS }
        } + KEY_INPUTS;
    }

    pub fn gamepad_button_bit(input: GamepadButtonType) -> u32 {
        return match input {
            GamepadButtonType::South => { 0 }
            GamepadButtonType::East => { 1 }
            GamepadButtonType::North => { 2 }
            GamepadButtonType::West => { 3 }
            GamepadButtonType::C => { 4 }
            GamepadButtonType::Z => { 5 }
            GamepadButtonType::LeftTrigger => { 6 }
            GamepadButtonType::LeftTrigger2 => { 7 }
            GamepadButtonType::RightTrigger => { 8 }
            GamepadButtonType::RightTrigger2 => { 9 }
            GamepadButtonType::Select => { 10 }
            GamepadButtonType::Start => { 11 }
            GamepadButtonType::Mode => { 12 }
            GamepadButtonType::LeftThumb => { 13 }
            GamepadButtonType::RightThumb => { 14 }
            GamepadButtonType::DPadUp => { 15 }
            GamepadButtonType::DPadDown => { 16 }
            GamepadButtonType::DPadLeft => { 17 }
            GamepadButtonType::DPadRight => { 18 }
            GamepadButtonType::Other(value) => { (value as u32).min(GAMEPAD_VARIABLE_BUTTONS) + GAMEPAD_BUTTONS }
        } + KEY_INPUTS + MOUSE_INPUTS + MOUSE_VARIABLE_INPUTS;
    }
}

#[derive(Component, Clone)]
pub struct RawAxisInput([f32; 32]);

impl Default for RawAxisInput {
    fn default() -> Self { Self { 0: [0.0; 32] } }
}

impl RawAxisInput {
    pub fn get(&self, axis: u32) -> f32 { self.0[axis as usize] }
    pub fn set(&mut self, axis: u32, value: f32) { self.0[axis as usize] = value; }
    pub fn clear_all(&mut self) { self.0 = [0.0; 32]; }

    pub fn gamepad_axis_bit(input: GamepadAxisType) -> u32 {
        return match input {
            GamepadAxisType::LeftStickX => { 0 }
            GamepadAxisType::LeftStickY => { 1 }
            GamepadAxisType::LeftZ => { 2 }
            GamepadAxisType::RightStickX => { 3 }
            GamepadAxisType::RightStickY => { 4 }
            GamepadAxisType::RightZ => { 5 }
            GamepadAxisType::Other(value) => { (value as u32).min(GAMEPAD_VARIABLE_AXES) + GAMEPAD_AXES }
        } + MOUSE_AXES;
    }
}