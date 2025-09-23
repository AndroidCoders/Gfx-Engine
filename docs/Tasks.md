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

- [ ] Create a data-driven demo game called "Captain Cat".
- [ ] Implement Platform Feature - Variable Jump Height.
- [ ] Implement Platform Feature - Momentum-Based Movement.
- [ ] Implement Platform Feature - Stomping on Enemies.
- [ ] Implement Platform Feature - Interactive Blocks.

### Phase 3: Polish & Expansion

- [ ] Implement `level.rs` to load level data from Tiled map files.
- [ ] Implement `audio.rs` to load and play sounds.
- [ ] Add text rendering support.
- [ ] Develop a simple UI system.
- [ ] Write comprehensive tests for the engine.