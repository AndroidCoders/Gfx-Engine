// src/level.rs

//! Manages loading and representing game levels from Tiled .tmx files.

use crate::math::Vector2D;
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
    pub position: Vector2D,
}

#[derive(Clone)]
pub struct Level {
    pub tileset: Tileset,
    pub map: Map,
    pub collision: Collision,
    pub entities: Vec<Entity>,
}

impl Level {
    pub fn is_solid(&self, x: usize, y: usize) -> bool {
        if let Some(row) = self.collision.tiles.get(y) {
            if let Some(&tile_id) = row.get(x) {
                return tile_id == 1;
            }
        }
        false
    }
}

// --- Structs for Deserializing TMX XML ---

#[derive(Debug, Deserialize)]
struct TmxMap {
    #[serde(rename = "@width")]
    width: u32,
    #[serde(rename = "layer", default)]
    tile_layers: Vec<TmxLayer>,
    #[serde(rename = "objectgroup", default)]
    object_groups: Vec<TmxObjectGroup>,
}

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

#[derive(Debug, Deserialize, Clone)]
pub struct TmxObject {
    #[serde(rename = "@type")]
    pub r#type: String,
    #[serde(rename = "@x")]
    pub x: f32,
    #[serde(rename = "@y")]
    pub y: f32,
}


#[derive(Debug, Deserialize)]
struct TmxObjectGroup {
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "object", default)]
    objects: Vec<TmxObject>,
}

/// Loads a level from a Tiled .tmx file.
pub fn load_level(path: &str) -> Result<Level, String> {
    let tmx_str = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
    let tmx_map: TmxMap = from_str(&tmx_str).map_err(|e| e.to_string())?;

    // Find and parse the first tile layer
    let tile_layer = tmx_map.tile_layers.get(0).ok_or("No tile layer found in TMX file")?;
    if tile_layer.data.encoding != "csv" {
        return Err("Level data must be CSV encoded.".to_string());
    }

    let tile_data: Vec<u32> = tile_layer.data.content
        .split(',')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<u32>().unwrap_or(0))
        .collect();

    let map_tiles: Vec<Vec<u32>> = tile_data.chunks(tmx_map.width as usize)
        .map(|chunk| chunk.to_vec())
        .collect();

    // Manually define which tiles are solid.
    let solid_tiles: Vec<u32> = vec![34, 35, 36];
    let collision_tiles: Vec<Vec<u32>> = map_tiles.iter()
        .map(|row| {
            row.iter().map(|&tile_id| if solid_tiles.contains(&tile_id) { 1 } else { 0 }).collect()
        })
        .collect();

    // Manually define the tileset based on our known file structure
    // TODO: Parse the .tsx file to get this information dynamically
    let tileset = Tileset {
        texture: "assets/graphics/tileset_1_outside.png".to_string(),
        tile_width: 32,
        tile_height: 32,
    };

    // Parse object layers
    let mut entities = Vec::new();
    for object_group in &tmx_map.object_groups {
        for object in &object_group.objects {
            entities.push(Entity {
                r#type: object.r#type.clone(),
                position: Vector2D::new(object.x, object.y),
            });
        }
    }

    // Manually add player for now
    entities.push(Entity {
        r#type: "Player".to_string(),
        position: Vector2D::new(80.0, 100.0),
    });


    Ok(Level {
        tileset,
        map: Map { tiles: map_tiles },
        collision: Collision { tiles: collision_tiles },
        entities,
    })
}
