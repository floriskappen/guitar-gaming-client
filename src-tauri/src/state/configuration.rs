use cpal::Device;
use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct ConfigurationState {
    pub device: Option<String>,
    pub selected_device_channels: Vec<u16>,
    pub approach_rate: f32 // Meters/units per second
}

impl Default for ConfigurationState {
    fn default() -> Self {

        ConfigurationState {
            device: None,
            selected_device_channels: vec![],
            approach_rate: 7.0,
        }
    }
}
