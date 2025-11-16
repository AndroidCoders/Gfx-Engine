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
