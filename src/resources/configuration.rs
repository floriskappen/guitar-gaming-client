use bevy::prelude::*;
use cpal::Device;

#[derive(Resource)]
pub struct ConfigurationResource {
    pub device: Option<Device>,
    pub selected_device_channels: Vec<u16>
}

impl Default for ConfigurationResource {
    fn default() -> Self {

        ConfigurationResource {
            device: None,
            selected_device_channels: vec![]
        }
    }
}