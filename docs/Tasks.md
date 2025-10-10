File version: 6.00

**TLDR:**
This document lists the Development Tasks for the `GfX-Engine` project, organized into a clear roadmap.

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
- [ ] **2. Implement Basic Sound Effects:** Integrate an audio system for key gameplay events.
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
- [ ] **4. Expand World Content:**
    - [ ] Create larger, more complex level maps (50-100% larger).
    - [ ] Implement a system for level start/goal locations.
- [ ] **5. Implement Debugging Tools:** Create the planned in-game `Benchmarker` and on-screen debug display.

### Phase 5: Player Abilities (Backlog)

- [ ] **Implement Power-Ups:**
    - [ ] Add a "Run Fast" power-up.
    - [ ] Add a "Shoot" power-up with projectiles.
    - [ ] Add a "Fly" power-up.

### Phase 6: Advanced Features & Tooling (Future)

- [ ] **1. Implement a Boss Fight:** Design and build a multi-phase boss encounter in a dedicated arena.
- [ ] **2. Implement a World Map:** Create a top-down world map screen for level selection.
- [ ] **3. Implement a Simple In-Game Level Editor.**
- [ ] **4. Write Comprehensive Tests.**

### Ongoing Tasks

- [ ] **Refactor for Modularity:** Continuously move hard-coded data from source files (`.rs`) into external configuration files.
