use std::fs::File;

use symphonia::core::audio::{AudioBufferRef, Signal};
use symphonia::core::{codecs::CODEC_TYPE_NULL, io::MediaSourceStream};
use symphonia::default::get_codecs;

use crate::features::audio_feature_extraction::structs::waveform_data::WaveformData;

pub fn generate_waveform(audio_path: &str) -> Result<WaveformData, Box<dyn std::error::Error>> {
    // Open the media source
    let file = File::open(audio_path)?;
    let mss = MediaSourceStream::new(Box::new(file), Default::default());
    let hint = symphonia::core::probe::Hint::new();

    // Probing the file to find the format
    let probed = symphonia::default::get_probe().format(&hint, mss, &Default::default(), &Default::default())?;
    let mut format = probed.format;

    // Find the first audio track
    let track = format.tracks().iter().find(|t| t.codec_params.codec != CODEC_TYPE_NULL).unwrap();

    // Decode the audio track
    let mut decoder = get_codecs().make(&track.codec_params, &Default::default())?;
    let mut amplitudes = Vec::new();
    let downsample_rate = 500; // Reduce resolution by a factor to save on data points

    while let Ok(packet) = format.next_packet() {
        // Decode the packet into audio frames
        if let Ok(audio_buffer) = decoder.decode(&packet) {
            if let AudioBufferRef::F32(buf) = audio_buffer {
                for (i, &sample) in buf.chan(0).iter().enumerate() {
                    // Downsample by taking every `downsample_rate` sample
                    if i % downsample_rate == 0 {
                        amplitudes.push(sample.abs()); // Take absolute value for amplitude
                    }
                }
            }
        }
    }

    Ok(WaveformData { amplitudes, waveform_sample_rate: downsample_rate })
}