use sdl3::rect::Rect;
use crate::config::{ObjectConfig, PhysicsConfig};
use crate::physics::{update_objects, resolve_object_collisions};

/// Represents a single object in the game world.
pub struct GameObject {
    /// The x-coordinate of the object's position.
    pub x: f32,
    /// The y-coordinate of the object's position.
    pub y: f32,
    /// The width of the object.
    pub width: u32,
    /// The height of the object.
    pub height: u32,
    /// The velocity of the object.
    pub velocity: (f32, f32),
    /// The mass of the object, used for physics calculations.
    pub mass: f32,
}

impl GameObject {
    /// Converts the `GameObject`'s position and dimensions to an SDL `Rect`.
    pub fn to_rect(&self) -> Rect {
        Rect::new(self.x.round() as i32, self.y.round() as i32, self.width, self.height)
    }
}

/// Holds the entire state of the game.
pub struct GameState {
    /// A list of all game objects currently in the world.
    pub objects: Vec<GameObject>,
    /// A counter for the number of frames that have passed.
    pub frame_counter: u32,
}

impl GameState {
    /// Creates a new `GameState` from a slice of `ObjectConfig`s.
    pub fn new(objects_config: &[ObjectConfig]) -> Self {
        let objects = objects_config.iter().map(|o| GameObject {
            x: o.x,
            y: o.y,
            width: o.width,
            height: o.height,
            velocity: (o.vx, o.vy),
            mass: (o.width * o.height) as f32,
        }).collect();

        Self {
            objects,
            frame_counter: 0,
        }
    }

    /// Updates the state of all game objects.
    ///
    /// This includes advancing their positions based on velocity and handling
    /// collisions with the boundaries of the given `width` and `height`.
    ///
    /// # Arguments
    ///
    /// * `width` - The width of the game area.
    /// * `height` - The height of the game area.
pub fn update(&mut self, width: u32, height: u32, physics_config: &PhysicsConfig) {
        self.frame_counter += 1;
        update_objects(
            &mut self.objects,
            width,
            height,
            physics_config.damping_factor,
            physics_config.max_speed,
            physics_config.min_speed,
        );
        resolve_object_collisions(&mut self.objects);
    }
}
