use cpal::{traits::{DeviceTrait, HostTrait}, Device};

#[tauri::command]
pub fn get_input_devices() -> Vec<String> {
    let host = cpal::default_host();
    let devices = host.input_devices().unwrap();
    let device_list: Vec<Device> = devices.collect();
    let device_names_list = device_list.iter().map(|device| {
        return device.name().unwrap()
    }).collect::<Vec<String>>();

    device_names_list
}
