use bevy::prelude::*;

use crate::states::app_state::AppState;
use crate::screens::input_device_overview::input_device_overview::{input_device_overview_cleanup, input_device_overview_update, input_device_overview_load};

pub struct InputDeviceOverviewPlugin;

impl Plugin for InputDeviceOverviewPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InputDeviceOverview), input_device_overview_load);
        app.add_systems(Update, input_device_overview_update.run_if(in_state(AppState::InputDeviceOverview)));
        app.add_systems(OnExit(AppState::InputDeviceOverview), input_device_overview_cleanup);
    }
}
