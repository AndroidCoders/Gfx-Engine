//! # Manager: ECS World
//! 
//! This module defines the 'World', the central container for the Entity-Component-System.
//! It owns the storage for all components (SoA), manages the entity lifecycle,
//! and provides the Event Bus for decoupled communication between systems.

use std::collections::HashMap;
use crate::ecs::component::*;
use crate::ecs::event::EventBus;
use crate::ecs::resources::{GameState, SpatialGrid, GameStats, UIState, FrameDebugInfo};

/// A unique handle for an object in the game world.
pub type Entity = usize;

/// The central repository for all game data and event queues.
#[derive(Default)]
pub struct World {
    /// The current high-level state of the game application.
    pub game_state: GameState,
    /// The uniform grid used for optimized spatial queries.
    pub spatial_grid: SpatialGrid,
    /// Persistent gameplay statistics like lives and coins.
    pub stats: GameStats,
    /// The visual state of the HUD, decoupled for juice effects.
    pub ui_state: UIState,
    /// Debug info updated each frame.
    pub frame_debug_info: FrameDebugInfo,
    /// The source of unique IDs for new entities.
    next_entity_id: usize,
    /// The type-based event bus for cross-system facts.
    pub event_bus: EventBus,
    
    // --- Component Storage (Structure of Arrays) ---
    pub positions: HashMap<Entity, Position>,
    pub previous_positions: HashMap<Entity, Position>,
    pub velocities: HashMap<Entity, Velocity>,
    pub accelerations: HashMap<Entity, Acceleration>,
    pub renderables: HashMap<Entity, Renderable>,
    pub animations: HashMap<Entity, Animation>,
    pub player_tags: HashMap<Entity, PlayerTag>,
    pub gold_coins: HashMap<Entity, GoldCoin>,
    pub enemy_tags: HashMap<Entity, EnemyTag>,
    pub dead_tags: HashMap<Entity, DeadTag>,
    pub patrols: HashMap<Entity, Patrol>,
    pub gravity_tags: HashMap<Entity, Gravity>,
    pub collisions: HashMap<Entity, Collision>,
    pub grounded_tags: HashMap<Entity, Grounded>,
    pub wall_hits: HashMap<Entity, WallHit>,
    pub state_components: HashMap<Entity, StateComponent>,
    pub respawn_tags: HashMap<Entity, RespawnTag>,
    pub respawn_timers: HashMap<Entity, RespawnTimer>,
    pub healths: HashMap<Entity, Health>,
    pub invincibilities: HashMap<Entity, Invincibility>,
    pub lifetimes: HashMap<Entity, Lifetime>,
    pub directions: HashMap<Entity, Directional>,
    pub goals: HashMap<Entity, Goal>,
    pub next_levels: HashMap<Entity, NextLevel>,
    pub music_state: MusicState,
    pub transition_finished: bool,
    pub movement_intentions: HashMap<Entity, MovementIntention>,
    pub dormant_tags: HashMap<Entity, DormantTag>,
}

impl World {
    /// Initializes a new, empty simulation world.
    pub fn new() -> Self {
        Self {
            transition_finished: false,
            game_state: GameState::default(),
            spatial_grid: SpatialGrid::new(64.0),
            stats: GameStats::default(),
            ui_state: UIState::default(),
            frame_debug_info: FrameDebugInfo::default(),
            ..Default::default()
        }
    }

    /// Flushes all pending facts from the event bus.
    pub fn clear_events(&mut self) {
        self.event_bus.clear_events();
    }

    /// Generates a new unique Entity ID.
    pub fn create_entity(&mut self) -> Entity {
        let entity_id = self.next_entity_id;
        self.next_entity_id += 1;
        entity_id
    }

    // --- Component Storage Helpers ---

    pub fn add_position(&mut self, entity: Entity, component: Position) {
        // 1. Initialize both current and previous to prevent lerp jumps on first frame.
        self.previous_positions.insert(entity, component);
        self.positions.insert(entity, component);
    }

    pub fn add_velocity(&mut self, entity: Entity, velocity: Velocity) {
        self.velocities.insert(entity, velocity);
    }

    pub fn add_acceleration(&mut self, entity: Entity, acceleration: Acceleration) {
        self.accelerations.insert(entity, acceleration);
    }

    pub fn add_movement_intention(&mut self, entity: Entity, component: MovementIntention) {
        self.movement_intentions.insert(entity, component);
    }

    pub fn add_renderable(&mut self, entity: Entity, renderable: Renderable) {
        self.renderables.insert(entity, renderable);
    }

    pub fn add_animation(&mut self, entity: Entity, component: Animation) {
        self.animations.insert(entity, component);
    }

    pub fn add_player_tag(&mut self, entity: Entity, component: PlayerTag) {
        self.player_tags.insert(entity, component);
    }

    pub fn add_gravity(&mut self, entity: Entity, component: Gravity) {
        self.gravity_tags.insert(entity, component);
    }

    pub fn add_collision(&mut self, entity: Entity, component: Collision) {
        self.collisions.insert(entity, component);
    }

    pub fn add_grounded(&mut self, entity: Entity, component: Grounded) {
        self.grounded_tags.insert(entity, component);
    }

    pub fn is_grounded(&self, entity: Entity) -> bool {
        self.grounded_tags.contains_key(&entity)
    }

    pub fn add_wall_hit(&mut self, entity: Entity, component: WallHit) {
        self.wall_hits.insert(entity, component);
    }

    pub fn add_state_component(&mut self, entity: Entity, component: StateComponent) {
        self.state_components.insert(entity, component);
    }

    pub fn add_respawn_tag(&mut self, entity: Entity, component: RespawnTag) {
        self.respawn_tags.insert(entity, component);
    }

    pub fn add_respawn_timer(&mut self, entity: Entity, component: RespawnTimer) {
        self.respawn_timers.insert(entity, component);
    }

    pub fn add_enemy_tag(&mut self, entity: Entity, component: EnemyTag) {
        self.enemy_tags.insert(entity, component);
    }

    pub fn add_patrol(&mut self, entity: Entity, component: Patrol) {
        self.patrols.insert(entity, component);
    }

    pub fn add_dead_tag(&mut self, entity: Entity, component: DeadTag) {
        self.dead_tags.insert(entity, component);
    }

    pub fn add_gold_coin(&mut self, entity: Entity, component: GoldCoin) {
        self.gold_coins.insert(entity, component);
    }

    pub fn add_health(&mut self, entity: Entity, component: Health) {
        self.healths.insert(entity, component);
    }

    pub fn add_invincibility(&mut self, entity: Entity, component: Invincibility) {
        self.invincibilities.insert(entity, component);
    }

    pub fn add_lifetime(&mut self, entity: Entity, component: Lifetime) {
        self.lifetimes.insert(entity, component);
    }

    pub fn add_direction(&mut self, entity: Entity, component: Directional) {
        self.directions.insert(entity, component);
    }

    pub fn add_goal(&mut self, entity: Entity, component: Goal) {
        self.goals.insert(entity, component);
    }

    pub fn add_next_level(&mut self, entity: Entity, component: NextLevel) {
        self.next_levels.insert(entity, component);
    }

    pub fn add_dormant_tag(&mut self, entity: Entity, tag: DormantTag) {
        self.dormant_tags.insert(entity, tag);
    }

    pub fn remove_dormant_tag(&mut self, entity: Entity) {
        self.dormant_tags.remove(&entity);
    }

    pub fn is_dormant(&self, entity: Entity) -> bool {
        self.dormant_tags.contains_key(&entity)
    }

    /// Snapshots current positions to previous storage to enable sub-pixel rendering interpolation.
    pub fn snapshot_positions(&mut self) {
        // 1. Clone the current positions map into the 'previous' buffer.
        // This is efficient for small structs like Position (Vector2D).
        self.previous_positions = self.positions.clone();
    }
}
