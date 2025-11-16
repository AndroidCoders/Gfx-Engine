//! This system reads player input and updates player components accordingly.

use crate::ecs::systems::{System, SystemContext};
use crate::ecs::world::{Entity, World};
use crate::ecs::event::PlayerJumpEvent;
use crate::input::PlayerAction;

/// The system responsible for processing player input.
pub struct InputSystem;

impl System<SystemContext<'_>> for InputSystem {
    /// Reads the current input state and applies it to the player entity.
    ///
    /// This includes:
    /// - Horizontal movement (acceleration and deceleration).
    /// - Publishing a `PlayerJumpEvent` on the initial jump press.
    /// - Applying a "jump hold" force if the jump button is held.
    fn update(&mut self, world: &mut World, context: &mut SystemContext) {
        let player_entities: Vec<Entity> = world.player_tags.keys().copied().collect();

        for &player_entity in &player_entities {
            // --- Horizontal Movement ---
            if let Some(velocity) = world.velocities.get_mut(&player_entity) {
                let acceleration = context.config.physics.acceleration;
                let deceleration = context.config.physics.deceleration;
                let max_speed = context.config.physics.max_speed;

                if context.input_state.is_action_active(PlayerAction::MoveLeft) {
                    velocity.0.x -= acceleration * context.delta_time;
                    if velocity.0.x < -max_speed {
                        velocity.0.x = -max_speed;
                    }
                } else if context.input_state.is_action_active(PlayerAction::MoveRight) {
                    velocity.0.x += acceleration * context.delta_time;
                    if velocity.0.x > max_speed {
                        velocity.0.x = max_speed;
                    }
                } else {
                    // Deceleration
                    if velocity.0.x > 0.0 {
                        velocity.0.x -= deceleration * context.delta_time;
                        if velocity.0.x < 0.0 {
                            velocity.0.x = 0.0;
                        }
                    } else if velocity.0.x < 0.0 {
                        velocity.0.x += deceleration * context.delta_time;
                        if velocity.0.x > 0.0 {
                            velocity.0.x = 0.0;
                        }
                    }
                }
            }

            // --- Jump Logic ---
            // Initial jump event
            if context.input_state.is_action_just_pressed(PlayerAction::Jump) {
                world.event_bus.publish(PlayerJumpEvent { player: player_entity });
            }

            // Jump hold
            if context.input_state.is_action_active(PlayerAction::Jump) {
                let is_grounded = world.is_grounded(player_entity);
                if !is_grounded
                    && let Some(velocity) = world.velocities.get_mut(&player_entity) {
                        velocity.0.y -= context.config.physics.jump_hold_force * context.delta_time;
                    }
            }
        }
    }
}
