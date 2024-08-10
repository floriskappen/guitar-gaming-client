use bevy::prelude::*;
use resources::{configuration::Configuration, input_device::InputDevice, input_devices::InputDevices};
use screens::{input_device_detail::plugin::InputDeviceDetailPlugin, input_device_overview::plugin::InputDeviceOverviewPlugin};
use states::app_state::AppState;

mod resources {
    pub mod input_devices;
    pub mod configuration;
    pub mod input_device;
}
mod components {
    pub mod button_primary;
}
mod screens {
    pub mod input_device_overview {
        pub mod plugin;
        pub mod input_device_overview;
        pub mod button_select_input_device;
    }
    pub mod input_device_detail {
        pub mod plugin;
        pub mod input_device_detail;
        pub mod audio_bar;
    }
}
mod states {
    pub mod app_state;
}
mod helpers {
    pub mod input_device;
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
        .insert_resource(InputDevices::default())
        .insert_resource(InputDevice::default())

        .add_plugins(InputDeviceOverviewPlugin)
        .add_plugins(InputDeviceDetailPlugin)

        .run();
}
