//! # Manager: System Orchestration
//! 
//! This module is the "Main Brain" of the ECS. It defines the explicit 
//! execution order of all systems, implementing the WYSIWID scheduler. 
//! It ensures that logic, physics, and presentation stay in perfect sync.

use crate::ecs::systems::{
    System, SystemContext,
    physics::SystemPhysics,
    movement::SystemMovement,
    animation_update::SystemAnimationUpdate,
    input::SystemInput,
    tile_collision::SystemTileCollision,
    gui_render::SystemGUIRender,
    debug_render::SystemDebugRender,
    game_flow::SystemGameFlow,
    transition::SystemTransition,
    interaction::SystemInteraction,
    concept_health::ConceptHealth,
    concept_vitality::ConceptVitality,
    rule_player_death::RulePlayerDeath,
    rule_respawn::RuleRespawn,
    game_resolution::SystemGameResolution,
    audio_synchronization::SystemAudioSynchronization,
    level_transition::SystemWorldLevelTransition,
    spatial_update::SystemSpatialUpdate,
    enemy_rhythm::SystemEnemyRhythm,
    state_machine::SystemStateMachine,
    audio::SystemAudio,
    camera_shake::SystemCameraShake,
    animation_synchronization::SystemAnimationSynchronization,
    EnemyRhythmContext,
};
use crate::ecs::world::World;
use crate::audio::GameAudioManager;
use crate::ecs::resources::GameState;

/// The container for all engine systems, responsible for the main update pass.
pub struct SystemManager {
    input_system: SystemInput,
    movement_system: SystemMovement,
    physics_system: SystemPhysics,
    spatial_update_system: SystemSpatialUpdate,
    tile_collision_system: SystemTileCollision,
    interaction_system: SystemInteraction,
    animation_synchronization_system: SystemAnimationSynchronization,
    animation_update_system: SystemAnimationUpdate,
    state_machine_system: SystemStateMachine,
    synchronization_system: crate::ecs::systems::synchronization::SystemSynchronization,
    audio_system: SystemAudio,
    audio_synchronization_system: SystemAudioSynchronization,
    game_flow_system: SystemGameFlow,
    level_transition_system: SystemWorldLevelTransition,
    enemy_rhythm_system: SystemEnemyRhythm,
    camera_shake_system: SystemCameraShake,
    dormancy_system: crate::ecs::systems::dormancy::SystemDormancy,
    concept_health: ConceptHealth,
    concept_vitality: ConceptVitality,
    rule_player_death: RulePlayerDeath,
    rule_respawn: RuleRespawn,
    game_resolution_system: SystemGameResolution,
    pub debug_render_system: SystemDebugRender,
    pub gui_render_system: SystemGUIRender,
    pub transition_system: SystemTransition,
}

impl SystemManager {
    pub fn new() -> Self {
        Self {
            input_system: SystemInput, movement_system: SystemMovement,
            physics_system: SystemPhysics, spatial_update_system: SystemSpatialUpdate,
            tile_collision_system: SystemTileCollision, interaction_system: SystemInteraction,
            animation_synchronization_system: SystemAnimationSynchronization,
            animation_update_system: SystemAnimationUpdate, state_machine_system: SystemStateMachine,
            synchronization_system: crate::ecs::systems::synchronization::SystemSynchronization,
            audio_system: SystemAudio, audio_synchronization_system: SystemAudioSynchronization,
            game_flow_system: SystemGameFlow, level_transition_system: SystemWorldLevelTransition::new(),
            enemy_rhythm_system: SystemEnemyRhythm::new(), camera_shake_system: SystemCameraShake::new(),
            dormancy_system: crate::ecs::systems::dormancy::SystemDormancy, 
            concept_health: ConceptHealth, concept_vitality: ConceptVitality,
            rule_player_death: RulePlayerDeath, rule_respawn: RuleRespawn,
            game_resolution_system: SystemGameResolution,
            debug_render_system: SystemDebugRender, gui_render_system: SystemGUIRender, transition_system: SystemTransition::new(),
        }
    }

    /// Orchestrates the entire engine update sequence in a deterministic order.
    pub fn update(
        &mut self,
        world: &mut World,
        context: &mut SystemContext,
        audio_manager: &mut GameAudioManager,
    ) {
        world.snapshot_positions();
        audio_manager.process_events(); 

        if let Some(handle) = &audio_manager.current_music_handle {
             let position = handle.position();
             if position < world.music_state.current_time { world.music_state.last_beat = None; }
             world.music_state.current_time = position;
             if let Some(beat_map) = &audio_manager.current_beat_map {
                 let last_beat_time = world.music_state.last_beat.map(|b| b.time).unwrap_or(-1.0);
                 let mut best_beat = None;
                 for beat in beat_map {
                     if beat.time as f64 <= position && beat.time as f64 > last_beat_time as f64 { best_beat = Some(*beat); }
                     if beat.time as f64 > position { break; }
                 }
                 if let Some(new_beat) = best_beat {
                      world.music_state.last_beat = Some(new_beat);
                      use crate::ecs::event::EventMusicBeat;
                      world.event_bus.publish(EventMusicBeat { beat_number: 0, intensity: new_beat.intensity });
                 }
             }
        }

        context.benchmarker.push("Input"); self.input_system.update(world, context); context.benchmarker.pop();

        let should_run_gameplay = match &world.game_state {
            GameState::Playing | GameState::GameOver => !context.is_paused,
            GameState::Menu(_) => context.is_attract_mode,
            _ => false,
        };

        if should_run_gameplay {
            context.benchmarker.push("Dormancy"); self.dormancy_system.update(world, context); context.benchmarker.pop();
            context.benchmarker.push("Movement"); self.movement_system.update(world, context); context.benchmarker.pop();
            context.benchmarker.push("Physics"); self.physics_system.update(world, context); context.benchmarker.pop();
            context.benchmarker.push("SpatialUpdate"); self.spatial_update_system.update(world, context); context.benchmarker.pop();
            context.benchmarker.push("Interaction"); self.interaction_system.update(world, context); context.benchmarker.pop();
            
            // New Decomposed Lifecycle Systems
            context.benchmarker.push("ConceptVitality"); self.concept_vitality.update(world, context); context.benchmarker.pop();
            context.benchmarker.push("ConceptHealth"); self.concept_health.update(world, context); context.benchmarker.pop();
            context.benchmarker.push("RulePlayerDeath"); self.rule_player_death.update(world, context); context.benchmarker.pop();
            context.benchmarker.push("RuleRespawn"); self.rule_respawn.update(world, context); context.benchmarker.pop();
            
            context.benchmarker.push("Synchronization"); self.synchronization_system.update(world, context); context.benchmarker.pop();
            context.benchmarker.push("GameResolution"); self.game_resolution_system.update(world, context); context.benchmarker.pop();
            context.benchmarker.push("TileCollision"); self.tile_collision_system.update(world, context); context.benchmarker.pop();
            context.benchmarker.push("GameFlow"); self.game_flow_system.update(world, context); context.benchmarker.pop();
            context.benchmarker.push("LevelTransition"); self.level_transition_system.update(world, context); context.benchmarker.pop();
            { let mut erc = EnemyRhythmContext { game_config: context.game_config, delta_time: context.delta_time, camera: context.camera };
                context.benchmarker.push("EnemyRhythm"); self.enemy_rhythm_system.update(world, &mut erc); context.benchmarker.pop(); }
            context.benchmarker.push("StateMachine"); self.state_machine_system.update(world, context); context.benchmarker.pop();
            context.benchmarker.push("AnimationSync"); self.animation_synchronization_system.update(world, context); context.benchmarker.pop();
            context.benchmarker.push("AnimationUpdate"); self.animation_update_system.update(world, context); context.benchmarker.pop();
        }

        match &world.game_state {
            GameState::Playing | GameState::GameOver => {
                context.benchmarker.push("Transition"); self.transition_system.update(world, context); context.benchmarker.pop();
                context.benchmarker.push("CameraShake"); self.camera_shake_system.update(world, context); context.benchmarker.pop();
                context.benchmarker.push("AudioSync"); self.audio_synchronization_system.update(world, context); context.benchmarker.pop();
            },
            GameState::Menu(_) => {
                context.benchmarker.push("Transition"); self.transition_system.update(world, context); context.benchmarker.pop();
                context.benchmarker.push("AudioSync"); self.audio_synchronization_system.update(world, context); context.benchmarker.pop();
            },
            _ => {}
        }
        self.audio_system.update(world, audio_manager);
    }
}

impl Default for SystemManager {
    fn default() -> Self {
        Self::new()
    }
}