File version: 7.00

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

    - [ ] Re-implement lerp-based smoothing to restore damping effect.

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



### Phase 7: Engine Hardening & Polish (Future)



- [ ] **1. Implement Robust Error Handling:** Replace `.map_err(|e| e.to_string())` with a dedicated `EngineError` enum for more structured and debuggable error management.

- [ ] **2. Write Comprehensive Tests:** Build out a suite of unit and integration tests to ensure code quality and prevent regressions.



### Ongoing Tasks



- [ ] **Refactor for Modularity:** Continuously move hard-coded data from source files (`.rs`) into external configuration files.

- [x] **Redesign to ECS Architecture:** Plan and execute a major refactoring to adopt an Entity-Component-System (ECS) architecture for improved modularity, reusability, and performance.
