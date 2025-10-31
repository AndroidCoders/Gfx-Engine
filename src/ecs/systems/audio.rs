use crate::ecs::systems::System;
use crate::ecs::world::World;
use crate::audio::GameAudioManager;

pub struct AudioSystem;
impl System<GameAudioManager> for AudioSystem {
    fn update(&mut self, _world: &mut World, context: &mut GameAudioManager) {
        context.process_events();
    }
}
