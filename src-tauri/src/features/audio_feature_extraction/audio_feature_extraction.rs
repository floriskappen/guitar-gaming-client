use std::path::PathBuf;

use super::helpers::sonic_annotator::get_tempo as get_tempo_helper;

pub fn get_tempo(resource_dir: PathBuf, file_path: PathBuf) -> Result<f64, String> {
    return get_tempo_helper(resource_dir, file_path)
}
