// src/level.rs

//! This module is responsible for loading and representing game levels.
//!
//! Levels are created in the Tiled Map Editor and saved as `.tmx` files. This
//! module uses the `quick-xml` crate to parse these files and constructs a `Level`
//! struct, which contains all the data needed by the engine to render and
//! interact with the level, including tile layers, collision data, and entity placements.

use crate::math::Vector2D;
use serde::Deserialize;
use quick_xml::de::from_str;

// --- Internal Engine Structs ---

/// Represents a tileset used in a level, defining the texture and tile dimensions.
#[derive(Clone)]
pub struct Tileset {
    /// The path to the tileset texture image.
    pub texture: String,
    /// The width of a single tile in pixels.
    pub tile_width: u32,
    /// The height of a single tile in pixels.
    pub tile_height: u32,
}

/// Represents the visual tile map of a level.
#[derive(Clone)]
pub struct Map {
    /// A 2D vector of tile GIDs that define the visual layout of the level.
    pub tiles: Vec<Vec<u32>>,
}

/// Represents the collision map of a level.
#[derive(Clone)]
pub struct Collision {
    /// A 2D vector where non-zero values represent solid tiles.
    pub tiles: Vec<Vec<u32>>,
}

/// Represents a game object or entity loaded from an object layer in a TMX file.
#[derive(Clone)]
pub struct Entity {
    /// The type of the entity (e.g., "Player", "EnemySpider", "GoldCoin").
    pub r#type: String,
    /// The initial position of the entity in world coordinates.
    pub position: Vector2D,
    /// A map of custom properties for the entity.
    pub properties: std::collections::HashMap<String, String>,
}

/// Represents a complete game level.
///
/// Contains all the necessary data for the engine to run a level, including the
/// visual tilemap, collision data, entity placements, and tileset information.
#[derive(Clone)]
pub struct Level {
    /// The tileset used by the level.
    pub tileset: Tileset,
    /// The visual tilemap.
    pub map: Map,
    /// The collision grid.
    pub collision: Collision,
    /// A list of all entities to be spawned in the level.
    pub entities: Vec<Entity>,
}

impl Level {
    /// Checks if a tile at a given grid coordinate is solid for collision purposes.
    ///
    /// # Arguments
    ///
    /// * `x` - The x-coordinate of the tile in the grid.
    /// * `y` - The y-coordinate of the tile in the grid.
    ///
    /// # Returns
    ///
    /// `true` if the tile is solid, `false` otherwise. Returns `false` if the
    /// coordinates are out of bounds.
    pub fn is_solid(&self, x: usize, y: usize) -> bool {
        if let Some(row) = self.collision.tiles.get(y)
            && let Some(&tile_id) = row.get(x) {
                return tile_id != 0; // Any non-zero tile is considered solid
            }
        false
    }
}

// --- Structs for Deserializing TSX XML ---

#[derive(Debug, Deserialize, Clone)]
pub struct TmxProperty {
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@type")]
    property_type: String,
    #[serde(rename = "@value")]
    value: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct TmxProperties {
    #[serde(rename = "property", default)]
    properties: Vec<TmxProperty>,
}

#[derive(Debug, Deserialize)]
struct TmxTile {
    #[serde(rename = "@id")]
    id: u32,
    properties: Option<TmxProperties>,
}

#[derive(Debug, Deserialize)]
struct TmxImage {
    #[serde(rename = "@source")]
    #[allow(dead_code)]
    source: String,
}

#[derive(Debug, Deserialize)]
enum TmxTilesetContent {
    #[serde(rename = "image")]
    Image(TmxImage),
    #[serde(rename = "tile")]
    Tile(TmxTile),
}

#[derive(Debug, Deserialize)]
struct TmxTileset {
    #[serde(rename = "@name")]
    #[allow(dead_code)]
    name: String,
    #[serde(rename = "@tilewidth")]
    tile_width: u32,
    #[serde(rename = "@tileheight")]
    tile_height: u32,
    #[serde(rename = "$value")]
    content: Vec<TmxTilesetContent>,
}

// --- Structs for Deserializing TMX XML ---

#[derive(Debug, Deserialize)]
struct TmxTilesetRef {
    #[serde(rename = "@firstgid")]
    #[allow(dead_code)]
    firstgid: u32,
    #[serde(rename = "@source")]
    source: String,
}

/// Represents the top-level `<map>` element in a TMX file.
#[derive(Debug, Deserialize)]
struct TmxMap {
    #[serde(rename = "@width")]
    width: u32,
    #[serde(rename = "tileset", default)]
    tilesets: Vec<TmxTilesetRef>,
    #[serde(rename = "layer", default)]
    tile_layers: Vec<TmxLayer>,
    #[serde(rename = "objectgroup", default)]
    object_groups: Vec<TmxObjectGroup>,
}

/// Represents a `<layer>` element in a TMX file.
#[derive(Debug, Deserialize)]
struct TmxLayer {
    data: TmxData,
}

/// Represents a `<data>` element within a `<layer>` in a TMX file.
#[derive(Debug, Deserialize)]
struct TmxData {
    #[serde(rename = "@encoding")]
    encoding: String,
    #[serde(rename = "$text")]
    content: String,
}

/// Represents an `<object>` from an `<objectgroup>` in a TMX file.
#[derive(Debug, Deserialize, Clone)]
pub struct TmxObject {
    /// The user-defined type of the object.
    #[serde(rename = "@type")]
    pub r#type: String,
    /// The x-coordinate of the object in pixels.
    #[serde(rename = "@x")]
    pub x: f32,
    /// The y-coordinate of the object in pixels.
    #[serde(rename = "@y")]
    pub y: f32,
    /// An optional block of custom properties for the object.
    pub properties: Option<TmxProperties>,
}

/// Represents an `<objectgroup>` in a TMX file, used for placing entities.
#[derive(Debug, Deserialize)]
struct TmxObjectGroup {
    #[serde(rename = "@name")]
    #[allow(dead_code)]
    name: String,
    #[serde(rename = "object", default)]
    objects: Vec<TmxObject>,
}

/// Loads and parses a level from a Tiled `.tmx` file.
///
/// # Errors
///
/// This function will return an error string if the file cannot be read,
/// if the XML is malformed, or if the level data is not in the expected CSV format.
pub fn load_level(path: &str) -> Result<Level, String> {
    let tmx_str = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
    let tmx_map: TmxMap = from_str(&tmx_str).map_err(|e| e.to_string())?;

    // Find and parse the first tile layer
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

    // --- Parse Tileset Data (TSX) ---
    let tileset_ref = tmx_map.tilesets.first().ok_or("No <tileset> reference found in TMX file")?;
    let tmx_path = std::path::Path::new(path);
    let tsx_path = tmx_path.parent().unwrap_or_else(|| std::path::Path::new("")).join(&tileset_ref.source);
    
    let tsx_str = std::fs::read_to_string(&tsx_path).map_err(|e| format!("Failed to read TSX file at {:?}: {}", tsx_path, e))?;
    let tmx_tileset: TmxTileset = from_str(&tsx_str).map_err(|e| format!("Failed to parse TSX file: {}", e))?;

    let mut solid_tiles = std::collections::HashSet::new();
    let mut image_source = "".to_string();

    for item in tmx_tileset.content {
        match item {
            TmxTilesetContent::Image(image) => {
                // Construct path relative to the TSX file's location
                let image_path = tsx_path.parent().unwrap_or_else(|| std::path::Path::new("")).join(image.source);
                let canonical_path = std::fs::canonicalize(&image_path)
                    .map_err(|e| format!("Failed to canonicalize path {:?}: {}", image_path, e))?;
                image_source = canonical_path.to_string_lossy().to_string();
            }
            TmxTilesetContent::Tile(tile) => {
                if let Some(properties) = tile.properties {
                    for prop in properties.properties {
                        if prop.name == "solid" && prop.property_type == "bool" && prop.value == "true" {
                            // The GID in the TMX is 1-indexed and includes the `firstgid`.
                            // The ID in the TSX is 0-indexed.
                            solid_tiles.insert(tile.id + tileset_ref.firstgid);
                        }
                    }
                }
            }
        }
    }

    // --- Create Engine-Specific Structs ---

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

    // Parse object layers to create entity templates
    let mut entities = Vec::new();
    for object_group in &tmx_map.object_groups {
        for object in &object_group.objects {
            let mut properties = std::collections::HashMap::new();
            if let Some(props) = &object.properties {
                for prop in &props.properties {
                    properties.insert(prop.name.clone(), prop.value.clone());
                }
            }

            entities.push(Entity {
                r#type: object.r#type.clone(),
                position: Vector2D::new(object.x, object.y),
                properties,
            });
        }
    }




    Ok(Level {
        tileset,
        map: Map { tiles: map_tiles },
        collision: Collision { tiles: collision_tiles },
        entities,
    })
}
