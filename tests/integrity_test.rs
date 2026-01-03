use gfx_engine::ecs::world::World;
use gfx_engine::ecs::component::*;
use gfx_engine::ecs::systems::game_resolution::SystemGameResolution;
use gfx_engine::ecs::systems::{System, SystemContext};
use gfx_engine::animation::AnimationController;
use gfx_engine::state_machine::StateMachine;
use gfx_engine::player::states::IdleState;
use gfx_engine::math::Vector2D;
use gfx_engine::config::{load_config, load_game_config};
use std::sync::mpsc;

#[test]
fn test_entity_cleanup_integrity() {
    // 1. Setup World and System
    let mut world = World::new();
    let mut resolution_system = SystemGameResolution;
    
    // Load minimal configs for context
    let config = load_config().unwrap_or_else(|_| {
        // Fallback for CI environments where files might be missing (mocking)
        // For now, we assume tests run in project root.
        panic!("Failed to load config.toml");
    });
    let game_config = load_game_config("assets/game_config.toml").unwrap();
    let (tx, _) = mpsc::channel();
    let mut next_level = None;
    let mut camera = gfx_engine::camera::Camera::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
    let mut benchmarker = gfx_engine::benchmarker::Benchmarker::new();
    
    // Mock Level (Empty)
    let level = gfx_engine::level::Level {
        map: gfx_engine::level::Map { tiles: vec![] },
        tileset: gfx_engine::level::Tileset { texture: "".to_string(), tile_width: 0, tile_height: 0 },
        collision: gfx_engine::level::Collision { tiles: vec![] },
        entities: vec![],
    };

    let mut context = SystemContext {
        level: &level,
        input_state: &gfx_engine::input::InputState::default(),
        config: &config,
        game_config: &game_config,
        audio_sender: &tx,
        next_level: &mut next_level,
        delta_time: 0.016,
        camera: &mut camera,
        benchmarker: &mut benchmarker,
        current_soundtrack: None,
        is_paused: false,
        is_attract_mode: false,
    };

    // 2. Create a "Kitchen Sink" Entity with EVERY component type
    let entity = world.create_entity();
    
    world.add_position(entity, Position(Vector2D::default()));
    world.add_velocity(entity, Velocity(Vector2D::default()));
    world.add_acceleration(entity, Acceleration(Vector2D::default()));
    world.add_movement_intention(entity, MovementIntention::default());
    world.add_renderable(entity, Renderable { width: 0, height: 0, horizontal_offset: 0, vertical_offset: 0, z_index: 0, rotation: 0.0, flip_horizontal: false, flip_vertical: false });
    world.add_animation(entity, Animation { controller: AnimationController::new() });
    world.add_player_tag(entity, PlayerTag);
    world.add_gold_coin(entity, GoldCoin);
    world.add_enemy_tag(entity, EnemyTag);
    world.add_patrol(entity, Patrol { speed: 0.0, anim_prefix: "".to_string(), direction: 0.0 });
    world.add_gravity(entity, Gravity);
    world.add_collision(entity, Collision { rect: sdl3::rect::Rect::new(0,0,0,0) });
    world.add_grounded(entity, Grounded);
    world.add_wall_hit(entity, WallHit { normal_x: 0.0 });
    world.add_state_component(entity, StateComponent { state_machine: StateMachine::new(IdleState) });
    world.add_respawn_tag(entity, RespawnTag);
    world.add_respawn_timer(entity, RespawnTimer { timer: 0.0, transition_started: false });
    world.add_health(entity, Health { current: 0, max: 0 });
    world.add_invincibility(entity, Invincibility { timer: 0.0 });
    world.add_lifetime(entity, Lifetime { timer: 0.0 });
    world.add_direction(entity, Directional { direction: Direction::Right });
    world.add_goal(entity, Goal);
    world.add_next_level(entity, NextLevel("".to_string()));
    world.add_dormant_tag(entity, DormantTag);

    // 3. Mark the entity for death
    world.add_dead_tag(entity, DeadTag);

    // 4. Run the Resolution System (cleanup)
    resolution_system.update(&mut world, &mut context);

    // 5. Assert CLEANUP INTEGRITY
    // If any assertion fails, it means we have a memory leak (logic inconsistency).
    assert!(!world.positions.contains_key(&entity), "Leaked Position");
    assert!(!world.velocities.contains_key(&entity), "Leaked Velocity");
    assert!(!world.accelerations.contains_key(&entity), "Leaked Acceleration");
    assert!(!world.movement_intentions.contains_key(&entity), "Leaked MovementIntention");
    assert!(!world.renderables.contains_key(&entity), "Leaked Renderable");
    assert!(!world.animations.contains_key(&entity), "Leaked Animation");
    assert!(!world.player_tags.contains_key(&entity), "Leaked PlayerTag");
    assert!(!world.gold_coins.contains_key(&entity), "Leaked GoldCoin");
    assert!(!world.enemy_tags.contains_key(&entity), "Leaked EnemyTag");
    assert!(!world.dead_tags.contains_key(&entity), "Leaked DeadTag");
    assert!(!world.patrols.contains_key(&entity), "Leaked Patrol");
    assert!(!world.gravity_tags.contains_key(&entity), "Leaked Gravity");
    assert!(!world.collisions.contains_key(&entity), "Leaked Collision");
    assert!(!world.grounded_tags.contains_key(&entity), "Leaked Grounded");
    assert!(!world.wall_hits.contains_key(&entity), "Leaked WallHit"); 
    
    assert!(!world.state_components.contains_key(&entity), "Leaked StateComponent");
    assert!(!world.respawn_tags.contains_key(&entity), "Leaked RespawnTag");
    assert!(!world.respawn_timers.contains_key(&entity), "Leaked RespawnTimer");
    assert!(!world.healths.contains_key(&entity), "Leaked Health");
    assert!(!world.invincibilities.contains_key(&entity), "Leaked Invincibility");
    assert!(!world.lifetimes.contains_key(&entity), "Leaked Lifetime");
    assert!(!world.directions.contains_key(&entity), "Leaked Direction");
    assert!(!world.goals.contains_key(&entity), "Leaked Goal");
    assert!(!world.next_levels.contains_key(&entity), "Leaked NextLevel");
    assert!(!world.dormant_tags.contains_key(&entity), "Leaked DormantTag");
}
