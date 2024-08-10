use bevy::prelude::*;
use cpal::SupportedStreamConfig;

use crate::helpers::input_device::AudioStream;

#[derive(Resource)]
#[derive(Default)]
pub struct InputDeviceResource {
    pub audio_stream_main: Option<AudioStream>,
    pub audio_stream_channels: Option<Vec<Option<AudioStream>>>,
}
