//! # Concept: Entity Interaction (Detection)
//!
//! This module is responsible for the pure detection of entity-to-entity 
//! physical overlaps. It identifies when two bounding boxes intersect and 
//! publishes a raw 'Collision' fact, leaving the interpretation of that fact 
//! to synchronization systems.

use crate::ecs::event::EventCollision;
use crate::ecs::systems::{System, SystemContext};

/// A system that identifies overlaps between entities and publishes raw collision facts.
pub struct SystemInteraction;

impl System<SystemContext<'_>> for SystemInteraction {
    /// Detects physical intersections between collidable entities.
    ///
    /// ⚠️ **Hotpath**: Called 120x per second. Performs spatial queries.
    ///
    /// # Side Effects
    /// * Publishes [crate::ecs::event::EventCollision] when two active entities intersect.
    fn update(&mut self, world: &mut crate::ecs::world::World, _context: &mut SystemContext<'_>) {
        // 1. Collect all entities that have a physical presence (Collision component).
        let entities: Vec<_> = world.collisions.keys().copied().collect();

        for entity_a in entities {
            if let Some(coll_a) = world.collisions.get(&entity_a) {
                
                // 2. Query the Spatial Grid for nearby entities to avoid O(N^2) complexity.
                let neighbors = world.spatial_grid.query(coll_a.rect);

                for entity_b in neighbors {
                    // 3. Ensure we don't check an entity against itself or re-check the same pair.
                    if entity_a >= entity_b { continue; }

                    if let Some(coll_b) = world.collisions.get(&entity_b)
                        && let Some(intersection) = coll_a.rect.intersection(coll_b.rect) {
                            
                            // 4. Publish a raw 'Fact' that a collision has occurred.
                            world.event_bus.publish(EventCollision {
                                entity_a,
                                entity_b,
                                intersection,
                            });
                    }
                }
            }
        }
    }
}
