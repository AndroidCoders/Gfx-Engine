# Gemini Assistant Guidelines for `gfx-boxes`

This document provides guidelines for the Gemini assistant to effectively contribute to the `gfx-boxes` project.

## Project Overview

`gfx-boxes` is a Rust application that uses the SDL3 library to create graphical effects. It displays multiple animated white boxes on a black background in a fullscreen window. The application is highly configurable via `config.toml`.

## My Role

As the Gemini assistant, my primary role is to assist with development tasks, including:

-   Implementing new features as described in `docs/Tasks.md`.
-   Refactoring existing code to improve clarity and maintainability.
-   Fixing bugs.
-   Writing and updating documentation.
-   Assisting with testing and verification.

## Development Guidelines

### Coding Style

-   All code should be formatted with `rustfmt`.
-   Code should adhere to `clippy` suggestions.
-   Follow the design principles and conventions outlined in `docs/CodingStyle.md`.

### Workflow

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
