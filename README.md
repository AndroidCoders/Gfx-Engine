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
*   The demo, "Captain Cat," is a platformer inspired by classic 16-bit era games.

Gfx-Engine is a modular 2D game engine written in Rust, designed for creating pixel-art platformers.

The project includes a playable "Captain Cat" demo game that showcases the engine's capabilities, which include:
*   A robust physics engine for platforming.
*   A state machine-driven animation system.
*   A data-driven design where levels, assets, and game properties are loaded from TOML configuration files.

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
