//! # Concept: Event Bus
//! 
//! This module provides the type-based communication infrastructure.
//! It allows systems to publish semantic 'Facts' (Events) or 'Intents' (Commands) 
//! without direct coupling, enabling the WYSIWID behavioral rule layer.

use crate::ecs::world::Entity;
use crate::math::Vector2D;
use std::any::{Any, TypeId};
use std::collections::HashMap;

// --- Gameplay Facts (Events) ---
#[derive(Clone, Copy)] pub struct EventCoinCollected { pub coin: Entity }
#[derive(Clone, Copy)] pub struct EventPlayerEnemyStomped { pub player: Entity, pub enemy: Entity }
#[derive(Clone, Copy)] pub struct EventPlayerDamaged { pub player: Entity, pub knockback_x: f32, pub position: Vector2D }
#[derive(Clone, Copy)] pub struct EventEntityJumped { pub entity: Entity }
#[derive(Clone, Copy)] pub struct EventCollision { pub entity_a: Entity, pub entity_b: Entity, pub intersection: sdl3::rect::Rect }
#[derive(Clone, Copy)] pub struct EventGameOver;
#[derive(Clone, Copy)] pub struct EventRespawnStarted { pub player: Entity }
#[derive(Clone, Copy)] pub struct EventMusicBeat { #[allow(dead_code)] pub beat_number: u32, #[allow(dead_code)] pub intensity: f32 }
#[derive(Clone, Copy)] pub struct EventScreenShake { pub duration: f32, pub intensity: f32 }

#[derive(Clone, Copy, Debug, PartialEq)] pub enum PlayerDeathReason { HealthDepleted, FellOutOfBounds }
#[derive(Clone, Copy)] pub struct EventPlayerDied { pub player: Entity, pub reason: PlayerDeathReason }

#[derive(Clone, Copy, PartialEq, Debug)] pub enum TransitionType { IrisIn, IrisOut }
#[derive(Clone, Copy)] pub struct EventStartTransition { pub transition_type: TransitionType, pub duration: f32, pub center: Option<(i32, i32)> }
#[derive(Clone, Copy)] pub struct EventTransitionComplete { #[allow(dead_code)] pub transition_type: TransitionType }

// --- Semantic Intents (Commands) ---
#[derive(Clone, Copy)] pub struct CommandJump { pub entity: Entity }

/// A central bus for managing strongly-typed fact and intent queues.
#[derive(Default)]
pub struct EventBus {
    /// Storage for all pending events, indexed by their specific Rust TypeId.
    queues: HashMap<TypeId, Vec<Box<dyn Any>>>,
}

impl EventBus {
    /// Publishes a new fact or intent to the appropriate typed queue.
    pub fn publish<T: Any + 'static>(&mut self, event: T) {
        // 1. Identify the unique TypeId for the provided data structure.
        let type_id = TypeId::of::<T>();
        // 2. File the data into the corresponding vector for consumption this frame.
        let queue = self.queues.entry(type_id).or_default();
        queue.push(Box::new(event));
    }

    /// Provides an iterator over all facts of a specific type published this frame.
    pub fn read<T: Any + 'static>(&self) -> impl Iterator<Item = &T> {
        // 1. Look up the specific queue for the requested type.
        let type_id = TypeId::of::<T>();
        // 2. Provide a filtered iterator that safely downcasts back to the concrete type.
        self.queues
            .get(&type_id)
            .map_or_else(
                || [].iter(), 
                |queue| queue.iter(),
            )
            .filter_map(|event| event.downcast_ref::<T>())
    }

    /// Wipes all queues to prepare for the next simulation tick.
    pub fn clear_events(&mut self) {
        // 1. Clear all mappings to flush every event from memory.
        self.queues.clear();
    }
}