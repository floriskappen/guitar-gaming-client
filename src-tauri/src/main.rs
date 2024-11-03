// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

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
        pub mod structs;
    }
}
pub mod state {
    pub mod configuration;
}

fn main() {
    guitar_gaming_client_lib::run()
}
