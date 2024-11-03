use std::{path::PathBuf, process::Command};

#[cfg(all(target_os = "windows", target_pointer_width = "64"))]
const SONIC_ANNOTATOR_BINARY: &str = "win64/sonic-annotator-win64.exe";
#[cfg(all(target_os = "windows", target_pointer_width = "64"))]
const VAMP_PLUGINS_PATH: &str = "win64/";

#[cfg(all(target_os = "windows", target_pointer_width = "32"))]
const SONIC_ANNOTATOR_BINARY: &str = "win32/sonic-annotator-win32.exe";
#[cfg(all(target_os = "windows", target_pointer_width = "32"))]
const VAMP_PLUGINS_PATH: &str = "win32/";

#[cfg(all(target_os = "macos"))]
const SONIC_ANNOTATOR_BINARY: &str = "sonic-annotator-macos";
#[cfg(all(target_os = "macos"))]
const VAMP_PLUGINS_PATH: &str = "macos/";

#[cfg(all(target_os = "linux"))]
const SONIC_ANNOTATOR_BINARY: &str = "sonic-annotator-linux";
#[cfg(all(target_os = "linux"))]
const SONIC_ANNOTATOR_BINARY: &str = "linux/";

fn extract_bpm(result_string: String) -> Option<f64> {
    println!("str: {}", result_string);
    // Split the string by commas to get each part
    let parts: Vec<&str> = result_string.split(',').collect();
    
    println!("parts: {:?}", parts);
    // The last part contains the BPM with units, so split by space to separate the number
    let bpm_part = parts.last()?.split_whitespace().next()?;
    println!("bpm_part: {:?}", bpm_part);

    // Parse the bpm_part as a floating-point number
    bpm_part.parse::<f64>().ok()
}

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
