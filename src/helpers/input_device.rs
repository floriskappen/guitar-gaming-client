use std::sync::{Arc, Mutex};

use cpal::{traits::{DeviceTrait, StreamTrait}, BufferSize, Device, StreamConfig};

use crate::resources::configuration::DeviceChannel;

pub struct AudioStream {
    pub buffer: Arc<Mutex<Vec<f32>>>,
    pub channel: DeviceChannel,
    buffer_size: usize,
}

impl AudioStream {
    pub fn new(
        device: Device,
        channel: DeviceChannel,
        buffer_size: usize,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let config = device.default_input_config()?;
        let channels = config.channels();
        let sample_rate = config.sample_rate();
        let buffer = Arc::new(Mutex::new(Vec::with_capacity(buffer_size)));

        let buffer_clone = Arc::clone(&buffer);
        let channel_clone = channel.clone();
        let stream = device.build_input_stream(
            &StreamConfig {
                buffer_size: BufferSize::Default,
                channels,
                sample_rate,
            },
            move |data: &[f32], _: &cpal::InputCallbackInfo| {
                let mut buffer = buffer_clone.lock().unwrap();

                if channel_clone == DeviceChannel::One {
                    for frame in data.chunks(channels as usize) {
                        buffer.push(frame[0]); // Left channel
                    }
                } else if channel_clone == DeviceChannel::Two {
                    for frame in data.chunks(channels as usize) {
                        buffer.push(frame[1]); // Right channel
                    }
                } else {
                    buffer.extend(data.iter().take(data.len() / channels as usize));
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
            channel,
            buffer_size,
        })
    }

    pub fn get_current_amplitude(&self) -> f32 {
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
}