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
- **Standard Formatting (`rustfmt`)**: All code will be formatted using the standard `rustfmt` tool to ensure a consistent style.
- **Linter Suggestions (`clippy`)**: Code should adhere to the recommendations of the `clippy` linter to follow idiomatic Rust practices.

## Documentation and Commenting

We adhere to a "comment-first" philosophy. Good comments are crucial for maintainability and collaboration.

- **High-Level Documentation (`///`)**: All public items (functions, structs, enums, etc.) must have high-level doc-comments that explain their purpose and the "why" behind their existence. Follow the best practices outlined in `docs/Documentation.md`.

- **Implementation Comments (`//`)**: Use standard comments to clarify complex, non-obvious, or important parts of an implementation.

- **Placeholder-Driven Development**: Before writing the implementation for a complex function, write out the high-level logic as a series of comments. This "plan" serves as a placeholder, clarifies your thinking, and makes the code easier for others to review and understand.

  ```rust
  // Example of placeholder-driven development
  fn process_complex_data(&mut self, data: &Data) {
      // 1. Validate the incoming data structure.
      // 2. Find the corresponding entity in the world.
      // 3. If found, update its state based on the data.
      // 4. If not found, create a new entity.
      // 5. Trigger a sound effect for feedback.
  }
  ```

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
- If you move data out from the source code and into data files (or config files), the source code does not have to be recompiled after updating data.

### Incremental and Modular Development

To ensure the project remains robust, maintainable, and easy to contribute to, we adhere to the following principles of incremental and modular development:

*   **One Feature at a Time:** Each new feature should be developed in a dedicated branch and be self-contained. This practice, often referred to as "atomic changes", ensures that we can test each feature in isolation and merge it with confidence. Before starting a new feature, the current one must be 100% complete and tested.

*   **Modularity and Decoupling:** The codebase is designed to be highly modular. When adding or modifying a feature, changes should be confined to a single module whenever possible. This reduces the risk of unintended side effects in other parts of theapplication and makes the code easier to understand, test, and refactor.

*   **Always Be Releasable:** The `master` branch should always be in a releasable state. This means that every change merged into `master` must be fully tested and functional. This practice is a cornerstone of incremental development and allows us to have a stable codebase at all times.