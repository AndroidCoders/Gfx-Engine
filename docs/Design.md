File version: 2.03

**TLDR:**
This document outlines the architecture and components of the `GfX-Engine`:
*   Simple, data-driven game loop architecture, with a future plan for ECS redesign.
*   Key components are modular files in `src/` (e.g., `renderer`, `audio`, `physics`).
*   Configurable VSync and fixed-timestep loop.

## Architecture

The engine's core architecture is built around the **Entity-Component-System (ECS)** pattern. This enhances modularity, reusability, and performance by strictly separating data (Components), logic (Systems), and entities (IDs). The engine follows a data-driven design, where configuration is loaded from external `.toml` files and assets are loaded from the `assets/` directory.

The long-term vision is to evolve this into a full **ECSC (Entity-Component-System-Concept)** architecture, where systems are fully decoupled and communicate via a central, type-based **Event Bus**. This will make the engine even more modular and easier to maintain.

**Architectural Priority:** The implementation of a **Type-Based Event Bus** is the highest priority architectural improvement. It is the foundational step toward the ECSC model and will provide immediate benefits by decoupling systems, improving code clarity, and simplifying the implementation of new features. The detailed plan for this is outlined in the "ECSC Implementation Plan" section below.

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

To ensure frame-rate independent movement and logic, the engine uses a **variable timestep loop**.

-   **Delta Time:** On each iteration of the main loop, the time elapsed since the last frame (`delta_time`) is calculated.
-   **System Updates:** This `delta_time` value is passed to all relevant systems (e.g., `PhysicsSystem`, `InvincibilitySystem`), allowing them to scale their updates accordingly. For example, movement is calculated as `velocity * delta_time`.
-   **VSync:** VSync can be enabled in `config.toml` to synchronize rendering with the display's refresh rate, which helps to prevent screen tearing.

## Rendering Pipeline

The rendering pipeline in `Gfx-Engine` is designed for modularity and clear separation of concerns.

*   **Centralized `Renderer`:** The `renderer.rs` module, which owns the `WindowCanvas`, encapsulates all drawing logic. It is responsible for all rendering operations.
*   **Simplified `App` Loop:** The main application loop in `app.rs` orchestrates the game loop phases (input, update, render) and calls the `Renderer` each frame. It does not contain rendering-specific logic.
*   **Resource Management:** The `TextureCreator` is created during initialization and used by the `TextureManager` to load all game textures. This ensures that texture management is handled separately from the main application logic.

### Z-Layer Rendering

To control the draw order of sprites and enable effects like parallax scrolling, the engine uses a z-layer system.

*   **`z_index` Component Field:** The `Renderable` component contains a `z_index` field, which is a `u8` integer.
*   **Convention:** The `z_index` ranges from 1 to 255. A higher value means the object is "further back" and will be rendered *first*. A lower value means the object is "closer" and will be rendered *last* (appearing on top).
*   **Sorting:** Before drawing, all renderable entities are collected into a list and sorted by their `z_index` in ascending order.
*   **Usage:**
    *   **Backgrounds:** High `z_index` values (e.g., 200-255).
    *   **Gameplay Layer:** Mid-range `z_index` values. The Player and Enemies are at `100`. Effects like explosions are slightly behind at `101`.
    *   **Foregrounds/UI:** Low `z_index` values (e.g., 1-50).

## Gameplay Mechanics

To create a robust and engaging platformer experience, the following gameplay mechanics, inspired by classics like "Super Mario World," are the design goals for the engine. See the "Implemented Core Features" section for current implementation status.

*   **Variable Jump Height:** The player will be able to control the height of their jump by holding down the jump button. This allows for more nuanced platforming challenges.

*   **Momentum-Based Movement:** The player character will have acceleration and deceleration, giving the movement a sense of weight and making the controls feel smoother and more natural.

*   **Stomping on Enemies:** As a primary form of interaction, the player will be able to defeat enemies by jumping on top of them.

*   **Interactive Blocks:** The game levels will include simple interactive elements, such as breakable blocks that the player can hit from below.

### Health and Power-ups

To create a more engaging and challenging experience, the game will feature a health system and collectible items that affect the player's state.

*   **Player Health:** The player will have a health meter (e.g., hearts). Collisions with enemies will decrease the player's health. When health reaches zero, the player loses a life.
*   **Health Pickups:** Players can replenish their health by collecting "Medical Kits" or "Potions of Health" found in the levels. These could be dropped by defeated enemies or found in treasure chests.

#### Power-Ups, Weapons, and Magical Items Ideas

##### Consumables (Instant Use)

*   **Potion of Health:** A red potion that instantly restores one heart of the player's health.
*   **Extra Life:** A special item (perhaps a "1-Up" icon) that grants the player an extra life.
*   **Key:** A key that can be used to unlock a single treasure chest or door.

##### Temporary Power-Ups

*   **Star of Invincibility:** A classic glowing star that grants the player temporary invincibility. The player can defeat enemies by simply touching them, and the game's music could speed up to create a sense of urgency and power.
*   **Boots of Speed:** Special boots that temporarily increase the player's maximum speed and acceleration, leaving a trail of dust behind them.
*   **Feather of Flight:** A magical feather that allows the player to perform a double jump or to glide for a short period by holding down the jump button.
*   **Shield of Protection:** A shimmering shield that orbits the player and protects them from a single instance of damage, shattering on impact.

##### Weapons & Combat Items

*   **Fire Flower:** A flower that allows the player to shoot bouncing fireballs to defeat enemies from a distance.
*   **Ice Flower:** A variation of the Fire Flower that shoots ice balls, which can temporarily freeze enemies in place, turning them into platforms.
*   **Sword of Slashing:** A short-range sword that allows the player to perform a quick slash attack. This would introduce a new combat mechanic beyond stomping.
*   **Hammer of Smashing:** A heavy hammer that can be used to break special blocks or deal extra damage to armored enemies.
*   **Ninja Star:** A throwable projectile that can be used to defeat enemies from a distance.

**Design Constraint for Weapons:** The player can only hold and use one special weapon at a time. Picking up a new weapon will replace the currently equipped one.

##### Permanent Upgrades (Metroidvania-style)

*   **Climbing Gloves:** Gloves that allow the player to cling to and climb up walls, opening up new vertical paths in the levels.
*   **Dash Boots:** Boots that grant the player a permanent dash ability, allowing them to cross large gaps or break through certain barriers.
*   **Goggles of Seeing:** Special goggles that, when activated, reveal hidden platforms, secret passages, or invisible treasures in the level.

### Camera Design

The camera system is designed to be both smooth and responsive, keeping the player in focus without feeling jarring or restrictive. It is inspired by the camera in classic platformers like "Super Mario World" and is built on the following principles:

*   **Slow Zone (formerly Dead Zone):** An invisible area in the center of the screen where the player can move freely without the camera moving. This prevents distracting jitter from small movements.

*   **Fast Zone (formerly Panic Zone):** A zone near the edge of the screen. When the player enters this zone, the camera's speed is dramatically increased to prevent the player from going off-screen.

*   **Look-Ahead (Directional Bias):** The Slow Zone is not always centered. It shifts based on the direction the player is facing, showing more of the screen in front of the player. This gives the player a better view of what's ahead.

*   **Platform Snap:** The camera's vertical movement is tied to the platform the player is standing on. The camera does not move vertically during a jump, only "snapping" to the new vertical position when the player lands on a different platform. This prevents a nauseating up-and-down motion during normal gameplay.

## Entity State Management

To manage the behavior of all dynamic entities (including the player and AI enemies), we will implement a **Hierarchical State Machine (HSM)**. This unified pattern provides a robust and scalable foundation for both player control and AI logic. It organizes behavior into distinct states (e.g., `Idle`, `Patrolling`, `Jumping`) and manages the transitions between them. This prevents bugs by ensuring an entity is only in one state at a time, simplifies adding new abilities, and serves as the core driver for the animation system by linking each state directly to its corresponding animation. Our implementation will be a hybrid, supporting both continuous actions within states and instantaneous actions on transitions.

### Generic AI with a Sensor-Based Approach

To make enemy AI more modular and reusable, the specific detection logic (e.g., for walls or ledges) should be extracted from individual states like `PatrolState`. Instead, we can create generic **sensor components** that can be attached to any entity.

-   **Sensor Components:** These would be simple, reusable components like `WallSensor` or `LedgeSensor`. Each sensor's corresponding system would be responsible for a single detection task (e.g., checking for a wall immediately in front of the entity) and adding a corresponding tag component (e.g., `WallDetected`) to the entity if the condition is met.
-   **State Machine Logic:** The entity's state machine (e.g., `PatrolState`) would then become much simpler. Instead of performing the detection logic itself, it would just check for the presence of the `WallDetected` or `LedgeDetected` components to decide whether to change its behavior (e.g., reverse direction).
-   **Data-Driven Behavior:** This approach allows for the creation of diverse and complex AI behaviors in a data-driven way. By mixing and matching different sensor components in an enemy's prefab in `game_config.toml`, we can create new types of enemies without writing new state machine code.

## Data-Driven World Design

A core principle of the engine is to empower designers to build and modify the game world without altering the core engine code. This is achieved by defining world elements—collision, entity properties, and even entity behavior—directly in the data files, primarily the Tiled map editor (`.tmx` and `.tsx` files) and configuration files (`.toml`).

### Data-Driven Collision

To move away from hardcoded collision logic, the engine will read collision information directly from the Tiled tileset file (`.tsx`).

1.  **Defining Collision in Tiled:** In the Tiled editor, tiles in a tileset can be assigned **Custom Properties**. We will establish a convention where a custom boolean property named `solid` is used to mark a tile as a physical barrier.

2.  **Engine Implementation:** The `level.rs` module will be responsible for parsing the `.tsx` file, identifying which tile IDs have the `solid` property set to `true`, and building a dynamic collision map for the level. This eliminates the need for hardcoded lists of tile IDs in the source code.

### Scripting and Extensibility

For behaviors that are too complex for simple properties (like one-way platforms or special item effects), the engine will support external scripting. This is the ultimate expression of data-driven design, allowing for dynamic logic without recompiling the engine.

This feature is tracked in the product backlog under **Phase 6: "Implement Scripting Engine"**.

#### The Prefab and Scripting Model

The data-driven model extends from simple properties to full behavioral scripts. An entity's prefab or a tile's properties in Tiled can include a `script` property that points to a script file.

**Example: Scripted Tile in Tiled**

A one-way platform tile could have the following custom properties set in the Tiled editor:
*   `solid`: `false` (so it's not treated as a standard wall)
*   `on_collide_script`: `"scripts/one_way_platform.rhai"` (a string pointing to the logic file)

**Example: Prefab with a Script**

The `EnemySpider` prefab in `assets/game_config.toml` could be simplified by offloading its AI to a script:

```toml
[prefabs.EnemySpider]
# This new property points to a script file that defines the enemy's logic.
script = "scripts/enemy/patrol.rhai" 

components = [
    { type = "Position" },
    { type = "Velocity", x = 1.0, y = 0.0 },
    { type = "Renderable", draw_width = 16, draw_height = 16, z_index = 100 },
    { type = "Collision", width = 16, height = 16 },
    { type = "Gravity" },
    { type = "EnemyTag" }
]
```

#### Example: Behavior Script

The `patrol.rhai` script would contain the logic that is currently implemented in Rust state machine files (like `src/enemy/states.rs`). The scripting language **Rhai** is a good candidate because its syntax is very similar to Rust.

```rust
// In assets/scripts/enemy/patrol.rhai

// The engine would call this function every frame for this entity.
fn on_update(entity) {
    // Get the entity's velocity component
    let vel = entity.get_velocity();

    // If the enemy has stopped, make it start moving again.
    if vel.x == 0.0 {
        vel.x = 1.0;
        entity.set_velocity(vel); // Save the change
    }

    // Check for walls or ledges and reverse direction
    if should_reverse_direction(entity) {
        vel.x = -vel.x;
        entity.set_velocity(vel);
    }
}
```

#### Implementation Overview

1.  **Integrate a Scripting Library:** Add a crate like `rhai` to the `Cargo.toml` dependencies.
2.  **Create a `ScriptingManager`:** This manager will be responsible for loading, compiling, and running the scripts.
3.  **Create "Bindings":** Expose a safe subset of the engine's functionality to the scripting environment. This includes functions for getting and setting component data (e.g., `entity.get_velocity()`, `entity.set_velocity()`) and interacting with the world (e.g., `world.spawn_entity("explosion")`).

#### Benefits

*   **Rapid Prototyping:** Change complex AI, item, or quest logic in seconds without waiting for the engine to recompile.
*   **Behavioral Variety:** Create many unique enemies from a single base type just by assigning them different scripts.
*   **Clear Separation of Concerns:** The high-performance "core" of the engine (rendering, physics) remains in fast, compiled Rust, while the more dynamic, high-level gameplay logic lives in flexible script files.

---
# Architecture for AI Collaboration

To facilitate effective and safe collaboration with Large Language Model (LLM) assistants, the engine's architecture is intentionally designed to be as "atomic" and modular as possible. The goal is to enable a workflow where an assistant can make small, isolated, and verifiable changes, rather than large, risky ones. This is achieved through three core principles:

### 1. Hyper-Specific Systems

Instead of large, monolithic systems that handle many responsibilities, we favor breaking logic down into many small systems, each with a single, clearly defined purpose.

*   **Principle:** A system should do one thing and do it well. For example, instead of a single `PhysicsSystem`, we might have a `GravitySystem`, a `MovementSystem`, and a `CollisionSystem`.
*   **LLM Benefit:** This provides a very small and focused "blast radius" for any change. An assistant tasked with "adjusting gravity" only needs to understand and modify the tiny `GravitySystem`, which is much safer than it attempting to parse a multi-hundred-line `PhysicsSystem`.

### 2. Event-Driven Communication

The **Type-Based Event Bus** is the primary mechanism for communication between systems. Systems should not call each other directly. Instead, a system should perform its core logic and then publish an event to announce what has happened.

*   **Principle:** Systems are decoupled. A system that causes an action (e.g., `InteractionSystem` detecting a stomp) does not need to know about the systems that react to it (e.g., `AudioConductorSystem`, `ScoreSystem`).
*   **LLM Benefit:** This allows for purely additive and non-destructive changes. To add a new feature, an assistant can be instructed to "create a new system that listens for `PlayerStompedEnemyEvent` and creates a particle effect." The assistant doesn't need to find and modify any existing code; it simply adds a new, isolated file, which is a very low-risk operation.

### 3. Data-Driven Entities (Prefabs)

As much as possible, the *definition* of game objects should live in data files (`.toml`), not in Rust code. The code should describe behaviors, while the data should describe the objects that have those behaviors.

*   **Principle:** Create a robust "prefab" system where an entity's components are defined in a configuration file. This includes its physics properties, renderable assets, and even what AI scripts it should use.
*   **LLM Benefit:** This transforms many coding tasks into simple data entry tasks. "Create a new fast enemy" becomes "copy the `Goomba` prefab, rename it to `SpeedyGoomba`, and change its `max_speed` value." This is a trivial and extremely safe modification for an LLM to perform, as it requires no logical reasoning about Rust code.

By adhering to these principles, we create a codebase that is not only clean and maintainable for human developers but is also perfectly structured for the small, iterative, and test-verified workflow used by AI coding assistants.

---
# ECSC Architecture: A Practical Guide (v4)

This guide outlines the architecture and implementation of a topic-based Event Bus, which is the cornerstone of our **ECSC (Entity-Component-System-Concept)** model.

## 1. The Core Data Flow

The ECSC architecture establishes a clear, one-way flow of information:

**Input** -> **Command** -> **System (Concept)** -> **Event** -> **EventConductor**

1.  **Input:** A raw hardware signal (e.g., a key press).
2.  **Command:** The input is translated into a semantic `Command` representing intent (e.g., `PlayerCommand::Jump`).
3.  **System (Concept):** A System processes the `Command`, updates the core world state (e.g., changes an entity's velocity), and publishes an `Event`.
4.  **Event:** An `Event` is a data struct published to the **Event Bus** with a specific **Topic** string, announcing a fact (e.g., topic: `"player.movement.jump"`, data: `PlayerJumpedEvent { ... }`).
5.  **EventConductor:** An `EventConductor` subscribes to topic patterns (e.g., `"player.movement.*"`) and executes reactive, secondary logic (e.g., playing sounds, updating UI).

## 2. The Topic-Based Event Bus

Instead of subscribing to a specific *type*, conductors subscribe to string-based **topic patterns**. This provides a highly flexible and efficient routing system.

### Topic Naming Convention

To maintain order, we will adopt a consistent naming convention for topics:

**`domain.subject.action`**

*   **`domain`**: The broad area of the game (e.g., `player`, `enemy`, `physics`, `ui`).
*   **`subject`**: The specific thing being acted upon (e.g., `movement`, `state`, `health`).
*   **`action`**: The specific action that occurred (e.g., `jump`, `death`, `changed`).

**Examples:**
*   `player.movement.jump`
*   `player.state.changed`
*   `physics.collision.started`
*   `enemy.health.decreased`

### Wildcard Subscriptions

EventConductors can subscribe to patterns using two wildcards:

*   `*` (asterisk): Matches exactly one word in a topic.
    *   *Example:* `physics.collision.*` matches `physics.collision.started` but **not** `physics.collision.player.wall`.
*   `#` (hash): Matches zero or more words at the end of a topic.
    *   *Example:* `player.#` matches `player.movement.jump` and `player.state.changed`.

### Implementation Details & Examples

Here is a practical look at how the Event Bus can be implemented.

#### The `EventConductor` Trait

First, we define a trait that all our conductors will implement.

```rust
use std::any::Any;

pub trait EventConductor {
    /// Called by the EventBus when a subscribed topic is matched.
    fn on_event(&mut self, topic: &str, event_data: &dyn Any);
}
```

#### The `EventBus` Struct

The bus itself manages a list of subscribers for each pattern.

```rust
use std::collections::HashMap;

pub struct EventBus {
    // The key is the subscription pattern, e.g., "player.*"
    subscribers: HashMap<String, Vec<Box<dyn EventConductor>>>,
}

impl EventBus {
    pub fn new() -> Self {
        EventBus { subscribers: HashMap::new() }
    }

    pub fn subscribe(&mut self, pattern: String, conductor: Box<dyn EventConductor>) {
        self.subscribers.entry(pattern).or_default().push(conductor);
    }

    pub fn publish(&mut self, topic: &str, event_data: &dyn Any) {
        // Iterate over all registered patterns
        for (pattern, conductors) in &mut self.subscribers {
            if topic_matches(pattern, topic) {
                for conductor in conductors {
                    conductor.on_event(topic, event_data);
                }
            }
        }
    }
}

/// Helper function to match a topic against a pattern.
fn topic_matches(pattern: &str, topic: &str) -> bool {
    let mut pattern_parts = pattern.split('.');
    let mut topic_parts = topic.split('.');

    loop {
        match (pattern_parts.next(), topic_parts.next()) {
            (Some("#"), _) => return true, // # matches the rest
            (Some("*"), None) => return false, // * needs a word to match
            (Some(p), Some(t)) if p == "*" || p == t => continue,
            (None, None) => return true, // Both ended, it's a match
            _ => return false, // Any other case is a mismatch
        }
    }
}
```

#### Example: An `AudioConductor`

This conductor plays a sound whenever any player event occurs.

```rust
// Define the event data struct
struct PlayerJumpedEvent { pub player_id: usize }

// The conductor implementation
struct AudioConductor;
impl EventConductor for AudioConductor {
    fn on_event(&mut self, topic: &str, event_data: &dyn Any) {
        // Use downcast_ref to safely cast the event data to the expected type.
        if let Some(_event) = event_data.downcast_ref::<PlayerJumpedEvent>() {
            println!("AudioConductor heard topic '{}': Playing jump sound!", topic);
            // self.audio_manager.play("jump_sound");
        }
    }
}

// How it would be used:
// let mut bus = EventBus::new();
// bus.subscribe("player.movement.jump".to_string(), Box::new(AudioConductor));
// bus.publish("player.movement.jump", &PlayerJumpedEvent { player_id: 0 });
```

This structure provides a robust, flexible, and clearly defined system for communication between the different parts of our engine.

## Implemented Core Features

The engine currently has the following core features implemented:

*   **Compiling Proof of Concept:** Displays the "Super Cat Bros" sprite.
*   **Configuration:** Loads configuration from `.toml` files.
*   **Input System:** A data-driven input system is in place.
*   **Texture Management:** A `TextureManager` for loading and managing textures.
*   **Renderer:** Can draw sprites and level geometry.
*   **Level Loading:** A basic level loading system from `.tmx` files.
*   **Advanced Camera:** A damped camera that smoothly follows the player, featuring look-ahead, platform snapping, and slow/fast zones.
*   **Physics:** Basic physics, including gravity, jumping, and tile-based collision detection.
*   **Player Movement:** Player movement (left/right) and sprite flipping.
*   **Animation:** A state-driven, multi-frame sprite animation system.
*   **Audio:** An event-driven audio system using the `kira` crate.
*   **Z-Layer Rendering:** A `z_index` component allows for controlling the draw order of entities.

## Debugging and Profiling

To facilitate robust testing and optimization, the following systems are planned:
*   **In-Game Benchmarking:** A real-time, in-game profiler (`Benchmarker.rs`) will be created to monitor and display key performance metrics such as frame time, FPS, and time spent in the update and render loops.
*   **On-Screen Display and Logging:** A comprehensive debug system (`debug.rs`) will be implemented. It will render key data (e.g., player coordinates, state) on-screen for video analysis and simultaneously record verbose, high-resolution data to a log file. A shared timestamp or frame number will link the video frames to the log entries.
*   **Programmatic Video Capture:** To enable automated analysis and capture hard-to-reproduce bugs, the engine will integrate programmatic video recording. This will be achieved by using a Rust wrapper for the `ffmpeg` library (such as `ffmpeg-next`), allowing the engine to start and stop video capture from within the code.

## Architectural Roadmap

The following sections outline the high-level direction for future engine and gameplay features. The detailed tasks and priorities for these items are managed in the **Product Backlog** (`docs/Tasks.md`).

### Core Engine Enhancements
*   **Refactor `app.rs` into Managers:** To improve modularity and clarify responsibilities, the `app.rs` file will be refactored. Its logic will be split into a `SystemManager` (for orchestrating ECS system updates) and a `GameStateManager` (for handling level transitions, entity creation, and overall game state like menus or game over screens).
*   **Menu System:** A generic menu system will be required to manage application states (e.g., Main Menu, Options, In-Game).
*   **Robust Text Rendering:** The temporary debug text renderer will be replaced with a full-featured system using a library like `sdl3_ttf`. This is a prerequisite for the menu system and enhanced UI.
*   **Enhanced Debugging Tools:** To facilitate robust testing and optimization, we plan to add an in-game profiler and more detailed on-screen debug displays.
*   **Spatial Partitioning:** To support large levels efficiently, a uniform grid or similar spatial partitioning system is planned.
*   **1:1 Pixel Coordinate System:** The engine will be refactored to use a 1:1 pixel-based coordinate system, removing the `PIXEL_SCALE` constant to simplify logic.
*   **Parallax Scrolling:** The renderer will be extended to support multi-layered parallax backgrounds for a greater sense of depth. This can be achieved by assigning different `z_index` values to background layers and applying a scroll factor based on that `z_index` when calculating the draw position relative to the camera. Layers with a higher `z_index` (further away) will scroll slower than layers with a lower `z_index`.
*   **Interactive Audio:** The audio system will be enhanced to support dynamic soundtracks and sound effects that respond to gameplay events. The plan includes a zone-based music system that will trigger crossfades between tracks as the player moves between different areas of the world map.

### Gameplay Features
*   **Advanced Physics and Terrain:** The physics engine will be enhanced to handle more complex terrain, such as sloped surfaces.
*   **Interactive World Elements:** The engine will support a variety of interactive blocks, such as power-up blocks and breakable blocks.
*   **Flexible Power-Up System:** A system will be designed to allow for power-ups that modify player state and grant new abilities.
*   **Companions/Sidekicks:** The engine will support companion characters with their own unique abilities.
*   **Improved Player Control:** Player movement, especially mid-air control, will be fine-tuned for a better feel.

## Coordinate System and Scaling

To ensure consistency and prevent bugs, the engine uses a strict separation between different coordinate systems.

*   **World Units:** All game logic, physics, and configuration files operate in **World Units**.
    *   **Definition:** 1 World Unit corresponds to 1 pixel of the source art assets (e.g., a sprite that is 32x32 pixels in its PNG file has a size of 32x32 World Units).
    *   **Usage:** Used for entity positions, sizes, physics calculations, and all values specified in configuration files like `game_config.toml`.

*   **Virtual Resolution:** The game is rendered internally to a fixed-size canvas, known as the **Virtual Resolution**.
    *   **Definition:** This is the ideal resolution the game is designed for, such as `1920x1080`. It is set in `config.toml`.
    *   **Purpose:** It provides a consistent rendering target, independent of the player's actual screen resolution.

*   **`PIXEL_SCALE`:** This is a global scaling factor used by the renderer.
    *   **Definition:** A constant value (e.g., `4.0`) that determines how many screen pixels are used to draw a single World Unit onto the Virtual Resolution canvas.
    *   **Example:** With a `PIXEL_SCALE` of `4.0`, a 32x32 World Unit sprite is rendered as a 128x128 pixel image on the internal virtual canvas.

*   **Screen Pixels (Final Resolution):** This is the actual resolution of the player's monitor.
    *   **Behavior:** The engine takes the rendered `1920x1080` Virtual Resolution canvas and scales it to fit the player's screen.
    *   **Aspect Ratio:** The 16:9 aspect ratio is always preserved. If the screen's aspect ratio is different, black bars will be added (pillarboxing or letterboxing). This guarantees that the game's appearance and gameplay are identical on all displays.

---
# ECSC Implementation Plan: Type-Based Event Bus

This section outlines a concrete plan for implementing a type-based event bus as a pragmatic first step towards the full ECSC architecture.

### Summary: Improving ECS with a Type-Based Event Bus

**Goal:** To better align with the ECSC (Entity-Component-System-Concept) architecture by implementing a type-based event bus. This will decouple systems from each other, making the code more modular, easier to maintain, and more performant.

**What It Is:** Instead of systems calling each other directly or using low-level channels, they will communicate through high-level, strongly-typed events.
1.  A **System** (e.g., `CoinCollectionSystem`) will detect a game event and *publish* a corresponding event struct (e.g., `CoinCollectedEvent`) to a central `EventBus` resource in the `World`.
2.  A **Conductor** (which is just another system, e.g., `AudioConductorSystem`) will *read* those events from the `EventBus` each frame and perform reactive, secondary logic (like playing a sound or updating the score).
3.  At the end of the frame, all events are cleared from the bus.

**Why It's an Improvement:**
*   **Decoupling:** The `CoinCollectionSystem` no longer needs to know about the audio system or the UI. It simply announces that a coin was collected by publishing a `CoinCollectedEvent`. Any number of other systems can then react to that event without the original system's knowledge. For example, an `AudioConductor` can listen for the event to play a sound, and a `ScoreSystem` can listen for it to update the player's score.
*   **Clarity:** This creates a clear, one-way data flow (`System` -> `Event` -> `Conductor`) that is easy to follow and debug.
*   **Performance:** It's faster than the topic-based bus described in `docs/Design.md` because it uses Rust's type system for event routing, avoiding runtime string matching and dynamic type casting.
*   **Pragmatism:** It's a significant and practical step towards the ideal ECSC architecture that can be implemented incrementally.

---

### Action Plan for Implementation

Here is a step-by-step guide for how we can implement this feature in a future session.

**Step 1: Create the Core Event Bus Module**
1.  Create a new file: `src/ecs/event.rs`.
2.  Inside this file, define a generic `EventBus` struct that can store and retrieve events based on their type. A good approach is to use a `HashMap` that maps a `TypeId` to a `Vec<Box<dyn Any>>`.
3.  Implement two key methods on the `EventBus`:
    *   `publish<T: 'static>(&mut self, event: T)`: Adds an event to the correct queue.
    *   `read<T: 'static>(&self) -> impl Iterator<Item = &T>`: Allows a system to read all events of a specific type for the current frame.

**Step 2: Integrate the Event Bus into the `World`**
1.  In `src/ecs/world.rs`, add the `EventBus` as a public field to the `World` struct: `pub event_bus: EventBus`.
2.  Initialize it in `World::new()`.
3.  Add a `clear_events()` method to `World` that clears all event queues in the `event_bus`.

**Step 3: Define and Publish the First Event**
1.  In `src/ecs/event.rs`, define our first event struct: `pub struct CoinCollectedEvent;`.
2.  In `src/ecs/systems/coin_collection.rs`, modify the `CoinCollectionSystem`. Instead of sending a message to the `audio_sender`, it will now publish the new event: `world.event_bus.publish(CoinCollectedEvent);`.

**Step 4: Create a "Conductor" System to Consume the Event**
1.  Create a new system file: `src/ecs/systems/audio_conductor.rs`.
2.  Define a new `AudioConductorSystem` struct.
3.  In its `update` method, it will read events from the bus:
    ```rust
    for _event in world.event_bus.read::<CoinCollectedEvent>() {
        // Send the sound command to the audio manager via the context
        let _ = context.audio_sender.send(AudioEvent::PlaySound("coin_pickup".to_string()));
    }
    ```

**Step 5: Update the Main Application Loop**
1.  In `app.rs`, remove the direct call to the `AudioSystem`.
2.  Add the new `AudioConductorSystem` to the list of systems that are run each frame.
3.  At the end of the main update phase (after all systems have run), call `self.world.clear_events()` to prepare the bus for the next frame.

### Future Refactoring Candidates
Beyond the initial implementation, the following systems are prime candidates for being refactored to use the event bus, further decoupling the engine architecture:

*   **Player Death:** A `PlayerDeathSystem` can publish a `PlayerDiedEvent`. This allows multiple, unrelated systems (audio, game state, UI, effects) to react to the player's death without being tightly coupled.
*   **Level Transition:** A `LevelTransitionSystem` can publish a `LevelCompleteEvent`. The core `App` or a `GameFlowSystem` can then listen for this event to handle the complex logic of loading the next level.
*   **Enemy Death:** Similar to player death, an `InteractionSystem` can publish an `EnemyDefeatedEvent`, allowing `ScoreSystem`, `AudioConductorSystem`, and `EffectsSystem` to react independently.
*   **Player Movement Input:** The `InputSystem` can translate raw key presses into `MoveCommand` events. This decouples the input hardware from the player physics and allows other systems (like AI or networking) to control characters by publishing the same events.
*   **Player Animation State:** The `PlayerAnimationSystem` can become a pure listener that reacts to state-change events like `PlayerLandedEvent` or `PlayerJumpedEvent`, rather than polling the player's components every frame. This simplifies the animation logic significantly.
