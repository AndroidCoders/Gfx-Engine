use crate::ecs::component::{Animation, DeadTag, Invincibility, Lifetime, Position, Renderable};
use crate::ecs::systems::{System, SystemContext};
use crate::ecs::world::{Entity, World};
use crate::animation::AnimationController;
use crate::math::Vector2D;
use crate::audio::AudioEvent;

struct StompEvent {
    enemy: Entity,
    player: Entity,
}

struct DamageCommand {
    player: Entity,
    knockback_x: f32,
    position: Vector2D,
}

pub struct InteractionSystem;

impl System<SystemContext<'_>> for InteractionSystem {
    fn update(&mut self, world: &mut World, context: &mut SystemContext) {
        let mut stomp_events = Vec::new();
        let mut damage_commands = Vec::new();

        let player_entities: Vec<Entity> = world.player_tags.keys().copied().collect();
        let enemy_entities: Vec<Entity> = world.enemy_tags.keys().copied().collect();

        for &player_entity in &player_entities {
            // Skip collision checks if player is invincible
            if world.invincibilities.contains_key(&player_entity) {
                continue;
            }

            if let (Some(player_pos), Some(player_vel), Some(player_collision)) = (
                world.positions.get(&player_entity),
                world.velocities.get(&player_entity),
                world.collisions.get(&player_entity),
            ) {
                for &enemy_entity in &enemy_entities {
                    if let Some(enemy_collision) = world.collisions.get(&enemy_entity) {
                        if let Some(intersection) = player_collision.rect.intersection(enemy_collision.rect) {
                            let player_is_falling = player_vel.0.y > 0.0;

                            // A stomp is a vertical collision where the player is falling.
                            // We can approximate this by checking if the intersection is wider than it is tall.
                            if player_is_falling && intersection.width() > intersection.height() {
                                stomp_events.push(StompEvent { enemy: enemy_entity, player: player_entity });
                            } else {
                                // Otherwise, it's a horizontal (damaging) collision.
                                let knockback_x = if player_pos.0.x < enemy_collision.rect.x() as f32 { -5.0 } else { 5.0 };
                                damage_commands.push(DamageCommand { player: player_entity, knockback_x, position: player_pos.0 });
                            }
                        }
                    }
                }
            }
        }

        // Process stomp events
        for event in stomp_events {
            world.add_dead_tag(event.enemy, DeadTag);
            if let Some(player_vel) = world.velocities.get_mut(&event.player) {
                player_vel.0.y = -4.0; // Bounce
            }
            let _ = context.audio_sender.send(AudioEvent::PlaySound("enemy_stomp".to_string()));
        }

        // Process damage commands
        for command in damage_commands {
            if let Some(health) = world.healths.get_mut(&command.player) {
                if health.current > 0 {
                    health.current -= 1;
                    world.add_invincibility(command.player, Invincibility { timer: 1.5 });

                    if let Some(player_vel) = world.velocities.get_mut(&command.player) {
                        player_vel.0.x = command.knockback_x;
                        player_vel.0.y = -command.knockback_x.abs(); // 45-degree knockback
                    }

                    let _ = context.audio_sender.send(AudioEvent::PlaySound("player_hit".to_string()));

                    // Spawn explosion entity
                    let explosion_entity = world.create_entity();
                    world.add_position(explosion_entity, Position(command.position)); // Position at player
                    world.add_renderable(explosion_entity, Renderable {
                        width: 96,
                        height: 96,
                        horizontal_offset: -48, // Center the explosion
                        vertical_offset: -48,
                        z_index: 101, // Behind player
                    });

                    let mut explosion_anim_controller = AnimationController::new();
                    if let Some(anim_config) = context.game_config.animation.get("explosion") {
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
                        let animation = crate::animation::Animation {
                            texture_name: anim_config.texture.clone(),
                            frames,
                            frame_duration: anim_config.frame_duration,
                            loops: anim_config.loops,
                        };
                        explosion_anim_controller.add_animation("explosion".to_string(), animation);
                        explosion_anim_controller.set_animation("explosion");

                        world.add_animation(explosion_entity, Animation { controller: explosion_anim_controller });
                        world.add_lifetime(explosion_entity, Lifetime { timer: (anim_config.frame_count * anim_config.frame_duration) as f32 / 60.0 });
                    }
                }
            }
        }
    }
}
