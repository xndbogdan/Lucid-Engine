use rodio::{Decoder, OutputStreamHandle, Sink, Source};
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

pub struct SoundEffect {
    name: String,
    path: String,
    volume: f32,
}

impl SoundEffect {
    pub fn new<S: Into<String>>(name: S, path: S) -> Self {
        Self {
            name: name.into(),
            path: path.into(),
            volume: 1.0,
        }
    }

    pub fn with_volume(mut self, volume: f32) -> Self {
        self.volume = volume;
        self
    }
}

pub struct SoundManager {
    effects: HashMap<String, SoundEffect>,
    stream_handle: OutputStreamHandle,
    active_sinks: HashMap<String, Sink>,
    master_volume: f32,
}

impl SoundManager {
    pub fn new(stream_handle: OutputStreamHandle) -> Self {
        Self {
            effects: HashMap::new(),
            stream_handle,
            active_sinks: HashMap::new(),
            master_volume: 1.0,
        }
    }

    pub fn register_sound<S: Into<String>>(&mut self, name: S, path: S) {
        let effect = SoundEffect::new(name.into(), path.into());
        self.effects.insert(effect.name.clone(), effect);
    }

    pub fn play_sound(&mut self, name: &str) -> Result<(), String> {
        if let Some(effect) = self.effects.get(name) {
            let file = File::open(&effect.path)
                .map_err(|e| format!("Failed to open sound file: {}", e))?;
            let reader = BufReader::new(file);
            let source =
                Decoder::new(reader).map_err(|e| format!("Failed to decode sound file: {}", e))?;

            let sink = Sink::try_new(&self.stream_handle)
                .map_err(|e| format!("Failed to create audio sink: {}", e))?;

            sink.set_volume(effect.volume * self.master_volume);
            sink.append(source);

            self.active_sinks.insert(name.to_string(), sink);
            Ok(())
        } else {
            Err(format!("Sound effect '{}' not found", name))
        }
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
        self.active_sinks.retain(|_, sink| !sink.empty());
    }
}

pub struct MusicPlayer {
    current_track: Option<String>,
    sink: Option<Sink>,
    stream_handle: OutputStreamHandle,
    volume: f32,
}

impl MusicPlayer {
    pub fn new(stream_handle: OutputStreamHandle) -> Self {
        Self {
            current_track: None,
            sink: None,
            stream_handle,
            volume: 0.7,
        }
    }

    pub fn play_music<P: AsRef<Path>>(&mut self, path: P) -> Result<(), String> {
        let file = File::open(path).map_err(|e| format!("Failed to open music file: {}", e))?;
        let reader = BufReader::new(file);
        let source =
            Decoder::new(reader).map_err(|e| format!("Failed to decode music file: {}", e))?;

        // Stop current music if playing
        self.stop_music();

        let sink = Sink::try_new(&self.stream_handle)
            .map_err(|e| format!("Failed to create music sink: {}", e))?;

        sink.set_volume(self.volume);
        sink.append(source.repeat_infinite());

        self.sink = Some(sink);
        Ok(())
    }

    pub fn stop_music(&mut self) {
        if let Some(sink) = self.sink.take() {
            sink.stop();
        }
        self.current_track = None;
    }

    pub fn set_volume(&mut self, volume: f32) {
        self.volume = volume.clamp(0.0, 1.0);
        if let Some(sink) = &self.sink {
            sink.set_volume(self.volume);
        }
    }
}
