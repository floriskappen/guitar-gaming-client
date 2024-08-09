use bevy::prelude::*;
use cpal::Device;

#[derive(PartialEq, Clone)]
pub enum DeviceChannel {
    L,
    R,
    Both
}

#[derive(Default, Resource)]
pub struct Configuration {
    pub device: Option<Device>,
    pub channel: Option<DeviceChannel>
}
