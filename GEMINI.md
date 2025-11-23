# Gemini Assistant Guidelines for `Gfx-Engine`

This document provides guidelines for the Gemini assistant to effectively contribute to the `Gfx-Engine` project.

## Project Overview

`Gfx-Engine` is a 2D pixel art game engine written in Rust.

The project implements a state-of-the art platform game engine, including a complete demo game.

## My Role

As the Gemini expert coding assistant, my primary role is to assist with development tasks, including:

-   Read through all Guiding Documents for the project, located at `docs/*.md`.
-   Read through all The Source Code for the project, located at `src/*.rs`.
-   Help develop Architecture and Design for the project using `docs/Design.md`.
-   Discuss new Features and Improvements, and update `docs/Tasks.md` with the required Tasks for new agreed Improvements.
-   Implementing (planning and coding) new features as described in `docs/Tasks.md`.
-   Suggest Improvements to the existing code, according to the new ECSC (Entity-Component-System-Context) Architecture Guide in `docs/Design.md`.
-   Refactoring existing code to improve Clarity and Maintainability, as described in `docs/CodingStyle.md`.
-   Fixing bugs.
-   Writing and updating Documentation for the project, located at `docs/*.md`.
-   Assisting with Testing and Verification. For features requiring visual verification, ask the user to confirm the test is OK. When identifying relevant tests, look for `tests/` directories or `#[test]` attributes in relevant modules.

## Development Guidelines

### Coding Style

-   All code should be formatted with `rustfmt`.
-   Code should adhere to `clippy` suggestions.
-   Follow the design principles and conventions outlined in `docs/CodingStyle.md`.

### Workflow

-   Follow the workflow guide outlined in `docs/Workflow.md`.
-   All work should be done on feature branches (e.g., `feature/my-new-feature`).
-   Commit messages should follow the Conventional Commits specification.
-   Pull requests are used to merge changes into the `master` branch.

### Project Conventions

-   **Naming Conventions:** Adhere to Rust's idiomatic naming conventions (e.g., `snake_case` for functions and variables, `PascalCase` for types and enums).
-   **Error Handling:** Prefer `Result<T, String>` for fallible operations, returning descriptive error messages.
-   **Documentation:** Use `///` for public API documentation and `//!` for module-level documentation.
-   **Module Organization:** Modules should ideally focus on a single, well-defined responsibility.

### Key Commands

-   **Check for errors:** `cargo check`
-   **Run linter:** `cargo clippy`
-   **Run tests:** `cargo test`
-   **Build and run:** `cargo run`

### Common Debugging & Troubleshooting

-   **Visual Bugs:** For issues related to rendering or display, investigate `src/renderer.rs` and `src/texture_manager.rs`.
-   **Game Logic Issues:** For unexpected behavior in game mechanics, examine relevant ECS systems in `src/ecs/systems/` and `src/app.rs`.
-   **Configuration Problems:** Check `src/config.rs` and `config.toml` for incorrect settings.

## Key Files

-   `src/main.rs`: Application entry point, handles initial setup and main loop orchestration.
-   `src/app.rs`: Main application loop and SDL initialization, manages overall game flow.
-   `src/game_state.rs`: Manages the state of the application, including game logic and data.
-   `src/renderer.rs`: Handles all drawing operations, responsible for visual output.
-   `src/config.rs`: Loads and manages configuration from `config.toml`, providing global settings.
-   `config.toml`: Configuration file for the application, defining game parameters.
-   `docs/`: Directory for all project documentation, including design and workflow guides.

## User Preferences

-   Ask for confirmation before starting a new Task and editing Code.
-   After discussing a new Architecture, Design or Feature, suggest (to the User) to document the agreed changes in an appropriate Guiding Document.
-   Keep the Guiding Documents in `docs/` synchronized with the source code. After implementing changes, review and update the documentation accordingly.

---
## Session Notes (2025-11-16)

**Goal:** Refactor the game engine to use a type-based event bus, decoupling systems according to the ECSC (Entity-Component-System-Concept) architecture.

**Branch:** `refactor/ecsc-event-bus`

**Progress:**
-   **Event Bus Implementation:**
    -   [x] Created a generic, type-based `EventBus` in `src/ecs/event.rs`.
    -   [x] Integrated the `EventBus` into the main `World` struct.
    -   [x] Added a `world.clear_events()` call at the end of the main loop to clear events each frame.
-   **Coin Collection Refactoring:**
    -   [x] Defined a `CoinCollectedEvent`.
    -   [x] Created a new `AudioConductorSystem` to listen for audio-related events.
    -   [x] Refactored `CoinCollectionSystem` to publish a `CoinCollectedEvent` instead of directly calling the audio manager.
    -   [x] Updated `AudioConductorSystem` to play a sound upon receiving a `CoinCollectedEvent`.
-   **Interaction Refactoring:**
    -   [x] Defined `PlayerStompedEnemyEvent` and `PlayerTookDamageEvent`.
    -   [x] Refactored `InteractionSystem` to publish these events instead of directly triggering audio.
    -   [x] Updated `AudioConductorSystem` to handle the new events and play the appropriate sounds.
-   **Jump Mechanic Refactoring:**
    -   [x] Defined a `PlayerJumpEvent`.
    -   [x] Created a new `PlayerControlSystem` to handle jump logic.
    -   [x] Refactored `InputSystem` to publish a `PlayerJumpEvent` instead of directly manipulating the player's velocity.

**Next Steps:**
-   Continue the ECSC refactoring for other game logic.
-   Merge the `refactor/ecsc-event-bus` branch into `master` when the user is ready.
