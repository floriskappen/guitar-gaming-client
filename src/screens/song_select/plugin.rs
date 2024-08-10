use bevy::prelude::*;

use crate::states::app_state::AppState;

use super::song_select::{song_select_cleanup, song_select_load, song_select_update};
pub struct SongSelectPlugin;

impl Plugin for SongSelectPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::SongSelect), song_select_load);
        app.add_systems(Update, song_select_update.run_if(in_state(AppState::SongSelect)));
        app.add_systems(OnExit(AppState::SongSelect), song_select_cleanup);
    }
}
