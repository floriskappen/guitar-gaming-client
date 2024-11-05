use std::{path::PathBuf, process::Command};

use crate::features::audio_feature_extraction::constants::{SONIC_ANNOTATOR_BINARY, VAMP_PLUGINS_PATH};

pub fn get_tempo(resource_dir: PathBuf, file_path: PathBuf) -> Result<f64, String> {
    let binary_path = resource_dir.join(format!("binaries/sonic-annotator/{}", SONIC_ANNOTATOR_BINARY));
    let vamp_plugins_path = resource_dir.join(format!("binaries/vamp-plugins/{}", VAMP_PLUGINS_PATH));

    println!("going to detect tempo for {:?} with binary {:?}", file_path, binary_path);

    let output = Command::new(binary_path)
        .env("VAMP_PATH", vamp_plugins_path)
        .args(&[
            "-d", "vamp:qm-vamp-plugins:qm-tempotracker:tempo",
            file_path.to_str().unwrap(),
            "-w", "csv", "--csv-stdout"
        ])
        .output()
        .map_err(|e| e.to_string())?;

    if output.status.success() {
        let result = String::from_utf8_lossy(&output.stdout).to_string();
        let bpm = result.split(",").nth(2).unwrap().parse::<f64>().ok().unwrap();

        return Ok(bpm)
    }

    Err(String::from_utf8_lossy(&output.stderr).to_string())
}
