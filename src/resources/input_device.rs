use bevy::prelude::*;
use cpal::SupportedStreamConfig;

use crate::helpers::input_device::AudioStream;

#[derive(Resource)]
#[derive(Default)]
pub struct InputDevice {
    pub audio_stream: Option<AudioStream>,
    pub configuration: Option<SupportedStreamConfig>,

    pub audio_stream_left: Option<AudioStream>,
    pub audio_stream_right: Option<AudioStream>,
}
