File version: 1.04

**TLDR:**
This document provides an overview of the `GfX-Engine` project's file structure:
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
src/audio.rs: Handles audio loading and playback.
src/camera.rs: Defines the camera for viewing the game world.
src/config.rs: Defines the application's configuration structures and handles loading from `config.toml`.
src/frame_capture.rs: Handles capturing frames for debugging or video output.
src/input.rs: Handles user input and translates it into abstract game actions.
src/level.rs: Manages loading and representing game levels.
src/math.rs: Defines common mathematical structures.
src/player.rs: Defines the player character's state and behavior.
src/renderer.rs: Handles all drawing operations for the engine.
src/texture_manager.rs: Manages loading and storing textures.