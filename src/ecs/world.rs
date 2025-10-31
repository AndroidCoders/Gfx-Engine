use std::collections::HashMap;
use crate::ecs::component::*;

pub type Entity = usize;

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
}

impl World {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn create_entity(&mut self) -> Entity {
        let entity_id = self.next_entity_id;
        self.next_entity_id += 1;
        entity_id
    }

    pub fn add_position(&mut self, entity: Entity, component: Position) {
        self.positions.insert(entity, component);
    }

    pub fn add_velocity(&mut self, entity: Entity, component: Velocity) {
        self.velocities.insert(entity, component);
    }

    pub fn add_renderable(&mut self, entity: Entity, component: Renderable) {
        self.renderables.insert(entity, component);
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

    #[allow(dead_code)]
    pub fn remove_grounded(&mut self, entity: Entity) {
        self.grounded_tags.remove(&entity);
    }

    pub fn is_grounded(&self, entity: Entity) -> bool {
        self.grounded_tags.contains_key(&entity)
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
}