//! This system is responsible for processing all pending audio events.

use crate::ecs::systems::System;
use crate::ecs::world::World;
use crate::audio::GameAudioManager;

/// The system that processes the audio event queue.
pub struct AudioSystem;
impl System<GameAudioManager> for AudioSystem {
    /// Calls the `process_events` method on the `GameAudioManager` to play
    /// any sounds that have been queued by other systems in the current frame.
    fn update(&mut self, _world: &mut World, context: &mut GameAudioManager) {
        context.process_events();
    }
}
