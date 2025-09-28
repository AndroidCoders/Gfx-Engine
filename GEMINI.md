# Gemini Assistant Guidelines for `Gfx-Engine`

This document provides guidelines for the Gemini assistant to effectively contribute to the `Gfx-Engine` project.

## Project Overview

`Gfx-Engine` is a 2D pixel art game engine written in Rust.

The project implements a state-of-the art platform game engine, including a complete demo game.

## My Role

As the Gemini expert coding assistant, my primary role is to assist with development tasks, including:

-   Read through all Guiding Documents for the project, located at `docs/*.md`.
-   Read through all source code for the project.
-   Implementing new features as described in `docs/Tasks.md`.
-   Help develop architecture and design for the project
-   Suggest improvements to the existing code
-   Refactoring existing code to improve clarity and maintainability, as described in `docs/CodingStyle.md`.
-   Fixing bugs.
-   Writing and updating documentation.
-   Assisting with testing and verification.

## Development Guidelines

### Coding Style

-   All code should be formatted with `rustfmt`.
-   Code should adhere to `clippy` suggestions.
-   Follow the design principles and conventions outlined in `docs/CodingStyle.md`.

### Workflow

-   Follow the workflow guide outlined in `docs/Workflow.md`.
-   All work should be done on feature branches (e.g., `feature/my-new-feature`).
-   Commit messages should follow the Conventional Commits specification.
-   Pull requests are used to merge changes into the `main` branch.

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
