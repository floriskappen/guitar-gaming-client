use bevy::prelude::Res;
use cpal::traits::DeviceTrait;

use crate::resources::configuration::Configuration;

pub fn state_input_device_detail(
    configuration: Res<Configuration>,
) {
    println!("Showing details for device: {}", configuration.device.clone().unwrap().name().unwrap());
    // Implement your logic for the InputDeviceDetail state
}