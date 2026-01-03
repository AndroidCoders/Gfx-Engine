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
    /// The current playback position in **seconds**.
    pub current_time: f64,
    /// The last rhythmic onset detected by the audio analysis system.
    pub last_beat: Option<DetectedBeat>,
}

/// # Concept: Position
/// The authoritative world-space coordinates of an entity.
#[derive(Debug, Clone, Copy, Default)]
pub struct Position(pub Vector2D);
impl Component for Position {}

/// # Concept: Velocity
/// The rate of change of an entity's position.
#[derive(Debug, Clone, Copy, Default)]
pub struct Velocity(pub Vector2D);
impl Component for Velocity {}

/// # Concept: Renderable
/// Visual metadata required to draw an entity to the screen.
#[derive(Clone)]
pub struct Renderable {
    /// The width of the sprite in **pixels**.
    pub width: u32,
    /// The height of the sprite in **pixels**.
    pub height: u32,
    /// Horizontal offset from the physical position in **pixels**.
    pub horizontal_offset: i32,
    /// Vertical offset from the physical position in **pixels**.
    pub vertical_offset: i32,
    /// Draw order (higher values are drawn first/behind).
    pub z_index: u8,
    /// Rotation angle in **degrees**.
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
/// The abstract horizontal direction an entity intends to move.
#[derive(Debug, Clone, Copy, Default)]
pub struct MovementIntention {
    /// The intended direction in the range `[-1.0, 1.0]`.
    /// * `-1.0`: Left
    /// * `1.0`: Right
    /// * `0.0`: None
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

/// # Concept: Patrol Behavior
/// Data required for simple back-and-forth enemy movement.
#[derive(Debug, Clone)]
pub struct Patrol {
    /// Movement speed in **World Units per Second** (px/s).
    pub speed: f32,
    pub anim_prefix: String,
    /// Current direction of movement (`-1.0` or `1.0`).
    pub direction: f32,
}
impl Component for Patrol {}

/// # Concept: Invincibility
/// A timer during which an entity ignores damage events.
#[derive(Debug, Clone, Copy)]
pub struct Invincibility { 
    /// Remaining time in **seconds**.
    pub timer: f32 
}
impl Component for Invincibility {}

/// # Concept: Lifetime
/// A timer for ephemeral entities (particles, projectiles) that self-terminate.
#[derive(Debug, Clone, Copy)]
pub struct Lifetime { 
    /// Remaining existence time in **seconds**.
    pub timer: f32 
}
impl Component for Lifetime {}

/// # Concept: Wall Collision Event
/// A transient component added when an entity hits a wall.
#[derive(Debug, Clone, Copy)]
pub struct WallHit { 
    /// The normal vector x-component of the wall hit (`-1.0` or `1.0`).
    pub normal_x: f32 
}
impl Component for WallHit {}

/// # Concept: Respawn Timer
/// Manages the delay between death and the start of the respawn sequence.
#[derive(Debug, Clone, Copy)]
pub struct RespawnTimer {
    /// Countdown in **seconds**.
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

/// # Concept: Acceleration
/// The rate of change of velocity.
#[derive(Debug, Clone, Copy, PartialEq)] 
pub struct Acceleration(pub Vector2D);
