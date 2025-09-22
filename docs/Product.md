File version: 1.03

**TLDR:**
This document describes the `gfx-boxes` project:
* Rust application for graphical effects using SDL3
* Displays animated white boxes on black background
* Highly configurable via `config.toml`

`gfx-boxes` is a Rust application that uses the SDL3 library to create graphical effects. It displays multiple animated white boxes on a black background in a fullscreen window. The application exits when the 'Escape' key is pressed.

The codebase is modular, with `app.rs` for the main application, `game_state.rs` for managing the boxes' positions and frame counter, and `renderer.rs` as a drawing module.

A key debug feature allows capturing and saving specific frames as PNG images to a configurable output directory upon exit. This debug feature is now controlled via the `config.toml` file, allowing runtime activation and configuration of parameters like the output directory, maximum captured frames, and capture interval.

The project follows a clear structure, coding style, and requirements, and is built with Cargo.
