use bevy::prelude::*;

use crate::states::app_state::AppState;
use crate::screens::input_device_detail::audio_bar::audio_bar_system;
use crate::screens::input_device_detail::input_device_detail::{cleanup_input_device_detail, state_input_device_detail};

pub struct InputDeviceDetailPlugin;

impl Plugin for InputDeviceDetailPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InputDeviceDetail), state_input_device_detail);
        app.add_systems(Update, audio_bar_system.run_if(in_state(AppState::InputDeviceDetail)));
        app.add_systems(OnExit(AppState::InputDeviceDetail), cleanup_input_device_detail);
    }
}
