File version: 2.01

**TLDR:**
This document outlines the architecture and components of the `GfX-Engine`:
*   Simple, data-driven game loop architecture, with a future plan for ECS redesign.
*   Key components are modular files in `src/` (e.g., `renderer`, `audio`, `physics`).
*   Configurable VSync and fixed-timestep loop.

## Architecture

A simple, data-driven game loop architecture will be used. The engine's logic is separated from the game's data. The engine reads `config.toml` and loads assets from the `assets/` directory to run the game.

**Future Direction: ECS Redesign**
In a future phase, the project aims to redesign the engine's core architecture around the Entity-Component-System (ECS) pattern. This will enhance modularity, reusability, and performance by strictly separating data (Components), logic (Systems), and entities (IDs).

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
*   **Camera:** A camera that follows the player.
*   **Physics:** Basic physics, including gravity, jumping, and collision detection.
*   **Player Movement:** Player movement (left/right) and sprite flipping.
*   **Animation:** A state-driven, multi-frame sprite animation system.

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

### Audio System Implementation (Using Kira and Event-Driven Approach)

An audio system is needed to handle sound effects and background music. This will be managed by a central `AudioManager` using the `kira` crate and an event-driven approach.

1.  **Add Dependencies:** Add the `kira` crate to `Cargo.toml`.
2.  **Create Audio Manager:** In `audio.rs`, create an `AudioManager` struct that wraps Kira's audio context. It will manage loaded sounds and music.
3.  **Initialize Audio:** In `App::new()`, initialize Kira's audio context and create an instance of the `AudioManager`.
4.  **Load Audio Assets:** Implement `load_sound()` and `load_music()` methods in the `AudioManager` to load audio files (e.g., `.ogg`) from the `assets/sounds` directory using Kira's API.
5.  **Implement Playback Methods:** Create `play_sound(name)` and `play_music(name)` methods in the `AudioManager` that use Kira's playback capabilities.
6.  **Integrate Audio Events:**
    *   Define an `AudioEvent` enum (e.g., `AudioEvent::PlayerJumped`).
    *   Create a channel or queue for game logic to send `AudioEvent`s.
    *   Modify the main application loop in `app.rs` to process these events and instruct the `AudioManager` to play sounds.
    *   In game logic (e.g., `player/state.rs`), emit `AudioEvent`s when specific events occur (e.g., `AudioEvent::PlayerJumped` when the player jumps).

## Future Improvements

*   **Mid-Air Control:** The player's control while in the air will be adjusted to prevent instant turning and acceleration, providing a more realistic feel.

*   **Damped Camera:** The camera system will be improved to implement a "damped" or "soft" follow, giving it a sense of weight and creating a smoother, more cinematic feel.