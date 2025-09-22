File version: 1.02

**TLDR:**
This document lists the core application requirements for `gfx-boxes`:
* Core application features
* Video debug output feature, configurable via `config.toml`

# Core Application Requirements

- The application SHALL display a fullscreen window (configurable via `config.toml`).
- The application SHALL render animated objects on the screen.
- The application SHALL animate the rendered objects.
- The application SHALL close when the 'Escape' key is pressed.

# Video Debug Output Requirements

- The video debug output SHALL be activated by a setting in `config.toml`.
- The application SHALL allocate a configurable number of frame buffers at program startup for storing screen content.
- The application SHALL copy the screen canvas content to a frame buffer at configurable intervals.
- The application SHALL save the captured frame buffers to PNG files upon program exit.
- The saved PNG files SHALL be named descriptively (e.g., `output/frame_0001.png`).
- The output directory for PNG files SHALL be configurable via `config.toml`.
- The purpose of these PNG files is to provide visual test output for debugging and verification.
