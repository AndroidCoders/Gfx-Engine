use crate::ecs::systems::{System, SystemContext};
use crate::ecs::world::World;

pub struct InputSystem;
impl System<SystemContext<'_>> for InputSystem {
    fn update(&mut self, _world: &mut World, _context: &mut SystemContext) {
        // No direct action here, input_state is updated in app.rs and passed to systems
    }
}
