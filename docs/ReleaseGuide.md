# Release and Deployment Guide

**TLDR:**
*   This document explains how to build the game for **Windows** (via cross-compilation from Linux) and **Linux**.
*   It also outlines the workflow for testing and editing levels on a secondary machine.

## 1. Building for Windows (Cross-Compilation)

Since we are developing on Linux (Fedora), we use **MinGW-w64** to cross-compile a Windows executable (`.exe`).

### Prerequisites

1.  **Rust Windows Target:**
    Add the generic Windows target to your Rust installation:
    ```bash
    rustup target add x86_64-pc-windows-gnu
    ```

2.  **MinGW-w64 Toolchain:**
    Install the MinGW compiler. On Fedora:
    ```bash
    sudo dnf install mingw64-gcc
    ```

3.  **SDL3 Development Libraries (MinGW):**
    *   Download the latest `SDL3-devel-*-mingw.zip` from [SDL Releases](https://github.com/libsdl-org/SDL/releases).
    *   Extract it to a known location (e.g., `~/Developer/sdks/SDL3-mingw`).
    *   **Note:** You will need the path to the `lib` folder inside (e.g., `.../x86_64-w64-mingw32/lib`).

### Configuration

Create or update `.cargo/config.toml` in your project root to tell Cargo how to link for Windows:

```toml
[target.x86_64-pc-windows-gnu]
linker = "x86_64-w64-mingw32-gcc"
rustflags = [
    "-L", "native=/path/to/your/SDL3-mingw/lib", # <--- UPDATE THIS PATH
    "-C", "link-arg=-static-libgcc",             # Static link generic C runtimes
    "-C", "link-arg=-static-libstdc++"           # Static link generic C++ runtimes
]
```

### Building

Run the build command:
```bash
cargo build --target x86_64-pc-windows-gnu --release
```

### Packaging for Release

To create a standalone playable folder for Windows:
1.  Create a folder (e.g., `Gfx-Engine-Win64`).
2.  Copy the executable: `target/x86_64-pc-windows-gnu/release/Gfx-Engine.exe`.
3.  Copy the **SDL3.dll**: Find this in the `bin` folder of the SDL3 MinGW package you downloaded.
4.  Copy the **assets** folder: Copy the entire `assets/` directory from the project root.

**Structure:**
```text
Gfx-Engine-Win64/
├── Gfx-Engine.exe
├── SDL3.dll
└── assets/
    ├── game_config.toml
    ├── levels/
    └── graphics/
```

## 2. Building for Linux

### Building

Run the standard release build:
```bash
cargo build --release
```

### Packaging for Release

1.  Create a folder (e.g., `Gfx-Engine-Linux`).
2.  Copy the executable: `target/release/Gfx-Engine`.
3.  Copy the **assets** folder.

**Note on Dependencies:**
The Linux binary is dynamically linked. The target machine must have **SDL3** installed.
*   If the target machine is also Fedora/modern Linux: `sudo dnf install SDL3`.
*   **Portable Option:** You can bundle `libSDL3.so` (copy it from `/usr/lib64/` or build it yourself) into the game folder and run the game with a script:
    ```bash
    #!/bin/bash
    export LD_LIBRARY_PATH=.
    ./Gfx-Engine
    ```

## 3. Workflow: Testing & Level Editing on Secondary Machine

To test the game and edit levels on a second computer, we recommend using **Git**.

### Setup on Secondary Machine

1.  **Install Tools:**
    *   **Git:** To sync files.
    *   **Tiled:** Map editor for opening `.tmx` files.
    *   **Text Editor:** (Optional) For editing `.toml` configs.

2.  **Clone the Repository:**
    Even if you only want to play the compiled binary, cloning the repo is the easiest way to get the assets and keep them valid.
    ```bash
    git clone https://github.com/AndroidCoders/Gfx-Engine.git
    ```

### Testing Workflow

1.  **Build (Primary Machine):** Build the executable (Windows or Linux) on your main dev machine.
2.  **Transfer:** Copy the **Executable** (and `SDL3.dll` if Windows) to the root of the repo on the secondary machine (or a `bin/` folder, as long as it can find `assets/`).
    *   *Alternative:* You can commit the binaries to a separate `builds` branch or use GitHub Releases to download them.
3.  **Run:** Double-click or run the executable.

### Level Editing Workflow

1.  **Edit (Secondary Machine):**
    *   Open `assets/levels/world_1_level_1.tmx` in **Tiled**.
    *   Make changes (move platforms, add enemies).
    *   Save.
2.  **Sync:**
    *   Commit and push changes from the secondary machine:
        ```bash
        git add assets/levels/
        git commit -m "design: Update level 1 layout"
        git push
        ```
3.  **Update (Primary Machine):**
    *   Pull the changes:
        ```bash
        git pull
        ```
    *   Run the game to see the changes.
