//! # Concept: Entity Locomotion
//! 
//! This module standardizes how all entities (Players and Enemies) move 
//! through the physical world. It translates abstract 'Intentions' and 
//! 'Commands' into concrete velocity changes, handling acceleration, 
//! friction, and air control.

use crate::ecs::systems::{System, SystemContext};
use crate::ecs::event::{CommandJump, EventEntityJumped};

/// A system that applies movement forces and jump logic to entities.
pub struct SystemMovement;

impl System<SystemContext<'_>> for SystemMovement {

    /// Processes movement intentions and jump commands to update entity velocities.

    ///

    /// ⚠️ **Hotpath**: Called 120x per second. Contains tight arithmetic loops.

    ///

    /// # Side Effects

    /// * Consumes [crate::ecs::event::CommandJump] to trigger impulses.

    /// * Publishes [crate::ecs::event::EventEntityJumped] when a jump occurs.

    fn update(&mut self, world: &mut crate::ecs::world::World, context: &mut SystemContext<'_>) {

        let physics_config = &context.config.physics;



        // 1. Process Horizontal Movement Intentions (Locomotion).

        let entities: Vec<_> = world.movement_intentions.keys().copied().collect();



        for entity in entities {

            // Skip entities outside the active simulation range.

            if world.is_dormant(entity) { continue; }



            if let Some(intention) = world.movement_intentions.get(&entity) {

                let direction = intention.x;

                

                // Determine the Maximum Speed based on entity type (Patrol vs Player).

                let max_speed = if let Some(patrol) = world.patrols.get(&entity) {

                    patrol.speed

                } else {

                    physics_config.max_speed

                };



                // Calculate the Target Velocity based on intent.

                let target_speed = direction * max_speed;

                let is_grounded = world.is_grounded(entity);



                if let (Some(vel), Some(accel)) = (world.velocities.get_mut(&entity), world.accelerations.get_mut(&entity)) {

                    let current_speed = vel.0.x;



                                    // Choose acceleration vs deceleration based on input presence.

                                    let accel_value = if direction != 0.0 {

                                        physics_config.acceleration

                                    } else {

                                        physics_config.deceleration

                                    };

                    

                                    // Apply Air Control damping if the entity is not grounded.

                                    let final_accel = if is_grounded { accel_value } else { accel_value * physics_config.air_control_factor };

                    

                                    // Integrate acceleration into velocity, clamping to the target speed to prevent overshoot.

                                    let delta_v = final_accel * context.delta_time;

                    

                                    if (current_speed - target_speed).abs() <= delta_v {

                                        vel.0.x = target_speed;

                                        accel.0.x = 0.0;

                                    } else if current_speed < target_speed {

                                        accel.0.x = final_accel;

                                                    } else {

                                                        accel.0.x = -final_accel;

                                                    }

                                                } else if let Some(intention) = world.movement_intentions.get(&entity) 

                                                    && intention.x != 0.0 && !world.accelerations.contains_key(&entity) {

                                                         println!("[SystemMovement] WARNING: Entity {:?} wants to move (Intention: {}) but has no Acceleration component!", entity, intention.x);

                                                }

                                            }        }



        // 2. Process Jump Commands (Impulse).

        let mut jump_commands = Vec::new();

        for cmd in world.event_bus.read::<CommandJump>() {

            jump_commands.push(*cmd);

        }



        for cmd in jump_commands {

            // Only allow jumping from a solid surface (Grounded).

            if world.is_grounded(cmd.entity)

                && let Some(vel) = world.velocities.get_mut(&cmd.entity) {

                    // Apply an instantaneous upward impulse.

                    vel.0.y = physics_config.jump_strength;

                    // Publish a fact that the entity has jumped (for Audio/FX).

                    world.event_bus.publish(EventEntityJumped { entity: cmd.entity });

                }

        }

    }

}
