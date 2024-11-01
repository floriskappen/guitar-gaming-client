use std::{fs, sync::Mutex};

use state::configuration::ConfigurationState;
use tauri::Manager;

pub mod helpers {
    pub mod notes;
    pub mod tuning;
}
pub mod commands {
    pub mod configuration {
        pub mod commands;
        pub mod helpers;
    }
    pub mod input_devices {
        pub mod commands;
        pub mod helpers;
    }
    pub mod songs {
        pub mod commands;
        pub mod helpers;
    }
}
pub mod state {
    pub mod configuration;
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            app.manage(Mutex::new(ConfigurationState::default()));

            // Make sure songs directory exists
            let songs_directory = app.path().app_data_dir().unwrap().join("songs");
            if !songs_directory.exists() {
                fs::create_dir_all(songs_directory)?;
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::input_devices::commands::get_input_devices,
            commands::configuration::commands::get_configuration,
            commands::songs::commands::get_songs,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
