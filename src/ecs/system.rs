use crate::config::{Config};
use crate::input::InputState;
use crate::level::Level;
use crate::ecs::world::{World, Entity};
use std::sync::mpsc::Sender;
use crate::audio::AudioEvent;

use crate::ecs::component::*;
use crate::state_machine::StateMachine;


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

fn resolve_vertical_collisions(
    pos: &mut Position,
    vel: &mut Velocity,
    bounds: sdl3::rect::Rect,
    context: &SystemContext,
) -> bool {
    let tile_height = context.level.tileset.tile_height as f32;
    let tile_width = context.level.tileset.tile_width as f32;
    let scaled_bounds_width = bounds.width() as f32;
    let scaled_bounds_height = bounds.height() as f32;

    let next_y = pos.0.y + vel.0.y;
    let mut grounded = false;

    if vel.0.y > 0.0 { // Moving down
        let left_x = pos.0.x + 0.1;
        let right_x = pos.0.x + scaled_bounds_width - 0.1;
        let bottom_y = next_y + scaled_bounds_height;

        let left_tile = (left_x / tile_width).floor() as usize;
        let right_tile = (right_x / tile_width).floor() as usize;
        let bottom_tile = (bottom_y / tile_height).floor() as usize;

        println!("[Collision] vert: pos={:?}, vel={:?}, bottom_y={}, bottom_tile={}", pos.0, vel.0, bottom_y, bottom_tile);

        if context.level.is_solid(left_tile, bottom_tile) || context.level.is_solid(right_tile, bottom_tile) {
            pos.0.y = (bottom_tile as f32 * tile_height) - scaled_bounds_height;
            vel.0.y = 0.0;
            grounded = true;
        }
    } else if vel.0.y < 0.0 { // Moving up
        let left_x = pos.0.x + 0.1;
        let right_x = pos.0.x + scaled_bounds_width - 0.1;
        let top_y = next_y;

        let left_tile = (left_x / tile_width).floor() as usize;
        let right_tile = (right_x / tile_width).floor() as usize;
        let top_tile = (top_y / tile_height).floor() as usize;

        if context.level.is_solid(left_tile, top_tile) || context.level.is_solid(right_tile, top_tile) {
            pos.0.y = (top_tile as f32 * tile_height) + tile_height;
            vel.0.y = 0.0;
        }
    }

    grounded
}

fn resolve_horizontal_collisions(
    pos: &mut Position,
    vel: &mut Velocity,
    bounds: sdl3::rect::Rect,
    context: &SystemContext,
) {
    let tile_height = context.level.tileset.tile_height as f32;
    let tile_width = context.level.tileset.tile_width as f32;
    let scaled_bounds_width = bounds.width() as f32;
    let scaled_bounds_height = bounds.height() as f32;

    let next_x = pos.0.x + vel.0.x;

    if vel.0.x > 0.0 { // Moving right
        let top_y = pos.0.y + 0.1;
        let bottom_y = pos.0.y + scaled_bounds_height - 0.1;
        let right_x = next_x + scaled_bounds_width;

        let top_tile = (top_y / tile_height).floor() as usize;
        let bottom_tile = (bottom_y / tile_height).floor() as usize;
        let right_tile = (right_x / tile_width).floor() as usize;

        if context.level.is_solid(right_tile, top_tile) || context.level.is_solid(right_tile, bottom_tile) {
            pos.0.x = (right_tile as f32 * tile_width) - scaled_bounds_width - 1.0;
            vel.0.x = 0.0;
        }
    } else if vel.0.x < 0.0 { // Moving left
        let top_y = pos.0.y + 0.1;
        let bottom_y = pos.0.y + scaled_bounds_height - 0.1;
        let left_x = next_x;

        let top_tile = (top_y / tile_height).floor() as usize;
        let bottom_tile = (bottom_y / tile_height).floor() as usize;
        let left_tile = (left_x / tile_width).floor() as usize;

        if context.level.is_solid(left_tile, top_tile) || context.level.is_solid(left_tile, bottom_tile) {
            pos.0.x = (left_tile as f32 * tile_width) + tile_width + 1.0;
            vel.0.x = 0.0;
        }
    }
}

impl System<SystemContext<'_>> for CollisionSystem {
    fn update(&mut self, world: &mut World, context: &mut SystemContext) {
        world.grounded_tags.clear();
        let mut entities_to_ground = Vec::new();

        for (entity, pos) in world.positions.iter_mut() {
            if let (Some(vel), Some(collision)) = (world.velocities.get_mut(entity), world.collisions.get_mut(entity)) {
                // Update the collision rect with the new position
                collision.rect.set_x(pos.0.x as i32);
                collision.rect.set_y(pos.0.y as i32);

                let grounded = resolve_vertical_collisions(pos, vel, collision.rect, context);
                resolve_horizontal_collisions(pos, vel, collision.rect, context);

                if grounded {
                    entities_to_ground.push(*entity);
                }
            }
        }

        for entity in entities_to_ground {
            world.add_grounded(entity, Grounded);
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
        let respawn_x = context.game_config.player.respawn_x;
        let respawn_y = context.game_config.player.respawn_y;

        let to_respawn: Vec<Entity> = world.respawn_tags.keys().copied().collect();

        for entity in to_respawn {
            world.respawn_tags.remove(&entity);

            if let Some(pos) = world.positions.get_mut(&entity) {
                pos.0.x = respawn_x;
                pos.0.y = respawn_y;
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
