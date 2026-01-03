//! # Concept: Game Resolution (The Executor)
//! 
//! This module acts as the authoritative state mutator. It consumes semantic 
//! events (from Synchronization) and applies the consequences to the world 
//! (Death, Score, Health). It also triggers sensory feedback (Audio/UI).

use crate::ecs::systems::{System, SystemContext};
use crate::ecs::component::{DeadTag, Invincibility, Position, Renderable, Lifetime, Animation};
use crate::ecs::event::{EventCoinCollected, EventPlayerDamaged, EventPlayerEnemyStomped};
use crate::animation::AnimationController;
use crate::audio::{AudioEvent, PlaySoundParams};

pub struct SystemGameResolution;

impl System<SystemContext<'_>> for SystemGameResolution {
    fn update(&mut self, world: &mut crate::ecs::world::World, context: &mut SystemContext<'_>) {
        self.handle_coin_collection(world, context);
        self.handle_player_damage(world, context);
        self.handle_enemy_stomp(world, context);
        self.cleanup_dead(world);
    }
}

impl SystemGameResolution {
    fn handle_coin_collection(&mut self, world: &mut crate::ecs::world::World, context: &mut SystemContext<'_>) {
        let events: Vec<crate::ecs::world::Entity> = world.event_bus.read::<EventCoinCollected>().map(|e| e.coin).collect();
        for coin_entity in events { 
            // 1. Mutate State
            world.add_dead_tag(coin_entity, DeadTag); 
            world.stats.gold_coin_count += 1; 
            
            // 2. Trigger Feedback
            if let Some(sound_name) = context.game_config.sound_events.get("coin_pickup") {
                let _ = context.audio_sender.send(AudioEvent::PlaySound(sound_name.clone(), PlaySoundParams::default()));
            }
        }
    }

    fn handle_player_damage(&mut self, world: &mut crate::ecs::world::World, context: &mut SystemContext<'_>) {
        let events: Vec<EventPlayerDamaged> = world.event_bus.read::<EventPlayerDamaged>().copied().collect();
        for event in events {
             if let Some(health) = world.healths.get_mut(&event.player) && health.current > 0 {
                    // 1. Mutate State
                    health.current -= 1;
                    world.add_invincibility(event.player, Invincibility { timer: context.game_config.gameplay.damage_invincibility_duration });
                    
                    // Knockback
                    if let Some(player_vel) = world.velocities.get_mut(&event.player) { 
                        player_vel.0.x = event.knockback_x; 
                        player_vel.0.y = -event.knockback_x.abs(); 
                    }
                    
                    // Spawn Effect (Explosion)
                    let explosion_config = &context.game_config.gameplay.explosion;
                    let explosion_entity = world.create_entity();
                    world.add_position(explosion_entity, Position(event.position));
                    world.add_renderable(explosion_entity, Renderable { width: explosion_config.width, height: explosion_config.height, horizontal_offset: explosion_config.horizontal_offset, vertical_offset: explosion_config.vertical_offset, z_index: explosion_config.z_index, rotation: 0.0, flip_horizontal: false, flip_vertical: false });
                    
                    if let Some(anim_config) = context.game_config.animation.get(&explosion_config.animation_name) {
                        let mut frames = Vec::new();
                        for i in 0..anim_config.frame_count {
                            let padding = anim_config.frame_padding.unwrap_or(0);
                            frames.push(sdl3::rect::Rect::new(anim_config.start_x + (i * (anim_config.frame_width + padding)) as i32, anim_config.start_y, anim_config.frame_width, anim_config.frame_height));
                        }
                        let mut explosion_anim_controller = AnimationController::new();
                        explosion_anim_controller.add_animation(explosion_config.animation_name.clone(), crate::animation::Animation { texture_name: anim_config.texture.clone(), frames, frame_duration: anim_config.frame_duration, loops: anim_config.loops });
                        explosion_anim_controller.set_animation(&explosion_config.animation_name);
                        world.add_animation(explosion_entity, Animation { controller: explosion_anim_controller });
                        world.add_lifetime(explosion_entity, Lifetime { timer: (anim_config.frame_count * anim_config.frame_duration) as f32 / 60.0 });
                    }

                    // 2. Trigger Feedback
                    if let Some(sound_name) = context.game_config.sound_events.get("player_hit") {
                        let _ = context.audio_sender.send(AudioEvent::PlaySound(sound_name.clone(), PlaySoundParams::default()));
                    }
                }
        }
    }

    fn handle_enemy_stomp(&mut self, world: &mut crate::ecs::world::World, context: &mut SystemContext<'_>) {
        let events: Vec<EventPlayerEnemyStomped> = world.event_bus.read::<EventPlayerEnemyStomped>().copied().collect();
        for event in events { 
            // 1. Mutate State
            world.add_dead_tag(event.enemy, DeadTag); 
            if let Some(player_vel) = world.velocities.get_mut(&event.player) { 
                player_vel.0.y = context.game_config.gameplay.stomp_bounce_velocity; 
            }
            
            // 2. Trigger Feedback
            if let Some(sound_name) = context.game_config.sound_events.get("enemy_stomp") {
                let _ = context.audio_sender.send(AudioEvent::PlaySound(sound_name.clone(), PlaySoundParams::default()));
            }
        }
    }

    fn cleanup_dead(&mut self, world: &mut crate::ecs::world::World) {
        let to_remove: Vec<_> = world.dead_tags.keys().copied().collect();
        for entity in to_remove {
            world.positions.remove(&entity); world.velocities.remove(&entity); world.accelerations.remove(&entity); world.renderables.remove(&entity); world.animations.remove(&entity); world.player_tags.remove(&entity); world.gold_coins.remove(&entity); world.enemy_tags.remove(&entity); world.dead_tags.remove(&entity); world.patrols.remove(&entity); world.gravity_tags.remove(&entity); world.collisions.remove(&entity); world.grounded_tags.remove(&entity); world.state_components.remove(&entity); world.respawn_tags.remove(&entity); world.respawn_timers.remove(&entity); world.healths.remove(&entity); world.invincibilities.remove(&entity); world.lifetimes.remove(&entity); world.directions.remove(&entity); world.goals.remove(&entity); world.next_levels.remove(&entity); world.movement_intentions.remove(&entity); world.dormant_tags.remove(&entity); world.wall_hits.remove(&entity);
        }
    }
}
