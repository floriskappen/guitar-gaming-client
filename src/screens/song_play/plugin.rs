use bevy::prelude::*;

use crate::{components::{song_note::move_song_notes, song_timeline::manage_song_timeline}, states::app_state::AppState};

use super::song_play::{song_play_cleanup, song_play_load, song_play_update};
pub struct SongPlayPlugin;

impl Plugin for SongPlayPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::SongPlay), song_play_load);
        app.add_systems(Update, song_play_update.run_if(in_state(AppState::SongPlay)));
        app.add_systems(Update, manage_song_timeline.run_if(in_state(AppState::SongPlay)));
        app.add_systems(Update, move_song_notes.run_if(in_state(AppState::SongPlay)));
        app.add_systems(OnExit(AppState::SongPlay), song_play_cleanup);
    }
}
