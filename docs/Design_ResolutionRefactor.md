# Design: Logical Resolution Refactor (Scale-on-Draw)

**Date:** January 1, 2026
**Objective:** Simplify the engine's coordinate system to match the Tiled editor (1:1) while preserving High-Definition (HD) sub-pixel smoothness and visual scale.

## 1. The Core Problem
Currently, the engine uses an **"Upscale-on-Load"** architecture.
*   We multiply all asset sizes and map positions by `4.0` when loading.
*   **Result:** The internal world is 4x larger than the design files.
*   **Issues:** Debugging is confusing (Coordinate `12800` vs `3200`), physics values are inflated, and mental mapping between Tiled and Code is difficult.

## 2. The Solution: "Scale-on-Draw"
We will shift to a **"Logical Resolution"** architecture where the Game World matches the Design World (Retro Resolution), but the Renderer projects it to the Screen (HD Resolution).

### Coordinate Systems

| System | Resolution | Scale Factor | Role |
| :--- | :--- | :--- | :--- |
| **Asset Space** | 32x32 px (Tile) | **1:1** | Raw PNG source. |
| **Design Space** | 480x270 (Viewport) | **1:1** | Tiled Editor (`.tmx`). Coordinates match exactly. |
| **World Space (UCS)**| **480x270** | **1:1** | **The New Standard.** Physics and Logic run here. <br>Position `x=10.5` is valid (Sub-pixel). |
| **Screen Space** | 1920x1080 | **4x** | **Calculated at Runtime.** <br>`ScreenX = WorldX * 4.0`. |

### Key Benefits
1.  **Logical Consistency:** `x=100` in Tiled means `x=100.0` in the debugger.
2.  **Visual Fidelity:** By using floats (`10.5`) and scaling at draw time, we render at `x=42.0` on screen. Movement remains buttery smooth (not blocky).
3.  **Correctness:** "Clustering" bugs caused by confusion over scale factors are eliminated.

## 3. Implementation Plan

### Phase 1: Configuration & Constants
1.  **Rename Constant:** Change `DATA_SCALE_FACTOR` to `RENDER_SCALE_FACTOR` (Value: `4.0`) in `src/config/mod.rs`.
2.  **Define Resolution:** Explicitly define `LOGICAL_WIDTH = 480` and `LOGICAL_HEIGHT = 270` if needed, derived from `VIRTUAL_WIDTH / SCALE`.

### Phase 2: Loading Logic (Refactor)
1.  **`src/level.rs`:** Remove the `* SCALE` multiplication when parsing TMX objects. Load raw coordinates.
2.  **`src/texture_manager.rs`:** Remove the image resizing logic. Load textures at native resolution (32x32).

### Phase 3: Rendering Logic (The Bridge)
1.  **`src/renderer.rs`:**
    *   In `draw_sprite` and `draw_level`, apply `RENDER_SCALE_FACTOR` to:
        *   Source Rectangles (Dest Width/Height).
        *   Positions (`(pos.x - camera.x) * SCALE`).
    *   This ensures a 32x32 sprite is drawn as a 128x128 quad on the 1080p screen.

### Phase 4: Physics & Gameplay Conversion (Crucial)
To maintain the exact same "Game Feel", we must scale down all physics constants by **4.0**.

**Conversion Table (`game_config.toml` & `config.toml`):**

| Parameter | Old Value (Example) | New Value (÷ 4.0) |
| :--- | :--- | :--- |
| **Tile Size** | 128 | 32 |
| **Gravity** | 3200.0 | 800.0 |
| **Max Speed** | 220.0 | 55.0 |
| **Jump Strength** | -550.0 | -137.5 |
| **Acceleration** | 800.0 | 200.0 |
| **Camera Speed** | (Depends on logic) | *Review Camera Logic* |

**Verification:**
*   Old: Move 220px in 1 sec (World 15360) = 1.4% of width.
*   New: Move 55px in 1 sec (World 3840) = 1.4% of width.
*   **Result:** Visual speed is identical.

### Phase 5: UI & Text
*   **UI Coordinates:** If UI positions are hardcoded in 1080p, they should remain in "Screen Space" or be explicitly scaled.
*   **Recommendation:** Treat UI as "Screen Space Overlay". `Renderer` should support a `draw_ui` mode that ignores the Scale Factor (1:1 with screen).

## 4. Execution Checklist
- [ ] Update `src/config/mod.rs` (Scale Constant).
- [ ] Refactor `src/level.rs` (Load Logic).
- [ ] Refactor `src/texture_manager.rs` (Load Logic).
- [ ] Refactor `src/renderer.rs` (Draw Logic).
- [ ] Update `config.toml` (Physics Values).
- [ ] Update `assets/game_config.toml` (Entities & Animations).
- [ ] Verify Demo Game (Visual & Feel check).
