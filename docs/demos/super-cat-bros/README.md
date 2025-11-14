# Super Cat Bros: A Gfx-Engine Demo Game

## Overview

"Super Cat Bros" is a 2D platformer demo game created to showcase the capabilities of the `Gfx-Engine`. The game is inspired by classic 16-bit era platformers.

## Character

The protagonist of our game is a charming and adventurous cat.

### Character Assets

The character's animations are defined in `assets/game_config.toml` and use the following sprite sheets:

-   **Idle:** `assets/graphics/captain_cat_idle_left.png`, `assets/graphics/captain_cat_idle_right.png`
-   **Walk:** `assets/graphics/captain_cat_walk_left.png`, `assets/graphics/captain_cat_walk_right.png`
-   **Jump:** `assets/graphics/captain_cat_jump_left.png`, `assets/graphics/captain_cat_jump_right.png`

## Gameplay Mechanics

The core gameplay mechanics will be a simplified version of those found in Super Mario World:

-   **2D Platforming:** The player will navigate a 2D world by running and jumping.
-   **Level-based Progression:** The game will be divided into levels, each with a start and an end point.
-   **Collectibles:** Levels will contain items for the player to collect (e.g., coins, power-ups).
-   **Enemies:** Simple enemies will patrol the levels. The player will be able to defeat them by jumping on their heads.
-   **Physics:** The game will feature basic physics for player movement, including acceleration, deceleration, and jumping.

## Configuration

All game data and logic will be driven by configuration files located in the `assets/` directory. This includes level layouts, character properties, and enemy definitions. This approach allows for easy modification and extension of the game without changing the engine's source code.
