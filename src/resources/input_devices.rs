use bevy::prelude::*;
use cpal::Device;

#[derive(Resource)]
pub struct InputDevicesResource {
    pub host: cpal::Host,
    pub input_devices: Vec<Device>,
}

impl Default for InputDevicesResource {
    fn default() -> Self {
        let host = cpal::default_host();

        InputDevicesResource {
            host,
            input_devices: vec![]
        }
    }
}
