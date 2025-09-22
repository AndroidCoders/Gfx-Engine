File version: 1.02

**TLDR:**
This document provides an overview of the `gfx-boxes` project's file structure:
* Main directories
* Key source code and documentation files

README.md: Project Description.

docs/: Folder for The Guiding Documents.
docs/Structure.md: File Structure Overview.
docs/Product.md: Product Description.
docs/Tech.md: Technology Stack.
docs/Requirements.md: Project Requirements.
docs/Tasks.md: Project Tasks.
docs/Design.md: Design & Architecture.
docs/CodingStyle.md: Coding Conventions & Development Guidelines.
docs/Workflow.md: Development Workflow with GitHub.

src/main.rs: Main application source code.
src/app.rs: Initializes SDL (graphics), creates the window, and runs the main application loop.
src/config.rs: Defines the application's configuration structures and handles loading from `config.toml`.
src/renderer.rs: Handles all drawing operations on the SDL Canvas.
src/game_state.rs: Game state module.
src/frame_capture.rs: A module for saving screenshots from the running application, configurable via `config.toml`.
