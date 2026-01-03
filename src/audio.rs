//! # Manager: Audio Engine
//! 
//! This module acts as the authoritative manager for audio assets and 
//! hardware playback. It encapsulates the Kira audio backend, managing 
//! pre-loaded SFX, streaming music tracks, and asynchronous beat detection.

use crate::audio_analysis::{BeatDetector, DetectedBeat};
use crate::config::{GameConfig, SoundtrackConfig};
use kira::{
    manager::{AudioManager, AudioManagerSettings, backend::DefaultBackend},
    sound::{
        static_sound::{StaticSoundData, StaticSoundSettings},
        streaming::{StreamingSoundData, StreamingSoundSettings, StreamingSoundHandle},
        FromFileError,
    },
};
use std::collections::HashMap;
use std::path::Path;
use std::sync::mpsc;
use std::thread;

#[derive(Clone, Copy, Debug)] pub struct PlaySoundParams { pub volume: f64 }
impl Default for PlaySoundParams { fn default() -> Self { Self { volume: 1.0 } } }

pub enum AudioEvent { PlaySound(String, PlaySoundParams), PlayMusic(String, PlaySoundParams), StopMusic, FadeOutMusic(f64) }

pub struct GameAudioManager {
    manager: AudioManager<DefaultBackend>,
    sounds: HashMap<String, AudioAsset>,
    pub event_receiver: mpsc::Receiver<AudioEvent>,
    pub event_sender: mpsc::Sender<AudioEvent>,
    loading_receiver: mpsc::Receiver<LoadedMusic>,
    loading_sender: mpsc::Sender<LoadedMusic>,
    pub current_beat_map: Option<Vec<DetectedBeat>>,
    pub current_music_handle: Option<StreamingSoundHandle<FromFileError>>,
    pub current_music_name: Option<String>,
    soundtrack_properties: HashMap<String, SoundtrackConfig>,
}

impl GameAudioManager {
    pub fn new(game_config: &GameConfig) -> Result<Self, String> {
        let mut settings = AudioManagerSettings::<DefaultBackend>::default();
        settings.capacities.command_capacity = 256;
        let manager = AudioManager::<DefaultBackend>::new(settings).map_err(|e| format!("Failed to create Kira audio manager: {}", e))?;
        let (event_sender, event_receiver) = mpsc::channel();
        let (loading_sender, loading_receiver) = mpsc::channel();
        let mut sounds = HashMap::new();
        for (name, path) in &game_config.audio {
            if name.starts_with("soundtrack") { sounds.insert(name.clone(), AudioAsset::StreamingPath(path.clone())); } 
            else {
                let sound_data = StaticSoundData::from_file(Path::new(path), StaticSoundSettings::default()).map_err(|e| format!("Failed to load sound '{}': {}", path, e))?;
                sounds.insert(name.clone(), AudioAsset::Static(Box::new(sound_data)));
            }
        }
        Ok(Self { manager, sounds, event_receiver, event_sender, loading_receiver, loading_sender, current_beat_map: None, current_music_handle: None, current_music_name: None, soundtrack_properties: game_config.soundtrack_properties.clone() })
    }

    pub fn event_sender(&self) -> mpsc::Sender<AudioEvent> { self.event_sender.clone() }

    fn play_sound(&mut self, name: &str, params: PlaySoundParams) -> Result<(), String> {
        if let Some(AudioAsset::Static(sound_data)) = self.sounds.get(name) {
            let mut sound = *sound_data.clone();
            sound.settings.volume = kira::tween::Value::Fixed(kira::Volume::Amplitude(params.volume));
            self.manager.play(sound).map_err(|e| e.to_string())?;
            return Ok(());
        }
        Err(format!("Sound '{}' not found", name))
    }

    fn stop_music(&mut self) {
        if let Some(mut handle) = self.current_music_handle.take() { let _ = handle.stop(kira::tween::Tween { duration: std::time::Duration::from_millis(100), ..Default::default() }); }
        self.current_music_name = None; self.current_beat_map = None;
    }

    fn fade_out_music(&mut self, duration_seconds: f64) {
        if let Some(mut handle) = self.current_music_handle.take() { let _ = handle.stop(kira::tween::Tween { duration: std::time::Duration::from_secs_f64(duration_seconds), ..Default::default() }); }
        self.current_music_name = None; self.current_beat_map = None;
    }

    fn play_music(&mut self, name: &str, params: PlaySoundParams) -> Result<(), String> {
        if self.current_music_name.as_deref() == Some(name) && let Some(handle) = &mut self.current_music_handle { let _ = handle.seek_to(0.0); return Ok(()); }
        if let Some(AudioAsset::StreamingPath(path)) = self.sounds.get(name) {
            self.current_music_name = Some(name.to_string());
            let sender = self.loading_sender.clone();
            let path = path.clone(); let name = name.to_string();
            let target_bpm = self.soundtrack_properties.get(&name).and_then(|p| p.bpm);
            thread::spawn(move || {
                let result = StreamingSoundData::from_file(Path::new(&path), StreamingSoundSettings::default()).map_err(|e| format!("Failed to load streaming sound '{}': {}", path, e));
                let _ = sender.send(LoadedMusic::MusicReady { name: name.clone(), result: Box::new(result), params });
                if path.ends_with(".wav") && let Ok(b) = BeatDetector::analyze_beats(&path, target_bpm) { let _ = sender.send(LoadedMusic::BeatsReady(b)); }
            });
        }
        Ok(())
    }

    pub fn process_events(&mut self) {
        while let Ok(event) = self.event_receiver.try_recv() {
            match event {
                AudioEvent::PlaySound(name, p) => { let _ = self.play_sound(&name, p); }
                AudioEvent::PlayMusic(name, p) => { let _ = self.play_music(&name, p); }
                AudioEvent::StopMusic => self.stop_music(),
                AudioEvent::FadeOutMusic(d) => self.fade_out_music(d),
            }
        }
        while let Ok(loaded) = self.loading_receiver.try_recv() {
            match loaded {
                LoadedMusic::MusicReady { name: _, result, params } => {
                    if let Ok(mut sound) = *result {
                        if let Some(mut handle) = self.current_music_handle.take() { let _ = handle.stop(kira::tween::Tween { duration: std::time::Duration::from_millis(500), ..Default::default() }); }
                        sound.settings.volume = kira::tween::Value::Fixed(kira::Volume::Amplitude(params.volume));
                        sound.settings.loop_region = Some(kira::sound::Region::from(..));
                        if let Ok(handle) = self.manager.play(sound) { self.current_music_handle = Some(handle); self.current_beat_map = None; }
                    }
                },
                LoadedMusic::BeatsReady(beats) => self.current_beat_map = Some(beats),
            }
        }
    }
}

enum AudioAsset { Static(Box<StaticSoundData>), StreamingPath(String) }
enum LoadedMusic { MusicReady { #[allow(dead_code)] name: String, result: Box<Result<StreamingSoundData<FromFileError>, String>>, params: PlaySoundParams }, BeatsReady(Vec<DetectedBeat>) }