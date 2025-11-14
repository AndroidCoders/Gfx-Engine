use crate::animation::AnimationController;
use crate::math::Vector2D;
use sdl3::rect::Rect;

// A marker trait for all components.
#[allow(dead_code)]
pub trait Component {}

/// Represents the position of an entity in the world.
#[derive(Debug, Clone, Copy, Default)]
pub struct Position(pub Vector2D);
impl Component for Position {}

/// Represents the velocity of an entity.
#[derive(Debug, Clone, Copy, Default)]
pub struct Velocity(pub Vector2D);
impl Component for Velocity {}

/// A tag component that identifies the player entity.
#[derive(Debug, Clone, Copy)]
pub struct PlayerTag;
impl Component for PlayerTag {}

/// A tag component that identifies a gold coin entity.
#[derive(Debug, Clone, Copy)]
pub struct GoldCoin;
impl Component for GoldCoin {}

/// Holds the animation state for an entity.
#[derive(Clone)]
pub struct Animation {
    /// The animation controller responsible for updating and managing animations.
    pub controller: AnimationController,
}
impl Component for Animation {}

/// A component that makes an entity renderable.
#[derive(Clone)]
pub struct Renderable {
    /// The width of the sprite for rendering, in world units.
    pub width: u32,
    /// The height of the sprite for rendering, in world units.
    pub height: u32,
    /// The horizontal rendering offset of the sprite.
    pub horizontal_offset: i32,
    /// The vertical rendering offset of the sprite.
    pub vertical_offset: i32,
    /// The z-index for controlling render order (higher is further back).
    pub z_index: u8,
}
impl Component for Renderable {}

/// A tag component that indicates an entity is affected by gravity.
#[derive(Debug, Clone, Copy)]
pub struct Gravity;
impl Component for Gravity {}

/// Represents the collision bounding box for an entity.
#[derive(Debug, Clone, Copy)]
pub struct Collision {
    /// The rectangular bounds of the collision box.
    pub rect: Rect,
}
impl Component for Collision {}

/// A tag component indicating that an entity is currently on the ground.
#[derive(Debug, Clone, Copy)]
pub struct Grounded;
impl Component for Grounded {}

/// Holds the state machine for an entity, controlling its behavior.
pub struct StateComponent {
    /// The entity's state machine instance.
    pub state_machine: crate::state_machine::StateMachine,
}
impl Component for StateComponent {}

/// A tag component that marks an entity to be respawned.
#[derive(Debug, Clone, Copy)]
pub struct RespawnTag;
impl Component for RespawnTag {}

/// A component that gives an entity a temporary grace period after respawning.
#[derive(Debug, Clone, Copy)]
pub struct RespawnTimer {
    /// The remaining time in seconds for the respawn grace period.
    pub timer: f32,
}
impl Component for RespawnTimer {}

/// A tag component that identifies an enemy entity.
#[derive(Debug, Clone, Copy)]
pub struct EnemyTag;
impl Component for EnemyTag {}

/// A component for entities that patrol back and forth.
#[derive(Debug, Clone, Copy)]
pub struct Patrol {
    /// The horizontal speed of the patrol movement.
    pub speed: f32,
}
impl Component for Patrol {}

/// A tag component that marks an entity for removal from the world.
#[derive(Debug, Clone, Copy)]
pub struct DeadTag;
impl Component for DeadTag {}

/// Represents the health of an entity.
#[derive(Debug, Clone, Copy)]
pub struct Health {
    /// The current health points.
    pub current: u32,
    /// The maximum health points.
    pub max: u32,
}
impl Component for Health {}

/// A component that grants an entity temporary invincibility.
#[derive(Debug, Clone, Copy)]
pub struct Invincibility {
    /// The remaining time in seconds for the invincibility.
    pub timer: f32,
}
impl Component for Invincibility {}

/// A component that gives an entity a limited lifetime before it is removed.
#[derive(Debug, Clone, Copy)]
pub struct Lifetime {
    /// The remaining time in seconds before the entity is destroyed.
    pub timer: f32,
}
impl Component for Lifetime {}

/// Represents the direction an entity is facing.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    /// Facing left.
    Left,
    /// Facing right.
    Right,
}

/// A component that gives an entity a direction.
#[derive(Debug, Clone, Copy)]
pub struct Directional {
    /// The direction the entity is facing.
    pub direction: Direction,
}
impl Component for Directional {}

/// A tag component that identifies the level goal entity.
#[derive(Debug, Clone, Copy)]
pub struct Goal;
impl Component for Goal {}