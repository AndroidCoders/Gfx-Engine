# Gfx-Engine

**TLDR:**
*   Gfx-Engine is a 2D pixel-art game engine built with Rust and SDL3.
*   It features a data-driven architecture and includes a complete demo game.
*   The demo, "Super Cat Bros - Episode 1 - The Pirate Gold Adventure," is a platformer inspired by classic 16-bit era games.

Gfx-Engine is a modular 2D game engine written in Rust, designed for creating pixel-art platformers.

The project includes a playable demo game that showcases the engine's capabilities, which include:
*   A robust physics engine for platforming.
*   A state machine-driven animation system.
*   A data-driven design where levels, assets, and game properties are loaded from TOML configuration files.

## The Demo Game: "Super Cat Bros - Episode 1 - The Pirate Gold Adventure"

The current demo is a vertical slice of a larger adventure.
*   **Premise:** Join the courageous Captain Cat as he explores the dangerous "Skull Islands" in search of the legendary Pirate Gold.
*   **Theme:** Tropical/Pirate. Bright blue skies, palm trees, sandy beaches, and hidden caves.
*   **Enemies:** Spiders (placeholder for Crabs?), Pirate Parrots (planned), and other island inhabitants.
*   **Goal:** Collect gold coins and reach the Treasure Chest (Goal) at the end of each level.
*   **Characters:**
    *   **Barry White:** (White Cat) - Soundtrack: Smooth Soul/Funk.
    *   **Freddy Meowcury:** (Yellow/Orange Cat) - Soundtrack: Rock Opera.
    *   **James Brown:** (Brown Cat) - Soundtrack: Funk/Soul.
    *   **Dolly Pawton:** (Blonde Cat) - Soundtrack: Country/Pop.
    *   **Whitney Meowston:** (Grey Cat) - Soundtrack: R&B/Pop.

## Architecture

The `Gfx-Engine` is built on a high-fidelity **WYSIWID (What You See Is What It Does)** architecture. This pattern ensures that the engine's behavior is explicit, legible, and highly modular, making it a perfect environment for collaborative development with **AI coding assistants**.

For a formal definition of our architectural principles and the roadmap for its implementation, see [**Architecture.md**](Architecture.md).

### The Primary Pillars

1.  **Event-Driven ECS:** The core data structure where **Entities** are IDs, **Components** are raw data, and **Systems** are the logic processors.
2.  **Concepts vs. Synchronizations:** A strict separation between autonomous services (Concepts) and behavioral rules (Synchronizations).
3.  **Atomic Modules:** A rigorous enforcement of file structure—modules are limited to **200-300 lines** and must have **Single Responsibility** for exactly one process.

In this model, the codebase is organized into three distinct layers:

1.  **Concepts (ECS Systems & Components):**
    *   These are fully independent services that own their specific domain logic and data.
    *   **Examples:** `SystemPhysics` (manages Position/Velocity), `SystemAudio` (manages Sound), `SystemInput` (manages Hardware Input).
    *   **Rule:** A Concept **never** directly calls another Concept. The Physics System does not know the Audio System exists.

2.  **Synchronizations (Event-Based Rules):**
    *   These are the "glue" rules that mediate between Concepts. They define *what happens next*.
    *   **Mechanism:** They listen for **Events (Facts)** published by one Concept and trigger **Commands/Actions** in another.
    *   **Current Implementation:** Specialized systems (e.g., `SystemAudioSynchronization`) that manually subscribe to events.
    *   **Future Implementation:** A generic **Synchronization Engine** that reads declarative rules from data files or **Rhai** scripts (e.g., `when PlayerJumped then Audio.play('jump')`).

### 4. Application State (Resource)
Instead of a monolithic manager, "State" is a first-class citizen in the ECS, stored as a Resource in the `World`. This allows any system to query or request state transitions by publishing events.

```rust
pub enum GameState {
    Menu(String), // The string identifies the screen (e.g., "main", "options")
    Playing,
    Paused,
    GameOver,
    Cinematic,
}
```

### 5. The Explicit Scheduler (SystemManager)
To satisfy **WYSIWID**, the `SystemManager` explicitly defines which systems run in which state using a `match` block. This eliminates "hidden" logic and makes the execution flow of the entire engine legible in a single file.

```rust
match &world.game_state {
    GameState::Menu(_) => {
        self.menu_system.update(world, context);
    },
    GameState::Playing => {
        self.physics_system.update(world, context);
        self.movement_system.update(world, context);
        // ... gameplay systems
    },
    // ...
}
```

## Performance Optimizations

To handle large, complex levels with thousands of entities, the engine implements advanced spatial partitioning and simulation culling.

### 1. Spatial Partitioning (Uniform Grid)
**Purpose:** Accelerates collision detection and visibility queries.
*   **Mechanism:** The world is divided into a grid of 256x256 pixel cells.
*   **Indexing:** `SystemSpatialUpdate` runs every frame, filing every active entity into its corresponding grid cells based on its bounding box.
*   **Benefit:** Systems like `SystemInteraction` only check for collisions between entities in the same or adjacent cells, reducing complexity from $O(N^2)$ to nearly $O(1)$.

### 2. Simulation Culling (Dormancy)
**Purpose:** Freezes logic for entities far away from the player to save CPU cycles.
*   **Active Zone:** A 3840x2160 pixel "Mega-Partition" (2x the size of a 1080p screen) centered on the camera.
*   **System:** `SystemDormancy` calculates the distance of all entities from the camera. Entities outside the Active Zone are tagged with a `DormantTag`.
*   **Throttling:** Core systems (Physics, AI, Animation) immediately skip entities with the `DormantTag`.
*   **Legibility:** This follows the **WYSIWID** pattern—if you can't see it, it isn't doing anything expensive.

### 3. Frustum Culling
**Purpose:** Only render what is visible.
*   **Mechanism:** Before drawing, the main loop queries the `SpatialGrid` for only those entities within the camera's `view_rect`. Off-screen tiles are also skipped in `Renderer::draw_level`.

## System Architecture

To improve cohesion and maintainability, the engine consolidates game logic into a few robust, domain-specific systems. This reduces fragmentation and simplifies the data flow.

### 1. `SystemLifecycle` (The Vitality Engine)
**Purpose:** The single authority on entity existence. It manages the entire pipeline of "Being Alive."
*   **Responsibilities:**
    *   **Injury:** Consumes `EntityTookDamageEvent`, deducts Health, and manages Invincibility frames.
    *   **Mortality:** Detects `Health <= 0` and transitions entities to `Dying` or `Dead` states.
    *   **Purgatory:** Manages death timers and transition animations (e.g., the Player's "Angel" ascent).
    *   **Resolution:** Executes the final Respawn (for players) or Despawn/Cleanup (for enemies and particles).
*   **Consolidates:** `SystemDeath`, `SystemPlayerDeath`, `SystemKill`, `SystemRespawn`, `SystemRespawnTimer`, `SystemInvincibility`, `SystemLifetime`.

### 2. `SystemMovement` (The Motor)
**Purpose:** Standardizes entity locomotion. It translates "Intent" (User Input or AI Decisions) into "Physics" (Velocity/Acceleration).
*   **Responsibilities:**
    *   Applies movement forces (Running, Jumping) based on `MovementCommand` components.
    *   Ensures physics consistency (Gravity, Friction, Air Control) across all entities (Player and Enemies).
*   **Consolidates:** `SystemPlayerControl` and specific movement logic currently inside AI States.

### 3. `SystemInteraction` (The Arbiter)
**Purpose:** The central rule-book for Entity-vs-Entity collisions. It decouples "Detection" from "Consequence."
*   **Responsibilities:**
    *   Detects overlaps (e.g., Player vs. Enemy, Player vs. Coin).
    *   Publishes strictly typed events (`CoinCollectedEvent`, `EntityTookDamageEvent`) instead of modifying entity data directly.
*   **Consolidates:** The existing `SystemInteraction` and `SystemCoinCollection`.

### 4. `SystemAnimation` (The Visualizer)
**Purpose:** A unified driver for visual representation.
*   **Responsibilities:**
    *   Reads the abstract state of an entity (Velocity, Grounded, State Machine) to select the correct Sprite Animation.
    *   Updates animation frame counters.
*   **Consolidates:** `SystemPlayerAnimation` and `SystemAnimationUpdate`.

### 5. `SystemMenu` (The Interface)
**Purpose:** Manages the application's user interface and menu states.
*   **Responsibilities:**
    *   Displays the Title Screen, Main Menu, and Sub-menus (Options, Character Select).
    *   Handles menu navigation and selection input.
    *   Manages transitions between game states (e.g., Menu -> Playing).
*   **Data-Driven Layout:** Menu screens are defined in `game_config.toml`, specifying titles, items, and actions (e.g., `Goto(Options)`, `StartGame`).
*   **Implementation:** The `SystemMenu` reads the current `MenuScreen` from `GameState::Menu(screen)`, lookups the configuration, and renders the interactive elements.

### User Interface (HUD)
The Head-Up Display (HUD) provides vital information to the player during gameplay.
*   **Design:** Minimalist and unobtrusive.
*   **Elements:**
    *   **Lives:** displayed as a row of Heart icons.
    *   **Coins:** Displayed as a Coin icon followed by the count (e.g., `x 10`).
*   **Configuration:** Layout positions and assets are defined in the `[ui]` section of `game_config.toml`.
*   **Implementation:** Rendered by `SystemGUIRender` on top of the game world.

### Foundation Systems
These systems remain distinct as they handle low-level simulation or hardware interfaces:
*   **`SystemInput`**: Maps raw hardware inputs to abstract Input State.
*   **`SystemPhysics`**: The core integrator (`Pos += Vel * dt`).
*   **`SystemTileCollision`**: Resolves collisions between entities and the static level geometry.
*   **`SystemAudioSynchronization`**: Bridges Gameplay Events to the Audio Engine.

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

## Game Loop: Fixed Timestep & Determinism

The engine utilizes a **Fixed Timestep Loop** with the **Accumulator Pattern** to decouple gameplay logic from rendering, ensuring deterministic physics across different hardware.

1.  **Physics & Logic (The "Heartbeat"):**
    *   Runs at a strict **120 Hz** (approx. 8.33ms per tick).
    *   **Accumulator:** Frame time is added to an accumulator. Logic steps are executed in fixed increments until the accumulator is depleted. This guarantees consistent results regardless of the variable frame rate.
    *   **Determinism:** This architecture enables the **Replay System** by recording inputs at each fixed tick.

2.  **Rendering (The "Skin"):**
    *   Runs as fast as the monitor allows (VSync).
    *   **Interpolation (lerp):** To prevent visual jitter, the renderer calculates an `alpha` factor (`accumulator / fixed_dt`). It draws entities at an interpolated position between their previous and current physics state: `pos = prev * (1 - alpha) + curr * alpha`.

### Deterministic Replay System
The engine features a built-in replay system enabled by its fixed timestep architecture.
*   **Mechanism:** At every 120Hz tick, the `InputState` (pressed actions) is recorded into a `Replay` struct.
*   **Determinism:** Because physics and logic advance in discrete, fixed steps, playing back the exact same sequence of inputs from the same starting seed guarantees an identical outcome.
*   **Modes:** Supports **Recording** (saving sessions to `.replay` files) and **Playback** (driving the game from a file, used for the "Attract Mode" in the main menu).

### Runtime Audio Analysis & Beat Detection
To enable rhythm-synced gameplay (e.g., enemies jumping on the beat), the engine performs runtime analysis of `.wav` soundtracks.
*   **Spectral Flux:** The engine uses Fast Fourier Transform (FFT) via the `spectrum-analyzer` crate to calculate the energy difference between frames.
*   **Auto-Tuning:** A custom algorithm iteratively adjusts the detection threshold to match a target BPM (e.g., 116 BPM).
*   **Caching:** Detected beats are serialized to a sidecar `.beats` file to ensure instant loading on subsequent runs.
*   **Event Integration:** The `SystemManager` acts as a conductor, publishing `EventMusicBeat` whenever a rhythmic onset is detected.

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

### Texture Upscaling Strategies

To achieve a retro pixel-art aesthetic on modern high-resolution displays, the engine uses **Load-Time Upscaling**.

*   **Load-Time Upscaling (In-Memory Scaling):**
    *   **Concept:** Small source assets (e.g., 32x32 sprites) are loaded from disk but immediately upscaled in memory (e.g., 4x to 128x128) by the `TextureManager` using nearest-neighbor interpolation.
    *   **Pros:** Allows for "sub-pixel" smooth movement (sprites can move 1/4 of a "retro pixel"); allows mixing with high-res UI natively; removes the need for a global `PIXEL_SCALE` multiplier in rendering code.
    *   **Coordinate System:** The engine uses a 1:1 mapping between world units and the Virtual Resolution (e.g., 1920x1080).

**Implementation:** The engine upscales all textures and configuration values (positions, speeds, sizes) by a `DATA_SCALE_FACTOR` (currently 4.0) during the loading phase. This ensures that the gameplay logic and rendering operate in a consistent, high-resolution 1:1 pixel space while preserving the pixel-art aesthetic.

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

### Unified Life & Death System

To ensure consistency and reduce code duplication, the engine uses a unified, event-driven system for managing the lifecycle of all entities (Players, Enemies, etc.).

1.  **Data:** Entities are assigned a `Health` component and, optionally, a `Lives` component (typically for players).
2.  **Input:** Interactions (e.g., getting hit, falling) do not directly modify data. Instead, they publish an `EntityTookDamageEvent`.
3.  **Processing (HealthSystem):** A central `HealthSystem` listens for damage events. It deducts health and, if `Health <= 0`, publishes an `EntityDiedEvent`.
4.  **Consequences:** Specialized systems listen for the `EntityDiedEvent` to trigger specific reactions:
    *   **Player:** The `GameFlowSystem` decrements the `Lives` component. If lives remain, a respawn is triggered; otherwise, the "Game Over" state is activated.
    *   **Enemy:** The `DeathSystem` triggers a death animation, drops loot, and removes the entity.
    *   **Audio/UI:** Synchronization systems play sound effects and update the HUD independent of the core logic.

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
# Event-Driven ECS Architecture: A Practical Guide (v4)

This guide outlines the architecture and implementation of a topic-based Event Bus, which is the cornerstone of our **Event-Driven ECS** model.

## 1. The Core Data Flow

The Event-Driven ECS architecture establishes a clear, one-way flow of information:

**Input** -> **Command** -> **System (Concept)** -> **Event** -> **Synchronization**

1.  **Input:** A raw hardware signal (e.g., a key press).
2.  **Command:** The input is translated into a semantic `Command` representing intent (e.g., `PlayerCommand::Jump`).
3.  **System (Concept):** A System processes the `Command`, updates the core world state (e.g., changes an entity's velocity), and publishes an `Event`.
4.  **Event:** An `Event` is a data struct published to the **Event Bus** with a specific **Topic** string, announcing a fact (e.g., topic: `"player.movement.jump"`, data: `PlayerJumpedEvent { ... }`).
5.  **Synchronization:** A `SynchronizationSystem` subscribes to topic patterns (e.g., `"player.movement.*"`) and executes reactive, secondary logic (e.g., playing sounds, updating UI).

## 2. The Topic-Based Event Bus

Instead of subscribing to a specific *type*, synchronizations subscribe to string-based **topic patterns**. This provides a highly flexible and efficient routing system.

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

Synchronization Systems can subscribe to patterns using two wildcards:

*   `*` (asterisk): Matches exactly one word in a topic.
    *   *Example:* `physics.collision.*` matches `physics.collision.started` but **not** `physics.collision.player.wall`.
*   `#` (hash): Matches zero or more words at the end of a topic.
    *   *Example:* `player.#` matches `player.movement.jump` and `player.state.changed`.

### Implementation Details & Examples

Here is a practical look at how the Event Bus can be implemented.

#### The `EventSynchronization` Trait

First, we define a trait that all our synchronizations will implement.

```rust
use std::any::Any;

pub trait EventSynchronization {
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
    subscribers: HashMap<String, Vec<Box<dyn EventSynchronization>>>,
}

impl EventBus {
    pub fn new() -> Self {
        EventBus { subscribers: HashMap::new() }
    }

    pub fn subscribe(&mut self, pattern: String, synchronization: Box<dyn EventSynchronization>) {
        self.subscribers.entry(pattern).or_default().push(synchronization);
    }

    pub fn publish(&mut self, topic: &str, event_data: &dyn Any) {
        // Iterate over all registered patterns
        for (pattern, synchronizations) in &mut self.subscribers {
            if topic_matches(pattern, topic) {
                for synchronization in synchronizations {
                    synchronization.on_event(topic, event_data);
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

#### Example: An `AudioSynchronizationSystem`

This synchronization plays a sound whenever any player event occurs.

```rust
// Define the event data struct
struct PlayerJumpedEvent { pub player_id: usize }

// The synchronization implementation
struct AudioSynchronizationSystem;
impl EventSynchronization for AudioSynchronizationSystem {
    fn on_event(&mut self, topic: &str, event_data: &dyn Any) {
        // Use downcast_ref to safely cast the event data to the expected type.
        if let Some(_event) = event_data.downcast_ref::<PlayerJumpedEvent>() {
            println!("AudioSynchronizationSystem heard topic '{}': Playing jump sound!", topic);
            // self.audio_manager.play("jump_sound");
        }
    }
}

// How it would be used:
// let mut bus = EventBus::new();
// bus.subscribe("player.movement.jump".to_string(), Box::new(AudioSynchronizationSystem));
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

### Hierarchical Stack-Based Profiler
The engine includes a low-overhead instrumentation system (`Benchmarker.rs`) to identify CPU bottlenecks in real-time.
*   **Mechanism:** Systems use `push("Name")` and `pop()` to track execution duration.
*   **Hotspots HUD:** A debug overlay (toggled with F1) displays a list of systems sorted by their impact on the frame budget. Values are smoothed using a 100-frame rolling average.
*   **Session Reporting:** Aggregated performance data (Min/Max/Avg FPS and system breakdown) is written to `benchmark.log` upon application exit.

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
        *   **Dynamic Soundtrack Management:** The `SystemAudioSynchronization` will be responsible for managing the music state based on high-level game flow events.
            *   **Level Start:** Automatically start the specific track defined for the level.
            *   **Respawn:** Restart the current track (or seek to a specific loop point) when the player respawns, to sync the music energy with the new attempt.
            *   **Character Selection:** Change the active soundtrack theme based on the selected character (e.g., "Funk" for one cat, "Rock" for another).
    *   **Audio Synchronization & Beat Detection:** To achieve precise synchronization between gameplay events (like enemy movements) and the music, we have implemented a runtime analysis system.
        *   **Runtime FFT Analysis:** The engine uses the `spectrum-analyzer` crate to perform Fast Fourier Transform (FFT) on the audio waveform at runtime.
        *   **Flux-Based Onset Detection:** A beat detector calculates the "Spectral Flux" (energy difference) to identify rhythmic onsets (beats). It features an **Auto-Tuning** algorithm that iteratively adjusts sensitivity to match a target BPM range (e.g., 110-130 BPM).
        *   **Caching:** Detected beats are serialized to a sidecar file (`.beats`) to ensure instant loading times on subsequent runs.
        *   **Loop Detection:** The `SystemManager` monitors the audio playback position and detects loop points (backward jumps in time), automatically resetting the beat tracker to maintain sync indefinitely.
        *   **Event Integration:** A `MusicBeatEvent` is published to the central Event Bus whenever a beat occurs, allowing any system (e.g., `EnemyRhythmSystem`) to react to the music without tight coupling.

### Gameplay Features*   **Advanced Physics and Terrain:** The physics engine will be enhanced to handle more complex terrain, such as sloped surfaces.
*   **Interactive World Elements:** The engine will support a variety of interactive blocks, such as power-up blocks and breakable blocks.
*   **Flexible Power-Up System:** A system will be designed to allow for power-ups that modify player state and grant new abilities.
*   **Companions/Sidekicks:** The engine will support companion characters with their own unique abilities.
*   **Improved Player Control:** Player movement, especially mid-air control, will be fine-tuned for a better feel.

## Coordinate System and Scaling

To ensure consistency and prevent bugs, the engine uses a strict separation between different coordinate systems.

*   **World Units:** All game logic, physics, and configuration files operate in **World Units**.
    *   **Definition:** 1 World Unit corresponds to 1 pixel on the **Virtual Resolution** canvas (e.g., 1920x1080).
    *   **Scale at Load-Time:** Assets designed at a lower "retro" resolution (e.g., 32x32) are upscaled by a **`DATA_SCALE_FACTOR`** (e.g., 4.0) upon loading. Their positions and sizes in configuration files are also upscaled by this factor.
    *   **Benefit:** This allows for sub-pixel precision relative to the retro art (e.g., moving by 1/4 of a "retro pixel").

*   **Virtual Resolution:** The game is rendered internally to a fixed-size canvas, known as the **Virtual Resolution**.
    *   **Definition:** This is the ideal resolution the game is designed for, such as `1920x1080`. It is set in `config.toml`.
    *   **Purpose:** It provides a consistent rendering target, independent of the player's actual screen resolution.

*   **Screen Pixels (Final Resolution):** This is the actual resolution of the player's monitor.
    *   **Behavior:** The engine takes the rendered `1920x1080` Virtual Resolution canvas and scales it to fit the player's screen.
    *   **Aspect Ratio:** The 16:9 aspect ratio is always preserved. If the screen's aspect ratio is different, black bars will be added (pillarboxing or letterboxing). This guarantees that the game's appearance and gameplay are identical on all displays.

## Offline Rendering & Replay System

The engine features a high-fidelity "Offline Rendering Pipeline" designed for creating professional-quality gameplay trailers and content for platforms like YouTube. This system allows the engine to render perfect, lag-free 60 FPS video regardless of the host machine's performance by decoupling the simulation and rendering from real-time constraints.

### 1. Input Recording (The "Score")
The foundation of this system is the deterministic **Replay System**. By leveraging the engine's Fixed Timestep loop (120Hz), we record the `InputState` (buttons pressed) at every physics tick into a `Replay` struct. Because the game logic is deterministic, feeding these same inputs back into the engine from the initial state guarantees the exact same gameplay outcome every time. This allows us to "simulate" the game for video rendering without needing to capture heavy video data in real-time.

### 2. Offline Video Rendering (The "Camera")
Instead of running the standard game loop which waits for VSync, we implement a special "Render Mode".
*   **Pipeline:**
    1.  **Step Physics:** The engine advances the game state by a fixed amount (e.g., 2 ticks for 60 FPS video).
    2.  **Draw:** The frame is rendered to the SDL Canvas.
    3.  **Capture:** `canvas.read_pixels` is used to copy the raw pixel buffer from the GPU to system RAM.
    4.  **Pipe:** These raw bytes are written directly to the standard input (`stdin`) of an `ffmpeg` subprocess.
*   **Benefit:** This guarantees a solid 60 FPS output with zero lag, regardless of how complex the scene is or how slow the computer is.

### 3. Audio Integration (The "Mixer")
*   **Soundtrack:** Since the soundtrack is usually a single continuous file, we pass it as an input to FFmpeg (`-i soundtrack.wav`). FFmpeg automatically mixes ("muxes") it with the generated video stream.
*   **Sound Effects (SFX):** Because the render loop is not real-time, we cannot record the system audio. Instead, we implement a **Software Mixer**. When a game event triggers a sound (e.g., "Jump"), the engine mathematically adds the sound's waveform data to a memory buffer. This raw PCM audio buffer is then piped to FFmpeg alongside the video frames, ensuring frame-perfect synchronization.

### 4. Output Format
The system targets high-quality output suitable for archiving and uploading.
*   **Codec:** **H.264** (x264) inside an **MP4** or **MKV** container.
*   **Lossless Mode:** We use `-crf 0` (Constant Rate Factor 0) to tell the encoder to discard *no* data. The file size is larger than standard video but reasonable, and the image is mathematically identical to the raw pixels.
*   **Pixel Format:** We use `-pix_fmt yuv444p` (or `rgb24`) to preserve sharp colored edges, which is critical for pixel art (standard `yuv420p` blurs color information).