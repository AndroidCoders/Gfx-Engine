//! # Concept: Vitality
//! 
//! This module manages time-based life-state components.
//! It is responsible for decrementing [crate::ecs::component::Invincibility] 
//! and [crate::ecs::component::Lifetime] timers, and triggering state changes 
//! (like death) when they expire.

use crate::ecs::systems::{System, SystemContext};
use crate::ecs::component::DeadTag;

pub struct ConceptVitality;

impl System<SystemContext<'_>> for ConceptVitality {
    fn update(&mut self, world: &mut crate::ecs::world::World, context: &mut SystemContext<'_>) {
        // 1. Manage Invincibility Timers
        let mut remove_inv = Vec::new();
        for (&entity, inv) in world.invincibilities.iter_mut() {
            if !world.dormant_tags.contains_key(&entity) {
                inv.timer -= context.delta_time;
                if inv.timer <= 0.0 {
                    remove_inv.push(entity);
                }
            }
        }
        for e in remove_inv {
            world.invincibilities.remove(&e);
        }

        // 2. Manage Lifetime Timers (e.g., Particles, Projectiles)
        let mut kill_life = Vec::new();
        for (&entity, life) in world.lifetimes.iter_mut() {
            if !world.dormant_tags.contains_key(&entity) {
                life.timer -= context.delta_time;
                if life.timer <= 0.0 {
                    kill_life.push(entity);
                }
            }
        }
        for e in kill_life {
            world.add_dead_tag(e, DeadTag);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ecs::world::World;
    use crate::ecs::component::{Invincibility, Lifetime};
    use crate::ecs::systems::SystemContext;
    use crate::config::{load_config, load_game_config};
    use crate::level::Level;
    use crate::input::InputState;

    #[test]
    fn test_invincibility_expiration() {
        let mut world = World::new();
        let entity = world.create_entity();
        world.add_invincibility(entity, Invincibility { timer: 0.1 });

        let mut system = ConceptVitality;
        
        // Mock context
        let config = load_config().unwrap();
        let game_config = load_game_config("assets/game_config.toml").unwrap();
        let (audio_sender, _) = std::sync::mpsc::channel();
        let mut camera = crate::camera::Camera::new(0.0, 0.0, 0.1, 480.0, 270.0, 1000.0, 1000.0, 0.1, 0.1, 0.1, 0.1, 0.1, 0.1, 1000.0, 100.0, 0.1);
        let level = Level::default(); // Level derives Default
        let input_state = InputState::default();
        let mut next_level = None;
        let mut benchmarker = crate::benchmarker::Benchmarker::new();

        let mut mock_context = SystemContext {
            config: &config,
            game_config: &game_config,
            delta_time: 0.2, // Exceeds timer
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

        assert!(!world.invincibilities.contains_key(&entity), "Invincibility should be removed after expiration");
    }

    #[test]
    fn test_lifetime_expiration() {
        let mut world = World::new();
        let entity = world.create_entity();
        world.add_lifetime(entity, Lifetime { timer: 0.1 });

        let mut system = ConceptVitality;
        
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
            delta_time: 0.2, // Exceeds timer
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

        assert!(world.dead_tags.contains_key(&entity), "Entity should have DeadTag after lifetime expiration");
    }
}
