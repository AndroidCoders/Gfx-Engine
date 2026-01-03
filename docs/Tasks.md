File version: 9.05

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

### Phase 3: "Super Cat Bros" Demo - Gameplay Polish (Complete)

- [x] **1. Core Gameplay Feel:**
    - [x] Tune Player Controls: Adjust player physics parameters in `config.toml` for a tighter feel.
    - [x] Improve Stomping Mechanics: Refine player-enemy collision detection for more accurate stomp registration.
    - [x] Tune Player Collision Box: Fine-tune the main character's collision box for better gameplay feel.
    - [x] Fix player orientation reset glitch.
    - [x] **Fix 'Moonwalking' Animation:** The player walking animation looks like moonwalking when moving right. Verify if the `walk_right` sprite sheet or animation frame order needs to be reversed.
- [ ] **2. Player Experience & Feedback:**
    - [ ] **Update Game Title:** Change window title and game branding to "Super Cat Bros - Episode 1 - The Pirate Gold Adventure".
    - [x] Improve Player Spawning and Death:
        - [x] Adjust player start position in `assets/levels/world_1_level_1.tmx`.
        - [x] Implement an "angel" death sequence with animation and sound.
        - [x] Implement a respawn animation.
    - [x] Implement Player Lives and Game Over:
        - [x] Display player lives in the debug text.
        - [x] **Implement Game Over Screen:** Create a simple "Game Over" overlay with a restart prompt (requires Text Rendering).
    - [x] Implement Player Health & Damage:
        - [x] Add a `health` component to the player.
        - [x] Make enemies deal damage on contact.
        - [x] Display player health (hearts) in the debug text.
        - [ ] **Implement Damage Feedback:** Add visual flashing (invincibility) and knockback when taking damage.
        - [ ] Add "Medical Kit" or "Potion of Health" items to the game.
        - [ ] Implement logic for picking up health items to restore player health.
        - [ ] Brainstorm ideas for how health items can appear (e.g., dropped by enemies, in chests).
    - [x] Implement Damped Camera Movement:
        - [x] Re-implement lerp-based smoothing to restore damping effect.
        - [x] Implement "Look-Ahead" (Directional Bias).
        - [x] Implement "Platform Snap".
    - [x] Hide Mouse Cursor in Fullscreen: Call the appropriate SDL3 function to hide the mouse cursor during gameplay.
    - [x] **Implement Gold Coin Counter HUD:** Display the current coin count on screen using the coin sprite and text.
- [x] **3. Content & Progression:**
    - [x] Implement a Simple Enemy: Add an enemy with basic patrol AI and stomp mechanics.
    - [x] Implement Level Goal and Progression:
        - [x] Add a "Goal" object to the level.
        - [x] Implement level transitions.
- [x] **4. Visual Polish & Game Feel:**
    - [x] **Implement Screen Shake:** Add a system that applies a temporary, decaying offset to the camera when high-impact events occur (e.g., Enemy Stomp, Player Damage, Explosions).
    - [x] **Implement Retro Transitions:**
        - [x] **Circle Fade:** Create a "Looney Tunes" style iris-out effect.
        - [ ] **Pixel Dissolve:** Implement a fade effect that uses a dither pattern or pixelation to transition, rather than smooth transparency, to maintain the retro aesthetic.
- [x] **5. Refactor: Externalize Gameplay Values:**
    - [x] Identify hardcoded values in `src/` (e.g., `interaction.rs`, `respawn.rs`, `app.rs`).
    - [x] Move these values to `assets/game_config.toml` or `config.toml`.
    - [x] Update the code to read from the configuration structs.

### Phase 4: Architectural Improvements & Core Features (Current Sprint)

- [ ] **1. Implement Event-Driven ECS Architecture:**
    - [x] Create `src/ecs/event.rs` with a generic `EventBus` struct.
    - [x] Integrate the `EventBus` into the `World` struct.
    - [x] Refactor `CoinCollectionSystem` and `InteractionSystem` to publish strongly-typed events.
    - [x] Create an `AudioSynchronizationSystem` that listens for audio-related events.
    - [x] Update the main loop to clear the event bus each frame.
    - [ ] Refactor **Level Transition** logic to publish a `LevelCompleteEvent`.
    - [x] Refactor **Player Input** to publish `MoveCommand` events.
    - [ ] Refactor **Player Animation** to be driven by state-change events like `PlayerLandedEvent`.
- [ ] **2. System Consolidation & Refactoring:**
    - [x] **Refactor Physics & Collision Integration (Critical):**
        - [x] Fix bug where `SystemPhysics` and `SystemTileCollision` both apply movement, leading to incorrect integration.
        - [x] Standardize on "Pixels per Second" units for velocity.
        - [x] Ensure `SystemPhysics` handles velocity integration (`Vel += Accel * dt`) and `SystemTileCollision` handles position integration (`Pos += Vel * dt`) and constraint resolution.
    - [x] **Step 1: Create `LifecycleSystem`:**
        - [x] Implement `Health`, `Lives`, and `State` management in a single system.
        - [x] Consolidate `DeathSystem`, `PlayerDeathSystem`, `KillSystem`, `RespawnSystem`, `InvincibilitySystem`.
        - [x] Handle `EntityTookDamageEvent` and `EntityDiedEvent`.
    - [x] **Step 2: Create `InteractionSystem`:**
        - [x] Refactor to be the "Arbiter" that only detects collisions and publishes events.
        - [x] Merge `CoinCollectionSystem` logic.
    - [x] **Step 3: Create `MovementSystem`:**
        - [x] Create generic `MovementCommand` component (Refactored to `MovementIntention`).
        - [x] Standardize movement logic for Player and Enemies.
    - [x] **Step 4: Create `AnimationSystem`:**
        - [x] Unify `PlayerAnimationSystem` and `AnimationUpdateSystem`.
- [x] **3. Refactor `app.rs` into Managers:**
    - [x] Create a `SystemManager` to hold all ECS systems and orchestrate their execution.
    - [x] Create a `GameStateManager` to handle level loading, transitions, and entity creation/teardown.
    - [x] Create a `PlayerFactory` or similar mechanism to centralize player entity creation and avoid code duplication during level transitions.
    - [x] Simplify the `App` struct to delegate most of its responsibilities to these new managers.
- [x] **4. Implement Robust Text Rendering:**
    - [x] Research and add the `sdl3_ttf` crate to the project. (Note: Used `rusttype` for a pure Rust solution)
    - [x] Create a `FontManager` to load and manage `.ttf` fonts.
    - [x] Create a `Text` component and a `TextSystem` for rendering UI and debug text. (Note: Implemented via direct rendering in Renderer for now)
    - [x] Replace the temporary `SDL_RenderDebugText` with the new system.
- [x] **5. Implement Menu System:** Develop a data-driven UI system for the Main Menu.
    - [x] **Prerequisite:** Implement robust text rendering (Completed).
    - [x] **Step 1: Configuration:** Add `[menu]` section to `assets/game_config.toml` and update `src/config.rs` to support `MenuConfig`.
    - [x] **Step 2: Logic:** Create `src/menu_system.rs` with `MenuManager` struct to handle state, selection index, and input.
    - [x] **Step 3: Integration:** Add `AppState::MainMenu` to `GameStateManager` and integrate the `MenuManager`.
    - [x] **Step 4: Rendering:** Implement `MenuManager::draw` to render the title and options based on the config.
    - [x] **Step 5: Transitions:** Implement logic to switch from Menu to Game (`start_game` action) and Exit (`quit_game` action).
    - [x] **Step 6: Enable Menu:** Change initial application state from `Playing` (debug) to `MainMenu`.
    - [x] **Step 7: Character Selection & Soundtrack:** Allow selecting a character in the menu, which changes the active player sprite and the background music.
- [ ] **6. Visual Polish & Bug Fixes:**
    - [x] **Fix Bug:** Game Over screen (bitmap) flickers.
    - [x] **Fix Bug:** Player animation glitch when pushing against walls.
    - [x] **Player Death:** Display player upside-down when killed by an enemy. (Implemented using `injured` sprites instead of rotation).
    - [ ] **Enemy Death:** Display enemy (e.g., spider) sprites squashed flat when stomped on. (Deferred: Will use dedicated graphics).
- [ ] **6. Implement Audio Features:**
    - [x] **Phase 1: Basic Music Implementation**
        - [x] Find and add a freely-licensed music track.
        - [x] Enable streaming support in the `kira` crate.
        - [x] Update `GameAudioManager` to handle music streaming and looping.
        - [x] Add a `[music]` section to `game_config.toml`.
        - [x] Start music playback on game launch.
    - [ ] **Phase 2: Zone-Based Music**
        - [ ] Design and implement music zones in level data.
        - [ ] Create a new `MusicSystem` to track player position and trigger music changes.
        - [ ] Enhance `GameAudioManager` to support crossfading between music tracks.
- [x] **7. Add Collectible Treasures:**
    - [x] Add items like Stars and Gold to levels.
    - [x] Implement a scoring system and UI display.
- [ ] **8. Implement Interactive Blocks:** Create a system for various types of interactive blocks, such as power-up blocks and breakable blocks.
    - [ ] **Interactive Treasure Chest:** A chest that spawns coins/items when touched.
- [ ] **9. Implement Power-Ups:**
    - [ ] Design a flexible system for power-ups that can modify the player's state and grant new abilities.
    - [ ] Add a "Run Fast" power-up.
    - [ ] Add a "Shoot" power-up with projectiles.
    - [ ] Add a "Fly" power-up.

### Phase 5: Gameplay Expansion (Backlog)

- [ ] **1. Create More Content (Pirate Adventure Theme):**
    - [ ] Create larger, more complex level maps (50-100% larger).
    - [x] Create multiple levels (2-3 to start).
        - [x] Add World Level 2.
    - [ ] **New Enemy: Pirate Parrot:** A flying enemy that patrols horizontally.
    - [ ] **New Enemy: Crab:** A ground enemy that cannot be stomped (armored)?
- [ ] **2. Add a Boss Fight:** Design and build a multi-phase boss encounter in a dedicated arena at the end of a level.
- [ ] **3. Implement a Companion/Sidekick:** Introduce a companion character with unique abilities.
- [ ] **4. Implement a World Map:** Implement a top-down world map for level selection.
- [ ] **5. Improve Menu System (Kid-Friendly):**
    - [x] **Visual Menu:** Replace text-based menu with a loaded "Menu Level" containing live graphics and animations.
    - [x] **Attract Mode:** Show the player character running and jumping (e.g., on an enemy).
    - [ ] **Audio:** Add a catchy soundtrack specifically for the menu.
    - [ ] **Navigation:** Simplify navigation for a younger audience (4-6 years old), focusing on visual cues over text.
- [ ] **6. Improve Game Over Screen:**
    - [ ] **Sequence:** Implement a Fade Out effect -> Stop Music -> Display Game Over Graphics.
    - [ ] **Audio:** Consider a specific short Game Over track or jingle.
    - [ ] **Flow:** Automatically return to the Main Menu after a 5-10 second delay (instead of just freezing or immediate restart).

### Phase 6: Advanced Engine Features (Backlog)

- [x] **1. Implement Parallax Scrolling:** Add support for multi-layered, parallax backgrounds to create a sense of depth.
- [x] **2. Implement Z-Layer Rendering:** Add a z-layer system to control the draw order of entities.
- [ ] **3. Implement Save/Load System:** Allow players to save and load their progress in the game.
- [ ] **4. Add Support for Sloped Surfaces:** Enhance the physics engine to correctly handle player and object interaction with sloped terrain.
- [ ] **5. Implement Spatial Partitioning:** Implement a Uniform Grid (e.g., 32x32 tile chunks) spatial partitioning system to efficiently manage large levels.
- [ ] **6. Implement Interactive Audio:** Create a system for dynamic music and sound effects that respond to gameplay events.
    - [x] **Audio Synchronization & Beat Detection:**
        - [x] Integrate `spectrum-analyzer` and `hound` for WAV processing.
        - [x] Implement `BeatDetector` with Spectral Flux algorithm.
        - [x] Implement Auto-Tuning to match target BPM (116 BPM).
        - [x] Implement Caching (`.beats` files) for performance.
        - [x] Update `SystemManager` to act as Conductor (publishing `EventMusicBeat`).
        - [x] Refactor `SystemEnemyRhythm` to subscribe to `EventMusicBeat`.
    - [ ] **Dynamic Soundtrack Logic:**
        - [ ] Implement Soundtrack Restart on Respawn.
        - [ ] Implement Soundtrack Restart on Start of New Level.
        - [ ] Change Soundtrack/Animation on Character Selection.
        - [ ] Adjust "Coin collect" sound to use musical notes (e.g., C-E-G arpeggio) for fast collections.

### Phase 7: Synchronization Engine & Tooling

- [ ] **1. Implement Event Tracing:**
    - [ ] Update `Event` trait to include an `event_source_id` (Flow Scoping).
    - [ ] Implement a tracing system to log the chain of events (Input -> Game Logic -> Audio/UI) for debugging.
- [ ] **2. Rename Conductors to Synchronizations:**
    - [ ] Rename `AudioConductorSystem` to `AudioSynchronizationSystem`.
    - [ ] Rename `PlayerControlSystem` to `JumpSynchronizationSystem` (or similar).
- [ ] **3. Implement Synchronization Engine (WYSIWID):**
    - [ ] Design a data-driven rule format (e.g., TOML or Script) for defining Synchronizations (`when Event then Action`).
    - [ ] Create a generic `SynchronizationSystem` that executes these rules.
    - [ ] Replace manual "glue" systems with this engine.
- [ ] **4. WYSIWID System Audit & Refactor (Critical for Legibility):**
    - [ ] **Task 1: Goal Collision Refactor:** Move Goal reached detection from `SystemWorldLevelTransition` to `SystemSynchronization`. `SystemWorldLevelTransition` should only listen for `EventGoalReached`.
    - [ ] **Task 2: Command-Based Enemy Movement:** Refactor `SystemEnemyRhythm` to publish `CommandJump` instead of modifying velocity directly.
    - [ ] **Task 3: Generic Interaction Arbiter:** Ensure `SystemInteraction` only publishes `EventCollision` and all logic for "what happens" (Coin collection, Damage, etc.) is in `SystemSynchronization`.
    - [ ] **Task 4: Pure Concept States:** Remove animation setting and secondary logic from `src/player/states.rs` and `src/enemy/states.rs`. Ensure `SystemAnimationSynchronization` is the sole authority for visuals.
- [ ] **5. Scripting Integration:**
    - [ ] Integrate the **Rhai** scripting language to define complex Synchronization Actions.
- [ ] **6. Create a Level Editor:** Build an in-game or external tool for creating and editing levels.
- [ ] **6. Implement Debugging Tools:**
    - [x] Create the planned in-game `Benchmarker` and on-screen debug display.
    - [ ] Replace the temporary debug text renderer with the new `sdl3_ttf` based system.
    - [x] Add FPS counter to debug display.
- [ ] **7. Implement Video Recording:** Add a system for saving gameplay videos using ffmpeg or a similar library.
- [ ] **8. Implement Cross-Platform Builds and Releases:**
    - [ ] **Windows Build:**
        - [ ] Fix target error: `rustup target add x86_64-pc-windows-gnu`.
        - [ ] Compile command: `cargo build --release --target x86_64-pc-windows-gnu`.
    - [ ] **Cross-Compilation:**
        - [ ] Research `cross-rs` for easier cross-compilation (https://github.com/cross-rs/cross).
    - [ ] **Linux:** Compile to a standard Linux executable.
    - [ ] **WebAssembly (WASM):** Compile the demo game to WASM to run in a web browser.
    - [ ] Automate the process of creating a GitHub Release with the compiled binaries attached.
- [ ] **9. Renderer 2.0: Dual Renderer Strategy (TDD Compatible):**
    - **Goal:** Support both a deterministic CPU renderer (for tests) and a high-fidelity GPU renderer (for the demo).
    - **Plan:**
        - [ ] **Step 1: Define `RendererTrait`:** Abstract the rendering interface (draw_sprite, draw_level, etc.) into a trait.
        - [ ] **Step 2: Refactor `Renderer`:** Rename current `Renderer` to `SoftwareRenderer` and implement the trait.
        - [ ] **Step 3: Implement `ShaderRenderer`:** Create a new SDL3 GPU / WGPU backend that implements the trait and supports custom shaders.
        - [ ] **Step 4: Smooth Pixel Filtering:** Implement the "Smooth Pixel Filtering" shader in the `ShaderRenderer`.
        - [ ] **Step 5: Runtime Switch:** Allow toggling between renderers in `config.toml` or via a hotkey.

### Phase 8: Fixed Timestep & Determinism

- [ ] **1. Refactor Main Loop (Accumulator Pattern):**
    - [x] Update `app.rs` to implement the "Accumulator" loop.
    - [x] Set fixed timestep to **120 Hz** (8.3333 ms).
    - [x] Ensure `SystemManager::update` is called in the fixed step.
    - [x] Ensure `Renderer` is called in the variable step.
- [ ] **2. Implement State Interpolation:**
    - [x] Update `World` to store `previous_positions` alongside `current_positions`.
    - [x] Update `SystemPhysics` (and others) to snapshot state before updating.
    - [x] Calculate `alpha` (interpolation factor) in the main loop.
    - [x] Update `Renderer` to draw using the interpolated position: `lerp(prev, curr, alpha)`.
- [ ] **3. Implement Replay System:**
    - [x] Define `ReplayData` struct (Seed + Vector of InputFrames).
    - [x] **Recorder:** Create a system/mode that captures `InputState` every fixed tick.
    - [x] **Playback:** Create a system/mode that ignores hardware input and feeds `InputState` from the replay buffer.
    - [x] **Serialization:** Implement saving/loading replays to `assets/replays/`.
- [ ] **4. Feature: Attract Mode:**
    - [x] Record a gameplay run of Level 1.
    - [x] Update `MainMenu` to load the level in the background and play the recording.

### Phase 9: Observability & Benchmarking (Next Priority)

- [x] **1. Enhanced Debug Display:**
    - [x] **Always Visible:** Ensure debug text is rendered on top of all other graphics (including transitions) when toggled on.
    - [x] **Improved FPS:** Display Min, Max, and Average FPS.
    - [x] **Research:** Investigate best practices for pro-grade game debug HUDs.
- [ ] **2. Benchmarking Strategy:**
    - [ ] **Research:** Determine best practices for game engine benchmarking.
    - [ ] **Plan:** Benchmark Input, Update, and Render loops separately.
    - [ ] **Key Performance Indicators (PKIs):** Measure Time/Cycles used for:
        - [ ] Input Processing.
        - [ ] Logic Update (Physics/ECS).
        - [ ] Rendering.
        - [ ] Screen Refresh Sync Waiting (Idle time).
    - [ ] **Hotspots:** Identify and list CPU-intensive functions.
    - [ ] **Goal:** Achieve a stable 60+ FPS on target hardware.
- [x] **3. Implement Profiling Tools (Implementation Plan):**
    - [x] **Step 1: Benchmarker Module:** Implement `src/benchmarker.rs` with `RingBuffer` (100 frames), Stack-based Scope logic (`push`/`pop`), and Percentage calculations.
    - [x] **Step 2: App Loop Integration:** Instrument the core `Input`, `Update`, and `Render` phases in `app.rs`.
    - [x] **Step 3: System Context & Scopes:** Add `Benchmarker` to `SystemContext` and wrap individual systems in `SystemManager` with named scopes (e.g., "Physics", "Animation").
    - [x] **Step 4: Smart HUD:** Implement the Debug Overlay to render the sorted list of hotspots (re-sorted every 30s) showing name and percentage.
    - [x] **Step 5: Session Logging:** Implement `start_session` / `end_session` logic in `GameStateManager` to write aggregated reports to `benchmark.log`.
- [ ] **4. Implement Structured Logging:**
    - [ ] Replace `println!` with the `log` crate interface.
    - [ ] Integrate `env_logger` or `tracing-subscriber`.
    - [ ] Move all debug output to a generic debug log file.
- [ ] **5. Enhanced Performance HUD:**
    - [ ] Frame Time (ms) with 1% low metric.
    - [ ] ECS Entity Count.
    - [ ] Memory Usage.
    - [ ] Audio Stream Buffer status.

### Phase 10: Engine Hardening & Refactoring (Ongoing/Future)

- [ ] **1. Refactor for Maintainability:**
    - [ ] Refactor `player/states.rs` Logic.
    - [ ] Improve `level.rs` Loading.
    - [ ] Decompose `ecs/system.rs` Collision System.
    - [ ] Code refactoring.
    - [ ] Performance Benchmarking and Optimization.
- [ ] **2. Implement Robust Error Handling:**
    - [ ] Create a Custom Error Enum.
    - [ ] Implement Error Conversions.
    - [ ] Refactor for Robust Error Handling.
- [ ] **3. Write Comprehensive Tests:**
    - [ ] Create a Core Integration Test.
    - [ ] **Physics Test:** Verify player falls and stops at ground.
    - [ ] **Physics Test:** Verify player stands still on ground.
    - [ ] **Asset Test:** Verify graphics, sound, and config files load properly.
    - [ ] Implement an Input and Movement Test.
    - [ ] Add robust testing for file loading.
- [ ] **4. Refactor for Data-Driven Design:**
    - [x] **Data-driven Tile Collision:**
        - [x] In the Tiled editor, add a custom boolean property (e.g., `solid: true`) to the tiles in the `.tsx` tileset file to mark them as collidable.
        - [x] Research and add a Rust crate for parsing XML/TSX files (e.g., `xml-rs` or `quick-xml`).
        - [x] Update `level.rs` to parse the `.tsx` file, read the custom properties for each tile, and dynamically build the list of solid tile IDs.
        - [x] Remove the hardcoded `solid_tiles` vector.
    - [ ] **Data-driven Scripting for Complex Logic:**
        - [ ] In Tiled, add a custom string property (e.g., `on_collide_script: "scripts/one_way_platform.rhai"`) to tiles or objects that require complex collision logic.
        - [ ] In the collision system, if this property exists, hand off the event to the scripting engine instead of using the default solid/non-solid check.
    - [ ] Identify and Externalize Hardcoded Values.
    - [ ] Refactor Code to Load from Configuration.
- [x] **5. Refactor to a 1:1 Pixel Coordinate System:**
    - [x] Remove the `PIXEL_SCALE` constant.
    - [x] **Implement Load-Time Upscaling:** Update `TextureManager` to automatically upscale small textures (e.g., 4x) upon loading, using nearest-neighbor interpolation. This removes the need for scaling during the render call.
    - [ ] Suggestion: Load graphics assets and resize in memory instead of resizing every frame.
    - [ ] Edit graphics assets to use prescaling (1:2).
- [ ] **6. Refactor Enemy AI for Generic Behavior:**
    - [ ] Design and implement reusable AI 'sensor' components (e.g., `WallSensor`, `LedgeSensor`).
    - [ ] Update `PatrolState` to use sensor components instead of hardcoded detection logic.
- [ ] **7. Add Multiplayer Support:** Integrate 2-player local co-op or competitive gameplay.
- [ ] **8. Continue Data-Driven Refactoring:**
    - [x] **Phase 1: Frame-Rate Independence:**
        - [x] Calculate `delta_time` in the main loop.
        - [x] Pass `delta_time` to all systems via the `SystemContext`.
        - [x] Refactor all timer-based logic to use `delta_time`.
    - [x] **Phase 2: Externalize Assets and Configs:**
        - [x] Move hardcoded asset paths from `app.rs` to `game_config.toml`.
        - [x] Move the `start_level` path to `config.toml`.
    - [x] **Phase 4: Create Entity Prefabs:**
        - [x] Refactor enemy states to be generic and not tied to a specific enemy type.
        - [x] Expand `game_config.toml` to support full entity prefabs that define all components for an entity type.
    - [x] **Phase 5: Data-Driven Sound Events:**
        - [x] Move hardcoded sound effect names to `game_config.toml`, associated with the events that trigger them.
    - [x] **Phase 6: Data-Driven Rhythm:**
        - [x] Expose enemy rhythm parameters (`beats_per_jump`, `active_beats`) to `game_config.toml`.

### Phase 11: Content Creation Tools

- [ ] **1. Implement Video Rendering Pipeline:**
    - [ ] **Task 1: FrameCapture Module:** Implement `src/frame_capture.rs` to spawn an `ffmpeg` subprocess and pipe raw video frames to it.
    - [ ] **Task 2: Render Mode Loop:** Create a special render loop in `App` or `GameStateManager` that loads a replay, steps the physics, renders to an SDL Surface/Texture, and feeds the `FrameCapture` module.
    - [ ] **Task 3: Configuration:** Add a `[video_render]` section to `config.toml` to allow users to specify FFmpeg arguments (resolution, codec, framerate).
- [ ] **2. Implement Offline Audio Mixing:**
    - [ ] **Task 1: Soundtrack Muxing:** Update the FFmpeg command to accept the soundtrack file as an input (`-i soundtrack.wav`) and mix it with the video.
    - [ ] **Task 2: SFX Software Mixer:** Implement a software mixer that accumulates sound effects into a PCM buffer based on game events, independent of the real-time audio system.
    - [ ] **Task 3: Audio Pipe:** Pipe the mixed SFX buffer to FFmpeg as a separate audio stream.
- [ ] **3. Implement Keystroke Recorder:** Implement a "record all keystrokes" feature that can be replayed to render a video file (enhanced Replay System).

### Phase 13: WYSIWID Refactoring & Atomic Modules (High Priority)

- [ ] **1. Project-wide Standards Audit:**
    - [ ] Perform a "Legibility Audit" on all `.rs` files: ensure 3-level documentation is present.
    - [ ] Perform a "Testing Audit": ensure "One Test Per Module" rule is satisfied.
    - [ ] Enforce "Atomic Module" limits: flag any file > 300 lines for refactoring.
- [ ] **2. Decompose `SystemLifecycle`:**
    - [ ] Create `ConceptHealth`: Pure arithmetic logic for health and death facts.
    - [ ] Create `ConceptVitality`: Pure logic for managing `Invincibility` and `Lifetime` timers.
    - [ ] Create `RuleInteraction`: Synchronization rule to interpret collisions (stomp vs. damage).
    - [ ] Create `RuleCollection`: Synchronization rule for collecting coins and items.
- [ ] **2. Refactor Level Transition:**
    - [ ] Create `ConceptGoalDetector`: Purely detects the goal and publishes `EventGoalReached`.
    - [ ] Create `RuleLevelTransition`: Synchronization rule that orchestrates the iris transition, audio fade, and level loading.
- [ ] **3. Purify State Machines:**
    - [ ] Refactor `PlayerStates` to publish facts (`EventEntityJumped`, `EventEntityLanded`) instead of triggering side effects.
    - [ ] Ensure `SystemAudioSynchronization` and `SystemAnimationSynchronization` are the sole presentation authorities.
- [ ] **4. Module Size Audit & Refactoring:**
    - [ ] Refactor the following files to meet the < 300 line Atomic Module standard:
        - [ ] `src/config/game.rs` (516 lines)
        - [ ] `src/game_state_manager.rs` (443 lines)
        - [ ] `src/app.rs` (411 lines)
        - [ ] `src/physics.rs` (346 lines)
        - [ ] `src/audio.rs` (333 lines)
        - [ ] `src/ecs/systems/lifecycle.rs` (328 lines)
        - [ ] `src/player/states.rs` (314 lines)
        - [ ] `src/level.rs` (313 lines)
- [ ] **5. Topic-Based Event Bus:**
    - [ ] Upgrade `EventBus` to support hierarchical string topics for granular declarative rules.
