pub mod sound;

use rodio::{OutputStream, OutputStreamHandle, Sink};
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

/// Manages audio playback for the game
pub struct AudioManager {
    _stream: OutputStream,
    stream_handle: OutputStreamHandle,
    sinks: HashMap<String, Sink>,
    volume: f32,
}

impl AudioManager {
    pub fn new() -> Result<Self, String> {
        let (stream, stream_handle) = OutputStream::try_default()
            .map_err(|e| format!("Failed to create audio output stream: {}", e))?;

        Ok(Self {
            _stream: stream,
            stream_handle,
            sinks: HashMap::new(),
            volume: 1.0,
        })
    }

    pub fn play_sound(&mut self, path: &str) -> Result<(), String> {
        let file =
            File::open(path).map_err(|e| format!("Failed to open audio file {}: {}", path, e))?;
        let reader = BufReader::new(file);
        let sink = Sink::try_new(&self.stream_handle)
            .map_err(|e| format!("Failed to create audio sink: {}", e))?;

        sink.set_volume(self.volume);
        sink.append(
            rodio::Decoder::new(reader)
                .map_err(|e| format!("Failed to decode audio file: {}", e))?,
        );

        self.sinks.insert(path.to_string(), sink);
        Ok(())
    }

    pub fn set_volume(&mut self, volume: f32) {
        self.volume = volume.clamp(0.0, 1.0);
        for sink in self.sinks.values() {
            sink.set_volume(self.volume);
        }
    }

    pub fn stop_all(&mut self) {
        for sink in self.sinks.values() {
            sink.stop();
        }
        self.sinks.clear();
    }
}

impl Drop for AudioManager {
    fn drop(&mut self) {
        self.stop_all();
    }
}
