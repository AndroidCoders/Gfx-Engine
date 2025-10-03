File version: 3.00

**TLDR:**
This document lists the Development Tasks for the `GfX-Engine` project.

### Phase 1: Core Engine Refactoring

- [x] Define new project vision and goals.
- [x] Implement new, simplified directory structure.
- [x] Update project identity (`README.md`, `Cargo.toml`).
- [x] Update all documentation in `docs/`.
- [x] Create a minimalistic compiling POC (white box on black background).
- [x] Refactor configuration to support sprite animations.
- [x] Implement a device-independent, data-driven input system.
- [x] Implement a sprite animation system.

### Phase 2: "Captain Cat" Demo - Core Gameplay

-   **Initial Setup:**
    -   [x] Create a new `feature/captain-cat-demo` branch for the demo implementation.
-   **Asset Loading and Rendering:**
    -   [x] Load and display the Captain Cat character sprite instead of the white box.
    -   [x] Implement sprite flipping for left/right movement.
-   **Player Control:**
    -   [x] Implement basic player movement (left/right) based on input.
    -   [x] Implement Platform Feature - Variable Jump Height.
-   **Game World & Gameplay:**
    -   [x] Create a simple level layout in a config file and render it.

### Phase 3: Next Sprint

-   [ ] **1. Implement Multi-Frame Animations:** Extend the animation system to support sprite sheets with multiple frames to create fluid walking cycles.
-   [ ] **2. Add Textured World Graphics:** Replace the placeholder level blocks with textured graphics loaded from files.
-   [ ] **3. Implement a Simple Enemy:** Add an enemy with basic patrol AI and stomp mechanics, using our new state machine pattern.
-   [ ] **4. Implement Damped Camera Movement:** Improve the camera to follow the player smoothly, enhancing the game's professional feel.

### Future Goals (Backlog)

-   [x] Implement Platform Feature - Momentum-Based Movement.
-   [ ] Develop a very basic UI system with a title screen and menus.
-   [ ] Refactor hard-coded data out of the source code.
-   [ ] Improve player control in mid-air to prevent instant turning and acceleration.
-   [ ] Implement a debugging and profiling system.
-   [ ] Add collectible items (e.g., treasure chests) and a score display.
-   [ ] Implement `level.rs` to load level data from Tiled map files.
-   [ ] Implement `audio.rs` to load and play sounds.
-   [ ] Add text rendering support.
-   [ ] Write comprehensive tests for the engine.