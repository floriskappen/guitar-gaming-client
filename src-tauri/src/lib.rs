use std::sync::Mutex;

use state::configuration::ConfigurationState;
use tauri::Manager;

pub mod helpers {
    pub mod configuration;
    pub mod input_device;
    pub mod notes;
    pub mod persistence;
    pub mod tuning;
}
pub mod commands {
    pub mod input_devices;
    pub mod configuration;
}
pub mod state {
    pub mod configuration;
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            app.manage(Mutex::new(ConfigurationState::default()));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::input_devices::get_input_devices,
            commands::configuration::get_configuration,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
