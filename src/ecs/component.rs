//! # Concept: Component Definitions
//! 
//! This module defines the 'C' in ECS—the raw data structures that represent
//! entity properties. Each component is a pure data struct, designed to be
//! stored in high-performance Structure of Arrays (SoA) containers.

use crate::animation::AnimationController;
use crate::math::Vector2D;
use sdl3::rect::Rect;
use crate::audio_analysis::DetectedBeat;

/// Marker trait for all data containers in the ECS.
#[allow(dead_code)]
pub trait Component {}

/// # Concept: Global Music State
/// Tracks the temporal position and rhythmic facts of the active soundtrack.
#[derive(Debug, Clone, Default)]
pub struct MusicState {
    pub current_time: f64,
    pub last_beat: Option<DetectedBeat>,
}

/// # Concept: Position
/// The authoritative world-space coordinates of an entity.
#[derive(Debug, Clone, Copy, Default)]
pub struct Position(pub Vector2D);
impl Component for Position {}

/// # Concept: Velocity
/// The rate of change of an entity's position in world units per second.
#[derive(Debug, Clone, Copy, Default)]
pub struct Velocity(pub Vector2D);
impl Component for Velocity {}

/// # Concept: Renderable
/// Visual metadata required to draw an entity to the screen.
#[derive(Clone)]
pub struct Renderable {
    pub width: u32,
    pub height: u32,
    pub horizontal_offset: i32,
    pub vertical_offset: i32,
    pub z_index: u8,
    pub rotation: f64,
    pub flip_horizontal: bool,
    pub flip_vertical: bool,
}
impl Component for Renderable {}

/// # Concept: Collision
/// The physical bounding box used for environment and entity interactions.
#[derive(Debug, Clone, Copy)]
pub struct Collision {
    pub rect: Rect,
}
impl Component for Collision {}

/// # Concept: Health
/// The vitality state of an entity, determining its mortality.
#[derive(Debug, Clone, Copy)]
pub struct Health {
    pub current: u32,
    pub max: u32,
}
impl Component for Health {}

/// # Concept: Movement Intention
/// The abstract horizontal direction (-1.0 to 1.0) an entity intends to move.
#[derive(Debug, Clone, Copy, Default)]
pub struct MovementIntention {
    pub x: f32, 
}
impl Component for MovementIntention {}

// --- Tag Components (ZSTs) ---
#[derive(Debug, Clone, Copy)] pub struct PlayerTag;
impl Component for PlayerTag {}

#[derive(Debug, Clone, Copy)] pub struct EnemyTag;
impl Component for EnemyTag {}

#[derive(Debug, Clone, Copy)] pub struct GoldCoin;
impl Component for GoldCoin {}

#[derive(Debug, Clone, Copy)] pub struct Gravity;
impl Component for Gravity {}

#[derive(Debug, Clone, Copy)] pub struct Grounded;
impl Component for Grounded {}

#[derive(Debug, Clone, Copy)] pub struct DeadTag;
impl Component for DeadTag {}

#[derive(Debug, Clone, Copy, Default)] pub struct DormantTag;
impl Component for DormantTag {}

// --- Stateful Components ---
#[derive(Clone)]
pub struct Animation { pub controller: AnimationController }
impl Component for Animation {}

pub struct StateComponent { pub state_machine: crate::state_machine::StateMachine }
impl Component for StateComponent {}

#[derive(Debug, Clone)]
pub struct Patrol {
    pub speed: f32,
    pub anim_prefix: String,
    pub direction: f32,
}
impl Component for Patrol {}

#[derive(Debug, Clone, Copy)]
pub struct Invincibility { pub timer: f32 }
impl Component for Invincibility {}

#[derive(Debug, Clone, Copy)]
pub struct Lifetime { pub timer: f32 }
impl Component for Lifetime {}

#[derive(Debug, Clone, Copy)]
pub struct WallHit { pub normal_x: f32 }
impl Component for WallHit {}

#[derive(Debug, Clone, Copy)]
pub struct RespawnTimer {
    pub timer: f32,
    pub transition_started: bool,
}
impl Component for RespawnTimer {}

#[derive(Debug, Clone, Copy)] pub struct RespawnTag;
impl Component for RespawnTag {}

#[derive(Debug, Clone, Copy)] pub struct Goal;
impl Component for Goal {}

#[derive(Debug, Clone)] pub struct NextLevel(pub String);
impl Component for NextLevel {}

#[derive(Debug, Clone, Copy, PartialEq)] pub enum Direction { Left, Right }
#[derive(Debug, Clone, Copy)] pub struct Directional { pub direction: Direction }
impl Component for Directional {}

#[derive(Debug, Clone, Copy, PartialEq)] pub struct Acceleration(pub Vector2D);
