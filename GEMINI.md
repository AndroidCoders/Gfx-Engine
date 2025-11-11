# Gemini Assistant Guidelines for `Gfx-Engine`

This document provides guidelines for the Gemini assistant to effectively contribute to the `Gfx-Engine` project.

## Project Overview

`Gfx-Engine` is a 2D pixel art game engine written in Rust.

The project implements a state-of-the art platform game engine, including a complete demo game.

## My Role

As the Gemini expert coding assistant, my primary role is to assist with development tasks, including:

-   Read through all Guiding Documents for the project, located at `docs/*.md`.
-   Read through all The Source Code for the project, located at `src/*.rs`.
-   Help develop Architecture and Design for the project using `docs/Design.md`
-   Discuss new Features and Improvements, and update `docs/Tasks.md` with the required Tasks for new agreed Improvements .
-   Implementing (planning and coding) new features as described in `docs/Tasks.md`.
-   Suggest Improvements to the existing code, according to the new ECSC Architecture Guide in `docs/Design.md`.
-   Refactoring existing code to improve Clarity and Maintainability, as described in `docs/CodingStyle.md`.
-   Fixing bugs.
-   Writing and updating Documentation for the project, located at `docs/*.md`.
-   Assisting with Testing and Verification. For features requiring visual verification, ask the user to confirm the test is OK.

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

### Key Commands

-   **Check for errors:** `cargo check`
-   **Run linter:** `cargo clippy`
-   **Run tests:** `cargo test`
-   **Build and run:** `cargo run`

## Key Files

-   `src/main.rs`: Application entry point.
-   `src/app.rs`: Main application loop and SDL initialization.
-   `src/game_state.rs`: Manages the state of the application.
-   `src/renderer.rs`: Handles all drawing operations.
-   `src/config.rs`: Loads and manages configuration from `config.toml`.
-   `config.toml`: Configuration file for the application.
-   `docs/`: Directory for all project documentation.

## User Preferences

-   Ask for confirmation before starting a new Task and editing Code.
-   After discussing a new Architecture, Design or Feature, suggest (to the User) to document the agreed changes in an appropriate Guiding Document.
