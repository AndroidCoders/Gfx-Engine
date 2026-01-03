//! # Concept: ECS Resources
//! 
//! This module defines the 'Global State' of the ECS. These are singleton 
//! data structures that can be accessed by any system, providing a shared 
//! context for engine-wide properties and configurations.

use crate::math::Vector2D;



/// The high-level state of the game application, driving the main system scheduler.

#[derive(Debug, Clone, Copy, PartialEq)]

pub enum GameState { Menu(Screen), Playing, Paused, GameOver, Cinematic }

#[derive(Debug, Clone, Copy, PartialEq)]

pub enum Screen { Main, Options, Credits, CharacterSelect, Editor }



impl Default for GameState { fn default() -> Self { GameState::Menu(Screen::Main) } }



/// A spatial hash grid for accelerating collision detection and visibility queries.

#[derive(Default)]

pub struct SpatialGrid {

    entities: Vec<crate::ecs::world::Entity>,

}



impl SpatialGrid {

    pub fn new(_cell_size: f32) -> Self { Self::default() }

    pub fn clear(&mut self) { self.entities.clear(); }

    pub fn insert(&mut self, entity: crate::ecs::world::Entity, _rect: sdl3::rect::Rect) {

        self.entities.push(entity);

    }

    // TEMPORARY FIX: Return all entities, bypassing spatial query logic.

    pub fn query(&self, _rect: sdl3::rect::Rect) -> Vec<crate::ecs::world::Entity> { 

        self.entities.clone()

    }

}



/// Persistent gameplay statistics that survive across level transitions.

#[derive(Debug, Clone, Default)]

pub struct GameStats {

    pub lives: u32,

    pub gold_coin_count: u32,

}



/// The visual state of the HUD, decoupled for juice effects like score counters.

#[derive(Debug, Clone, Default)]

pub struct UIState {

    pub display_lives: u32,

    pub display_coin_count: u32,

}



/// A debug resource for tracking critical runtime state.

#[derive(Default, Clone, Copy)]

pub struct FrameDebugInfo {

    pub player_pos: Option<Vector2D>,

    pub player_prev_pos: Option<Vector2D>,

    pub player_render_w: u32,

    pub player_render_h: u32,

    pub camera_pos: Option<Vector2D>,

    pub renderable_count: usize,

}
