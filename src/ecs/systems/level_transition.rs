//! This system is responsible for handling the transition to the next level.

use crate::ecs::systems::{System, SystemContext};
use crate::ecs::world::World;

/// The system that triggers a level transition when the player reaches the goal.
pub struct LevelTransitionSystem;
impl System<SystemContext<'_>> for LevelTransitionSystem {
    /// Checks for collisions between any player and goal entities.
    ///
    /// If an intersection is found, it sets the `next_level` field in the
    /// `SystemContext`, which will be detected by the main application loop
    /// to initiate the loading of the next level.
    fn update(&mut self, world: &mut World, context: &mut SystemContext) {
        let player_entities: Vec<_> = world.player_tags.keys().copied().collect();
        let goal_entities: Vec<_> = world.goals.keys().copied().collect();

        for &player_entity in &player_entities {
            for &goal_entity in &goal_entities {
                if let Some(player_collision) = world.collisions.get(&player_entity)
                    && let Some(goal_collision) = world.collisions.get(&goal_entity)
                        && player_collision.rect.has_intersection(goal_collision.rect) {
                            // Trigger level transition by setting the path to the next level file.
                            // TODO: Make the next level path data-driven instead of hardcoded.
                            *context.next_level = Some("assets/levels/world_1_level_2.tmx".to_string());
                        }
            }
        }
    }
}
