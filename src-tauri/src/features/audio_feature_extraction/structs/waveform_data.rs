use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
pub struct WaveformData {
    pub amplitudes: Vec<f32>,
    pub sample_interval: usize
}
