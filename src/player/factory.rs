//! # Manager: Player Construction
//! 
//! This module acts as the authoritative factory for the Player character. 
//! It encapsulates the complex logic of assembling physical, visual, and 
//! logical components required to bring a playable hero into the ECS world.

use crate::ecs::world::World;
use crate::ecs::component::*;
use crate::config::GameConfig;
use crate::animation::AnimationController;
use crate::player::states::IdleState;
use crate::state_machine::StateMachine;
use crate::math::Vector2D;

/// A utility for instantiating complete Player entities.
pub struct PlayerFactory;

impl PlayerFactory {
    /// Constructs a fully-equipped Player entity within the provided World.
    pub fn create(world: &mut World, game_config: &GameConfig) -> crate::ecs::world::Entity {
        // 1. Generate a new unique Entity ID.
        let player_entity = world.create_entity();
        let player_position = Position(game_config.player.start_pos);

        // 2. Assemble physical components (Position, Velocity, Gravity, Collision).
        world.add_position(player_entity, player_position);
        world.add_velocity(player_entity, Velocity(Vector2D::default()));
        world.add_acceleration(player_entity, Acceleration(Vector2D::default()));
        
        world.add_renderable(player_entity, Renderable {
            width: game_config.player.draw_width,
            height: game_config.player.draw_height,
            horizontal_offset: game_config.player.horizontal_draw_offset,
            vertical_offset: game_config.player.vertical_draw_offset,
            z_index: 100,
            rotation: 0.0,
            flip_horizontal: false,
            flip_vertical: false,
        });

        // 3. Build and filter the animation controller from global game configuration.
        let mut player_animation_controller = AnimationController::new();
        for (name, anim_config) in &game_config.animation {
            // Identify player-specific clips based on naming conventions.
            if !name.starts_with("enemy") && !name.starts_with("gold_coin") && !name.starts_with("explosion") {
                let mut frames = Vec::new();
                for i in 0..anim_config.frame_count {
                    let padding = anim_config.frame_padding.unwrap_or(0);
                    frames.push(sdl3::rect::Rect::new(
                        anim_config.start_x + (i * (anim_config.frame_width + padding)) as i32,
                        anim_config.start_y,
                        anim_config.frame_width,
                        anim_config.frame_height,
                    ));
                }
                if anim_config.reverse.unwrap_or(false) { frames.reverse(); }
                let animation = crate::animation::Animation {
                    texture_name: anim_config.texture.clone(),
                    frames,
                    frame_duration: anim_config.frame_duration,
                    loops: anim_config.loops,
                };
                player_animation_controller.add_animation(name.clone(), animation);
            }
        }
        player_animation_controller.set_animation("idle_right");
        world.add_animation(player_entity, Animation { controller: player_animation_controller });
        
        // 4. Register semantic tags and logic controllers (State Machine, Health, Direction).
        world.add_player_tag(player_entity, PlayerTag);
        world.add_gravity(player_entity, Gravity);
        world.add_collision(player_entity, Collision {
            rect: sdl3::rect::Rect::new(
                player_position.0.x as i32,
                player_position.0.y as i32,
                game_config.player.width,
                game_config.player.height,
            ),
        });
        
        world.add_state_component(player_entity, StateComponent { state_machine: StateMachine::new(IdleState) });
        world.add_health(player_entity, Health { current: game_config.player.max_health, max: game_config.player.max_health });
        world.add_direction(player_entity, Directional { direction: Direction::Right });

        player_entity
    }
}