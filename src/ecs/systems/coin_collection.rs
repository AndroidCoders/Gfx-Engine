use crate::ecs::component::DeadTag;
use crate::ecs::systems::{System, SystemContext};
use crate::ecs::world::{Entity, World};
use crate::audio::AudioEvent;

pub struct CoinCollectionSystem;
impl System<SystemContext<'_>> for CoinCollectionSystem {
    fn update(&mut self, world: &mut World, context: &mut SystemContext) {
        let player_entities: Vec<Entity> = world.player_tags.keys().copied().collect();
        let gold_coin_entities: Vec<Entity> = world.gold_coins.keys().copied().collect();
        let mut collected_coins = Vec::new();

        for &player_entity in &player_entities {
            for &coin_entity in &gold_coin_entities {
                if let Some(player_collision) = world.collisions.get(&player_entity) {
                    if let Some(coin_collision) = world.collisions.get(&coin_entity) {
                        if player_collision.rect.has_intersection(coin_collision.rect) {
                            collected_coins.push(coin_entity);
                        }
                    }
                }
            }
        }

        for coin_entity in collected_coins {
            world.add_dead_tag(coin_entity, DeadTag);
            *context.gold_coin_count += 1;
            let _ = context.audio_sender.send(AudioEvent::PlaySound("coin_pickup".to_string()));
        }
    }
}
