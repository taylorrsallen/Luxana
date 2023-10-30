use crate::*;

const DEVICE_COUNT: usize = 32;
const MOUSE_OFFSET: usize = DEVICE_COUNT;
const GAMEPAD_OFFSET: usize = DEVICE_COUNT * 2;

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum InputDevice {
    Keyboard(u8),
    Mouse(u8),
    Gamepad(u8),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Component)]
pub struct InputDeviceReceiver(Bitmask<2>);
impl Default for InputDeviceReceiver {
    fn default() -> Self { Self { 0: Bitmask::new(false) } }
}

impl InputDeviceReceiver {
    pub fn get_devices(&self) -> Vec<InputDevice> {
        let mut devices = vec![];

        for i in 0..DEVICE_COUNT as u8 {
            if self.is_device_enabled(InputDevice::Keyboard(i)) { devices.push(InputDevice::Keyboard(i)); }
            if self.is_device_enabled(InputDevice::Mouse(i)) { devices.push(InputDevice::Mouse(i)); }
            if self.is_device_enabled(InputDevice::Gamepad(i)) { devices.push(InputDevice::Gamepad(i)); }
        }

        devices
    }

    pub fn from_devices(devices: &[InputDevice]) -> Self {
        let mut input_device_receiver = Self::default();
        for device in devices { input_device_receiver.enable_device(*device); }
        input_device_receiver
    }

    pub fn is_device_enabled(&self, device: InputDevice) -> bool {
        match device {
            InputDevice::Keyboard(id) => { self.0.is_bit_on(id as usize) }
            InputDevice::Mouse(id) => { self.0.is_bit_on(id as usize + MOUSE_OFFSET) }
            InputDevice::Gamepad(id) => { self.0.is_bit_on(id as usize + GAMEPAD_OFFSET) }
        }
    }

    pub fn set_device(&mut self, device: InputDevice, enabled: bool) {
        match device {
            InputDevice::Keyboard(id) => {
                if !Self::is_device_id_valid(id) { return; }
                self.0.set_bit(id as usize, enabled);
            }
            InputDevice::Mouse(id) => {
                if !Self::is_device_id_valid(id) { return; }
                self.0.set_bit(id as usize + MOUSE_OFFSET, enabled);
            }
            InputDevice::Gamepad(id) => {
                if !Self::is_device_id_valid(id) { return; }
                self.0.set_bit(id as usize + GAMEPAD_OFFSET, enabled);
            }
        }
    }

    pub fn enable_device(&mut self, device: InputDevice) {
        self.set_device(device, true);
    }
    
    pub fn disable_device(&mut self, device: InputDevice) {
        self.set_device(device, false);
    }

    pub fn toggle_device(&mut self, device: InputDevice) {
        match device {
            InputDevice::Keyboard(id) => {
                if !Self::is_device_id_valid(id) { return; }
                if self.0.is_bit_on(id as usize) { self.0.set_bit_off(id as usize) } else { self.0.set_bit_on(id as usize) }
            }
            InputDevice::Mouse(id) => {
                if !Self::is_device_id_valid(id) { return; }
                let index = id as usize + MOUSE_OFFSET;
                if self.0.is_bit_on(index) { self.0.set_bit_off(index) } else { self.0.set_bit_on(index) }
            }
            InputDevice::Gamepad(id) => {
                if !Self::is_device_id_valid(id) { return; }
                let index = id as usize + GAMEPAD_OFFSET;
                if self.0.is_bit_on(index) { self.0.set_bit_off(index) } else { self.0.set_bit_on(index) }
            }
        }
    }

    pub fn with_device(mut self, device: InputDevice) -> Self {
        self.enable_device(device);
        self
    }

    fn is_device_id_valid(device_id: u8) -> bool { device_id < 32 }
}