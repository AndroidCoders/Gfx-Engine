File version: 3.00

**TLDR:**
This document lists the Development Tasks for the `GfX-Engine` project.

### Phase 1: Core Engine Refactoring

- [x] Define new project vision and goals.
- [x] Implement new, simplified directory structure.
- [x] Update project identity (`README.md`, `Cargo.toml`).
- [x] Update all documentation in `docs/`.
- [x] Create a minimalistic compiling POC (white box on black background).
- [ ] Refactor configuration to support sprite animations.
- [ ] Implement a device-independent, data-driven input system.
- [ ] Implement a sprite animation system.

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

### Phase 3: Next Steps

-   **Core Gameplay Features:**
    -   [ ] Implement Platform Feature - Momentum-Based Movement.
    -   [ ] Load and use textures for the world and platform graphics.
    -   [ ] Add simple enemies (e.g., crocodiles) and the ability to stomp on them.
-   **UI:**
    -   [ ] Develop a very basic UI system with a title screen and menus.
-   **Graphics:**
    -   [ ] Implement a sprite animation system.
-   **Polish & Expansion:**
    -   [ ] Improve player control in mid-air to prevent instant turning and acceleration.
    -   [ ] Implement a damped/soft camera follow system for more realistic camera movement.
    -   [ ] Implement a debugging and profiling system.
    -   [ ] Add collectible items (e.g., treasure chests) and a score display.
    -   [ ] Implement `level.rs` to load level data from Tiled map files.
    -   [ ] Implement `audio.rs` to load and play sounds.
    -   [ ] Add text rendering support.
    -   [ ] Write comprehensive tests for the engine.