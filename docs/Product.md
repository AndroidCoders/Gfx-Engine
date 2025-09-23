File version: 1.04

**TLDR:**
This document describes the `GfX-Engine` project:
* Rust application for a minimal graphical POC using SDL3
* Displays a white box on a black background
* Configurable via `config.toml`

`GfX-Engine` is a Rust application that uses the SDL3 library to create a minimal graphical Proof of Concept. It displays a single white box on a black background in a window. The application exits when the 'Escape' key is pressed.

The codebase is modular, with `app.rs` for the main application, and `renderer.rs` as a drawing module.

The project follows a clear structure, coding style, and requirements, and is built with Cargo.