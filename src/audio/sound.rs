use rodio::{Decoder, OutputStreamHandle, Sink, Source};
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};

pub struct SoundEffect {
    name: String,
    path: PathBuf,
    volume: f32,
}

impl SoundEffect {
    pub fn new<S: Into<String>>(name: S, path: S) -> Self {
        Self {
            name: name.into(),
            path: PathBuf::from(path.into()),
            volume: 1.0,
        }
    }

    pub fn with_volume(mut self, volume: f32) -> Self {
        self.volume = volume.clamp(0.0, 1.0);
        self
    }
}

/// Manages sound effects and their playback
pub struct SoundManager {
    stream_handle: OutputStreamHandle,
    effects: HashMap<String, SoundEffect>,
    active_sinks: HashMap<String, Sink>,
    master_volume: f32,
}

impl SoundManager {
    pub fn new(stream_handle: OutputStreamHandle) -> Self {
        Self {
            stream_handle,
            effects: HashMap::new(),
            active_sinks: HashMap::new(),
            master_volume: 1.0,
        }
    }

    pub fn register_sound<S: Into<String>>(&mut self, name: S, path: S) {
        let effect = SoundEffect::new(name.into(), path.into());
        self.effects.insert(effect.name.clone(), effect);
    }

    pub fn play_sound(&mut self, name: &str) -> Result<(), String> {
        let effect = self
            .effects
            .get(name)
            .ok_or_else(|| format!("Sound effect '{}' not found", name))?;

        // Create a new sink for this playback
        let sink = Sink::try_new(&self.stream_handle)
            .map_err(|e| format!("Failed to create audio sink: {}", e))?;

        let file =
            File::open(&effect.path).map_err(|e| format!("Failed to open audio file: {}", e))?;
        let source = Decoder::new(BufReader::new(file))
            .map_err(|e| format!("Failed to decode audio file: {}", e))?;

        sink.set_volume(effect.volume * self.master_volume);
        sink.append(source);

        // Store the sink
        self.active_sinks.insert(name.to_string(), sink);

        Ok(())
    }

    pub fn stop_sound(&mut self, name: &str) {
        if let Some(sink) = self.active_sinks.remove(name) {
            sink.stop();
        }
    }

    pub fn set_master_volume(&mut self, volume: f32) {
        self.master_volume = volume.clamp(0.0, 1.0);
        for sink in self.active_sinks.values() {
            sink.set_volume(self.master_volume);
        }
    }

    pub fn cleanup(&mut self) {
        // Stop and remove all active sinks
        for sink in self.active_sinks.values() {
            sink.stop();
        }
        self.active_sinks.clear();
    }
}

pub struct MusicPlayer {
    stream_handle: OutputStreamHandle,
    current_track: Option<Sink>,
    volume: f32,
}

impl MusicPlayer {
    pub fn new(stream_handle: OutputStreamHandle) -> Self {
        Self {
            stream_handle,
            current_track: None,
            volume: 0.3,
        }
    }

    pub fn play_music<P: AsRef<Path>>(&mut self, path: P) -> Result<(), String> {
        // Stop any currently playing music
        if let Some(sink) = &self.current_track {
            sink.stop();
        }

        let sink = Sink::try_new(&self.stream_handle)
            .map_err(|e| format!("Failed to create audio sink: {}", e))?;

        // Load and decode the music file
        let file = File::open(path).map_err(|e| format!("Failed to open music file: {}", e))?;
        let source = Decoder::new(BufReader::new(file))
            .map_err(|e| format!("Failed to decode music file: {}", e))?;

        let source = source.repeat_infinite();

        sink.set_volume(self.volume);
        sink.append(source);

        self.current_track = Some(sink);
        Ok(())
    }

    pub fn stop_music(&mut self) {
        if let Some(sink) = &self.current_track {
            sink.stop();
        }
        self.current_track = None;
    }

    pub fn set_volume(&mut self, volume: f32) {
        self.volume = volume.clamp(0.0, 1.0);
        if let Some(sink) = &self.current_track {
            sink.set_volume(self.volume);
        }
    }
}
