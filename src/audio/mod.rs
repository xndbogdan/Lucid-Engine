use rodio::{OutputStream, OutputStreamHandle, Sink};
use std::collections::HashMap;

pub mod sound;
pub use sound::{MusicPlayer, SoundEffect, SoundManager};

/// Manages all audio playback in the game
pub struct AudioManager {
    _stream: OutputStream,
    stream_handle: OutputStreamHandle,
    sinks: HashMap<String, Sink>,
    volume: f32,
}

impl AudioManager {
    /// Create a new audio manager
    pub fn new() -> Result<Self, String> {
        let (stream, stream_handle) = OutputStream::try_default()
            .map_err(|e| format!("Failed to open audio output stream: {}", e))?;

        Ok(Self {
            _stream: stream,
            stream_handle,
            sinks: HashMap::new(),
            volume: 1.0,
        })
    }

    /// Play a sound effect from a file
    pub fn play_sound(&mut self, path: &str) -> Result<(), String> {
        let sink = Sink::try_new(&self.stream_handle)
            .map_err(|e| format!("Failed to create audio sink: {}", e))?;

        // Load and decode the audio file
        let file = std::fs::File::open(path)
            .map_err(|e| format!("Failed to open audio file {}: {}", path, e))?;
        let source = rodio::Decoder::new(std::io::BufReader::new(file))
            .map_err(|e| format!("Failed to decode audio file {}: {}", path, e))?;

        // Play the sound
        sink.append(source);
        sink.set_volume(self.volume);
        self.sinks.insert(path.to_string(), sink);

        Ok(())
    }

    /// Set the global volume for all sounds
    pub fn set_volume(&mut self, volume: f32) {
        self.volume = volume.clamp(0.0, 1.0);
        for sink in self.sinks.values() {
            sink.set_volume(self.volume);
        }
    }
}
