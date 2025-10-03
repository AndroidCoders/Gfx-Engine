// src/animation.rs

//! Defines the structures for managing sprite animations.

use sdl3::rect::Rect;
use std::collections::HashMap;

/// Represents a single animation clip.
#[derive(Clone)]
pub struct Animation {
    pub texture_name: String,
    pub frames: Vec<Rect>,
    pub frame_duration: u32, // Duration of each frame in game ticks
    pub loops: bool,
}

/// Manages the animations for an entity.
pub struct AnimationController {
    animations: HashMap<String, Animation>,
    current_animation: Option<String>,
    current_frame_index: usize,
    frame_timer: u32,
}

impl AnimationController {
    /// Creates a new, empty AnimationController.
    pub fn new() -> Self {
        Self {
            animations: HashMap::new(),
            current_animation: None,
            current_frame_index: 0,
            frame_timer: 0,
        }
    }

    /// Adds an animation to the controller.
    pub fn add_animation(&mut self, name: String, animation: Animation) {
        self.animations.insert(name, animation);
    }

    /// Sets the currently playing animation.
    /// If the new animation is different from the current one, it resets the frame index and timer.
    pub fn set_animation(&mut self, name: &str) {
        if self.current_animation.as_deref() != Some(name) {
            if self.animations.contains_key(name) {
                self.current_animation = Some(name.to_string());
                self.current_frame_index = 0;
                self.frame_timer = 0;
            }
        }
    }

    /// Updates the animation timer and advances the frame if necessary.
    /// This should be called once per game loop update.
    pub fn update(&mut self) {
        if let Some(current_anim_name) = &self.current_animation {
            if let Some(animation) = self.animations.get(current_anim_name) {
                self.frame_timer += 1;
                if self.frame_timer >= animation.frame_duration {
                    self.frame_timer = 0;
                    self.current_frame_index += 1;
                    if self.current_frame_index >= animation.frames.len() {
                        if animation.loops {
                            self.current_frame_index = 0;
                        } else {
                            self.current_frame_index = animation.frames.len() - 1; // Stay on last frame
                        }
                    }
                }
            }
        }
    }

    /// Returns the rectangle of the current animation frame.
    /// Returns None if no animation is playing.
    pub fn current_frame_rect(&self) -> Option<&Rect> {
        self.current_animation
            .as_ref()
            .and_then(|name| self.animations.get(name))
            .and_then(|anim| anim.frames.get(self.current_frame_index))
    }

    /// Returns the texture name of the current animation.
    pub fn current_texture_name(&self) -> Option<&str> {
        self.current_animation
            .as_ref()
            .and_then(|name| self.animations.get(name))
            .map(|anim| anim.texture_name.as_str())
    }
}
