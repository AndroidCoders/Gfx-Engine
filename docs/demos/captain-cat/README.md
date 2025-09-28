# Captain Cat: A Gfx-Engine Demo Game

## Overview

"Captain Cat" is a 2D platformer demo game created to showcase the capabilities of the `Gfx-Engine`. The game is inspired by the classic Super Mario World and features a swashbuckling feline hero.

## Character

The protagonist of our game is Captain Cat, a charming and adventurous pirate cat, in the vein of "Puss in Boots". He is a brave explorer, always ready for a new adventure.

### Character Assets

-   **Default:** `assets/graphics/Captain_Cat_Image_Front_Side.png`
-   **Facing Left:** `assets/graphics/Captain_Cat_Image_Left_Side.png`
-   **Facing Right:** `assets/graphics/Captain_Cat_Image_Right_Side.png`

## Gameplay Mechanics

The core gameplay mechanics will be a simplified version of those found in Super Mario World:

-   **2D Platforming:** The player will navigate a 2D world by running and jumping.
-   **Level-based Progression:** The game will be divided into levels, each with a start and an end point.
-   **Collectibles:** Levels will contain items for the player to collect (e.g., coins, power-ups).
-   **Enemies:** Simple enemies will patrol the levels. The player will be able to defeat them by jumping on their heads.
-   **Physics:** The game will feature basic physics for player movement, including acceleration, deceleration, and jumping.

## Configuration

All game data and logic will be driven by configuration files located in the `assets/` directory. This includes level layouts, character properties, and enemy definitions. This approach allows for easy modification and extension of the game without changing the engine's source code.
