File version: 2.02

**TLDR:**
This document outlines the architecture and components of the `GfX-Engine`:
*   Simple, data-driven game loop architecture, with a future plan for ECS redesign.
*   Key components are modular files in `src/` (e.g., `renderer`, `audio`, `physics`).
*   Configurable VSync and fixed-timestep loop.

## Architecture

A simple, data-driven game loop architecture will be used. The engine's logic is separated from the game's data. The engine reads `config.toml` and loads assets from the `assets/` directory to run the game.

**Current Architecture: ECS-based**
The engine's core architecture is now built around the Entity-Component-System (ECS) pattern. This enhances modularity, reusability, and performance by strictly separating data (Components), logic (Systems), and entities (IDs).

## Components

The engine is composed of several modules within the `src/` directory:

-   `main.rs`: Entry point.
-   `app.rs`: Initializes SDL, creates the window, and runs the main game loop.
-   `renderer.rs`: Responsible for all drawing operations, including sprites.
-   `level.rs`: Manages loading and interaction with game levels.
-   `audio.rs`: Handles loading and playing sounds and music.
-   `input.rs`: Manages user input.
-   `player.rs`: Defines the player character's state and behavior.
-   `camera.rs`: Defines the camera for viewing the game world.
-   `texture_manager.rs`: Manages loading and storing textures.
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

*   **Interactive Blocks:** The game levels will include simple interactive elements, suchs as breakable blocks that the player can hit from below.

### Camera Design

The camera system is designed to be both smooth and responsive, keeping the player in focus without feeling jarring or restrictive. It is inspired by the camera in classic platformers like "Super Mario World" and is built on the following principles:

*   **Slow Zone (formerly Dead Zone):** An invisible area in the center of the screen where the player can move freely without the camera moving. This prevents distracting jitter from small movements.

*   **Fast Zone (formerly Panic Zone):** A zone near the edge of the screen. When the player enters this zone, the camera's speed is dramatically increased to prevent the player from going off-screen.

*   **Look-Ahead (Directional Bias):** The Slow Zone is not always centered. It shifts based on the direction the player is facing, showing more of the screen in front of the player. This gives the player a better view of what's ahead.

*   **Platform Snap:** The camera's vertical movement is tied to the platform the player is standing on. The camera does not move vertically during a jump, only "snapping" to the new vertical position when the player lands on a different platform. This prevents a nauseating up-and-down motion during normal gameplay.

## Entity State Management

To manage the behavior of all dynamic entities (including the player and AI enemies), we will implement a **Hierarchical State Machine (HSM)**. This unified pattern provides a robust and scalable foundation for both player control and AI logic. It organizes behavior into distinct states (e.g., `Idle`, `Patrolling`, `Jumping`) and manages the transitions between them. This prevents bugs by ensuring an entity is only in one state at a time, simplifies adding new abilities, and serves as the core driver for the animation system by linking each state directly to its corresponding animation. Our implementation will be a hybrid, supporting both continuous actions within states and instantaneous actions on transitions.

## Implemented Core Features

The engine currently has the following core features implemented:

*   **Compiling Proof of Concept:** Displays the "Super Cat Bros" sprite.
*   **Configuration:** Loads configuration from `.toml` files.
*   **Input System:** A data-driven input system is in place.
*   **Texture Management:** A `TextureManager` for loading and managing textures.
*   **Renderer:** Can draw sprites and level geometry.
*   **Level Loading:** A basic level loading system from `.toml` files.
*   **Camera:** A damped camera that smoothly follows the player.
*   **Physics:** Basic physics, including gravity, jumping, and collision detection.
*   **Player Movement:** Player movement (left/right) and sprite flipping.
*   **Animation:** A state-driven, multi-frame sprite animation system.
*   **Audio:** An event-driven audio system using the `kira` crate.

## Debugging and Profiling

To facilitate robust testing and optimization, the following systems are planned:

*   **In-Game Benchmarking:** A real-time, in-game profiler (`Benchmarker.rs`) will be created to monitor and display key performance metrics such as frame time, FPS, and time spent in the update and render loops.

*   **On-Screen Display and Logging:** A comprehensive debug system (`debug.rs`) will be implemented. It will render key data (e.g., player coordinates, state) on-screen for video analysis and simultaneously record verbose, high-resolution data to a log file. A shared timestamp or frame number will link the video frames to the log entries.

*   **Programmatic Video Capture:** To enable automated analysis and capture hard-to-reproduce bugs, the engine will integrate programmatic video recording. This will be achieved by using a Rust wrapper for the `ffmpeg` library (such as `ffmpeg-next`), allowing the engine to start and stop video capture from within the code.

## Future Implementations

### Menu System Implementation

A menu system is required to manage different application states, such as the main menu, options screen, and the game itself. This involves creating a game state manager and a UI rendering system.

1.  **Add Dependencies:** Add the `sdl3_ttf` crate to `Cargo.toml` to enable text rendering.
2.  **Create Game State Manager:** In `app.rs`, define a `GameState` enum (e.g., `MainMenu`, `Options`, `InGame`). Add a field `game_state: GameState` to the `App` struct to track the current state.
3.  **Refactor Main Loop:** Modify the `run()` method in `app.rs` to delegate logic based on the current `game_state`. Use a `match` statement to call state-specific update and render functions (e.g., `update_game()`, `render_game()`, `update_menu()`, `render_menu()`).
4.  **Implement UI Module:** Create a new `ui.rs` module. This module will define the structure for menus, including buttons, text labels, and logic for handling navigation (e.g., tracking the currently selected button).
5.  **Implement Text Rendering:** Create a `FontManager` to load and manage font files, similar to the `TextureManager`. Extend the `Renderer` to include a method for drawing text to the screen using the loaded fonts.
6.  **Handle Menu Input:** In the main loop, when the `game_state` is `MainMenu` or `Options`, process input differently. Instead of player actions, listen for `Up`, `Down`, and `Enter` keys to navigate the menu. An `Enter` press will trigger a change in the `GameState` (e.g., from `MainMenu` to `InGame`).

## Future Improvements

*   **Mid-Air Control:** The player's control while in the air will be adjusted to prevent instant turning and acceleration, providing a more realistic feel.

## Coordinate System and Scaling

To ensure consistency and prevent bugs, the engine uses a strict separation between different coordinate systems.

*   **World Units:** All game logic, physics, and configuration files operate in **World Units**.
    *   **Definition:** 1 World Unit corresponds to 1 pixel of the source art assets (e.g., a sprite that is 32x32 pixels in its PNG file has a size of 32x32 World Units).
    *   **Usage:** Used for entity positions, sizes, physics calculations, and all values specified in configuration files like `game_config.toml`.

*   **Virtual Resolution:** The game is rendered internally to a fixed-size canvas, known as the **Virtual Resolution**.
    *   **Definition:** This is the ideal resolution the game is designed for, such as `1920x1080`. It is set in `config.toml`.
    *   **Purpose:** It provides a consistent rendering target, independent of the player's actual screen resolution.

*   **`PIXEL_SCALE`:** This is a global scaling factor used by the renderer.
    *   **Definition:** A constant value (e.g., `2.0`) that determines how many screen pixels are used to draw a single World Unit onto the Virtual Resolution canvas.
    *   **Example:** With a `PIXEL_SCALE` of `2.0`, a 32x32 World Unit sprite is rendered as a 64x64 pixel image on the internal virtual canvas.

*   **Screen Pixels (Final Resolution):** This is the actual resolution of the player's monitor.
    *   **Behavior:** The engine takes the rendered `1920x1080` Virtual Resolution canvas and scales it to fit the player's screen.
    *   **Aspect Ratio:** The 16:9 aspect ratio is always preserved. If the screen's aspect ratio is different, black bars will be added (pillarboxing or letterboxing). This guarantees that the game's appearance and gameplay are identical on all displays.