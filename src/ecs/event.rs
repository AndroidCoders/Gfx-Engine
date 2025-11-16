//! This module defines the type-based event bus for the ECSC architecture.

use crate::ecs::world::Entity;
use crate::math::Vector2D;
use std::any::{Any, TypeId};
use std::collections::HashMap;

/// An event published when the player collects a gold coin.
pub struct CoinCollectedEvent;

/// An event published when the player stomps on an enemy.
#[derive(Clone, Copy)]
pub struct PlayerStompedEnemyEvent {
    pub player: Entity,
    pub enemy: Entity,
}

/// An event published when the player takes damage from an enemy.
#[derive(Clone, Copy)]
pub struct PlayerTookDamageEvent {
    pub player: Entity,
    pub knockback_x: f32,
    pub position: Vector2D,
}

/// An event published when the player presses the jump button.
#[derive(Clone, Copy)]
pub struct PlayerJumpEvent {
    pub player: Entity,
}

/// A type-based event bus for decoupled communication between systems.
///
/// The `EventBus` allows systems to publish event structs without needing to
/// know about the systems that will handle them. Other systems ("Conductors")
/// can then read all events of a specific type for the current frame.
#[derive(Default)]
pub struct EventBus {
    /// A map where the key is the `TypeId` of an event struct, and the value
    /// is a vector of `Box<dyn Any>` instances of that event.
    queues: HashMap<TypeId, Vec<Box<dyn Any>>>,
}

impl EventBus {
    /// Publishes an event to the bus.
    ///
    /// The event is stored in a queue corresponding to its type.
    ///
    /// # Type Parameters
    ///
    /// * `T`: The type of the event struct. Must have a `'static` lifetime.
    ///
    /// # Arguments
    ///
    /// * `event`: The event instance to publish.
    pub fn publish<T: Any + 'static>(&mut self, event: T) {
        let type_id = TypeId::of::<T>();
        let queue = self.queues.entry(type_id).or_default();
        queue.push(Box::new(event));
    }

    /// Reads all events of a specific type for the current frame.
    ///
    /// This returns an iterator over references to the event structs. The events
    /// themselves remain on the bus until `clear_events` is called.
    ///
    /// # Type Parameters
    ///
    /// * `T`: The type of the event struct to read. Must have a `'static` lifetime.
    ///
    /// # Returns
    ///
    /// An iterator that yields references to all published events of type `T`.
    /// Returns an empty iterator if no events of that type have been published.
    pub fn read<T: Any + 'static>(&self) -> impl Iterator<Item = &T> {
        let type_id = TypeId::of::<T>();
        self.queues
            .get(&type_id)
            .map_or_else(
                || [].iter(), // Return an empty iterator if no queue for this type
                |queue| queue.iter(),
            )
            .filter_map(|event| event.downcast_ref::<T>())
    }

    /// Clears all event queues.
    ///
    /// This should be called at the end of each frame after all systems have
    /// processed the events for that frame.
    pub fn clear_events(&mut self) {
        self.queues.clear();
    }
}
