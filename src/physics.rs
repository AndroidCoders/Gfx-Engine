// src/physics.rs

use crate::game_state::GameObject;

/// Updates the state of all game objects, including movement and wall collisions.
///
/// # Arguments
///
/// * `objects` - A mutable slice of `GameObject`s to update.
/// * `width` - The width of the game area.
/// * `height` - The height of the game area.
pub fn update_objects(objects: &mut [GameObject], width: u32, height: u32, damping_factor: f32, max_speed: f32, min_speed: f32) {
    for object in objects.iter_mut() {
        // Apply damping
        object.velocity.0 *= damping_factor;
        object.velocity.1 *= damping_factor;

        // Clamp max speed
        let current_speed_sq = object.velocity.0.powi(2) + object.velocity.1.powi(2);
        if current_speed_sq > max_speed.powi(2) {
            let current_speed = current_speed_sq.sqrt();
            object.velocity.0 = (object.velocity.0 / current_speed) * max_speed;
            object.velocity.1 = (object.velocity.1 / current_speed) * max_speed;
        }

        // Apply min speed threshold
        let current_speed_sq = object.velocity.0.powi(2) + object.velocity.1.powi(2);
        if current_speed_sq < min_speed.powi(2) {
            object.velocity.0 = 0.0;
            object.velocity.1 = 0.0;
        }

        let next_x = object.x + object.velocity.0;
        if next_x < 0.0 || (next_x + object.width as f32) > width as f32 {
            object.velocity.0 = -object.velocity.0;
        }

        let next_y = object.y + object.velocity.1;
        if next_y < 0.0 || (next_y + object.height as f32) > height as f32 {
            object.velocity.1 = -object.velocity.1;
        }

        object.x += object.velocity.0;
        object.y += object.velocity.1;
    }
}

/// Detects and resolves collisions between objects.
///
/// # Arguments
///
/// * `objects` - A mutable slice of `GameObject`s to check for collisions.
pub fn resolve_object_collisions(objects: &mut [GameObject]) {
    // Simple AABB collision detection and resolution
    for i in 0..objects.len() {
        for j in (i + 1)..objects.len() {
            let (obj1, obj2) = {
                let (left, right) = objects.split_at_mut(j);
                (&mut left[i], &mut right[0])
            };

            if obj1.to_rect().has_intersection(obj2.to_rect()) {
                // Optional: Separate overlapping objects to prevent sticking
                // This is a simplified approach and can be improved for more realistic physics
                let intersection = obj1.to_rect().intersection(obj2.to_rect()).unwrap();

                let overlap_x = intersection.width() as i32;
                let overlap_y = intersection.height() as i32;

                if overlap_x < overlap_y {
                    // Collision is more horizontal
                    let v1 = obj1.velocity.0;
                    let v2 = obj2.velocity.0;
                    let m1 = obj1.mass;
                    let m2 = obj2.mass;
                    let e = 1.0; // Perfectly elastic collision

                    obj1.velocity.0 = (v1 * (m1 - e * m2) + v2 * m2 * (1.0 + e)) / (m1 + m2);
                    obj2.velocity.0 = (v1 * m1 * (1.0 + e) + v2 * (m2 - e * m1)) / (m1 + m2);

                    if obj1.x < obj2.x {
                        obj1.x -= overlap_x as f32 / 2.0;
                        obj2.x += overlap_x as f32 / 2.0;
                    } else {
                        obj1.x += overlap_x as f32 / 2.0;
                        obj2.x -= overlap_x as f32 / 2.0;
                    }
                } else {
                    // Collision is more vertical
                    let v1 = obj1.velocity.1;
                    let v2 = obj2.velocity.1;
                    let m1 = obj1.mass;
                    let m2 = obj2.mass;
                    let e = 1.0; // Perfectly elastic collision

                    obj1.velocity.1 = (v1 * (m1 - e * m2) + v2 * m2 * (1.0 + e)) / (m1 + m2);
                    obj2.velocity.1 = (v1 * m1 * (1.0 + e) + v2 * (m2 - e * m1)) / (m1 + m2);

                    if obj1.y < obj2.y {
                        obj1.y -= overlap_y as f32 / 2.0;
                        obj2.y += overlap_y as f32 / 2.0;
                    } else {
                        obj1.y += overlap_y as f32 / 2.0;
                        obj2.y -= overlap_y as f32 / 2.0;
                    }
                }
            }
        }
    }
}
