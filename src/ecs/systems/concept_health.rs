//! # Concept: Health
//! 
//! This module acts as the "Mortality Engine". 
//! It monitors [crate::ecs::component::Health] components and detects death conditions.
//! 
//! # Responsibilities
//! * Detects when Health drops to zero.
//! * Publishes [crate::ecs::event::EventPlayerDied] if the entity is a player.
//! * Adds [crate::ecs::component::DeadTag] to non-player entities (enemies).

use crate::ecs::systems::{System, SystemContext};
use crate::ecs::component::DeadTag;
use crate::ecs::event::{EventPlayerDied, PlayerDeathReason};

pub struct ConceptHealth;

impl System<SystemContext<'_>> for ConceptHealth {
    fn update(&mut self, world: &mut crate::ecs::world::World, context: &mut SystemContext<'_>) {
        // 1. Detect Dead Enemies
        // Rule: If a non-player entity has 0 health, mark it as Dead.
        let mut dead_enemies = Vec::new();
        for (entity, health) in &world.healths {
            if !world.dormant_tags.contains_key(entity) 
                && health.current == 0 
                && !world.player_tags.contains_key(entity) 
            {
                dead_enemies.push(*entity);
            }
        }
        for entity in dead_enemies {
            world.add_dead_tag(entity, DeadTag);
        }

        // 2. Detect Player Fallen Out of Bounds
        // Rule: If a player falls below the death plane, trigger death by falling.
        let mut fell_players = Vec::new();
        for entity in world.player_tags.keys() {
            // Skip if already dead or respawning
            if world.respawn_timers.contains_key(entity) || world.respawn_tags.contains_key(entity) {
                continue;
            } 
            
            if let Some(pos) = world.positions.get(entity) {
                 if pos.0.y > context.game_config.world.death_plane_y {
                     fell_players.push(*entity);
                 }
            }
        }
        for entity in fell_players {
            world.event_bus.publish(EventPlayerDied { 
                player: entity, 
                reason: PlayerDeathReason::FellOutOfBounds 
            });
        }

        // 3. Detect Player Health Depletion
        // Rule: If a player reaches 0 health, trigger death by health depletion.
        let mut dead_players = Vec::new();
        for (entity, health) in &world.healths {
            if health.current == 0 && world.player_tags.contains_key(entity) {
                 // Check if we already processed this death (check state machine)
                 let mut already_dead = false;
                 if let Some(state) = world.state_components.get(entity) {
                     let name = state.state_machine.current_state.as_ref().map(|s| s.get_name()).unwrap_or("");
                     if name == "DyingState" || name == "DeadState" {
                         already_dead = true;
                     }
                 }
                 
                 if !already_dead {
                     dead_players.push(*entity);
                 }
            }
        }
        for entity in dead_players {
            world.event_bus.publish(EventPlayerDied { 
                player: entity, 
                reason: PlayerDeathReason::HealthDepleted 
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ecs::world::World;
    use crate::ecs::component::Health;
    use crate::ecs::systems::SystemContext;
    use crate::config::{load_config, load_game_config};
    use crate::level::Level;
    use crate::input::InputState;

    #[test]
    fn test_enemy_death() {
        let mut world = World::new();
        let entity = world.create_entity();
        world.add_health(entity, Health { current: 0, max: 10 });
        // No PlayerTag, so it's an enemy

        let mut system = ConceptHealth;
        
        // Mock context
        let config = load_config().unwrap();
        let game_config = load_game_config("assets/game_config.toml").unwrap();
        let (audio_sender, _) = std::sync::mpsc::channel();
        let mut camera = crate::camera::Camera::new(0.0, 0.0, 0.1, 480.0, 270.0, 1000.0, 1000.0, 0.1, 0.1, 0.1, 0.1, 0.1, 0.1, 1000.0, 100.0, 0.1);
        let level = Level::default();
        let input_state = InputState::default();
        let mut next_level = None;
        let mut benchmarker = crate::benchmarker::Benchmarker::new();

        let mut mock_context = SystemContext {
            config: &config,
            game_config: &game_config,
            delta_time: 0.1,
            camera: &mut camera,
            audio_sender: &audio_sender,
            is_paused: false,
            is_attract_mode: false,
            benchmarker: &mut benchmarker,
            level: &level,
            input_state: &input_state,
            next_level: &mut next_level,
            current_soundtrack: None,
        };

        system.update(&mut world, &mut mock_context);

        assert!(world.dead_tags.contains_key(&entity), "Enemy with 0 health should be marked dead");
    }
}
