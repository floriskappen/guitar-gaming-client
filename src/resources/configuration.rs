use bevy::prelude::*;
use cpal::Device;

#[derive(PartialEq, Clone)]
pub enum DeviceChannel {
    One,
    Two,
    All
}

#[derive(Default, Resource)]
pub struct Configuration {
    pub device: Option<Device>,
    pub selected_device_channel: Option<DeviceChannel>
}
