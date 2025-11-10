// src/animation.rs

//! This module defines the structures for managing sprite animations.
//!
//! It provides an `AnimationController` that can be used to add, set, and update
//! animations for an entity.

use sdl3::rect::Rect;
use std::collections::HashMap;

/// Represents a single animation clip, composed of a sequence of frames from a texture.
#[derive(Clone)]
pub struct Animation {
    /// The name of the texture in the `TextureManager` used for this animation.
    pub texture_name: String,
    /// A vector of `Rect`s, each defining a single frame on the texture's sprite sheet.
    pub frames: Vec<Rect>,
    /// The duration of each frame in game ticks (i.e., how many frames to wait before advancing).
    pub frame_duration: u32,
    /// Whether the animation should loop back to the beginning after it finishes.
    pub loops: bool,
}

/// Manages the animations for a single entity.
///
/// This controller holds all possible animations for an entity and tracks the
/// state of the currently playing animation.
#[derive(Clone)]
pub struct AnimationController {
    /// A map of all animations available to the entity, indexed by a unique name.
    animations: HashMap<String, Animation>,
    /// The name of the currently active animation, if any.
    current_animation: Option<String>,
    /// The index of the current frame within the active animation's `frames` vector.
    current_frame_index: usize,
    /// A timer that counts up to the `frame_duration` of the current animation.
    frame_timer: u32,
}

impl AnimationController {
    /// Creates a new, empty `AnimationController`.
    pub fn new() -> Self {
        Self {
            animations: HashMap::new(),
            current_animation: None,
            current_frame_index: 0,
            frame_timer: 0,
        }
    }

    /// Adds a new animation clip to the controller.
    ///
    /// # Arguments
    ///
    /// * `name` - A unique name to identify this animation.
    /// * `animation` - The `Animation` struct to add.
    pub fn add_animation(&mut self, name: String, animation: Animation) {
        self.animations.insert(name, animation);
    }

    /// Sets the currently playing animation by name.
    ///
    /// If the new animation is different from the current one, it resets the
    /// frame index and timer to start the new animation from the beginning.
    /// If the animation name does not exist, nothing happens.
    pub fn set_animation(&mut self, name: &str) {
        if self.current_animation.as_deref() != Some(name)
            && self.animations.contains_key(name) {
                self.current_animation = Some(name.to_string());
                self.current_frame_index = 0;
                self.frame_timer = 0;
            }
    }

    /// Updates the animation timer and advances the frame if necessary.
    ///
    /// This should be called once per game loop update.
    pub fn update(&mut self) {
        if let Some(current_anim_name) = &self.current_animation
            && let Some(animation) = self.animations.get(current_anim_name) {
                self.frame_timer += 1;
                if self.frame_timer >= animation.frame_duration {
                    self.frame_timer = 0;
                    self.current_frame_index += 1;
                    if self.current_frame_index >= animation.frames.len() {
                        if animation.loops {
                            self.current_frame_index = 0;
                        } else {
                            // If the animation doesn't loop, stay on the last frame.
                            self.current_frame_index = animation.frames.len() - 1;
                        }
                    }
                }
            }
    }

    /// Returns a reference to the `Rect` of the current animation frame.
    ///
    /// Returns `None` if no animation is currently playing or if the frame index is invalid.
    pub fn current_frame_rect(&self) -> Option<&Rect> {
        self.current_animation
            .as_ref()
            .and_then(|name| self.animations.get(name))
            .and_then(|anim| anim.frames.get(self.current_frame_index))
    }

    /// Returns the texture name of the current animation.
    ///
    /// Returns `None` if no animation is currently playing.
    pub fn current_texture_name(&self) -> Option<&str> {
        self.current_animation
            .as_ref()
            .and_then(|name| self.animations.get(name))
            .map(|anim| anim.texture_name.as_str())
    }

    /// Checks if an animation with the given name has been added to the controller.
    pub fn has_animation(&self, name: &str) -> bool {
        self.animations.contains_key(name)
    }
}
