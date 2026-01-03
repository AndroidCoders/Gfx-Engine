//! # Synchronization: Universal Rules
//!
//! This module acts as the "Grand Rules Engine" of the engine. 
//! It implements high-level behavioral rules that bridge independent concepts, 
//! such as interpreting raw collisions into semantic gameplay facts (Damage, Collection)
//! and triggering visual transition sequences.

use crate::ecs::event::{
    EventRespawnStarted, EventStartTransition, TransitionType, 
    EventGameOver, EventCollision, EventCoinCollected, 
    EventPlayerEnemyStomped, EventPlayerDamaged
};
use crate::ecs::systems::{System, SystemContext};

/// A system that coordinates multi-domain interactions and visual sequences.
pub struct SystemSynchronization;

impl System<SystemContext<'_>> for SystemSynchronization {
    /// Orchestrates the engine's response to disparate gameplay facts.
    ///
    /// ⚠️ **Hotpath**: Called 120x per second. Contains high-level rule logic.
    ///
    /// # Side Effects
    /// * Publishes [EventStartTransition] during respawn.
    /// * Publishes [EventCoinCollected] on collision with coins.
    /// * Publishes [EventPlayerEnemyStomped] or [EventPlayerDamaged] on collision with enemies.
    fn update(&mut self, world: &mut crate::ecs::world::World, context: &mut SystemContext<'_>) {
        
        // --- 1. Rule: Orchestrate Respawn Sequence ---
        // When a respawn sequence begins, calculate the screen-space center 
        // of the player and trigger a visual Iris Out shutter.
        let respawn_events: Vec<_> = world.event_bus.read::<EventRespawnStarted>().copied().collect();
        for event in respawn_events {
            let mut center = None;
            if let Some(pos) = world.positions.get(&event.player) {
                let mut cx = pos.0.x;
                let mut cy = pos.0.y;
                
                if let Some(rend) = world.renderables.get(&event.player) {
                    cx += rend.horizontal_offset as f32 + (rend.width as f32 / 2.0);
                    cy += rend.vertical_offset as f32 + (rend.height as f32 / 2.0);
                }
                
                let screen_x = cx - context.camera.position.x;
                let screen_y = cy - context.camera.position.y;

                if screen_x.is_nan() || screen_y.is_nan() {
                    center = None;
                } else {
                    center = Some((screen_x as i32, screen_y as i32));
                }
            }

            world.event_bus.publish(EventStartTransition {
                transition_type: TransitionType::IrisOut,
                duration: 2.0,
                center,
            });
            let _ = context.audio_sender.send(crate::audio::AudioEvent::FadeOutMusic(2.0));
        }

        // --- 2. Rule Set: Interpret Raw Collisions ---
        // Consume raw 'Intersection' facts and determine if they represent 
        // semantic actions like collecting items or combat interactions.
        let collisions: Vec<_> = world.event_bus.read::<EventCollision>().copied().collect();
        for collision in collisions {
            self.resolve_collision(world, context, collision);
        }

        // --- 3. Rule: Respond to Game Over ---
        for _ in world.event_bus.read::<EventGameOver>() {
            // Future: Trigger high-level game over visuals or sequence.
        }

        // --- 4. Rule: Keep UI in sync with Game Stats ---
        // Ensure that the visual state of the HUD (ui_state) matches 
        // the authoritative gameplay statistics (stats).
        world.ui_state.display_lives = world.stats.lives;
        world.ui_state.display_coin_count = world.stats.gold_coin_count;
    }
}

impl SystemSynchronization {
    /// Interprets a single collision fact into a semantic gameplay event.
    fn resolve_collision(&self, world: &mut crate::ecs::world::World, context: &SystemContext<'_>, event: EventCollision) {
        let (e1, e2) = (event.entity_a, event.entity_b);

        // Identify the types of entities involved in the interaction.
        let p1 = world.player_tags.contains_key(&e1);
        let p2 = world.player_tags.contains_key(&e2);
        let c1 = world.gold_coins.contains_key(&e1);
        let c2 = world.gold_coins.contains_key(&e2);
        let en1 = world.enemy_tags.contains_key(&e1);
        let en2 = world.enemy_tags.contains_key(&e2);

        // Rule: If a Player overlaps with a Gold Coin -> Publish a Collection fact.
        if (p1 && c2) || (p2 && c1) {
            let coin = if c1 { e1 } else { e2 };
            world.event_bus.publish(EventCoinCollected { coin });
        }

        // Rule: If a Player overlaps with an Enemy -> Determine Stomp vs. Injury.
        if (p1 && en2) || (p2 && en1) {
            let player = if p1 { e1 } else { e2 };
            let enemy = if en1 { e1 } else { e2 };

            // Logic: Skip if the interaction is invalid (invincible or dead player).
            let player_is_invincible = world.invincibilities.contains_key(&player);
            let player_is_dead = world.healths.get(&player).is_some_and(|h| h.current == 0);

            if !player_is_invincible && !player_is_dead {
                // Arbiter Logic: A 'Stomp' is defined as a vertical collision while falling.
                if let (Some(player_pos), Some(player_vel)) = (world.positions.get(&player), world.velocities.get(&player)) {
                    let player_is_falling = player_vel.0.y > 0.0;
                    
                    if player_is_falling && event.intersection.width() > event.intersection.height() {
                        // Vertical overlap is smaller than horizontal -> Stomp fact.
                        world.event_bus.publish(EventPlayerEnemyStomped { enemy, player });
                    } else {
                        // Otherwise, it is a damaging horizontal contact -> Injury fact.
                        let knockback_force = context.game_config.gameplay.damage_knockback_force;
                        
                        let enemy_rect = world.collisions.get(&enemy).map(|c| c.rect);
                        let knockback_x = if let Some(er) = enemy_rect {
                            if player_pos.0.x < er.x() as f32 { -knockback_force } else { knockback_force }
                        } else {
                            knockback_force
                        };

                        world.event_bus.publish(EventPlayerDamaged { 
                            player, 
                            knockback_x, 
                            position: player_pos.0 
                        });
                    }
                }
            }
        }
    }
}
