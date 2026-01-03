//! # Concept: Sprite Animation Logic
//!
//! This module provides the temporal progression for visuals. It defines 
//! the 'Animation' clip and the 'AnimationController', which advances 
//! frames based on game time and handles looping/clamping logic.

use sdl3::rect::Rect;
use std::collections::HashMap;

/// A sequence of frames mapped to a single texture.
#[derive(Clone)]
pub struct Animation {
    pub texture_name: String,
    pub frames: Vec<Rect>,
    pub frame_duration: u32,
    pub loops: bool,
}

/// A stateful controller that tracks the temporal position of an animation.
#[derive(Clone, Default)]
pub struct AnimationController {
    animations: HashMap<String, Animation>,
    current_animation: Option<String>,
    current_frame_index: usize,
    frame_timer: f32,
}

impl AnimationController {
    pub fn new() -> Self { Self::default() }

    pub fn add_animation(&mut self, name: String, animation: Animation) {
        self.animations.insert(name, animation);
    }

    pub fn current_frame_rect(&self) -> Option<&Rect> {
        self.current_animation.as_ref()
            .and_then(|name| self.animations.get(name))
            .and_then(|anim| anim.frames.get(self.current_frame_index))
    }

    pub fn current_texture_name(&self) -> Option<&str> {
        self.current_animation.as_ref()
            .and_then(|name| self.animations.get(name))
            .map(|anim| anim.texture_name.as_str())
    }

    pub fn current_animation_name(&self) -> Option<&str> { self.current_animation.as_deref() }

    pub fn has_animation(&self, name: &str) -> bool { self.animations.contains_key(name) }

    /// Advances the internal timers and frame indices for the active clip.
    pub fn update(&mut self, delta_time: f32) {
        if let Some(current_anim_name) = &self.current_animation
            && let Some(animation) = self.animations.get(current_anim_name) {
                
                self.frame_timer += delta_time;
                let duration_seconds = animation.frame_duration as f32 / 60.0;
                
                if self.frame_timer >= duration_seconds {
                    self.frame_timer -= duration_seconds; 
                    self.current_frame_index += 1;
                    
                    if self.current_frame_index >= animation.frames.len() {
                        if animation.loops { self.current_frame_index = 0; } 
                        else { self.current_frame_index = animation.frames.len() - 1; }
                    }
                }
            }
    }

    /// Sets the currently active animation, resetting the playback position.
    pub fn set_animation(&mut self, name: &str) {
        if self.current_animation.as_deref() != Some(name)
            && self.animations.contains_key(name) {
                self.current_animation = Some(name.to_string());
                self.current_frame_index = 0;
                self.frame_timer = 0.0;
            }
    }
}