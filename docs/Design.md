File version: 1.02

**TLDR:**
This document outlines the architecture and components of the `gfx-boxes` project:
* Simple game loop architecture
* Key components: `app`, `renderer`, `game_state`, `frame_capture`
* Configurable VSync and fixed-timestep

## Architecture

A simple game loop architecture will be used.

## Components

- `main.rs`: Entry point
- `app.rs`: Initializes SDL, creates the window, and runs the main loop.
- `renderer.rs`: A module responsible for drawing on the canvas.
- `game_state.rs`: A module to hold the state of the application (e.g., a list of game objects with positions and velocities).
- `frame_capture.rs`: A module for capturing and saving frames for debugging, configurable via `config.toml`.

## Game Loop

To ensure a stable and smooth visual experience, we use a **fixed-timestep game loop** with **VSync enabled**. These parameters are now configurable via `config.toml`. This approach decouples the game logic from the rendering rate, providing a consistent animation speed on all hardware.

### VSync (Vertical Synchronization)

VSync synchronizes our application's rendering with the monitor's refresh rate. This prevents screen tearing and results in a smooth animation that automatically adapts to the user's display (e.g., 60 FPS on a 60Hz monitor, 144 FPS on a 144Hz monitor). This can be enabled or disabled in `config.toml`.

### Fixed-Timestep

The game logic (e.g., animation, physics) is updated at a fixed rate, configurable via `updates_per_second` in `config.toml`. This ensures that the animation is deterministic and behaves consistently on all computers.

## Version Control

This project uses Git, with development done on `feature/` branches that are merged into `main` upon completion.
