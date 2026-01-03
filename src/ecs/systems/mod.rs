//! # Manager: System Infrastructure
//! 
//! This module defines the architectural backbone for the ECS Systems. 
//! It provides the 'System' trait and the shared 'Context' structs that 
//! grant systems safe, scoped access to global engine resources.

pub mod movement;
pub mod physics;
pub mod animation_update;
pub mod input;
pub mod tile_collision;
pub mod gui_render;
pub mod debug_render;
pub mod game_flow;
pub mod transition;
pub mod interaction;
pub mod concept_health;
pub mod concept_vitality;
pub mod rule_player_death;
pub mod rule_respawn;
pub mod game_resolution;
pub mod audio_synchronization;
pub mod level_transition;
pub mod spatial_update;
pub mod enemy_rhythm;
pub mod state_machine;
pub mod audio;
pub mod camera_shake;
pub mod animation_synchronization;
pub mod dormancy;
pub mod menu;
pub mod synchronization;

use crate::ecs::world::World;
use crate::config::{Config, GameConfig};
use crate::camera::Camera;
use crate::level::Level;
use crate::input::InputState;
use std::sync::mpsc::Sender;
use crate::audio::AudioEvent;
use crate::benchmarker::Benchmarker;

pub struct SystemContext<'a> {
    pub config: &'a Config,
    pub game_config: &'a GameConfig,
    pub delta_time: f32,
    pub camera: &'a mut Camera,
    pub audio_sender: &'a Sender<AudioEvent>,
    pub is_paused: bool,
    pub is_attract_mode: bool,
    pub benchmarker: &'a mut Benchmarker,
    pub level: &'a Level,
    pub input_state: &'a InputState,
    pub next_level: &'a mut Option<String>,
    pub current_soundtrack: Option<String>,
}

pub trait System<Context> {
    fn update(&mut self, world: &mut World, context: &mut Context);
}

/// # Concept: Rhythm Context
/// 
/// A specialized resource bundle for the Enemy Rhythm system, focusing 
/// exclusively on temporal and spatial data required for AI orchestration.
#[allow(dead_code)]
pub struct EnemyRhythmContext<'a> {
    pub game_config: &'a GameConfig,
    pub delta_time: f32,
    pub camera: &'a Camera,
}

/// # Concept: Render Context
/// 
/// A read-only resource bundle provided to visualization systems during 
/// the draw phase. It prevents logic updates during rendering.
pub struct RenderContext<'a> {
    pub config: &'a Config,
    pub game_config: &'a GameConfig,
    pub player_entity: Option<crate::ecs::world::Entity>,
    pub benchmarker: &'a crate::benchmarker::Benchmarker,
}