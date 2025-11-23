# Testing Strategy

**TLDR:**
* This document outlines the testing philosophy and strategy for the `GfX-Engine` project.
* We follow the "Test Ruthlessly" principle from The Pragmatic Programmer.
* Our strategy is based on the Testing Pyramid: a foundation of **Unit Tests**, a smaller number of **Integration Tests**, and manual **Playtesting**.
* All new features must be accompanied by tests.

## Philosophy

Testing is not an afterthought; it is an integral part of our development process. Our goal is to catch bugs as early as possible and to build a robust, reliable engine. Every developer is responsible for writing tests for the code they create.

We embrace the "Shift-Left" approach, where testing happens continuously throughout the development cycle, not as a final phase.

## The Testing Pyramid

Our testing strategy is structured like a pyramid:

### 1. Unit Tests (The Foundation)

These are small, fast tests that verify a single piece of code (like a function or method) in isolation. They form the base of our testing pyramid, and we should have a large number of them.

*   **Purpose:** To test individual algorithms, logic, and boundary conditions.
*   **Example:** Testing a physics calculation, a state transition, or a configuration parsing function.

#### System Unit Tests

For our ECSC architecture, every ECS System should have a corresponding unit test. The methodology is as follows:
1.  **Setup:** Create a mock `World` and a mock `SystemContext`.
2.  **Arrange:** Populate the `World` with the specific entities and components required for the test case.
3.  **Act:** Create an instance of the system under test and call its `update()` method, passing in the mock `World` and `SystemContext`.
4.  **Assert:** Inspect the state of the `World` after the system has run and assert that the components were modified as expected.

This approach provides a perfect, isolated sandbox for verifying a system's logic, which is ideal for an AI-assisted, test-driven workflow.

### 2. Integration Tests (The Middle)

These tests verify that different modules of the engine work together correctly. They are larger than unit tests and test a complete "slice" of functionality.

*   **Purpose:** To find bugs at the boundaries between components (e.g., does the `InputSystem` correctly cause a state change in the `Player` that the `AnimationSystem` then uses to play the right animation?).
*   **Example:** A test that creates a `World`, adds entities, runs the systems for a few frames, and then asserts that the state of the `World` is correct.

#### Event Integration Tests

To verify our event-driven architecture, we create integration tests that confirm the entire chain of cause and effect for a given event.

1.  **Setup:** Create a `World` with an `EventBus`.
2.  **Arrange:** Populate the `World` with the necessary entities and publish a specific event to the `EventBus` (e.g., `PlayerStompedEnemyEvent`).
3.  **Act:** Run the relevant "conductor" systems (e.g., `AudioConductorSystem`, `ScoreSystem`).
4.  **Assert:** Check that the correct downstream effects have occurred. For example, assert that the `AudioSystem` received a `PlaySound` command or that the player's score component was incremented.

This ensures that when a new event or listener is added, it integrates correctly with the rest of the engine without causing unintended side effects.

### 3. End-to-End & Playtesting (The Peak)

This level involves testing the game as a whole, from the user's perspective.

*   **Purpose:** To ensure the game is fun, the difficulty is balanced, and to find bugs that only emerge from complex, unscripted interactions.
*   **Manual Playtesting:** This is the most important part of E2E testing for a game. We must regularly play the game to test the "feel" and perform **Exploratory Testing** (creatively trying to break the game).
*   **Soak Testing:** For finding long-term issues like memory leaks, the game should be left running for extended periods.

## Testing in Rust

We leverage Rust's excellent built-in testing framework.

*   **Unit Tests:** Are placed in a `#[cfg(test)] mod tests { ... }` block within the same file as the code they are testing. This allows them to access private functions if necessary.
*   **Integration Tests:** Are placed in the `tests/` directory at the root of the project. Each file is a separate test crate that uses the engine's public API, just like a real consumer of the engine would.
*   **Documentation Tests:** All public functions should have documentation examples (`/// # Examples`). These examples are run as tests by `cargo test`, ensuring our documentation is always correct.

## Our Gfx-Engine Testing Strategy

1.  **Doc Tests are Mandatory:** All new public functions and methods must have at least one documentation test example.
2.  **Unit Test Critical Logic:** Any complex or critical logic (especially in physics, state machines, and data parsing) must be covered by unit tests.
3.  **Integration Tests for Core Gameplay:** We will build integration tests in the `tests/` directory to verify core gameplay loops and system interactions.
4.  **Playtest Every Sprint:** As part of the "Sprint Review," we will manually playtest the new features to ensure they work as expected and "feel right."
5.  **Follow the Definition of Done:** No task is considered "Done" until its corresponding tests are written and passing, as per our `Workflow.md`.

## Test Implementation Plan

This section outlines a concrete roadmap for improving the engine's test coverage with simple, focused unit and integration tests.

### 1. Level Loading & Parsing (`level.rs`)

*   **Test: Load a valid level.**
    *   **Description:** Create a small, valid `.tmx` file and a corresponding `.tsx` tileset file in a `tests/fixtures` directory. The test will call `level::load_level()` with the path to this file.
    *   **Assert:** Check that the returned `Level` struct is `Ok`, and verify that the map dimensions, tileset info, and entity count match the contents of the test file.
    *   **Location:** `src/level.rs` (as a unit test).

*   **Test: Fail to load a missing level.**
    *   **Description:** Call `level::load_level()` with a path to a file that does not exist.
    *   **Assert:** Check that the function returns an `Err` result.
    *   **Location:** `src/level.rs` (as a unit test).

*   **Test: Fail to load a malformed level.**
    *   **Description:** Create a `.tmx` file with invalid XML or incorrect data encoding (e.g., not CSV).
    *   **Assert:** Check that `level::load_level()` returns an `Err` result.
    *   **Location:** `src/level.rs` (as a unit test).

### 2. Configuration Loading (`config.rs`)

*   **Test: Load player configuration.**
    *   **Description:** Create a simple `game_config.toml` file in a test fixture directory. The test will call `config::load_game_config()`.
    *   **Assert:** Check that the returned `GameConfig` struct contains the correct values for player properties like `start_pos`, `width`, and `height`.
    *   **Location:** `src/config.rs` (as a unit test).

### 3. Rendering Logic (`app.rs` & `renderer.rs`)

*   **Test: Z-Layer rendering order.**
    *   **Description:** This test will verify the entity sorting logic. Create a `World` and add three renderable entities with `z_index` values of `10`, `200`, and `100`. Mimic the sorting logic found in the `app.rs` main loop.
    *   **Assert:** Check that the final sorted list of entities is in the correct order: `200`, `100`, `10`.
    *   **Location:** `src/app.rs` (as a unit test).

### 4. Player & Enemy Movement (Systems)

*   **Test: Player moves right on input.**
    *   **Description:** Following the "System Unit Tests" guide in `Testing.md`, create a `World` with a player entity. Create a mock `SystemContext` where the `InputState` shows `PlayerAction::MoveRight` is active. Run the `InputSystem::update()` method.
    *   **Assert:** Check that the player entity's `Velocity` component has a positive `x` value.
    *   **Location:** `src/ecs/systems/input.rs` (as a unit test).

*   **Test: Gravity affects an entity.**
    *   **Description:** Create a `World` with an entity that has `Position`, `Velocity`, and `Gravity` components. Run the `PhysicsSystem::update()` method.
    *   **Assert:** Check that the entity's `Velocity` has increased in the positive `y` direction, and its `Position` has been updated accordingly.
    *   **Location:** `src/ecs/systems/physics.rs` (as a unit test).

### 5. Collision Detection (Systems)

*   **Test: Player is grounded after falling on a tile.**
    *   **Description:** Create a `World` and a mock `SystemContext` with a level that has a solid floor. Place a player entity with a downward `Velocity` just above the floor. Run the `TileCollisionSystem::update()` method.
    *   **Assert:** Check that the player entity now has a `Grounded` component.
    *   **Location:** `src/ecs/systems/tile_collision.rs` (as a unit test).

*   **Test: Player stomps an enemy.**
    *   **Description:** Create a `World` with a player and an enemy entity, positioned so their collision boxes overlap. Give the player a downward `Velocity`. Run the `InteractionSystem::update()` method.
    *   **Assert:** Check that the enemy entity now has a `DeadTag`, the player's `Velocity` has a small upward bounce, and a `PlayerStompedEnemyEvent` was published to the event bus.
    *   **Location:** `src/ecs/systems/interaction.rs` (as a unit test).

*   **Test: Player takes damage from an enemy.**
    *   **Description:** Create a `World` with a player and an enemy colliding horizontally (player `Velocity` is not downward). Run the `InteractionSystem::update()` method.
    *   **Assert:** Check that the player's `Health` component has decreased, they have an `Invincibility` component, and a `PlayerTookDamageEvent` was published.
    *   **Location:** `src/ecs/systems/interaction.rs` (as a unit test).

## High-Priority Stability Tests

This section lists the most critical tests for ensuring the stability of the engine and the demo game, ranked by importance. These tests form a "safety net" to prevent engine-breaking or game-breaking changes.

1.  **Test: Successful Level Loading**
    *   **Feature:** `level.rs`
    *   **Importance:** **Engine-breaking.** If a valid `.tmx` level file cannot be parsed, the game cannot start. This is the most fundamental test.
    *   **Description:** A unit test that loads a simple, correct level file and asserts that the `Level` object is created successfully and its properties (dimensions, entities) are correct.

2.  **Test: Player-Tile Vertical Collision**
    *   **Feature:** `TileCollisionSystem`
    *   **Importance:** **Game-breaking.** If players fall through the floor, the game is unplayable. This test ensures the most critical physical interaction with the world is stable.
    *   **Description:** A system unit test where a player entity with a downward velocity is placed above a solid tile. After running the `TileCollisionSystem`, assert that the player's vertical velocity is now zero and they have a `Grounded` component.

3.  **Test: Player Input Causes Movement**
    *   **Feature:** `InputSystem`
    *   **Importance:** **Game-breaking.** If player input doesn't translate into movement within the game world, the game is not interactive.
    *   **Description:** A system unit test that provides a mock `InputState` with `PlayerAction::MoveRight` active. After running the `InputSystem`, assert that the player entity's `Velocity` component has been updated correctly.

4.  **Test: Player Takes Damage on Enemy Contact**
    *   **Feature:** `InteractionSystem`
    *   **Importance:** **Game-breaking.** This is a core gameplay loop. A failure here could make the player invincible or cause unexpected crashes. It tests collision between entities, health mechanics, and event creation.
    *   **Description:** A system unit test where a player and an enemy are placed in a colliding position. Assert that after the `InteractionSystem` runs, the player's `Health` has decreased and they have an `Invincibility` component.

5.  **Test: Player Stomps and Defeats an Enemy**
    *   **Feature:** `InteractionSystem`
    *   **Importance:** **Game-breaking.** The primary combat mechanic. A failure makes it impossible to progress or interact with enemies.
    *   **Description:** A system unit test where a player (with downward velocity) is placed above an enemy. After the `InteractionSystem` runs, assert that the enemy has a `DeadTag` and a `PlayerStompedEnemyEvent` was published.

6.  **Test: Player State Machine Transition (Idle to Jump)**
    *   **Feature:** `player/states.rs`
    *   **Importance:** **Game-breaking.** The state machine governs all player abilities. If transitions fail, the player can get "stuck" and unable to act.
    *   **Description:** A unit test that sets up a player in `IdleState` on the ground. Provide an input state where the jump button is pressed. Run the state machine's update logic and assert that the new state is `JumpingState`.

7.  **Test: Successful Game Configuration Loading**
    *   **Feature:** `config.rs`
    *   **Importance:** **Engine-breaking.** So much of the game's setup (animations, prefabs, physics values) depends on `game_config.toml`. A failure to parse this file would prevent the world from being built correctly.
    *   **Description:** A unit test that loads a valid `game_config.toml` file and asserts that key values (e.g., player health, stomp velocity) are parsed correctly.

8.  **Test: Z-Layer Sorting Logic**
    *   **Feature:** Rendering Pipeline (`app.rs`)
    *   **Importance:** **Engine stability.** While not a crash, incorrect rendering order can make the game visually unplayable (e.g., player behind the background). This test ensures the data being sent to the renderer is correctly ordered.
    *   **Description:** A unit test that creates a `World` with multiple renderable entities with different `z_index` values. Mimic the sorting logic and assert the final draw order is correct.

9.  **Test: Enemy AI Wall Detection**
    *   **Feature:** `enemy/states.rs`
    *   **Importance:** **Demo-game stability.** While not a "crash", broken AI makes for a broken demo. This test ensures enemies can navigate their environment.
    *   **Description:** A unit test for the `PatrolState` where an enemy is placed moving towards a solid tile. After the state machine update, assert that the enemy's horizontal velocity has been reversed.

10. **Test: Event Bus Integration (Stomp -> Sound)**
    *   **Feature:** ECSC Event Bus (`event.rs`, `audio_conductor.rs`)
    *   **Importance:** **Engine architecture stability.** This verifies that the entire event-driven pipeline is working, ensuring that decoupled systems can communicate.
    *   **Description:** An integration test where you publish a `PlayerStompedEnemyEvent`. Run the `AudioConductorSystem` and assert that it correctly sends an `AudioEvent::PlaySound` message to the mock audio manager.
