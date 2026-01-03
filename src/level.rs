//! # Manager: Level Infrastructure
//! 
//! This module is the authority for game world representation. It parses 
//! Tiled (TMX) and Tileset (TSX) data to build the physical collision 
//! grid, visual tile map, and entity spawning templates.

use crate::math::Vector2D;
use serde::Deserialize;
use quick_xml::de::from_str;
use std::collections::{HashMap, HashSet};
use std::path::Path;
use std::fs;

/// # Concept: Level Data
/// The engine-ready representation of a game world.
#[derive(Clone, Default)]
pub struct Level {
    pub tileset: Tileset,
    pub map: Map,
    pub collision: Collision,
    pub entities: Vec<Entity>,
}

impl Level {
    /// Authoritative check for tile solidity at grid coordinates.
    pub fn is_solid(&self, x: usize, y: usize) -> bool {
        if let Some(row) = self.collision.tiles.get(y)
            && let Some(&tile_id) = row.get(x) {
                return tile_id != 0; 
            }
        false
    }
}

/// Decodes TMX and TSX XML files into an engine-ready Level structure.
pub fn load_level(path: &str) -> Result<Level, String> {
    // 1. Read the TMX map file from disk.
    let tmx_str = fs::read_to_string(path).map_err(|e| e.to_string())?;
    let tmx_map: TmxMap = from_str(&tmx_str).map_err(|e| e.to_string())?;

    // 2. Parse the primary tile layer (must be CSV encoded).
    let tile_layer = tmx_map.tile_layers.first().ok_or("No tile layer found in TMX file")?;
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

    // 3. Resolve the external TSX tileset reference.
    let tileset_ref = tmx_map.tilesets.first().ok_or("No <tileset> reference found in TMX file")?;
    let tmx_path = Path::new(path);
    let tsx_path = tmx_path.parent().unwrap_or_else(|| Path::new("")).join(&tileset_ref.source);
    
    let tsx_str = fs::read_to_string(&tsx_path).map_err(|e| format!("Failed to read TSX file: {}", e))?;
    let tmx_tileset: TmxTileset = from_str(&tsx_str).map_err(|e| format!("Failed to parse TSX file: {}", e))?;

    let mut solid_tiles = HashSet::new();
    let mut image_source = "".to_string();

    // 4. Identify solid tiles based on custom TSX properties.
    for item in tmx_tileset.content {
        match item {
            TmxTilesetContent::Image(image) => {
                let image_path = tsx_path.parent().unwrap_or_else(|| Path::new("")).join(image.source);
                let canonical_path = fs::canonicalize(&image_path).map_err(|e| e.to_string())?;
                image_source = canonical_path.to_string_lossy().to_string();
            }
            TmxTilesetContent::Tile(tile) => {
                if let Some(properties) = tile.properties {
                    for prop in properties.properties {
                        if prop.name == "solid" && prop.property_type.as_deref() == Some("bool") && prop.value == "true" {
                            solid_tiles.insert(tile.id + tileset_ref.firstgid);
                        }
                    }
                }
            }
        }
    }

    // 5. Construct the physical collision grid.
    let collision_tiles: Vec<Vec<u32>> = map_tiles.iter()
        .map(|row| {
            row.iter().map(|&tile_id| if solid_tiles.contains(&tile_id) { 1 } else { 0 }).collect()
        })
        .collect();

    let tileset = Tileset {
        texture: image_source,
        tile_width: tmx_tileset.tile_width,
        tile_height: tmx_tileset.tile_height,
    };

    // 6. Map TMX objects to internal Entity templates and apply scale.
    let mut entities = Vec::new();
    for object_group in &tmx_map.object_groups {
        for object in &object_group.objects {
            let mut properties = HashMap::new();
            if let Some(props) = &object.properties {
                for prop in &props.properties {
                    properties.insert(prop.name.clone(), prop.value.clone());
                }
            }

            entities.push(Entity {
                r#type: object.r#type.clone().unwrap_or("Default".to_string()),
                position: Vector2D::new(
                    object.x,
                    object.y
                ),
                properties,
            });
        }
    }

    Ok(Level { tileset, map: Map { tiles: map_tiles }, collision: Collision { tiles: collision_tiles }, entities })
}

// --- Internal Data Models ---
#[derive(Clone, Default)] pub struct Tileset { pub texture: String, pub tile_width: u32, pub tile_height: u32 }
#[derive(Clone, Default)] pub struct Map { pub tiles: Vec<Vec<u32>> }
#[derive(Clone, Default)] pub struct Collision { pub tiles: Vec<Vec<u32>> }
#[derive(Clone, Default)] pub struct Entity { pub r#type: String, pub position: Vector2D, pub properties: HashMap<String, String> }

#[derive(Debug, Deserialize)] struct TmxMap { #[serde(rename = "@width")] width: u32, #[serde(rename = "tileset", default)] tilesets: Vec<TmxTilesetRef>, #[serde(rename = "layer", default)] tile_layers: Vec<TmxLayer>, #[serde(rename = "objectgroup", default)] object_groups: Vec<TmxObjectGroup> }
#[derive(Debug, Deserialize)] struct TmxTilesetRef { #[serde(rename = "@firstgid")] firstgid: u32, #[serde(rename = "@source")] source: String }
#[derive(Debug, Deserialize)] struct TmxLayer { data: TmxData }
#[derive(Debug, Deserialize)] struct TmxData { #[serde(rename = "@encoding")] encoding: String, #[serde(rename = "$text")] content: String }
#[derive(Debug, Deserialize)] struct TmxObjectGroup { #[serde(rename = "object", default)] objects: Vec<TmxObject> }
#[derive(Debug, Deserialize)] struct TmxObject { #[serde(rename = "@type")] r#type: Option<String>, #[serde(rename = "@x")] x: f32, #[serde(rename = "@y")] y: f32, properties: Option<TmxProperties> }
#[derive(Debug, Deserialize)] struct TmxProperties { #[serde(rename = "property", default)] properties: Vec<TmxProperty> }
#[derive(Debug, Deserialize)] struct TmxProperty { #[serde(rename = "@name")] name: String, #[serde(rename = "@type")] property_type: Option<String>, #[serde(rename = "@value")] value: String }
#[derive(Debug, Deserialize)] struct TmxTileset { #[serde(rename = "@tilewidth")] tile_width: u32, #[serde(rename = "@tileheight")] tile_height: u32, #[serde(rename = "$value")] content: Vec<TmxTilesetContent> }
#[derive(Debug, Deserialize)] enum TmxTilesetContent { #[serde(rename = "image")] Image(TmxImage), #[serde(rename = "tile")] Tile(TmxTile) }
#[derive(Debug, Deserialize)] struct TmxImage { #[serde(rename = "@source")] source: String }
#[derive(Debug, Deserialize)] struct TmxTile { #[serde(rename = "@id")] id: u32, properties: Option<TmxProperties> }
