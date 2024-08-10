use bevy::prelude::*;

use crate::states::app_state::AppState;
use crate::screens::input_device_overview::button_select_input_device::button_select_input_device_interaction_system;
use crate::screens::input_device_overview::input_device_overview::{cleanup_input_device_overview, state_input_device_overview};

pub struct InputDeviceOverviewPlugin;

impl Plugin for InputDeviceOverviewPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InputDeviceOverview), state_input_device_overview);
        app.add_systems(Update, button_select_input_device_interaction_system.run_if(in_state(AppState::InputDeviceOverview)));
        app.add_systems(OnExit(AppState::InputDeviceOverview), cleanup_input_device_overview);
    }
}
