use std::sync::Mutex;

use tauri::State;

use crate::{helpers::configuration::load_from_disk, state::configuration::ConfigurationState};

#[tauri::command]
pub fn get_configuration(state: State<'_, Mutex<ConfigurationState>>) -> ConfigurationState {
    let mut state = state.lock().unwrap();

    let loaded_state = load_from_disk();
    *state = loaded_state.clone();
    return loaded_state;
}
