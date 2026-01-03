//! # Synchronization: Respawn Orchestration
//! 
//! This module manages the respawn sequence for the player.
//! 
//! # Responsibilities
//! * Decrements [crate::ecs::component::RespawnTimer].
//! * Publishes [crate::ecs::event::EventGameOver] if lives are depleted.
//! * Publishes [crate::ecs::event::EventRespawnStarted] to trigger transitions.
//! * Resets player data (Health, Position, State) when [crate::ecs::component::RespawnTag] is present.

use crate::ecs::systems::{System, SystemContext};
use crate::ecs::component::{RespawnTag, Collision, Gravity, Invincibility};
use crate::ecs::event::{EventGameOver, EventRespawnStarted, EventStartTransition, TransitionType};
use crate::player::states::IdleState;
use crate::state_machine::StateMachine;

pub struct RuleRespawn;

impl System<SystemContext<'_>> for RuleRespawn {
    fn update(&mut self, world: &mut crate::ecs::world::World, context: &mut SystemContext<'_>) {
        // 1. Manage Active Respawn Timers
        let mut respawn_actions = Vec::new();
        let mut game_over_action = false;
        
        for (entity, timer) in world.respawn_timers.iter_mut() {
             timer.timer -= context.delta_time;
             
             // Trigger transition and logic 2 seconds before the timer ends
             if timer.timer <= 2.0 && !timer.transition_started {
                 if world.stats.lives > 0 {
                     respawn_actions.push(*entity);
                 } else {
                     game_over_action = true;
                 }
                 timer.transition_started = true;
             }
        }
        
        if game_over_action {
            world.event_bus.publish(EventGameOver);
        }
        
        for entity in respawn_actions {
             world.event_bus.publish(EventRespawnStarted { player: entity });
             
             // Stop movement during transition
             if let Some(vel) = world.velocities.get_mut(&entity) { vel.0 = crate::math::Vector2D::default(); }
             if let Some(acc) = world.accelerations.get_mut(&entity) { acc.0 = crate::math::Vector2D::default(); }
             world.gravity_tags.remove(&entity);
             world.movement_intentions.remove(&entity);
             
             // Grant long-term invincibility during respawn
             world.add_invincibility(entity, Invincibility { timer: 999.0 });
        }

        // 2. Identify Entities Ready for Physical Respawn
        let mut to_respawn = Vec::new();
        if world.transition_finished {
             for (entity, timer) in &mut world.respawn_timers {
                 if timer.timer <= 0.0 {
                     to_respawn.push(*entity);
                 }
             }
        }
        
        // 3. Execute Respawn (Reset Data)
        if !to_respawn.is_empty() {
            world.transition_finished = false;
            for entity in to_respawn {
                world.respawn_timers.remove(&entity);
                // Mark for processing in step 4
                world.add_respawn_tag(entity, RespawnTag);
            }
        }

        // 4. Handle RespawnTag (Physical Reset)
        let to_reset: Vec<_> = world.respawn_tags.keys().copied().collect();
        for entity in to_reset {
            world.respawn_tags.remove(&entity);
            self.reset_player(entity, world, context);
        }
    }
}

impl RuleRespawn {
    fn reset_player(&self, entity: crate::ecs::world::Entity, world: &mut crate::ecs::world::World, context: &mut SystemContext<'_>) {
            if let Some(pos) = world.positions.get_mut(&entity) { pos.0 = context.game_config.player.respawn_pos; }
            if let Some(vel) = world.velocities.get_mut(&entity) { vel.0 = crate::math::Vector2D::default(); }
            if let Some(health) = world.healths.get_mut(&entity) { health.current = health.max; }
            if let Some(renderable) = world.renderables.get_mut(&entity) { 
                renderable.width = context.game_config.player.draw_width;
                renderable.height = context.game_config.player.draw_height;
                renderable.flip_vertical = false; 
                renderable.flip_horizontal = false; 
                renderable.rotation = 0.0; 
                renderable.vertical_offset = context.game_config.player.vertical_draw_offset; 
            }
            context.camera.snap_to(context.game_config.player.respawn_pos);
            if let Some(pos) = world.positions.get(&entity) { world.previous_positions.insert(entity, *pos); }
            
            // Remove the infinite invincibility from the timer phase
            world.invincibilities.remove(&entity);
            
            world.add_gravity(entity, Gravity);
            world.add_collision(entity, Collision { 
                rect: sdl3::rect::Rect::new(
                    context.game_config.player.respawn_pos.x as i32, 
                    context.game_config.player.respawn_pos.y as i32, 
                    context.game_config.player.width, 
                    context.game_config.player.height
                ) 
            });
            
            if let Some(anim) = world.animations.get_mut(&entity) { anim.controller.set_animation("idle_right"); }
            if let Some(state) = world.state_components.get_mut(&entity) { state.state_machine = StateMachine::new(IdleState); }

            let center = {
                let cx = context.game_config.player.respawn_pos.x + (context.game_config.player.draw_width as f32 / 2.0) + context.game_config.player.horizontal_draw_offset as f32;
                let cy = context.game_config.player.respawn_pos.y + (context.game_config.player.draw_height as f32 / 2.0) + context.game_config.player.vertical_draw_offset as f32;
                let screen_x = cx - context.camera.position.x;
                let screen_y = cy - context.camera.position.y;

                if screen_x.is_nan() || screen_y.is_nan() {
                    None
                } else {
                    Some((screen_x as i32, screen_y as i32))
                }
            };
            
            world.event_bus.publish(EventStartTransition { transition_type: TransitionType::IrisIn, duration: 2.0, center });
            let _ = context.audio_sender.send(crate::audio::AudioEvent::PlayMusic("soundtrack_01".to_string(), crate::audio::PlaySoundParams::default()));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ecs::world::World;
    use crate::ecs::component::RespawnTag;
    use crate::ecs::systems::SystemContext;
    use crate::config::{load_config, load_game_config};
    use crate::level::Level;
    use crate::input::InputState;

    #[test]
    fn test_respawn_execution() {
        let mut world = World::new();
        let entity = world.create_entity();
        world.add_respawn_tag(entity, RespawnTag);
        // Needs position for reset logic
        world.add_position(entity, crate::ecs::component::Position(crate::math::Vector2D::default()));

        let mut system = RuleRespawn;
        
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

        assert!(!world.respawn_tags.contains_key(&entity), "RespawnTag should be consumed");
        assert!(world.gravity_tags.contains_key(&entity), "Gravity should be restored");
    }
}
