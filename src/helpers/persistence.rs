use directories::ProjectDirs;
use std::fs::create_dir_all;
use std::path::PathBuf;

pub fn get_data_dir() -> Option<PathBuf> {
    if let Some(proj_dirs) = ProjectDirs::from("dev", "GreyMatcha", "GuitarGaming") {
        let data_dir = proj_dirs.data_dir().to_path_buf();
        create_dir_all(&data_dir).ok()?;
        Some(data_dir)
    } else {
        None
    }
}