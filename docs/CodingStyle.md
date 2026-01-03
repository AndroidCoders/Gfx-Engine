File version: 1.04

**TLDR:**
This document defines the coding conventions and design principles for the `GfX-Engine` project:
* Modular design
* Data-driven design
* A "comment-first" philosophy with comprehensive documentation.
* Clear, descriptive naming for all items.
* Encapsulation

# Coding Conventions

This document outlines the agreed-upon coding conventions and design principles for the `GfX-Engine` project.

## Design Principles

- **Modular Design**: The codebase should be split into logical modules with clear responsibilities (e.g., `renderer`, `player`).
- **Data-Driven Design**: Logic (functions) should be separated from the data it operates on (structs). Data is read from configuration files, not hardcoded in the source code.
- **Encapsulation**: All logic should be encapsulated within functions or methods.

## Code Style

- **Descriptive Naming**: All items, including variables, functions, structs, enums, and file names, must have clear and self-documenting names. Avoid short, cryptic names (e.g., `i`, `n`, `mgr`). Prefer longer, more descriptive names that make the code's purpose immediately obvious (e.g., `player_index`, `font_manager`).
- **Systems Naming**: Systems should use a prefix-based PascalCase naming convention `System<Name>` or `System<Domain><Name>`.
  - **Good:** `SystemPhysics`, `SystemInteraction`, `SystemAudioSynchronization`.
  - **Bad:** `PhysicsSystem`, `InteractionSystem`, `AudioSynchronizationSystem`.
- **Events Naming**: Events should use a prefix-based PascalCase naming convention `Event<Subject><Action>` or `Event<Category><Subject><Action>`.
  - **Good:** `EventEntityDamaged`, `EventWorldLevelComplete`, `EventCoinCollected`.
  - **Bad:** `EntityTookDamageEvent`, `LevelCompleteEvent`, `Event_Coin_Collected`.
- **Standard Formatting (`rustfmt`)**: All code will be formatted using the standard `rustfmt` tool to ensure a consistent style.
- **Linter Suggestions (`clippy`)**: Code should adhere to the recommendations of the `clippy` linter to follow idiomatic Rust practices.

## Documentation and Commenting

We adhere to a mandatory **3-Level Documentation Standard** to support our WYSIWID architecture. Every module must provide a clear Identity, Intent, and Logical explanation.

See [**Documentation.md**](Documentation.md) for the detailed standard and examples.

- **Level 1: Module Identity (`//!`)**: Define the file's role (Concept or Synchronization).
- **Level 2: Function Intent (`///`)**: Define the semantic purpose of every function.
- **Level 3: Logic Implementation (`//`)**: Step-by-step logic blueprints within functions.

- **Placeholder-Driven Development**: Before writing the implementation for a complex function, write out the high-level logic as a series of Level 3 comments. This "plan" serves as a placeholder, clarifies your thinking, and makes the code easier for others (and AI assistants) to review and understand.

## Error Handling

- **Postponed Implementation**: For the initial development phase, explicit error-handling logic is not required. Instead, a comment (`// TODO: Add error handling`) should be placed where error handling will be needed later.

## Configuration

- **Externalized Configuration**: All global configuration values and constants (e.g., screen dimensions, colors, physics parameters) should be defined in `config.toml` and loaded at application startup. This allows for easy modification of application behavior without recompilation.

### No Magic Numbers

To support our data-driven design goals, the use of "magic numbers" (hardcoded, unnamed numerical or string literals) in the application logic is strictly discouraged.

- **Bad:** `velocity.x = 5.0;`
- **Bad:** `if level_name == "world_1_level_2" { ... }`

Instead, these values must be defined in a configuration file (`config.toml`, `game_config.toml`, etc.) and accessed through the `Config` struct.

- **Good:** `velocity.x = context.config.physics.max_speed;`
- **Good:** `if level_name == context.game_config.levels.level_2_path { ... }`

This practice makes the code more readable, easier to maintain, and allows for rapid tuning of game parameters without recompiling the engine. It is also a critical enabler for the AI-assisted workflow, as it turns many potential code-editing tasks into simple, safe data-entry tasks for the LLM.

## Development Philosophy: The Pragmatic Programmer

We follow the principles outlined in "The Pragmatic Programmer" to guide our
development process:

- **Care About Your Craft**: Take pride in your work; don't tolerate "broken
  windows."
- **Don't Repeat Yourself (DRY)**: Every piece of knowledge should have a single, authoritative representation.
- **Orthogonality**: Design independent, self-contained components.
- **Test Ruthlessly**: Design for testability and test often.
- **Crash Early**: Fail fast and loudly when something goes wrong.
- **Know Your Tools**: Be fluent with your development environment.
- **Communicate Effectively**: Convey information clearly and concisely.
- **Good Enough Software**: Deliver "good enough" software that meets requirements, then iterate.

We also follow these principles for development:

- Build on what works, and make it better in iterations. This is a principle from ITIL.
- If you move data out from the source code and into data files (or config files), the source code does not have to be recompiled after updating data.

### Incremental and Modular Development

To ensure the project remains robust, maintainable, and easy to contribute to, we adhere to the following principles of incremental and modular development:

*   **One Feature at a Time:** Each new feature should be developed in a dedicated branch and be self-contained. This practice, often referred to as "atomic changes", ensures that we can test each feature in isolation and merge it with confidence. Before starting a new feature, the current one must be 100% complete and tested.

*   **Module Atomicity and Size Limits:**
    *   **Size Limit:** All source code modules (`.rs` files) should ideally be between **200-300 lines of code**. This ensures they are easy to comprehend at a glance.
    *   **Maximum Limit:** Any module exceeding **500 lines** is considered a "Broken Window" and must be refactored into smaller, more focused modules.
    *   **Single Responsibility:** Each module must have responsibility for **exactly one process**. Combining multiple domains (e.g., Physics and UI) in a single file is strictly prohibited.
    *   **Atomic Logic:** Systems and Rules should be "Atomic"—they should perform one discrete task well.

*   **Modularity and Decoupling:** The codebase is designed to be highly modular. When adding or modifying a feature, changes should be confined to a single module whenever possible. This reduces the risk of unintended side effects in other parts of theapplication and makes the code easier to understand, test, and refactor.

*   **Always Be Releasable:** The `master` branch should always be in a releasable state. This means that every change merged into `master` must be fully tested and functional. This practice is a cornerstone of incremental development and allows us to have a stable codebase at all times.