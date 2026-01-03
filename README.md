```
 █████  ██████ ██   ██      ██████ ███   ██  █████  ██ ███   ██ ██████
██      ██      ██ ██       ██     ██ █  ██ ██      ██ ██ █  ██ ██
██  ███ ████     ███  █████ ████   ██  █ ██ ██  ███ ██ ██  █ ██ ████
██   ██ ██      ██ ██       ██     ██  █ ██ ██   ██ ██ ██  █ ██ ██
 █████  ██     ██   ██      ██████ ██   ███  █████  ██ ██   ███ ██████
                                                                              
```

# Gfx-Engine

**TLDR:**
*   Gfx-Engine is a 2D pixel-art game engine built with Rust and SDL3.
*   It features a data-driven architecture and includes a complete demo game.
*   The demo, "Super Cat Bros - Episode 1 - The Pirate Gold Adventure," is a high-fidelity platformer.

Gfx-Engine is a modular 2D game engine written in Rust, designed for creating professional pixel-art platformers.

The project includes a playable demo that showcases the engine's capabilities:
*   **120Hz Fixed Timestep:** Deterministic physics and logic with state interpolation.
*   **Input Replay System:** Built-in recording and playback for "Attract Mode" and debugging.
*   **Audio Beat Detection:** Runtime FFT analysis for rhythm-synced gameplay.
*   **Observability:** Real-time hierarchical profiler and performance hotspots HUD.
*   **Data-Driven Design:** Everything (levels, prefabs, animations, input) is configured in TOML.

## How to Run

This project uses Cargo, the Rust package manager. To build and run the demo game:
```bash
cargo run
```

## Configuration

The engine's behavior is controlled by several TOML files:
*   `config.toml`: Core engine settings (window size, global physics).
*   `assets/game_config.toml`: Game-specific settings (player properties, asset paths, animations).
*   `assets/levels/`: Contains individual level layout files.
