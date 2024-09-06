use std::{fs::File, io::BufReader, sync::mpsc::{self, Sender}, thread};

use bevy::prelude::*;
use rodio::{Decoder, OutputStream, Sink};

#[derive(Clone)]
pub enum AudioCommand {
    Play(String),
    Pause,
    Resume,
    Stop,
}

#[derive(Resource)]
pub struct OutputAudioControllerSong {
    pub sender: Sender<AudioCommand>,
    pub thread_handle: Option<thread::JoinHandle<()>>,
}

pub fn output_audio_song_load(mut commands: Commands) {
    // Start the audio thread
    let (sender, receiver) = mpsc::channel::<AudioCommand>();

    let thread_handle = thread::spawn(move || {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let mut sink: Option<Sink> = None;

        // Audio thread main loop
        while let Ok(command) = receiver.recv() {
            match command {
                AudioCommand::Play(file_path) => {
                    // Load the file and start playing
                    if let Ok(file) = File::open(file_path) {
                        let source = Decoder::new(BufReader::new(file)).unwrap();
                        let new_sink = Sink::try_new(&stream_handle).unwrap();
                        new_sink.append(source);
                        sink = Some(new_sink);
                    }
                }
                AudioCommand::Pause => {
                    if let Some(s) = &sink {
                        s.pause(); // Pause the current sink
                    }
                }
                AudioCommand::Resume => {
                    if let Some(s) = &sink {
                        s.play(); // Resume playback from the paused position
                    }
                }
                AudioCommand::Stop => {
                    println!("Stopping audio thread");
                    if let Some(s) = &sink {
                        s.stop()
                    }
                    break;
                }
            }
        }
    });

    // Store the audio controller in a Bevy resource
    commands.insert_resource(OutputAudioControllerSong {
        sender,
        thread_handle: Some(thread_handle),
    });
}

pub fn output_audio_song_cleanup(
    mut commands: Commands, 
    mut output_audio_controller_song: ResMut<OutputAudioControllerSong>
) {
    // Stop the audio thread
    if let Some(thread_handle) = output_audio_controller_song.thread_handle.take() {
        if output_audio_controller_song.sender.send(AudioCommand::Stop).is_ok() {
            thread_handle.join().unwrap(); // Ensure the thread has finished
        }
    }

    // Remove the OutputAudioControllerSong resource
    commands.remove_resource::<OutputAudioControllerSong>();
}
