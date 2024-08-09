use bevy::prelude::*;

use crate::helpers::input_device::AudioStream;

#[derive(Resource)]
#[derive(Default)]
pub struct InputDevice {
    pub audio_stream: Option<AudioStream>,

    pub audio_stream_left: Option<AudioStream>,
    pub audio_stream_right: Option<AudioStream>,
}
