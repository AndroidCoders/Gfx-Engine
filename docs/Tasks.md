File version: 9.00

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

### Phase 3: "Super Cat Bros" Demo - Gameplay Polish (Current Sprint)

- [ ] **1. Core Gameplay Feel:**
    - [ ] Tune Player Controls: Adjust player physics parameters in `config.toml` for a tighter feel.
    - [ ] Improve Stomping Mechanics: Refine player-enemy collision detection for more accurate stomp registration.
    - [ ] Tune Player Collision Box: Fine-tune the main character's collision box for better gameplay feel.
- [ ] **2. Player Experience & Feedback:**
    - [ ] Improve Player Spawning and Death:
        - [x] Adjust player start position in `assets/levels/world_1_level_1.tmx`.
        - [ ] Implement an "angel" death sequence with animation and sound.
        - [ ] Implement a respawn animation.
    - [ ] Implement Player Lives and Game Over:
        - [ ] Display player lives in the debug text.
        - [ ] Implement a "Game Over" screen.
    - [ ] Implement Player Health & Damage:
        - [ ] Add a `health` component to the player.
        - [ ] Make enemies deal damage on contact.
        - [ ] Display player health (hearts) in the debug text.
        - [ ] Add "Medical Kit" or "Potion of Health" items to the game.
        - [ ] Implement logic for picking up health items to restore player health.
        - [ ] Brainstorm ideas for how health items can appear (e.g., dropped by enemies, in chests).
    - [ ] Implement Damped Camera Movement:
        - [x] Re-implement lerp-based smoothing to restore damping effect.
        - [ ] Implement "Look-Ahead" (Directional Bias).
        - [ ] Implement "Platform Snap".
    - [ ] Hide Mouse Cursor in Fullscreen: Call the appropriate SDL3 function to hide the mouse cursor during gameplay.
- [ ] **3. Content & Progression:**
    - [ ] Implement a Simple Enemy: Add an enemy with basic patrol AI and stomp mechanics.
    - [ ] Implement Level Goal and Progression:
        - [ ] Add a "Goal" object to the level.
        - [ ] Implement level transitions.

### Phase 4: Core Engine Features (Next Sprint)

- [ ] **1. Implement Menu System:** Develop a basic UI system with a title screen and menus (`MainMenu`, `InGame`, `Exit`).
- [ ] **2. Add Collectible Treasures:**
    - [ ] Add items like Stars and Gold to levels.
    - [ ] Implement a scoring system and UI display.
- [ ] **3. Implement Interactive Blocks:** Create a system for various types of interactive blocks, such as power-up blocks and breakable blocks.
- [ ] **4. Implement Power-Ups:**
    - [ ] Design a flexible system for power-ups that can modify the player's state and grant new abilities.
    - [ ] Add a "Run Fast" power-up.
    - [ ] Add a "Shoot" power-up with projectiles.
    - [ ] Add a "Fly" power-up.

### Phase 5: Gameplay Expansion (Backlog)

- [ ] **1. Create More Content:**
    - [ ] Create larger, more complex level maps (50-100% larger).
    - [ ] Create multiple levels (2-3 to start).
- [ ] **2. Add a Boss Fight:** Design and build a multi-phase boss encounter in a dedicated arena at the end of a level.
- [ ] **3. Implement a Companion/Sidekick:** Introduce a companion character with unique abilities.
- [ ] **4. Implement a World Map:** Implement a top-down world map for level selection.

### Phase 6: Advanced Engine Features (Backlog)

- [ ] **1. Implement Parallax Scrolling:** Add support for multi-layered, parallax backgrounds to create a sense of depth.
- [ ] **2. Implement Z-Layer Rendering:** Add a z-layer system to control the draw order of entities.
- [ ] **3. Implement Save/Load System:** Allow players to save and load their progress in the game.
- [ ] **4. Add Support for Sloped Surfaces:** Enhance the physics engine to correctly handle player and object interaction with sloped terrain.
- [ ] **5. Implement Spatial Partitioning:** Implement a Uniform Grid (e.g., 32x32 tile chunks) spatial partitioning system to efficiently manage large levels.
- [ ] **6. Implement Interactive Audio:** Create a system for dynamic music and sound effects that respond to gameplay events.

### Phase 7: Tooling (Future)

- [ ] **1. Create a Level Editor:** Build an in-game or external tool for creating and editing levels.
- [ ] **2. Implement Debugging Tools:** Create the planned in-game `Benchmarker` and on-screen debug display.
- [ ] **3. Implement Video Recording:** Add a system for saving gameplay videos using ffmpeg or a similar library.
- [ ] **4. Implement Cross-Platform Builds and Releases:**
    - [ ] Set up a CI/CD pipeline (e.g., using GitHub Actions) to automatically build and release binaries for the following platforms:
        - [ ] **Windows:** Compile to an `.exe` file using a target like `x86_64-pc-windows-gnu`.
        - [ ] **Linux:** Compile to a standard Linux executable.
        - [ ] **WebAssembly (WASM):** Compile the demo game to WASM to run in a web browser. This will likely involve using Emscripten to compile SDL3 and `wasm-pack` to package the Rust code.
    - [ ] Automate the process of creating a GitHub Release with the compiled binaries attached.

### Phase 8: Engine Hardening & Refactoring (Ongoing/Future)

- [ ] **1. Refactor for Maintainability:**
    - [ ] Refactor `app.rs` Initialization.
    - [ ] Refactor `player/states.rs` Logic.
    - [ ] Improve `level.rs` Loading.
    - [ ] Decompose `ecs/system.rs` Collision System.
- [ ] **2. Implement Robust Error Handling:**
    - [ ] Create a Custom Error Enum.
    - [ ] Implement Error Conversions.
    - [ ] Refactor for Robust Error Handling.
- [ ] **3. Write Comprehensive Tests:**
    - [ ] Create a Core Integration Test.
    - [ ] Add a Level Loading Test.
    - [ ] Implement an Input and Movement Test.
- [ ] **4. Refactor for Data-Driven Design:**
    - [ ] Identify and Externalize Hardcoded Values.
    - [ ] Refactor Code to Load from Configuration.
- [ ] **5. Refactor to a 1:1 Pixel Coordinate System:** Remove the `PIXEL_SCALE` constant.
- [ ] **6. Add Multiplayer Support:** Integrate 2-player local co-op or competitive gameplay.