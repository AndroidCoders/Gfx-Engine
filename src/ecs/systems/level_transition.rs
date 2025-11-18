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
                            // Check if the goal entity has a NextLevel component
                            if let Some(next_level_comp) = world.next_levels.get(&goal_entity) {
                                // Trigger level transition using the path from the component.
                                *context.next_level = Some(next_level_comp.0.clone());
                            }
                        }
            }
        }
    }
}
