use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
pub struct WaveformData {
    pub amplitudes: Vec<f32>,
    pub waveform_sample_rate: usize
}
