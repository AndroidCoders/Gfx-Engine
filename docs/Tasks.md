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
    -   [ ] Create a new `feature/captain-cat-demo` branch for the demo implementation.
-   **Asset Loading and Rendering:**
    -   [ ] Load and display the Captain Cat character sprite instead of the white box.
    -   [ ] Implement sprite flipping for left/right movement.
-   **Player Control:**
    -   [ ] Implement basic player movement (left/right) based on input.
    -   [ ] Implement Platform Feature - Variable Jump Height.
    -   [ ] Implement Platform Feature - Momentum-Based Movement.
-   **Game World & Gameplay:**
    -   [ ] Create a simple level layout in a config file and render it.
    -   [ ] Add collectible items (e.g., treasure chests) and a score display.
    -   [ ] Add simple enemies (e.g., crocodiles) and the ability to stomp on them.
-   **UI:**
    -   [ ] Create a basic title screen with "Start Game" and "Exit" options.

### Phase 3: Polish & Expansion

- [ ] Implement `level.rs` to load level data from Tiled map files.
- [ ] Implement `audio.rs` to load and play sounds.
- [ ] Add text rendering support.
- [ ] Develop a simple UI system.
- [ ] Write comprehensive tests for the engine.