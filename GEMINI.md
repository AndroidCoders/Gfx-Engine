# Gemini Assistant Guidelines for `Gfx-Engine`

This document provides guidelines for the Gemini assistant to effectively contribute to the `Gfx-Engine` project.

## Project Overview

`Gfx-Engine` is a 2D pixel art game engine written in Rust.

The project implements a state-of-the art platform game engine, including a complete demo game.

## My Role

As the Gemini expert coding assistant, my primary role is to assist with development tasks by adhering to the following responsibilities:

-   **Onboarding:** Always start by executing the "Onboarding Guide" at the end of this document.
-   **Planning:** Discuss new Features and Improvements using `docs/Design.md`, and update `docs/Tasks.md` before writing code.
-   **Implementation:** Implement features and fix bugs using the **Edit -> Test -> Commit** workflow.
-   **Refactoring:** Proactively suggest improvements to align with the Event-Driven ECS Architecture and "Atomic Module" (<300 lines) standards.
-   **Documentation:** Keep `docs/` synchronized with the source code.
-   **Verification:** Ensure all changes pass `cargo test` and `cargo clippy`. For visual features, ask the user for confirmation.

## Development Guidelines

### Coding Style

-   All code should be formatted with `rustfmt`.
-   Code should adhere to `clippy` suggestions.
-   Follow the design principles and conventions outlined in `docs/CodingStyle.md`.

### Workflow

-   **Edit -> Test -> Commit:** Strictly follow this loop. Make a small, atomic change, verify it immediately with `cargo check/test`, and commit before moving to the next step. Never accumulate large unverified changes.
-   **Feature Branches:** All work must be done on branches (e.g., `feature/my-new-feature`).
-   **Commits:** Use [Conventional Commits](https://www.conventionalcommits.org/).
-   **Reference:** See `docs/Workflow.md` for the detailed process.

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
-   **Game Logic Issues:** For unexpected behavior in game mechanics, examine relevant ECS systems in `src/ecs/systems/` and `src/app.rs`. Ensure that `world.game_state` is being updated correctly as it is the single source of truth for the engine's active systems.
-   **Performance Bottlenecks:** Check the **Hotspots** overlay (Top Right, F1) and `benchmark.log` for CPU-intensive systems.
-   **Audio Underruns (Linux):** If you hear crackling or see "underrun" errors, try running with `SDL_AUDIO_ALSA_SET_BUFFER=1 cargo run`.
-   **Configuration Problems:** Check the files in `src/config/` and `assets/game_config.toml`.

## Key Files

-   `src/main.rs`: Application entry point.
-   `src/app.rs`: Main application loop.
-   `src/ecs/system_manager.rs`: The **Explicit Scheduler** defining the WYSIWID execution order.
-   `src/ecs/systems/synchronization.rs`: The **Rules Engine** where cross-system logic resides.
-   `src/ecs/component.rs`: Core data definitions (The "What").
-   `src/ecs/resources.rs`: Global shared state (The "Context").
-   `src/renderer.rs`: Handles all drawing operations.
-   `config.toml`: Global engine settings.
-   `assets/game_config.toml`: Game-specific data, prefabs, and animations.
-   `docs/Design.md`: The primary architecture guide.

## User Preferences

-   **WYSIWID Architecture:** Rigorously adhere to the "What You See Is What It Does" pattern. Move all glue logic into Synchronization systems.
-   **Atomic Modules:** Strive to keep source files under **300 lines**. If a file grows larger, refactor it into smaller, single-responsibility modules.
-   **State Management:** Always use `world.game_state` as the single source of truth.
-   **Confirmation:** Ask for confirmation before starting a new Task.
-   **Documentation:** Keep the Guiding Documents in `docs/` synchronized with the source code.

## Onboarding Guide for LLM Coding Agents

-   Please execute the following Actions in sequence:

1. Acquire Context: Read README.md, Cargo.toml, config.toml, and all files in docs/.
    * Goal: Understand the project scope, architecture (Design.md), and development rules (Workflow.md).
2. Verify Baseline: Run cargo test and cargo clippy.
    * Goal: Confirm the environment is working and the codebase is currently stable/clean.
3. Map Architecture: Read src/main.rs, src/app.rs, src/lib.rs, src/ecs/mod.rs, and **src/ecs/system_manager.rs**.
    * Goal: Identify the entry point, the main loop, and the **Explicit Scheduler (SystemManager)** which defines the WYSIWID execution flow.
4. Analyze Rules & Systems: Read **src/ecs/systems/synchronization.rs**, src/ecs/systems/lifecycle.rs, and list/read other files in src/ecs/systems/.
    * Goal: Understand the "Game Rules" (Synchronizations) and independent logic units (Concepts).
5. Analyze Data & Assets: Read src/ecs/component.rs, src/ecs/resources.rs, and other source files in src/.
    * Goal: Understand the Data Schema (Components) and Global State (Resources).
6. Check and update the project backlog: Read docs/Tasks.md and update completed tasks if needed.
    * Goal: Find any tasks that are already completed (according to the source code) and check the boxes of those tasks.
    * Goal: Update the backlog with new tasks, and update current tasks if necessary
7. Synthesize & Report: Based on your analysis, provide a concise "Onboarding Report" that includes:
    * A summary of the architecture, in 200-1000 words.
    * Confirmation of test status.
    * One to twenty (1-20) concrete suggestions for improvements, refactoring candidates or next tasks, referencing docs/Tasks.md. List the 1-20 suggestions in order of importance.

-   Based on you initial Report, we will continue to develop and improve the project.
- 
