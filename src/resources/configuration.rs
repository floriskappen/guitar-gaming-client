use bevy::prelude::*;
use cpal::Device;

pub enum DeviceChannel {
    L,
    R,
    Both
}

#[derive(Resource)]
pub struct Configuration {
    pub device: Option<Device>,
    pub channel: Option<DeviceChannel>
}

// custom implementation for unusual values
impl Default for Configuration {
    fn default() -> Self {
        Configuration {
            device: None,
            channel: None
        }
    }
}
