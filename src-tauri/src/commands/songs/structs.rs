use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Song {
    pub uuid: String,
    pub title: Option<String>,
    pub artists: Option<Vec<String>>,
    pub tuning: Option<Vec<String>>,
    pub duration_seconds: Option<f32>,
    pub bpm: Option<f32>,
}