//! This module defines the common traits and context structs for all ECS systems.

pub mod animation_update;
pub mod audio;
pub mod coin_collection;
pub mod death;
pub mod input;
pub mod interaction;
pub mod kill;
pub mod physics;
pub mod player_animation;
pub mod respawn;
pub mod respawn_timer;
pub mod state_machine;
pub mod tile_collision;
pub mod invincibility;
pub mod player_death;
pub mod lifetime;
pub mod level_transition;

use crate::config::{Config, GameConfig};
use crate::input::InputState;
use crate::level::Level;
use crate::ecs::world::World;
use std::sync::mpsc::Sender;
use crate::audio::AudioEvent;
use crate::camera::Camera;

/// A generic trait for all systems in the ECS.
///
/// Each system is responsible for a specific piece of game logic and operates
/// on entities that have a certain set of components.
pub trait System<T> {
    /// Updates the system's logic for the current frame.
    ///
    /// # Arguments
    ///
    /// * `world` - A mutable reference to the `World` containing all entities and components.
    /// * `context` - A mutable reference to a context struct providing access to shared resources.
    fn update(&mut self, world: &mut World, context: &mut T);
}

/// A context struct that provides shared resources to most systems.
///
/// This allows systems to access global state like configuration, input,
/// and level data without needing to own them.
pub struct SystemContext<'a> {
    /// A reference to the current level data.
    pub level: &'a Level,
    /// The current state of user input.
    pub input_state: &'a InputState,
    /// The global application configuration.
    pub config: &'a Config,
    /// The game-specific configuration.
    pub game_config: &'a GameConfig,
    /// A sender for dispatching audio events.
    pub audio_sender: &'a Sender<AudioEvent>,
    /// A mutable reference to the player's gold coin count.
    pub gold_coin_count: &'a mut u32,
    /// A mutable reference that can be set to trigger a level transition.
    pub next_level: &'a mut Option<String>,
}

/// A specialized context struct for the `RespawnSystem`.
///
/// This provides the `RespawnSystem` with the specific resources it needs,
/// namely mutable access to the camera.
pub struct RespawnSystemContext<'a> {
    /// A mutable reference to the game camera.
    pub camera: &'a mut Camera,
    /// The game-specific configuration.
    pub game_config: &'a GameConfig,
}
