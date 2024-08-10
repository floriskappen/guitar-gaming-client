use bevy::prelude::*;

// Define the different states of the application
#[derive(Debug, Clone, Eq, PartialEq, Hash, States)]
pub enum AppState {
    InputDeviceOverview,
    InputDeviceDetail,
    Tune,
}
