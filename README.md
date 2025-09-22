File version: 1.02

**TLDR:**
* This project is a Rust application for creating graphical effects.
* It displays multiple animated white boxes on a fullscreen background.
* The application's behavior, including debug features, is configured via `config.toml`.

A project for learning the the Rust programming language by creating graphical effects. This application displays multiple white boxes that animate across a black fullscreen background.

## How to Run

This project uses Cargo, the Rust package manager.

To build and run the application:

```bash
cargo run
```

## Configuration

The application's behavior is controlled by the `config.toml` file at the project root. This includes window settings, renderer colors, physics parameters, and debug features.

## Debug Features

### Video Debug Output

To enable saving PNG screenshots for debugging, set `enable_frame_capture = true` in the `[debug]` section of `config.toml`. The output directory, maximum captured frames, and capture interval are also configured in this section.

