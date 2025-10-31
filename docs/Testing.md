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

### 2. Integration Tests (The Middle)

These tests verify that different modules of the engine work together correctly. They are larger than unit tests and test a complete "slice" of functionality.

*   **Purpose:** To find bugs at the boundaries between components (e.g., does the `InputSystem` correctly cause a state change in the `Player` that the `AnimationSystem` then uses to play the right animation?).
*   **Example:** A test that creates a `World`, adds entities, runs the systems for a few frames, and then asserts that the state of the `World` is correct.

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
