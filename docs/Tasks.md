File version: 8.00

**TLDR:**
This document is the **Product Backlog** for the `GfX-Engine` project. It lists all features, improvements, and bug fixes, organized into a prioritized roadmap that guides our Agile development process.

### Phase 1: Core Engine Refactoring (Complete)

- [x] Define new project vision and goals.
- [x] Implement new, simplified directory structure.
- [x] Update project identity (`README.md`, `Cargo.toml`).
- [x] Update all documentation in `docs/`.
- [x] Create a minimalistic compiling POC.
- [x] Refactor configuration to support sprite animations.
- [x] Implement a device-independent, data-driven input system.
- [x] Implement a sprite animation system.

### Phase 2: "Super Cat Bros" Demo - Core Gameplay (Complete)

- [x] Create a new `feature/captain-cat-demo` branch for the demo implementation.
- [x] Load and display the Super Cat Bros character sprite.
- [x] Implement sprite flipping for left/right movement.
- [x] Implement basic player movement (left/right) based on input.
- [x] Implement Platform Feature - Variable Jump Height.
- [x] Create a simple level layout in a config file and render it.
- [x] Implement Platform Feature - Momentum-Based Movement.
- [x] **1. Implement Multi-Frame Animations:** Extend the animation system to support sprite sheets with multiple frames to create fluid walking cycles.
- [x] **2. Add Textured World Graphics:** Replace the placeholder level blocks with textured graphics loaded from files.

### Phase 3: Gameplay Polish Sprint (Current)



- [ ] **1. Implement Damped Camera Movement:** Improve the camera to follow the player smoothly, enhancing the game's professional feel.

    - [x] Re-implement lerp-based smoothing to restore damping effect.
    - [ ] Implement Advanced Camera Mechanics:
        - [ ] Implement "Look-Ahead" (Directional Bias) to show more of the screen in front of the player.
        - [ ] Implement "Platform Snap" to prevent vertical camera movement during jumps.
        - [x] Rename camera zones to "Slow Zone" and "Fast Zone" and move their configuration to `config.toml`.

- [x] **2. Implement Basic Sound Effects (using Kira and Event-Driven Audio):** Integrate an audio system for key gameplay events.

    - [x] Add the `kira` crate to `Cargo.toml`.

    - [x] Create an `AudioManager` in `src/audio.rs` that wraps Kira's audio context.

    - [x] Define an `AudioEvent` enum and a channel for sending events.

    - [x] Modify `app.rs` to process `AudioEvent`s and instruct the `AudioManager` to play sounds.

    - [x] In game logic (e.g., `player/state.rs`), emit `AudioEvent`s when specific events occur (e.g., `AudioEvent::PlayerJumped`).

- [ ] **3. Refactor Physics Logic:** Move physics and collision logic from `player/state.rs` into a generic `physics.rs` module.

- [ ] **4. Implement a Simple Enemy:** Add an enemy with basic patrol AI and stomp mechanics.



### Phase 4: Core Gameplay Systems (Backlog)















- [ ] **1. Implement Menu System:** Develop a basic UI system with a title screen and menus (`MainMenu`, `InGame`, `Exit`).







- [ ] **2. Implement Player Health & Damage:** 







    - [ ] Add a `health` component to the player.







    - [ ] Make enemies deal damage on contact.







    - [ ] Add Health-Up items (Heart, Potion) to levels.







- [ ] **3. Add Collectible Treasures:** 







    - [ ] Add items like Stars and Gold to levels.







    - [ ] Implement a scoring system and UI display.







- [ ] **4. Implement Debugging Tools:** Create the planned in-game `Benchmarker` and on-screen debug display.















### Phase 5: Player Abilities (Backlog)















- [ ] **Implement Power-Ups:** Design a flexible system for power-ups that can modify the player's state and grant new abilities.







    - [ ] Add a "Run Fast" power-up.







    - [ ] Add a "Shoot" power-up with projectiles.







    - [ ] Add a "Fly" power-up.















### Phase 6: Advanced Features & Tooling (Future)















- [ ] **1. Implement a Simple In-Game Level Editor.**















### Phase 7: Engine Hardening & Polish (Future)















- [ ] **1. Implement Robust Error Handling:** Replace `.map_err(|e| e.to_string())` with a dedicated `EngineError` enum for more structured and debuggable error management.







- [ ] **2. Write Comprehensive Tests:** Build out a suite of unit and integration tests to ensure code quality and prevent regressions.















### Ongoing Tasks























- [ ] **Refactor for Modularity:** Continuously move hard-coded data from source files (`.rs`) into external configuration files.























- [x] **Redesign to ECS Architecture:** Plan and execute a major refactoring to adopt an Entity-Component-System (ECS) architecture for improved modularity, reusability, and performance.























### Phase 8: Engine Refactoring (Future)























- [ ] **1. Refactor to a 1:1 Pixel Coordinate System:** Remove the `PIXEL_SCALE` constant and update the renderer, camera, and configuration to work directly in pixel dimensions. This will simplify the logic and make asset integration more intuitive.







### Phase 9: Gameplay Expansion (Backlog)







- [ ] **1. Tune Player Collision Box:** Fine-tune the main character's collision box for better gameplay feel.



- [ ] **2. Implement Player Lives:** Give the cat 9 lives and handle game over conditions.



- [ ] **3. Implement Level Progression:** 



    - [ ] Create larger, more complex level maps (50-100% larger).



    - [ ] Implement a system for level start/goal locations.



    - [ ] Create multiple levels (2-3 to start).



    - [ ] Implement a top-down world map for level selection.



- [ ] **4. Add a Boss Fight:** Design and build a multi-phase boss encounter in a dedicated arena at the end of a level.



- [ ] **5. Implement Interactive Blocks:** Create a system for various types of interactive blocks, such as power-up blocks and breakable blocks.



- [ ] **6. Implement a Companion/Sidekick:** Introduce a companion character with unique abilities.







### Phase 10: Advanced Engine Features (Backlog)







- [ ] **1. Implement Parallax Scrolling:** Add support for multi-layered, parallax backgrounds to create a sense of depth.



- [ ] **2. Implement Interactive Audio:** Create a system for dynamic music and sound effects that respond to gameplay events.

- [ ] **3. Implement Z-Layer Rendering:** Add a z-layer system to control the draw order of entities.
    - [ ] Add a `z_index` field to the `Renderable` component.
    - [ ] Refactor the rendering loop in `app.rs` to sort entities by `z_index` before drawing.
    - [ ] Assign `z_index` values to Player, Enemies, and Effects.



- [ ] **3. Implement Save/Load System:** Allow players to save and load their progress in the game.



- [ ] **4. Add Multiplayer Support:** Integrate 2-player local co-op or competitive gameplay.



- [ ] **5. Implement Spatial Partitioning:** Implement a Uniform Grid (e.g., 32x32 tile chunks) spatial partitioning system to efficiently manage large levels.



- [ ] **6. Add Support for Sloped Surfaces:** Enhance the physics engine to correctly handle player and object interaction with sloped terrain.







### Phase 11: Tooling and Polish (Backlog)







- [ ] **1. Create a Level Editor:** Build an in-game or external tool for creating and editing levels.



- [ ] **2. Implement Video Recording:** Add a system for saving gameplay videos using ffmpeg or a similar library.







### Phase 12: Code Refactoring Candidates







- [ ] **1. Refactor `app.rs` Initialization:**



    - **Problem:** The `App::new()` function is excessively large and complex.



    - **Suggestion:** Break down the initialization process into smaller, focused functions or builders.



    - **Suggestion:** Move entity creation to a data-driven system.







- [ ] **2. Refactor `player/states.rs` Logic:**



    - **Problem:** Significant code duplication exists between `IdleState` and `WalkingState`.



    - **Suggestion:** Consolidate duplicated logic into a shared function or a base state to improve code reuse.







- [ ] **3. Improve `level.rs` Loading:**



    - **Problem:** TMX file parsing is manual, incomplete, and uses hardcoded values.



    - **Suggestion:** Integrate a dedicated TMX parsing crate to handle level loading dynamically and robustly.







- [ ] **4. Decompose `ecs/system.rs` Collision System:**



    - **Problem:** The `CollisionSystem` is becoming a "god object" handling all collision types.



    - **Suggestion:** Break it down into smaller, specialized systems (e.g., `PlayerEnemyCollisionSystem`, `TileCollisionSystem`) for better modularity.







### Phase 13: Engine Hardening (Backlog)

- [ ] **1. Implement Robust Error Handling:**
    - [ ] **1.1. Create a Custom Error Enum:** Create a new `src/error.rs` file and define a comprehensive `EngineError` enum.
    - [ ] **1.2. Implement Error Conversions:** Implement the `From` trait for standard error types to allow for clean and easy conversion into our custom `EngineError` using the `?` operator.
    - [ ] **1.3. Refactor for Robust Error Handling:** Refactor the existing code, replacing all instances of `.map_err(|e| e.to_string())` with our new, more structured `EngineError`.
- [ ] **2. Write Comprehensive Tests:**
    - [ ] **2.1. Create a Core Integration Test:** Add a new integration test that initializes the main `App`, runs the game loop for a number of frames, and asserts that no panics occur.
    - [ ] **2.2. Add a Level Loading Test:** Create a test to verify that a level is loaded correctly, and that all expected entities (player, enemies, etc.) are created in the ECS world.
    - [ ] **2.3. Implement an Input and Movement Test:** Write a test to simulate player input and confirm that the player character's position and state change as expected.
- [ ] **3. Refactor for Data-Driven Design:**
    - [ ] **3.1. Identify and Externalize Hardcoded Values:** Systematically search the codebase for hardcoded values (such as file paths, magic numbers, etc.) and move them into the appropriate configuration files (`config.toml` or `assets/game_config.toml`).
    - [ ] **3.2. Refactor Code to Load from Configuration:** Modify the source code to load these newly externalized values at runtime, further strengthening the data-driven design.




