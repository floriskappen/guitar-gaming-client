use bevy::prelude::*;

use crate::{components::{song_note::update_song_note, song_timeline::update_song_timeline}, resources::output_audio_song::{output_audio_song_cleanup, output_audio_song_load}, states::app_state::AppState};

use super::{camera::camera_system, song_play::{song_play_cleanup, song_play_load, song_play_update}};
pub struct SongPlayPlugin;

impl Plugin for SongPlayPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::SongPlay), output_audio_song_load);
        app.add_systems(OnEnter(AppState::SongPlay), song_play_load.after(output_audio_song_load));

        app.add_systems(Update, song_play_update.run_if(in_state(AppState::SongPlay)));
        app.add_systems(Update, update_song_timeline.run_if(in_state(AppState::SongPlay)));
        app.add_systems(Update, update_song_note.run_if(in_state(AppState::SongPlay)));
        app.add_systems(Update, camera_system.run_if(in_state(AppState::SongPlay)));

        app.add_systems(OnExit(AppState::SongPlay), song_play_cleanup);
        app.add_systems(OnExit(AppState::SongPlay), output_audio_song_cleanup);
    }
}
