use bevy::prelude::*;

use crate::states::app_state::AppState;
use crate::screens::input_device_detail::audio_bar::audio_bar_system;
use crate::screens::input_device_detail::input_device_detail::{input_device_detail_load, input_device_detail_cleanup};

use super::input_device_detail::input_device_detail_update;

pub struct InputDeviceDetailPlugin;

impl Plugin for InputDeviceDetailPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InputDeviceDetail), input_device_detail_load);
        app.add_systems(Update, input_device_detail_update.run_if(in_state(AppState::InputDeviceDetail)));
        app.add_systems(Update, audio_bar_system.run_if(in_state(AppState::InputDeviceDetail)));
        app.add_systems(OnExit(AppState::InputDeviceDetail), input_device_detail_cleanup);
    }
}
