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
## Session Notes (2025-11-12)

**Goal:** Make the `Gfx-Engine` project more data-driven by removing hardcoded values from the source code.

**Branch:** `refactor/data-driven-improvements`

**Progress:**
-   **Frame-Rate Independence:**
    -   [x] Calculated `delta_time` in the main loop (`app.rs`).
    -   [x] Passed `delta_time` to all systems via the `SystemContext`.
    -   [x] Refactored all timer-based logic to use `delta_time`, making it frame-rate independent.
-   **Externalized Assets and Configs:**
    -   [x] Moved the hardcoded `start_level` path from `app.rs` to `config.toml`.
    -   [x] Moved hardcoded texture paths (`bg_sky`, `goal`, `game_over_3`) from `app.rs` to `assets/game_config.toml`.
    -   [x] Refactored `app.rs` to load these assets using the new configuration values.

**Next Steps:**
-   Continue with the "Fully Embrace Data-Driven Design" plan from `docs/Tasks.md`.
-   Externalize game logic parameters (e.g., player health, knockback force) to config files.
-   Externalize sound effect names.
-   Refactor the enemy state machine to be generic and not tied to a specific enemy type.
-   Create a comprehensive entity prefab system in the configuration files.