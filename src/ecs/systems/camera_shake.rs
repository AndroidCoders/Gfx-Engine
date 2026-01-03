//! # Concept: Screen Shake
//! 
//! This module manages the visual impact of high-energy events. 
//! it tracks 'Trauma' levels and translates them into random camera offsets 
//! to create a physical sense of impact and intensity.

use crate::ecs::systems::SystemContext;
use crate::ecs::event::{EventScreenShake, EventPlayerDamaged, EventPlayerEnemyStomped};
use crate::math::Vector2D;
use rand::Rng;

/// A system that manages camera trauma and calculates random screen shake offsets.
pub struct SystemCameraShake {
    pub trauma: f32,
    pub shake_timer: f32,
}

impl SystemCameraShake {
    pub fn new() -> Self {
        Self {
            trauma: 0.0,
            shake_timer: 0.0,
        }
    }

    /// Updates trauma levels and calculates the current frame's camera offset.
    pub fn update(&mut self, world: &mut crate::ecs::world::World, context: &mut SystemContext<'_>) {
        // 1. Process explicit ScreenShake facts to increase trauma levels.
        for event in world.event_bus.read::<EventScreenShake>() {
            self.trauma = (self.trauma + event.intensity).min(1.0);
            self.shake_timer = event.duration.max(self.shake_timer);
        }

        // 2. Automatically trigger trauma based on significant gameplay facts.
        // Player Damaged -> Heavy Impact
        if world.event_bus.read::<EventPlayerDamaged>().count() > 0 {
             self.trauma = (self.trauma + 0.8).min(1.0);
             self.shake_timer = 0.5f32.max(self.shake_timer);
        }
        
        // Enemy Stomped -> Light Impact
        if world.event_bus.read::<EventPlayerEnemyStomped>().count() > 0 {
             self.trauma = (self.trauma + 0.3).min(1.0);
             self.shake_timer = 0.2f32.max(self.shake_timer);
        }

        // 3. Update the trauma decay and calculate the resulting offset.
        if self.shake_timer > 0.0 {
            self.shake_timer -= context.delta_time;
            
            // Linear decay of trauma over time.
            self.trauma = (self.trauma - context.delta_time).max(0.0);

            // Calculate magnitude using squared trauma for a more natural 'impact' feel.
            let shake_magnitude = self.trauma * self.trauma * 20.0;

            let mut rng = rand::rng();
            let offset_x = rng.random_range(-shake_magnitude..shake_magnitude);
            let offset_y = rng.random_range(-shake_magnitude..shake_magnitude);

            // 4. Apply the calculated displacement to the camera context.
            context.camera.shake_offset = Vector2D::new(offset_x, offset_y);
        } else {
            // Reset to identity when trauma is depleted.
            self.trauma = 0.0;
            context.camera.shake_offset = Vector2D::default();
        }
    }
}

impl Default for SystemCameraShake {
    fn default() -> Self {
        Self::new()
    }
}
