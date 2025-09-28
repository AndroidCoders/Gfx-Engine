File version: 1.03

**TLDR:**
This document defines the coding conventions and design principles for the `GfX-Engine` project:
* Modular design
* Data-driven design
* Self-documenting code
* Encapsulation
* Clear descriptions of objects and functions in comments placed at top of definitions

# Coding Conventions

This document outlines the agreed-upon coding conventions and design principles for the `GfX-Engine` project.

## Design Principles

- **Modular Design**: The codebase should be split into logical modules with clear responsibilities (e.g., `renderer`, `game_state`).
- **Data-Driven Design**: Logic (functions) should be separated from the data it operates on (structs). Data is read from configuration files, not hardcoded in the source code.
- **Encapsulation**: All logic should be encapsulated within functions or methods.

## Code Style

- **Descriptive Naming**: Functions, variables, and other items should have clear, descriptive names that make the code self-documenting.
- **Standard Formatting (`rustfmt`)**: All code will be formatted using the standard `rustfmt` tool to ensure a consistent style.
- **Linter Suggestions (`clippy`)**: Code should adhere to the recommendations of the `clippy` linter to follow idiomatic Rust practices.

## Documentation

- **High-Level Comments**: Public functions should have a high-level description of their purpose using documentation comments (`///`).

## Error Handling

- **Postponed Implementation**: For the initial development phase, explicit error-handling logic is not required. Instead, a comment (`// TODO: Add error handling`) should be placed where error handling will be needed later.

## Configuration

- **Externalized Configuration**: All global configuration values and constants (e.g., screen dimensions, colors, physics parameters) should be defined in `config.toml` and loaded at application startup. This allows for easy modification of application behavior without recompilation.

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
- When writing new code, get it working with 80 % accuracy (or better). Then improve the code in each iteration until it is working 96 % (or better).
- If you move data out from the source code and into data files (or config files), the source code does not have to be recompiled after updating data.
- Use a versioning system (v.1.00), increase the version number when making a change (v1.01), and test the code changes.

### Incremental and Modular Development

To ensure the project remains robust, maintainable, and easy to contribute to, we adhere to the following principles of incremental and modular development:

*   **One Feature at a Time:** Each new feature should be developed in a dedicated branch and be self-contained. This practice, often referred to as "atomic changes", ensures that we can test each feature in isolation and merge it with confidence. Before starting a new feature, the current one must be 100% complete and tested.

*   **Modularity and Decoupling:** The codebase is designed to be highly modular. When adding or modifying a feature, changes should be confined to a single module whenever possible. This reduces the risk of unintended side effects in other parts of theapplication and makes the code easier to understand, test, and refactor.

*   **Always Be Releasable:** The `main` branch should always be in a releasable state. This means that every change merged into `main` must be fully tested and functional. This practice is a cornerstone of incremental development and allows us to have a stable codebase at all times.