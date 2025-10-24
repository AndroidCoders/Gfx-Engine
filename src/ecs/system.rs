use crate::config::{Config};
use crate::input::InputState;
use crate::level::Level;
use crate::ecs::world::{World, Entity};
use std::sync::mpsc::Sender;
use crate::audio::AudioEvent;

use crate::ecs::component::*;
use crate::state_machine::StateMachine;


use crate::physics;


pub trait System<T> {
    fn update(&mut self, world: &mut World, context: &mut T);
}

pub struct SystemContext<'a> {
    pub level: &'a Level,
    pub _camera: &'a Camera,
    pub input_state: &'a InputState,
    pub config: &'a Config,
    pub game_config: &'a crate::config::GameConfig,
    pub audio_sender: &'a Sender<AudioEvent>,
}

// --- System Implementations ---

pub struct InputSystem;
impl System<SystemContext<'_>> for InputSystem {
    fn update(&mut self, _world: &mut World, _context: &mut SystemContext) {
        // No direct action here, input_state is updated in app.rs and passed to systems
    }
}

pub struct PhysicsSystem;
impl System<SystemContext<'_>> for PhysicsSystem {
    fn update(&mut self, world: &mut World, context: &mut SystemContext) {
        for (entity, _gravity) in world.gravity_tags.iter() {
            if let Some(vel) = world.velocities.get_mut(entity) {
                vel.0.y += context.config.physics.gravity;
                vel.0.y = vel.0.y.min(context.config.physics.max_fall_speed);
            }
        }

        for (entity, pos) in world.positions.iter_mut() {
            if let Some(vel) = world.velocities.get(entity) {
                pos.0.x += vel.0.x;
                pos.0.y += vel.0.y;
            }
        }
    }
}

pub struct CollisionSystem;

struct StompEvent {
    enemy: Entity,
    player: Entity,
}

impl System<SystemContext<'_>> for CollisionSystem {
    fn update(&mut self, world: &mut World, context: &mut SystemContext) {
        world.grounded_tags.clear();
        let mut entities_to_ground = Vec::new();
        let mut stomp_events = Vec::new();

        // --- Collision Detection ---

        // Player-enemy collision detection
        let player_entities: Vec<Entity> = world.player_tags.keys().copied().collect();
        let enemy_entities: Vec<Entity> = world.enemy_tags.keys().copied().collect();

        for &player_entity in &player_entities {
            for &enemy_entity in &enemy_entities {
                // Use immutable borrows for detection
                if let (Some(player_pos), Some(player_vel), Some(player_collision)) = (world.positions.get(&player_entity), world.velocities.get(&player_entity), world.collisions.get(&player_entity)) {
                    if let (Some(enemy_pos), Some(enemy_collision)) = (world.positions.get(&enemy_entity), world.collisions.get(&enemy_entity)) {
                        let player_rect = player_collision.rect;
                        let enemy_rect = enemy_collision.rect;

                        if player_rect.has_intersection(enemy_rect) {
                            // Stomp check
                            if player_vel.0.y > 0.0 && player_pos.0.y + player_rect.height() as f32 - player_vel.0.y <= enemy_pos.0.y {
                                stomp_events.push(StompEvent { enemy: enemy_entity, player: player_entity });
                            }
                        }
                    }
                }
            }
        }

        // Tile collision detection
        for (entity, pos) in world.positions.iter_mut() {
            if let (Some(vel), Some(collision)) = (world.velocities.get_mut(entity), world.collisions.get_mut(entity)) {
                collision.rect.set_x(pos.0.x as i32);
                collision.rect.set_y(pos.0.y as i32);

                let grounded = physics::resolve_vertical_collisions(pos, vel, collision.rect, context);
                physics::resolve_horizontal_collisions(pos, vel, collision.rect, context);

                if grounded {
                    entities_to_ground.push(*entity);
                }
            }
        }

        // --- Collision Resolution ---

        // Resolve stomp events
        for event in stomp_events {
            world.add_dead_tag(event.enemy, DeadTag);
            if let Some(player_vel) = world.velocities.get_mut(&event.player) {
                player_vel.0.y = -4.0; // Bounce
            }
        }

        // Resolve groundings
        for entity in entities_to_ground {
            world.add_grounded(entity, Grounded);
        }
    }
}

pub struct KillSystem;
impl System<SystemContext<'_>> for KillSystem {
    fn update(&mut self, world: &mut World, _context: &mut SystemContext) {
        let dead_entities: Vec<Entity> = world.dead_tags.keys().copied().collect();
        for entity in dead_entities {
            world.positions.remove(&entity);
            world.velocities.remove(&entity);
            world.renderables.remove(&entity);
            world.animations.remove(&entity);
            world.enemy_tags.remove(&entity);
            world.dead_tags.remove(&entity);
            world.patrols.remove(&entity);
            world.gravity_tags.remove(&entity);
            world.collisions.remove(&entity);
            world.grounded_tags.remove(&entity);
            world.state_components.remove(&entity);
        }
    }
}

pub struct AnimationSystem;
impl System<SystemContext<'_>> for AnimationSystem {
    fn update(&mut self, world: &mut World, _context: &mut SystemContext) {
        for (entity, animation) in world.animations.iter_mut() {
            if let Some(state_component) = world.state_components.get(entity) {
                let current_state_name = state_component.state_machine.current_state.as_ref().map_or("IdleState", |s| s.get_name());

                let direction = if let Some(vel) = world.velocities.get(entity) {
                    if vel.0.x < -0.1 { "left" } else { "right" }
                } else {
                    "right"
                };

                let animation_name = match current_state_name {
                    "IdleState" => format!("idle_{}", direction),
                    "WalkingState" => format!("walk_{}", direction),
                    "JumpingState" => format!("jump_{}", direction),
                    "FallingState" => format!("fall_{}", direction),
                    _ => format!("idle_{}", direction), // Fallback
                };
                
                if animation.controller.has_animation(&animation_name) {
                    animation.controller.set_animation(&animation_name);
                } else {
                    animation.controller.set_animation(&format!("idle_{}", direction));
                }

                animation.controller.update();
            }
        }
    }
}

pub struct StateMachineSystem;
impl System<SystemContext<'_>> for StateMachineSystem {
    fn update(&mut self, world: &mut World, context: &mut SystemContext) {
        let mut updates: Vec<(Entity, StateMachine)> = Vec::new();

        // Extract StateMachines to update
        for (entity, state_component) in world.state_components.drain() {
            updates.push((entity, state_component.state_machine));
        }

        // Perform updates and re-insert
        for (entity, mut state_machine) in updates {
            state_machine.update_with_context(world, context, entity);
            world.state_components.insert(entity, StateComponent { state_machine });
        }
    }
}

use crate::camera::Camera;


pub struct AudioSystem;
impl System<crate::audio::GameAudioManager> for AudioSystem {
    fn update(&mut self, _world: &mut World, context: &mut crate::audio::GameAudioManager) {
        context.process_events();
    }
}

pub struct DeathSystem;
impl System<SystemContext<'_>> for DeathSystem {
    fn update(&mut self, world: &mut World, context: &mut SystemContext) {
        let death_plane_y = context.game_config.world.death_plane_y;
        let mut to_respawn = Vec::new();

        for (entity, _player_tag) in world.player_tags.iter() {
            // Ignore players with a respawn timer (grace period)
            if world.respawn_timers.contains_key(entity) {
                continue;
            }

            if let Some(pos) = world.positions.get(entity) {
                if pos.0.y > death_plane_y {
                    to_respawn.push(*entity);
                }
            }
        }

        for entity in to_respawn {
            world.add_respawn_tag(entity, RespawnTag);
        }
    }
}

pub struct RespawnSystem;
impl System<SystemContext<'_>> for RespawnSystem {
    fn update(&mut self, world: &mut World, context: &mut SystemContext) {
        let respawn_pos = context.game_config.player.respawn_pos;

        let to_respawn: Vec<Entity> = world.respawn_tags.keys().copied().collect();

        for entity in to_respawn {
            world.respawn_tags.remove(&entity);

            if let Some(pos) = world.positions.get_mut(&entity) {
                pos.0 = respawn_pos;
            }
            if let Some(vel) = world.velocities.get_mut(&entity) {
                vel.0.x = 0.0;
                vel.0.y = 0.0;
            }

            // Add a respawn timer for a grace period (e.g., 2 seconds)
            world.add_respawn_timer(entity, RespawnTimer { timer: 2.0 });
        }
    }
}

pub struct RespawnTimerSystem;
impl System<SystemContext<'_>> for RespawnTimerSystem {
    fn update(&mut self, world: &mut World, _context: &mut SystemContext) {
        let mut to_remove = Vec::new();
        for (entity, timer) in world.respawn_timers.iter_mut() {
            // Assuming a fixed time step for simplicity for now
            // A better approach would be to use a delta time from the game loop
            timer.timer -= 1.0 / 60.0; // Assuming 60 FPS
            if timer.timer <= 0.0 {
                to_remove.push(*entity);
            }
        }

        for entity in to_remove {
            world.respawn_timers.remove(&entity);
        }
    }
}
