
#[cfg(test)]
mod tests {
    use gfx_engine::ecs::world::World;
    use gfx_engine::ecs::component::*;
    use gfx_engine::ecs::systems::{System, SystemContext};
    use gfx_engine::ecs::systems::synchronization::SystemSynchronization;
    use gfx_engine::ecs::systems::game_resolution::SystemGameResolution;
    use gfx_engine::config::{Config, GameConfig, load_config, load_game_config};
    use gfx_engine::math::Vector2D;
    use std::sync::mpsc;

    #[test]
    fn test_coin_collect_audio_trigger_count() {
        // 1. Setup World
        let mut world = World::new();
        
        // 2. Setup Systems (Order: Synchronization -> Resolution)
        let mut synchronization_system = SystemSynchronization;
        let mut resolution_system = SystemGameResolution;

        // 3. Setup Context with Mock Audio
        let (audio_tx, audio_rx) = mpsc::channel();
        let config = load_config().expect("Failed to load config.toml");
        let mut game_config = load_game_config("assets/game_config.toml").expect("Failed to load game_config.toml");
        // Register the sound event so the system attempts to play it
        game_config.sound_events.insert("coin_pickup".to_string(), "sfx_coin".to_string());
        
        let mut next_level = None;
        let mut camera = gfx_engine::camera::Camera::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let mut benchmarker = gfx_engine::benchmarker::Benchmarker::new();
        
        // Mock Level
        let level = gfx_engine::level::Level {
            map: gfx_engine::level::Map { tiles: vec![] },
            tileset: gfx_engine::level::Tileset { texture: "".to_string(), tile_width: 0, tile_height: 0 },
            collision: gfx_engine::level::Collision { tiles: vec![] },
            entities: vec![],
        };

        let input_state = gfx_engine::input::InputState::default();

        let mut context = SystemContext {
            level: &level,
            input_state: &input_state,
            config: &config,
            game_config: &game_config,
            audio_sender: &audio_tx,
            next_level: &mut next_level,
            delta_time: 0.016,
            camera: &mut camera,
            benchmarker: &mut benchmarker,
            current_soundtrack: None,
            is_paused: false,
            is_attract_mode: false,
        };

        // 4. Spawn Player and Coin at colliding positions
        let player = world.create_entity();
        world.add_player_tag(player, PlayerTag);
        world.add_collision(player, Collision { rect: sdl3::rect::Rect::new(100, 100, 32, 32) });
        world.add_position(player, Position(Vector2D::new(100.0, 100.0)));

        let coin = world.create_entity();
        world.add_gold_coin(coin, GoldCoin);
        world.add_collision(coin, Collision { rect: sdl3::rect::Rect::new(110, 110, 16, 16) });
        world.add_position(coin, Position(Vector2D::new(110.0, 110.0)));

        // 5. Run Simulation Loop for 5 Frames
        let mut audio_trigger_count = 0;

        for i in 0..5 {
            println!("--- Frame {} ---", i);

            // In Frame 0, manually publish a collision event to trigger the logic chain.
            if i == 0 {
                world.event_bus.publish(gfx_engine::ecs::event::EventCollision {
                    entity_a: player,
                    entity_b: coin,
                    intersection: sdl3::rect::Rect::new(110, 110, 10, 10), // Dummy intersection
                });
            }
            
            // B. Run Systems
            synchronization_system.update(&mut world, &mut context);
            resolution_system.update(&mut world, &mut context);
            
            // C. Clear Events (Critical step in main loop)
            world.clear_events();

            // D. Check Audio Channel
            while let Ok(_) = audio_rx.try_recv() {
                audio_trigger_count += 1;
                println!("Audio Event Received!");
            }
        }

        // 6. Assertions
        // It should trigger exactly ONCE.
        // Frame 0: Collision -> Collected -> Resolution (Kill + Sound).
        // Frame 1: Coin is dead (DeadTag). Interaction might still see it?
        // Let's verify if Interaction ignores DeadTag.
        // SystemInteraction logic: "let entities: Vec<_> = world.collisions.keys().copied().collect();"
        // It iterates all collisions. 
        // But Resolution ADDS DeadTag.
        // Does Resolution REMOVE Collision? No, 'cleanup_dead' removes it.
        // 'cleanup_dead' runs at the END of Resolution.
        // So Frame 1: Coin has DeadTag.
        // cleanup_dead removes components.
        // Frame 2: Coin has NO Collision component. Interaction loop skips it.
        
        assert_eq!(audio_trigger_count, 1, "Audio should be triggered exactly once.");
    }
}
