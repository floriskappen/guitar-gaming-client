use std::sync::Mutex;

use tauri::{AppHandle, Manager, State};

use crate::{commands::configuration::helpers::load_from_disk, state::configuration::ConfigurationState};

#[tauri::command]
pub fn get_configuration(
    state: State<'_, Mutex<ConfigurationState>>,
    app_handle: AppHandle,
) -> ConfigurationState {
    let mut state = state.lock().unwrap();

    let app_data_directory = app_handle.path().app_data_dir().unwrap();

    let loaded_state = load_from_disk(app_data_directory);
    *state = loaded_state.clone();
    return loaded_state;
}
