use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use symphonia::core::audio::{AudioBufferRef, Signal};
use symphonia::core::{codecs::CODEC_TYPE_NULL, io::MediaSourceStream};
use symphonia::default::get_codecs;

use crate::features::audio_feature_extraction::structs::waveform_data::WaveformData;

pub fn generate_waveform(audio_path: PathBuf) -> Result<WaveformData, Box<dyn std::error::Error>> {
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

    Ok(WaveformData { amplitudes, sample_interval: downsample_rate })
}

pub fn cache_waveform_by_song_uuid(app_data_directory: PathBuf, song_uuid: String, waveform_data: &WaveformData) -> Result<(), Box<dyn std::error::Error>> {
    let song_directory = app_data_directory.join(format!("songs/{}", song_uuid));
    let cache_path = song_directory.join("cache.json");
    let mut file = File::create(cache_path)?;
    let json_data = serde_json::to_string(waveform_data)?;
    file.write_all(json_data.as_bytes())?;

    Ok(())
}

pub fn get_cached_waveform_by_song_uuid(app_data_directory: PathBuf, song_uuid: String) -> Option<WaveformData> {
    let song_directory = app_data_directory.join(format!("songs/{}", song_uuid));
    let cache_path = song_directory.join("cache.json");
    if Path::new(&cache_path).exists() {
        let mut file = File::open(cache_path).ok()?;
        let mut json_data = String::new();
        file.read_to_string(&mut json_data).ok()?;
        return serde_json::from_str(&json_data).ok()
    } else {
        return None
    }
}

pub fn get_or_generate_waveform_by_song_uuid(app_data_directory: PathBuf, song_uuid: String) -> Result<WaveformData, Box<dyn std::error::Error>> {
    if let Some(waveform_data) = get_cached_waveform_by_song_uuid(app_data_directory.clone(), song_uuid.clone()) {
        return Ok(waveform_data);
    }

    let audio_path = app_data_directory.join(format!("songs/{}/audio.mp3", song_uuid.clone()));
    let waveform_data = generate_waveform(audio_path).unwrap();
    cache_waveform_by_song_uuid(app_data_directory, song_uuid, &waveform_data).unwrap();

    return Ok(waveform_data);
}
