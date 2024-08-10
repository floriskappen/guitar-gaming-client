use bevy::prelude::*;

use crate::states::app_state::AppState;

use super::tune::{tune_cleanup, tune_load, tune_update};

pub struct TunePlugin;

impl Plugin for TunePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Tune), tune_load);
        app.add_systems(Update, tune_update.run_if(in_state(AppState::Tune)));
        app.add_systems(OnExit(AppState::Tune), tune_cleanup);
    }
}
