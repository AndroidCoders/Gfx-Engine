File version: 1.06

**TLDR:**
This document provides an overview of the `GfX-Engine` project's file structure:
* Main directories
* Key source code and documentation files

README.md: Project Description.

docs/: Folder for The Guiding Documents.
docs/Home.md: Project Homepage.
docs/Structure.md: File Structure Overview.
docs/Product.md: Product Description.
docs/Tech.md: Technology Stack.
docs/Tasks.md: Project Tasks (Product Backlog).
docs/Design.md: Design & Architecture.
docs/CodingStyle.md: Coding Conventions & Development Guidelines.
docs/Workflow.md: Development Workflow with GitHub.
docs/Testing.md: Testing Strategy.
docs/MasterMerge.md: Master Branch Merge Workflow.
docs/Documentation.md: Rust Documentation Best Practices.
docs/research/: Research documents.
docs/demos/: Demo-specific documentation.

src/main.rs: Main application entry point.
src/app.rs: Initializes SDL, creates the window, and runs the main application loop.
src/ecs/: The core Entity-Component-System module.
src/ecs/mod.rs: Declares the sub-modules of the `ecs` crate.
src/ecs/world.rs: Defines the `World` that holds all entities and components.
src/ecs/component.rs: Defines all components used in the ECS.
src/ecs/event.rs: Defines the type-based event bus and event structs.
src/ecs/systems/: Contains all the systems that operate on entities.
src/renderer.rs: Handles all drawing operations.
src/physics.rs: Contains generic physics and collision detection logic.
src/level.rs: Manages loading and representing game levels from Tiled `.tmx` files.
src/config.rs: Defines and loads configuration from `.toml` files.
src/state_machine.rs: Defines the generic state machine for entities.
src/player/: Module for player-specific logic.
src/player/mod.rs: Declares the sub-modules of the `player` crate.
src/player/states.rs: Defines the states for the player's state machine.
src/enemy/: Module for enemy-specific logic.
src/enemy/mod.rs: Declares the sub-modules of the `enemy` crate.
src/enemy/states.rs: Defines the states for the enemy's state machine.
src/audio.rs: Handles audio loading and playback.
src/input.rs: Manages user input.
src/camera.rs: Defines the camera for viewing the game world.
src/texture_manager.rs: Manages loading and storing textures.
src/animation.rs: Defines structures for managing sprite animations.
src/math.rs: Defines common mathematical structures.
src/frame_capture.rs: Handles capturing frames for debugging or video output.