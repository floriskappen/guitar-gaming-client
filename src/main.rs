use bevy::prelude::*;
use resources::{configuration::ConfigurationResource, input_device::InputDeviceResource, input_devices::InputDevicesResource};
use screens::{input_device_detail::plugin::InputDeviceDetailPlugin, input_device_overview::plugin::InputDeviceOverviewPlugin, tune::plugin::TunePlugin};
use states::app_state::AppState;

mod resources {
    pub mod input_devices;
    pub mod configuration;
    pub mod input_device;
}
mod components {
    pub mod button_primary;
    pub mod button_minimal;
}
mod screens {
    pub mod input_device_overview {
        pub mod plugin;
        pub mod input_device_overview;
    }
    pub mod input_device_detail {
        pub mod plugin;
        pub mod input_device_detail;
        pub mod audio_bar;
    }
    pub mod tune {
        pub mod tune;
        pub mod plugin;
    }
}
mod states {
    pub mod app_state;
}
mod helpers {
    pub mod input_device;
    pub mod tuning;
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

        .insert_resource(ConfigurationResource::default())
        .insert_resource(InputDevicesResource::default())
        .insert_resource(InputDeviceResource::default())

        .add_plugins(InputDeviceOverviewPlugin)
        .add_plugins(InputDeviceDetailPlugin)
        .add_plugins(TunePlugin)

        .run();
}
