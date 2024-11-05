// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

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
pub mod features {
    pub mod audio_feature_extraction {
        pub mod audio_feature_extraction;
        pub mod constants;
        pub mod helpers {
            pub mod sonic_annotator;
            pub mod waveform;
        }
        pub mod structs {
            pub mod waveform_data;
        }
    }
}
pub mod helpers {
    pub mod notes;
    pub mod tuning;
}
pub mod state {
    pub mod configuration;
}

fn main() {
    guitar_gaming_client_lib::run()
}
