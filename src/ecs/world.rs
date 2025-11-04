//! The core of the Entity-Component-System (ECS) architecture.
//! 
//! This module defines the `World`, which is the container for all entities and their
//! associated components. The `World` provides methods for creating entities and
//! adding, removing, and accessing components.

use std::collections::HashMap;
use crate::ecs::component::*;

/// A unique identifier for an entity in the game world.
pub type Entity = usize;

/// The `World` struct holds all the data for the game state.
///
/// It contains HashMaps for each component type, where the key is the `Entity` ID
/// and the value is the component itself.
#[derive(Default)]
pub struct World {
    next_entity_id: usize,
    pub positions: HashMap<Entity, Position>,
    pub velocities: HashMap<Entity, Velocity>,
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
    pub state_components: HashMap<Entity, StateComponent>,
    pub respawn_tags: HashMap<Entity, RespawnTag>,
    pub respawn_timers: HashMap<Entity, RespawnTimer>,
    pub healths: HashMap<Entity, Health>,
    pub invincibilities: HashMap<Entity, Invincibility>,
    pub lifetimes: HashMap<Entity, Lifetime>,
}

impl World {
    /// Creates a new, empty `World`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a new entity with a unique ID.
    pub fn create_entity(&mut self) -> Entity {
        let entity_id = self.next_entity_id;
        self.next_entity_id += 1;
        entity_id
    }

    /// Adds a `Position` component to an entity.
    pub fn add_position(&mut self, entity: Entity, component: Position) {
        self.positions.insert(entity, component);
    }

    /// Adds a `Velocity` component to an entity.
    pub fn add_velocity(&mut self, entity: Entity, component: Velocity) {
        self.velocities.insert(entity, component);
    }

    /// Adds a `Renderable` component to an entity.
    pub fn add_renderable(&mut self, entity: Entity, component: Renderable) {
        self.renderables.insert(entity, component);
    }

    /// Adds an `Animation` component to an entity.
    pub fn add_animation(&mut self, entity: Entity, component: Animation) {
        self.animations.insert(entity, component);
    }

    /// Adds a `PlayerTag` component to an entity.
    pub fn add_player_tag(&mut self, entity: Entity, component: PlayerTag) {
        self.player_tags.insert(entity, component);
    }

    /// Adds a `Gravity` component to an entity.
    pub fn add_gravity(&mut self, entity: Entity, component: Gravity) {
        self.gravity_tags.insert(entity, component);
    }

    /// Adds a `Collision` component to an entity.
    pub fn add_collision(&mut self, entity: Entity, component: Collision) {
        self.collisions.insert(entity, component);
    }

    /// Adds a `Grounded` component to an entity.
    pub fn add_grounded(&mut self, entity: Entity, component: Grounded) {
        self.grounded_tags.insert(entity, component);
    }

    /// Removes a `Grounded` component from an entity.
    #[allow(dead_code)]
    pub fn remove_grounded(&mut self, entity: Entity) {
        self.grounded_tags.remove(&entity);
    }

    /// Checks if an entity has a `Grounded` component.
    pub fn is_grounded(&self, entity: Entity) -> bool {
        self.grounded_tags.contains_key(&entity)
    }

    /// Adds a `StateComponent` to an entity.
    pub fn add_state_component(&mut self, entity: Entity, component: StateComponent) {
        self.state_components.insert(entity, component);
    }

    /// Adds a `RespawnTag` to an entity.
    pub fn add_respawn_tag(&mut self, entity: Entity, component: RespawnTag) {
        self.respawn_tags.insert(entity, component);
    }

    /// Adds a `RespawnTimer` to an entity.
    pub fn add_respawn_timer(&mut self, entity: Entity, component: RespawnTimer) {
        self.respawn_timers.insert(entity, component);
    }

    /// Adds an `EnemyTag` to an entity.
    pub fn add_enemy_tag(&mut self, entity: Entity, component: EnemyTag) {
        self.enemy_tags.insert(entity, component);
    }

    /// Adds a `Patrol` component to an entity.
    pub fn add_patrol(&mut self, entity: Entity, component: Patrol) {
        self.patrols.insert(entity, component);
    }

    /// Adds a `DeadTag` to an entity.
    pub fn add_dead_tag(&mut self, entity: Entity, component: DeadTag) {
        self.dead_tags.insert(entity, component);
    }

    /// Adds a `GoldCoin` component to an entity.
    pub fn add_gold_coin(&mut self, entity: Entity, component: GoldCoin) {
        self.gold_coins.insert(entity, component);
    }

    /// Adds a `Health` component to an entity.
    pub fn add_health(&mut self, entity: Entity, component: Health) {
        self.healths.insert(entity, component);
    }

    /// Adds an `Invincibility` component to an entity.
    pub fn add_invincibility(&mut self, entity: Entity, component: Invincibility) {
        self.invincibilities.insert(entity, component);
    }

    /// Adds a `Lifetime` component to an entity.
    pub fn add_lifetime(&mut self, entity: Entity, component: Lifetime) {
        self.lifetimes.insert(entity, component);
    }
}