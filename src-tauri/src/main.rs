// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod helpers {
    pub mod configuration;
    pub mod input_device;
    pub mod notes;
    pub mod persistence;
    pub mod song_library;
    pub mod tuning;
}
mod state {
    pub mod configuration;
}
mod commands {
    pub mod configuration;
    pub mod input_devices;
    pub mod song_library;
}

fn main() {
    guitar_gaming_client_lib::run()
}
