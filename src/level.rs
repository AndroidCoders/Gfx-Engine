// src/level.rs

//! Manages loading and representing game levels from Tiled .tmx files.

use serde::Deserialize;
use quick_xml::de::from_str;

// --- Internal Engine Structs (Manually populated, no longer derived from file) ---

#[derive(Clone)]
pub struct Tileset {
    pub texture: String,
    pub tile_width: u32,
    pub tile_height: u32,
}

#[derive(Clone)]
pub struct Map {
    pub tiles: Vec<Vec<u32>>,
}

#[derive(Clone)]
pub struct Collision {
    pub tiles: Vec<Vec<u32>>,
}

#[derive(Clone)]
pub struct Entity {
    pub r#type: String,
    pub x: i32,
    pub y: i32,
}

#[derive(Clone)]
pub struct Level {
    pub tileset: Tileset,
    pub map: Map,
    pub collision: Collision,
    pub entities: Vec<Entity>,
}

// --- Structs for Deserializing TMX XML ---

#[derive(Debug, Deserialize)]
struct TmxMap {
    #[serde(rename = "@width")]
    width: u32,
    layer: TmxLayer,
    // We will manually load the tileset for now, so we don't need to parse this.
    // tileset: TmxTileset,
}

// #[derive(Debug, Deserialize)]
// struct TmxTileset {
//     #[serde(rename = "@source")]
//     source: String,
// }

#[derive(Debug, Deserialize)]
struct TmxLayer {
    data: TmxData,
}

#[derive(Debug, Deserialize)]
struct TmxData {
    #[serde(rename = "@encoding")]
    encoding: String,
    #[serde(rename = "$text")]
    content: String,
}

/// Loads a level from a Tiled .tmx file.
pub fn load_level(path: &str) -> Result<Level, String> {
    let tmx_str = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
    let tmx_map: TmxMap = from_str(&tmx_str).map_err(|e| e.to_string())?;

    if tmx_map.layer.data.encoding != "csv" {
        return Err("Level data must be CSV encoded.".to_string());
    }

    // Parse the CSV data
    let tile_data: Vec<u32> = tmx_map.layer.data.content
        .split(',')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<u32>().unwrap_or(0))
        .collect();

    // Reconstruct the 2D tile map
    let map_tiles: Vec<Vec<u32>> = tile_data.chunks(tmx_map.width as usize)
        .map(|chunk| chunk.to_vec())
        .collect();

    // For now, create a simple collision map based on the visual map.
    // Any tile that is not empty (0) is considered solid (1).
    let collision_tiles: Vec<Vec<u32>> = map_tiles.iter()
        .map(|row| {
            row.iter().map(|&tile_id| if tile_id > 0 { 1 } else { 0 }).collect()
        })
        .collect();

    // Manually define the tileset based on our known file structure
    // TODO: Parse the .tsx file to get this information dynamically
    let tileset = Tileset {
        texture: "assets/graphics/tileset_1_outside.png".to_string(),
        tile_width: 32,
        tile_height: 32,
    };

    // Manually define entities for now
    // TODO: Parse object layers in the TMX file
    let entities = vec![
        Entity {
            r#type: "Player".to_string(),
            x: 80,
            y: 1900,
        }
    ];

    Ok(Level {
        tileset,
        map: Map { tiles: map_tiles },
        collision: Collision { tiles: collision_tiles },
        entities,
    })
}
