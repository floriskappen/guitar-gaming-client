use bevy::prelude::*;
use plugins::input_devices::InputDevicePlugin;
use resources::configuration::Configuration;
use states::{app_state::AppState, menu::{input_device_detail::state_input_device_detail, input_device_overview::{cleanup_input_device_overview, state_input_device_overview}}};

mod resources {
    pub mod input_devices;
    pub mod configuration;
}
mod components {
    pub mod button_primary;
}
mod states {
    pub mod menu {
        pub mod input_device_overview;
        pub mod input_device_detail;
    }
    pub mod app_state;
}
mod plugins {
    pub mod input_devices;
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Guitar Gaming".to_string(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .insert_state(AppState::InputDeviceOverview)
        .insert_resource(Configuration::default())
        .add_plugins(InputDevicePlugin)
        .add_systems(OnEnter(AppState::InputDeviceOverview), state_input_device_overview)
        .add_systems(OnExit(AppState::InputDeviceOverview), cleanup_input_device_overview)
        .add_systems(OnEnter(AppState::InputDeviceDetail), state_input_device_detail)
        .run();
}
