File version: 2.00

**TLDR:**
This document outlines the architecture and components of the `GfX-Engine`:
*   Simple, data-driven game loop architecture.
*   Key components are modular files in `src/` (e.g., `renderer`, `audio`, `physics`).
*   Configurable VSync and fixed-timestep loop.

## Architecture

A simple, data-driven game loop architecture will be used. The engine's logic is separated from the game's data. The engine reads `config.toml` and loads assets from the `assets/` directory to run the game.

## Components

The engine is composed of several modules within the `src/` directory:

-   `main.rs`: Entry point.
-   `app.rs`: Initializes SDL, creates the window, and runs the main game loop.
-   `renderer.rs`: Responsible for all drawing operations, including sprites.
-   `game_state.rs`: Holds the overall state of the game.
-   `level.rs`: Manages loading and interaction with game levels.
-   `audio.rs`: Handles loading and playing sounds and music.
-   `input.rs`: Manages user input.
-   `physics.rs`: Handles the physics simulation.
-   `config.rs`: Loads and parses all configuration files.

## Game Loop

To ensure a stable and smooth visual experience, we use a **fixed-timestep game loop** with **VSync enabled**. These parameters are configurable via `config.toml`. This approach decouples the game logic from the rendering rate, providing a consistent animation speed on all hardware.

## Rendering Pipeline

The rendering pipeline in `Gfx-Engine` is designed for modularity and clear separation of concerns.

*   **Centralized `Renderer`:** The `renderer.rs` module encapsulates all drawing logic. It receives the necessary game data (`GameState`, `GameConfig`, `TextureManager`) and is solely responsible for determining what to draw and how to draw it.
*   **Simplified `App` Loop:** The main application loop (`app.rs`) is streamlined. Its role is to orchestrate the game loop phases (input, update, render) and trigger the `Renderer` each frame, without containing rendering-specific logic.
*   **Clear Ownership:** The `App` struct directly owns the `TextureCreator` and the `virtual_canvas_texture`. The `TextureManager` is dedicated to loading and managing game asset textures. This explicit ownership model prevents Rust's borrow checker conflicts and ensures resource lifetimes are correctly handled.

## Gameplay Mechanics

To create a robust and engaging platformer experience, the following gameplay mechanics, inspired by classics like "Super Mario World," are planned for implementation:

*   **Variable Jump Height:** The player will be able to control the height of their jump by holding down the jump button. This allows for more nuanced platforming challenges.

*   **Momentum-Based Movement:** The player character will have acceleration and deceleration, giving the movement a sense of weight and making the controls feel smoother and more natural.

*   **Stomping on Enemies:** As a primary form of interaction, the player will be able to defeat enemies by jumping on top of them.

*   **Interactive Blocks:** The game levels will include simple interactive elements, such as breakable blocks that the player can hit from below.