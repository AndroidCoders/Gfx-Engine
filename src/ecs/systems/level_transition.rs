//! This system is responsible for handling level transitions.

use crate::ecs::systems::{System, SystemContext};
use crate::ecs::world::World;

/// The `LevelTransitionSystem` checks for collisions between the player and goal entities.
pub struct LevelTransitionSystem;
impl System<SystemContext<'_>> for LevelTransitionSystem {
    /// Updates the system, checking for collisions and triggering a level transition if necessary.
    fn update(&mut self, world: &mut World, context: &mut SystemContext) {
        let player_entities: Vec<_> = world.player_tags.keys().copied().collect();
        let goal_entities: Vec<_> = world.goals.keys().copied().collect();

        for &player_entity in &player_entities {
            for &goal_entity in &goal_entities {
                if let Some(player_collision) = world.collisions.get(&player_entity) {
                    if let Some(goal_collision) = world.collisions.get(&goal_entity) {
                        if player_collision.rect.has_intersection(goal_collision.rect) {
                            // Trigger level transition
                            *context.next_level = Some("assets/levels/world_1_level_2.tmx".to_string());
                        }
                    }
                }
            }
        }
    }
}
