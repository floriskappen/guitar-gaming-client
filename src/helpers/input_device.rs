use std::{error::Error, sync::{Arc, Mutex}};

use aubio::{Onset, OnsetMode};
use cpal::{traits::{DeviceTrait, StreamTrait}, BufferSize, Device, SampleRate, StreamConfig};
use pitch_detection::detector::PitchDetector;
use pitch_detection::detector::autocorrelation::AutocorrelationDetector;

const POWER_THRESHOLD: f32 = 0.15;
const CLARITY_THRESHOLD: f32 = 0.6;

#[derive(Clone)]
pub struct AudioStream {
    pub buffer: Arc<Mutex<Vec<f32>>>,
    pub buffer_size: usize,
    pub target_channels: Vec<u16>,
    pub sample_rate: SampleRate,
}

impl AudioStream {
    pub fn new(
        device: Device,
        target_channels: Vec<u16>,
        buffer_size: usize,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let config = device.default_input_config()?;
        let channels = config.channels();
        let sample_rate = config.sample_rate();

        let buffer = Arc::new(Mutex::new(Vec::with_capacity(buffer_size)));
        let buffer_clone = Arc::clone(&buffer);

        let target_channels_clone = target_channels.clone();
        let stream = device.build_input_stream(
            &StreamConfig {
                buffer_size: BufferSize::Default,
                channels,
                sample_rate,
            },
            move |data: &[f32], _: &cpal::InputCallbackInfo| {
                let mut buffer = buffer_clone.lock().unwrap();

                if target_channels_clone.len() == channels as usize { // If all channels are selected, just take all data
                    buffer.extend(data.iter().take(data.len() / channels as usize));
                } else {
                    for frame in data.chunks(channels as usize) {
                        for &target_channel in &target_channels_clone {
                            buffer.push(frame[target_channel as usize]);
                        }
                    }
                }

                let buffer_clone = buffer.clone();

                // Maintain a rolling buffer of fixed size
                if buffer.len() > buffer_size {
                    buffer.drain(0..buffer_clone.len() - buffer_size);
                }
            },
            |err| eprintln!("an error occurred on the input audio stream: {}", err),
            None,
        )?;

        stream.play()?;

        Ok(AudioStream {
            buffer,
            buffer_size,
            target_channels,
            sample_rate,
        })
    }

    pub fn get_amplitude(&self) -> f32 {
        let samples = self.buffer.lock().unwrap();

        if samples.is_empty() {
            return 0.0;
        }

        // Apply a simple Hann window function
        let windowed_samples: Vec<f32> = samples
            .iter()
            .enumerate()
            .map(|(i, &x)| {
                let window_value = 0.5 * (1.0 - (2.0 * std::f32::consts::PI * i as f32 / (samples.len() - 1) as f32).cos());
                x * window_value
            })
            .collect();

        // Calculate weighted RMS
        let sum: f32 = windowed_samples.iter().map(|&x| x * x).sum();
        let mean = sum / windowed_samples.len() as f32;
         // Scale amplitude to a 0-100 range

        mean.sqrt() * 100.0
    }

    pub fn get_pitch(&self) -> Option<f32> {
        let signal = self.buffer.lock().unwrap().clone();
        if signal.len() < self.buffer_size {
            return None
        }

        let padding: usize = self.buffer_size / 2;

        let mut detector = AutocorrelationDetector::new(self.buffer_size, padding);
        if let Some(pitch) = detector.get_pitch(&signal, self.sample_rate.0 as usize, POWER_THRESHOLD, CLARITY_THRESHOLD) {
            return Some(pitch.frequency)
        }

        None
    }

    pub fn get_onset(&self) -> Result<bool, Box<dyn Error>> {
        let hop_size = self.buffer_size / 2;

        let mut onset = Onset::new(OnsetMode::SpecFlux, self.buffer_size, hop_size, self.sample_rate.0)?;
        onset.set_threshold(4.0);
        onset.set_silence(-30.0);

        
        let signal = self.buffer.lock().unwrap().clone();

        let mut onset_times: Vec<f32> = vec![];

        for chunk in signal.chunks(hop_size) {
            if onset.do_result(chunk)? > 0.0 {
                // Calculate the time of the onset (in seconds)
                let onset_time = onset.get_last_ms();
                if !onset_times.contains(&onset_time) {
                    onset_times.push(onset_time);
                    return Ok(true)
                }
            }
        }

        Ok(false)
    }
}
