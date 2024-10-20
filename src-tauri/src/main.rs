// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod helpers {
    mod input_device;
    mod notes;
    mod persistence;
    mod tuning;
}
mod state {
    pub mod configuration;
}

fn main() {
    guitar_gaming_client_lib::run()
}
