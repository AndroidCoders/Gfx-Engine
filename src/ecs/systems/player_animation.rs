use crate::ecs::systems::{System, SystemContext};
use crate::ecs::world::World;

pub struct PlayerAnimationSystem;
impl System<SystemContext<'_>> for PlayerAnimationSystem {
    fn update(&mut self, world: &mut World, _context: &mut SystemContext) {
        for (entity, _) in &world.player_tags {
            if let Some(animation) = world.animations.get_mut(entity) {
                if let Some(state_component) = world.state_components.get(entity) {
                    let current_state_name = state_component.state_machine.current_state.as_ref().map_or("IdleState", |s| s.get_name());

                    let direction = if let Some(vel) = world.velocities.get(entity) {
                        if vel.0.x < 0.0 { "left" } else { "right" }
                    } else {
                        "right"
                    };

                    let animation_name = match current_state_name {
                        "IdleState" => format!("idle_{}", direction),
                        "WalkingState" => format!("walk_{}", direction),
                        "JumpingState" => format!("jump_{}", direction),
                        "FallingState" => format!("fall_{}", direction),
                        _ => format!("idle_{}", direction), // Fallback
                    };
                    
                    if animation.controller.has_animation(&animation_name) {
                        animation.controller.set_animation(&animation_name);
                    } else {
                        animation.controller.set_animation(&format!("idle_{}", direction));
                    }
                }
            }
        }
    }
}
