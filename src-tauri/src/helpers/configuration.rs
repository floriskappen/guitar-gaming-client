use std::{fs::File, io::Read};

use cpal::traits::{DeviceTrait, HostTrait};
use serde::{Deserialize, Serialize};

use crate::state::configuration::ConfigurationState;

use super::persistence::get_data_dir;

pub const FILENAME: &str = "configuration.json";

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigurationStateSerializable {
    pub device_name: Option<String>,
    pub selected_device_channels: Vec<u16>
}

pub fn save_to_disk(configuration: ConfigurationState) {
    let serializable_configuration = ConfigurationStateSerializable {
        device_name: if configuration.device.is_some() {
            let device_clone = configuration.device.clone().unwrap();
            Some(device_clone)
        } else { None },
        selected_device_channels: configuration.selected_device_channels.clone()
    };
    let directory = get_data_dir().unwrap();
    let filepath = directory.join(FILENAME);
    let file = File::create(filepath).expect("Failed to create file");
    serde_json::to_writer(file, &serializable_configuration).expect("Failed to write JSON to file");
}

pub fn load_from_disk() -> ConfigurationState {
    let directory = get_data_dir().unwrap();
    let filepath = directory.join(FILENAME);

    if filepath.exists() {
        // Open the file and read its contents
        let mut file = File::open(filepath).expect("Failed to open file");
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("Failed to read file");

        // Deserialize the JSON contents into the serializable struct
        let serializable_configuration: ConfigurationStateSerializable =
            serde_json::from_str(&contents).expect("Failed to deserialize JSON");

        let host = cpal::default_host();
        let devices = host.devices().unwrap();

        if let Some(device_name) = &serializable_configuration.device_name {
            let device = devices.into_iter().find(|device| &device.name().unwrap() == device_name);
            if let Some(found_device) = device {

                // info!("{:?}", serializable_configuration);
                return ConfigurationState {
                    device: Some(found_device.name().unwrap()),
                    selected_device_channels: serializable_configuration.selected_device_channels,
                    ..ConfigurationState::default()
                }
            }
        }
    }

    ConfigurationState::default()

}
