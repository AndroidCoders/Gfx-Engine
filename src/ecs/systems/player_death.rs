// src/ecs/systems/player_death.rs

use crate::ecs::systems::{System, SystemContext};
use crate::ecs::world::World;
use crate::player::states::DyingState;

pub struct PlayerDeathSystem;

impl System<SystemContext<'_>> for PlayerDeathSystem {
    fn update(&mut self, world: &mut World, context: &mut SystemContext) {
        let mut to_die = Vec::new();
        for (entity, health) in &world.healths {
            if health.current == 0 {
                // Check if the entity is a player and not already in DyingState
                if world.player_tags.contains_key(entity) {
                    if let Some(state_comp) = world.state_components.get(entity) {
                        if state_comp.state_machine.current_state.as_ref().map_or(false, |s| s.get_name() != "DyingState") {
                            to_die.push(*entity);
                        }
                    }
                }
            }
        }

        for entity in to_die {
            if let Some(state_comp) = world.state_components.get_mut(&entity) {
                let death_animation_duration = if let Some(anim_config) = context.game_config.animation.get("explosion") {
                    (anim_config.frame_count * anim_config.frame_duration) as f32 / 60.0
                } else {
                    1.0 // Default duration if animation not found
                };
                state_comp.state_machine = crate::state_machine::StateMachine::new(DyingState { timer: death_animation_duration });
            }
            if let Some(animation) = world.animations.get_mut(&entity) {
                animation.controller.set_animation("explosion");
            }
            // Also reset health for the next life
            if let Some(health) = world.healths.get_mut(&entity) {
                health.current = health.max;
            }
        }
    }
}
