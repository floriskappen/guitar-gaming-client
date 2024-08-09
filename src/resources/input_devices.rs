use bevy::prelude::*;
use cpal::Device;

#[derive(Resource)]
pub struct InputDevices {
    pub host: cpal::Host,
    pub input_devices: Vec<Device>,
}
// custom implementation for unusual values
impl Default for InputDevices {
    fn default() -> Self {
        let host = cpal::default_host();

        InputDevices {
            host,
            input_devices: vec![]
        }
    }
}
