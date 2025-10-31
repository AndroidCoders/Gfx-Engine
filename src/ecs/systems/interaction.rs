use crate::ecs::component::{DeadTag};
use crate::ecs::systems::{System, SystemContext};
use crate::ecs::world::{Entity, World};

pub struct InteractionSystem;

struct StompEvent {
    enemy: Entity,
    player: Entity,
}

impl System<SystemContext<'_>> for InteractionSystem {
    fn update(&mut self, world: &mut World, _context: &mut SystemContext) {
        let mut stomp_events = Vec::new();

        let player_entities: Vec<Entity> = world.player_tags.keys().copied().collect();
        let enemy_entities: Vec<Entity> = world.enemy_tags.keys().copied().collect();

        for &player_entity in &player_entities {
            for &enemy_entity in &enemy_entities {
                if let (Some(player_pos), Some(player_vel), Some(player_collision)) = (world.positions.get(&player_entity), world.velocities.get(&player_entity), world.collisions.get(&player_entity)) {
                    if let Some(enemy_collision) = world.collisions.get(&enemy_entity) {
                        let player_rect = player_collision.rect;
                        let enemy_rect = enemy_collision.rect;

                        if player_rect.has_intersection(enemy_rect) {
                            // Stomp check
                            if player_vel.0.y > 0.0 && player_pos.0.y + player_rect.height() as f32 - player_vel.0.y <= enemy_collision.rect.y() as f32 {
                                stomp_events.push(StompEvent { enemy: enemy_entity, player: player_entity });
                            }
                        }
                    }
                }
            }
        }

        for event in stomp_events {
            world.add_dead_tag(event.enemy, DeadTag);
            if let Some(player_vel) = world.velocities.get_mut(&event.player) {
                player_vel.0.y = -4.0; // Bounce
            }
        }
    }
}
