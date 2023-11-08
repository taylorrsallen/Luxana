use crate::*;

use bevy::input::InputSystem;
use bevy::input::mouse::{MouseWheel, MouseMotion};
use bevy::input::{keyboard::KeyboardInput, ButtonState, mouse::MouseButtonInput};
use bevy::input::gamepad::{GamepadButtonChangedEvent, GamepadAxisChangedEvent};

mod device;
pub use device::*;
mod raw;
pub use raw::*;
mod actions;
pub use actions::*;

pub const KEY_INPUTS: u32 = 163;
pub const MOUSE_INPUTS: u32 = 5;
pub const MOUSE_VARIABLE_INPUTS: u32 = 16;
pub const GAMEPAD_BUTTONS: u32 = 19;
pub const GAMEPAD_VARIABLE_BUTTONS: u32 = 16;

pub const MOUSE_AXES: u32 = 4;
pub const GAMEPAD_AXES: u32 = 6;
pub const GAMEPAD_VARIABLE_AXES: u32 = 16;

#[derive(Default)]
pub struct TankInputPlugin;
impl Plugin for TankInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, (
                sys_clear_button_input,
                sys_keyboard_input,
                sys_mouse_button_input,
                sys_gamepad_button_input,

                sys_mouse_axis_input,
                sys_gamepad_axis_input,

                sys_update_input_actions,
            ).after(InputSystem).chain());
    }
}

fn sys_clear_button_input(mut raw_button_input_query: Query<&mut RawButtonInput>) {
    for mut raw_input in raw_button_input_query.iter_mut() { raw_input.bypass_change_detection().clear_all(); }
}

fn sys_keyboard_input(
    mut events: EventReader<KeyboardInput>,
    mut raw_button_input_query: Query<(&mut RawButtonInput, &InputDeviceReceiver)>,
) {
    let events: Vec<&KeyboardInput> = events.iter().collect();
    for (mut raw_input, device_receiver) in raw_button_input_query.iter_mut() {
        // Bevy is shaving off device id, so until we fix that, just check for 0
        if !device_receiver.is_device_enabled(InputDevice::Keyboard(0)) { continue; }
        for event in events.iter() {
            let key = if let Some(key) = event.key_code { key } else { continue };
            match event.state {
                ButtonState::Pressed => raw_input.press_key(key),
                ButtonState::Released => raw_input.release_key(key),
            }
        }
    }
}

fn sys_mouse_button_input(
    mut events: EventReader<MouseButtonInput>,
    mut raw_button_input_query: Query<(&mut RawButtonInput, &InputDeviceReceiver)>,
) {
    let events: Vec<&MouseButtonInput> = events.iter().collect();
    for (mut raw_input, device_receiver) in raw_button_input_query.iter_mut() {
        // Bevy is shaving off device id, so until we fix that, just check for 0
        if !device_receiver.is_device_enabled(InputDevice::Mouse(0)) { continue; }
        for event in events.iter() {
            match event.state {
                ButtonState::Pressed => raw_input.press_mouse_button(event.button),
                ButtonState::Released => raw_input.release_mouse_button(event.button),
            }
        }
    }
}

fn sys_gamepad_button_input(
    mut events: EventReader<GamepadButtonChangedEvent>,
    mut raw_button_input_query: Query<(&mut RawButtonInput, &InputDeviceReceiver)>,
    button_input: ResMut<Input<GamepadButton>>,
) {
    let events: Vec<&GamepadButtonChangedEvent> = events.iter().collect();
    for (mut raw_input, device_receiver) in raw_button_input_query.iter_mut() {
        for event in events.iter() {
            if !device_receiver.is_device_enabled(InputDevice::Gamepad(event.gamepad.id as u8)) { continue; }
            
            let gamepad_button = GamepadButton { gamepad: event.gamepad, button_type: event.button_type };
            println!("{:?}", gamepad_button);
            if button_input.just_released(gamepad_button) {
                raw_input.release_gamepad_button(event.button_type);
                println!("Released");
            } else if button_input.just_pressed(gamepad_button) {
                raw_input.press_gamepad_button(event.button_type);
                println!("Pressed");
            }
        }
    }
}

fn sys_mouse_axis_input(
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut raw_axis_input_query: Query<(&mut RawAxisInput, &InputDeviceReceiver)>,
) {
    let mouse_motion_events: Vec<&MouseMotion> = mouse_motion_events.iter().collect();
    let mouse_wheel_events: Vec<&MouseWheel> = mouse_wheel_events.iter().collect();
    for (mut raw_axis_input, device_receiver) in raw_axis_input_query.iter_mut() {
        if !device_receiver.is_device_enabled(InputDevice::Mouse(0)) { continue; }

        let mut motion_delta = Vec2::ZERO;
        for event in mouse_motion_events.iter() { motion_delta += event.delta; }
        raw_axis_input.set(0, motion_delta.x);
        raw_axis_input.set(1, motion_delta.y);
        
        let mut scroll_delta = Vec2::ZERO;
        for event in mouse_wheel_events.iter() { scroll_delta += Vec2::new(event.x, event.y); }
        raw_axis_input.set(2, scroll_delta.x);
        raw_axis_input.set(3, scroll_delta.y);
    }
}

fn sys_gamepad_axis_input(
    mut events: EventReader<GamepadAxisChangedEvent>,
    mut raw_axis_input_query: Query<(&mut RawAxisInput, &InputDeviceReceiver)>,
) {
    let events: Vec<&GamepadAxisChangedEvent> = events.iter().collect();
    for (mut raw_axis_input, device_receiver) in raw_axis_input_query.iter_mut() {
        for event in events.iter() {
            if !device_receiver.is_device_enabled(InputDevice::Gamepad(event.gamepad.id as u8)) { continue; }
            raw_axis_input.set(RawAxisInput::gamepad_axis_bit(event.axis_type), event.value);
        }
    }
}