use crate::animation::AnimationController;
use crate::math::Vector2D;
use sdl3::rect::Rect;

// A marker trait for all components
#[allow(dead_code)]
pub trait Component {}

// --- Position Component ---
#[derive(Debug, Clone, Copy, Default)]
pub struct Position(pub Vector2D);
impl Component for Position {}

// --- Velocity Component ---
#[derive(Debug, Clone, Copy, Default)]
pub struct Velocity(pub Vector2D);
impl Component for Velocity {}

// --- Player Tag Component ---
#[derive(Debug, Clone, Copy)]
pub struct PlayerTag;
impl Component for PlayerTag {}

// --- GoldCoin Tag Component ---
#[derive(Debug, Clone, Copy)]
pub struct GoldCoin;
impl Component for GoldCoin {}

// --- Animation Component ---
#[derive(Clone)]
pub struct Animation {
    pub controller: AnimationController,
}
impl Component for Animation {}

// --- Renderable Component ---
#[derive(Clone)]
pub struct Renderable {
    pub width: u32,
    pub height: u32,
    pub horizontal_offset: i32,
    pub vertical_offset: i32,
}
impl Component for Renderable {}

// --- Gravity Component (Tag) ---
#[derive(Debug, Clone, Copy)]
pub struct Gravity;
impl Component for Gravity {}

// --- Collision Component ---
#[derive(Debug, Clone, Copy)]
pub struct Collision {
    pub rect: Rect,
}
impl Component for Collision {}

// --- Grounded Tag Component ---
#[derive(Debug, Clone, Copy)]
pub struct Grounded;
impl Component for Grounded {}

// --- State Component ---

pub struct StateComponent {

    pub state_machine: crate::state_machine::StateMachine,

}

impl Component for StateComponent {}



// --- Respawn Tag Component ---

#[derive(Debug, Clone, Copy)]

pub struct RespawnTag;

impl Component for RespawnTag {}



// --- Respawn Timer Component ---

#[derive(Debug, Clone, Copy)]

pub struct RespawnTimer {

    pub timer: f32,

}

impl Component for RespawnTimer {}

// --- Enemy Tag Component ---
#[derive(Debug, Clone, Copy)]
pub struct EnemyTag;
impl Component for EnemyTag {}

// --- Patrol Component ---
#[derive(Debug, Clone, Copy)]
pub struct Patrol {
    pub speed: f32,
}
impl Component for Patrol {}

// --- Dead Tag Component ---
#[derive(Debug, Clone, Copy)]
pub struct DeadTag;
impl Component for DeadTag {}