use std::{fs::File, io::Read};

use bevy::prelude::*;
use cpal::{traits::{DeviceTrait, HostTrait}, Device};
use serde::{Deserialize, Serialize};

use crate::helpers::persistence::get_data_dir;

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigurationResourceSerializable {
    pub device_name: Option<String>,
    pub selected_device_channels: Vec<u16>
}

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

impl ConfigurationResource {
    pub fn save_to_disk(&self) {
        let serializable_configuration = ConfigurationResourceSerializable {
            device_name: if self.device.is_some() {
                let device_clone = self.device.clone().unwrap();
                Some(device_clone.name().unwrap())
            } else { None },
            selected_device_channels: self.selected_device_channels.clone()
        };
        let directory = get_data_dir().unwrap();
        let filepath = directory.join("configuration.json");
        let file = File::create(filepath).expect("Failed to create file");
        serde_json::to_writer(file, &serializable_configuration).expect("Failed to write JSON to file");
    }
    pub fn load_from_disk() -> Self {
        let directory = get_data_dir().unwrap();
        let filepath = directory.join("configuration.json");

        if filepath.exists() {
            // Open the file and read its contents
            let mut file = File::open(filepath).expect("Failed to open file");
            let mut contents = String::new();
            file.read_to_string(&mut contents).expect("Failed to read file");
    
            // Deserialize the JSON contents into the serializable struct
            let serializable_configuration: ConfigurationResourceSerializable =
                serde_json::from_str(&contents).expect("Failed to deserialize JSON");

            let host = cpal::default_host();
            let devices = host.devices().unwrap();

            println!("{:?}", serializable_configuration);

            if let Some(device_name) = serializable_configuration.device_name {
                let device = devices.into_iter().find(|device| device.name().unwrap() == device_name);
                if let Some(found_device) = device {
                    return ConfigurationResource {
                        device: Some(found_device),
                        selected_device_channels: serializable_configuration.selected_device_channels
                    }
                }
            }
        }

        return ConfigurationResource::default();

    }
}
