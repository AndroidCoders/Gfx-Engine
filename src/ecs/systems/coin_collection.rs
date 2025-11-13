//! This system handles the player's collection of gold coins.

use crate::ecs::component::DeadTag;
use crate::ecs::systems::{System, SystemContext};
use crate::ecs::world::{Entity, World};
use crate::audio::AudioEvent;

/// The system responsible for detecting and processing coin collection events.
pub struct CoinCollectionSystem;
impl System<SystemContext<'_>> for CoinCollectionSystem {
    /// Checks for collisions between the player and any gold coin entities.
    ///
    /// If a collision is detected, the coin is marked as "dead" to be removed,
    /// the player's gold count is incremented, and a pickup sound is played.
    fn update(&mut self, world: &mut World, context: &mut SystemContext) {
        let player_entities: Vec<Entity> = world.player_tags.keys().copied().collect();
        let gold_coin_entities: Vec<Entity> = world.gold_coins.keys().copied().collect();
        let mut collected_coins = Vec::new();

        for &player_entity in &player_entities {
            for &coin_entity in &gold_coin_entities {
                if let Some(player_collision) = world.collisions.get(&player_entity)
                    && let Some(coin_collision) = world.collisions.get(&coin_entity)
                        && player_collision.rect.has_intersection(coin_collision.rect) {
                            collected_coins.push(coin_entity);
                        }
            }
        }

        for coin_entity in collected_coins {
            world.add_dead_tag(coin_entity, DeadTag);
            *context.gold_coin_count += 1;
            if let Some(sound_name) = context.game_config.sound_events.get("coin_pickup") {
                let _ = context.audio_sender.send(AudioEvent::PlaySound(sound_name.clone()));
            }
        }
    }
}
