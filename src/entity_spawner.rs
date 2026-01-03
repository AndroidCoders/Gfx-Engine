//! # Manager: Level Spawning
//! 
//! This module acts as the "Builder" for the game world. It translates 
//! template data from Tiled (TMX) files into live ECS entities, matching 
//! generic objects against configured prefabs and component definitions.

use crate::ecs::world::World;
use crate::level::Level;
use crate::config::{GameConfig, ComponentConfig};
use crate::ecs::component::*;
use crate::state_machine::StateMachine;
use crate::enemy::states::PatrolState;
use crate::animation::AnimationController;

/// Populates the ECS world based on the entity templates defined in a level.
pub fn spawn_entities(world: &mut World, level: &Level, game_config: &GameConfig) {
    // 1. Iterate over every object definition parsed from the TMX file.
    for entity_data in &level.entities {
        let entity = world.create_entity();
        
        // 2. Locate the corresponding 'Prefab' definition in the game configuration.
        if let Some(prefab) = game_config.prefabs.get(&entity_data.r#type) {
            
            // 3. Iterate over the component list defined for this prefab.
            for component_config in &prefab.components {
                match component_config {
                    ComponentConfig::Position => {
                        // Use the world-space position specified in the level layout.
                        world.add_position(entity, Position(entity_data.position));
                    }
                    ComponentConfig::Velocity { x, y } => {
                        world.add_velocity(entity, Velocity(crate::math::Vector2D::new(*x, *y)));
                    }
                    ComponentConfig::Acceleration { x, y } => {
                        world.add_acceleration(entity, Acceleration(crate::math::Vector2D::new(*x, *y)));
                    }
                    ComponentConfig::Renderable { draw_width, draw_height, z_index, horizontal_offset, vertical_offset } => {
                        world.add_renderable(entity, Renderable {
                            width: *draw_width,
                            height: *draw_height,
                            horizontal_offset: *horizontal_offset,
                            vertical_offset: *vertical_offset,
                            z_index: *z_index,
                            rotation: 0.0,
                            flip_horizontal: false,
                            flip_vertical: false,
                        });
                    }
                    ComponentConfig::Animation { animations, initial_animation } => {
                        // Build the animation controller and pre-load all clips.
                        let mut anim_controller = AnimationController::new();
                        for anim_name in animations {
                            if let Some(anim_config) = game_config.animation.get(anim_name) {
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
                                if anim_config.reverse.unwrap_or(false) {
                                    frames.reverse();
                                }
                                let animation = crate::animation::Animation {
                                    texture_name: anim_config.texture.clone(),
                                    frames,
                                    frame_duration: anim_config.frame_duration,
                                    loops: anim_config.loops,
                                };
                                anim_controller.add_animation(anim_name.clone(), animation);
                            }
                        }
                        anim_controller.set_animation(initial_animation);
                        world.add_animation(entity, Animation { controller: anim_controller });
                    }
                    ComponentConfig::Collision { width, height } => {
                        world.add_collision(entity, Collision {
                            rect: sdl3::rect::Rect::new(
                                entity_data.position.x as i32,
                                entity_data.position.y as i32,
                                *width,
                                *height,
                            ),
                        });
                    }
                    ComponentConfig::Gravity => {
                        world.add_gravity(entity, Gravity);
                    }
                    ComponentConfig::Patrol { speed, anim_prefix } => {
                        world.add_patrol(entity, Patrol { speed: *speed, anim_prefix: anim_prefix.clone(), direction: 1.0 });
                    }
                    ComponentConfig::EnemyTag => {
                        world.add_enemy_tag(entity, EnemyTag);
                    }
                    ComponentConfig::GoldCoin => {
                        world.add_gold_coin(entity, GoldCoin);
                    }
                    ComponentConfig::Goal => {
                        world.add_goal(entity, Goal);
                    }
                    ComponentConfig::StateComponent { initial_state } => {
                        // Initialize logic state machines for AI-driven entities.
                        if initial_state == "PatrolState" {
                            world.add_state_component(entity, StateComponent { state_machine: StateMachine::new(PatrolState::new()) });
                        }
                    }
                }
            }
        }

        // 4. Resolve custom TMX properties (e.g., 'next_level' paths for goals).
        if let Some(next_level_path) = entity_data.properties.get("next_level") {
            use crate::ecs::component::NextLevel;
            world.add_next_level(entity, NextLevel(next_level_path.clone()));
        }
    }
}