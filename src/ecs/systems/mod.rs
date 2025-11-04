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

pub trait System<T> {
    fn update(&mut self, world: &mut World, context: &mut T);
}

pub struct SystemContext<'a> {
    pub level: &'a Level,
    pub input_state: &'a InputState,
    pub config: &'a Config,
    pub game_config: &'a GameConfig,
    pub audio_sender: &'a Sender<AudioEvent>,
    pub gold_coin_count: &'a mut u32,
    pub next_level: &'a mut Option<String>,
}

pub struct RespawnSystemContext<'a> {
    pub camera: &'a mut Camera,
    pub game_config: &'a GameConfig,
}
