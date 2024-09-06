use bevy::{diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin}, prelude::*};
use bevy_mod_billboard::plugin::BillboardPlugin;
use helpers::persistence::get_data_dir;
use resources::{configuration::ConfigurationResource, input_device::InputDeviceResource, input_devices::InputDevicesResource, song_library::SongLibraryResource, song_loaded::SongLoadedResource};
use screens::{input_device_detail::plugin::InputDeviceDetailPlugin, input_device_overview::plugin::InputDeviceOverviewPlugin, song_play::plugin::SongPlayPlugin, song_select::plugin::SongSelectPlugin, tune::plugin::TunePlugin};
use states::app_state::AppState;

mod constants {
    pub mod ingame;
}
mod resources {
    pub mod input_devices;
    pub mod configuration;
    pub mod input_device;
    pub mod song_library;
    pub mod song_loaded;
    pub mod output_audio_song;
}
mod components {
    pub mod button_primary;
    pub mod button_minimal;
    pub mod song_note;
    pub mod song_timeline;
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
        pub mod plugin;
        pub mod tune;
    }
    pub mod song_select {
        pub mod plugin;
        pub mod song_select;
        pub mod song_list;
    }
    pub mod song_play {
        pub mod plugin;
        pub mod song_play;
        pub mod camera;
    }
}
mod states {
    pub mod app_state;
}
mod helpers {
    pub mod input_device;
    pub mod tuning;
    pub mod persistence;
    pub mod song_notes;
}

fn main() {

    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Guitar Gaming".to_string(),
            ..Default::default()
        }),
        ..Default::default()
    }));
    app.add_plugins(FrameTimeDiagnosticsPlugin);
    app.add_plugins(LogDiagnosticsPlugin::default());

    app.add_plugins(BillboardPlugin);

    let configuration_resource = ConfigurationResource::load_from_disk();
    if configuration_resource.device.is_some() && !configuration_resource.selected_device_channels.is_empty() {
        app.insert_state(AppState::SongSelect);
    } else {
        app.insert_state(AppState::InputDeviceOverview);
    }

    app.insert_resource(configuration_resource);
    app.insert_resource(InputDevicesResource::default());
    app.insert_resource(InputDeviceResource::default());
    app.insert_resource(SongLibraryResource::load_from_disk());
    app.insert_resource(SongLoadedResource::default());

    app.add_plugins(InputDeviceOverviewPlugin);
    app.add_plugins(InputDeviceDetailPlugin);
    app.add_plugins(TunePlugin);
    app.add_plugins(SongSelectPlugin);
    app.add_plugins(SongPlayPlugin);

    let directory = get_data_dir().unwrap();
    info!("data directory: {:?}", directory);

    app.run();
}
