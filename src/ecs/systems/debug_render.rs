//! # Concept: Debug Visualization
//! 
//! This module provides developer-facing visual aids. It is responsible 
//! for rendering non-gameplay data like collision boundaries, engine 
//! performance metrics, and application state for observability.

use crate::ecs::systems::RenderContext;
use crate::renderer::{Renderer, TextRenderParams};
use sdl3::pixels::Color;

/// A system that renders hitboxes and engine metrics to the screen.
pub struct SystemDebugRender;

impl SystemDebugRender {
    /// Renders collision boxes and performance overlays to the current frame.
    #[allow(clippy::too_many_arguments)]
    pub fn update(
        &mut self,
        renderer: &mut Renderer,
        world: &crate::ecs::world::World,
        context: &RenderContext<'_>,
        camera: &crate::camera::Camera,
        font_manager: &crate::font_manager::FontManager,
        frame_count: u64,
        fps: u32,
    ) -> Result<(), String> {
        
        // 1. Visualize Collision Boundaries.
        if context.config.debug.debug_draw_collision_boxes {
            let scale = crate::config::RENDER_SCALE_FACTOR;
            for collision in world.collisions.values() {
                // Transform world-space collision rect to screen-space for drawing.
                let screen_rect = sdl3::rect::Rect::new(
                    ((collision.rect.x as f32 - camera.position.x) * scale) as i32,
                    ((collision.rect.y as f32 - camera.position.y) * scale) as i32,
                    (collision.rect.width() as f32 * scale) as u32,
                    (collision.rect.height() as f32 * scale) as u32,
                );
                renderer.draw_rect(&screen_rect, Color::RGB(255, 0, 0))?;
            }
        }

        // 2. Render Left-side Debug Text (Player state and core metrics).
        if let Some(player_entity) = context.player_entity
            && let (Some(_pos), Some(_vel), Some(state_comp), Some(_collision)) = (
                world.positions.get(&player_entity),
                world.velocities.get(&player_entity),
                world.state_components.get(&player_entity),
                world.collisions.get(&player_entity),
            ) {
                let _is_grounded = world.is_grounded(player_entity);
                let _state_name = state_comp.state_machine.current_state.as_ref().map_or("None", |s| s.get_name());

                let debug_text_color = Color::RGB(255, 255, 255);
                let start_x = context.config.debug.text_start_x;
                let mut current_y = 150; 
                let line_height = context.config.debug.text_line_spacing;

                let mut draw_debug_line = |text: String| -> Result<(), String> {
                    renderer.render_text(font_manager, TextRenderParams {
                        text: &text,
                        x: start_x,
                        y: current_y,
                        font_size: 24.0,
                        scale: 1.0,
                        color: debug_text_color,
                    })?;
                    current_y += line_height;
                    Ok(())
                };

                draw_debug_line(format!("Frame: {}", frame_count))?;
                draw_debug_line(format!("FPS: {}", fps))?;
                draw_debug_line(format!("Benchmark: Min: {}, Max: {}, Avg: {}", 
                    context.benchmarker.min_fps, 
                    context.benchmarker.max_fps, 
                    context.benchmarker.avg_fps
                ))?;
                
                // --- Frame Debug Info ---
                current_y += line_height; // Add a space
                let info = &world.frame_debug_info;
                let p_pos = info.player_pos.unwrap_or_default();
                let pp_pos = info.player_prev_pos.unwrap_or_default();
                let c_pos = info.camera_pos.unwrap_or_default();

                renderer.render_text(font_manager, TextRenderParams { text: &format!("[Player] Pos: ({:.1}, {:.1})", p_pos.x, p_pos.y), x: start_x, y: current_y, font_size: 24.0, scale: 1.0, color: debug_text_color })?;
                current_y += line_height;
                renderer.render_text(font_manager, TextRenderParams { text: &format!("[Player] Prev Pos: ({:.1}, {:.1})", pp_pos.x, pp_pos.y), x: start_x, y: current_y, font_size: 24.0, scale: 1.0, color: debug_text_color })?;
                current_y += line_height;
                renderer.render_text(font_manager, TextRenderParams { text: &format!("[Player] Render W/H: ({}, {})", info.player_render_w, info.player_render_h), x: start_x, y: current_y, font_size: 24.0, scale: 1.0, color: debug_text_color })?;
                current_y += line_height;
                renderer.render_text(font_manager, TextRenderParams { text: &format!("[Camera] Pos: ({:.1}, {:.1})", c_pos.x, c_pos.y), x: start_x, y: current_y, font_size: 24.0, scale: 1.0, color: debug_text_color })?;
                current_y += line_height;
                renderer.render_text(font_manager, TextRenderParams { text: &format!("[World] Renderables: {}", info.renderable_count), x: start_x, y: current_y, font_size: 24.0, scale: 1.0, color: debug_text_color })?;
        }

        // 3. Render Right-side Performance Hotspots.
        // Displays a sorted list of systems consuming the most frame budget.
        let start_x_right = 1000; 
        let mut current_y_right = 150; 
        let line_height = context.config.debug.text_line_spacing;

        let sorted_metrics = context.benchmarker.get_sorted_metrics();
        
        renderer.render_text(font_manager, TextRenderParams {
            text: "--- Hotspots ---",
            x: start_x_right,
            y: current_y_right,
            font_size: 24.0,
            scale: 1.0,
            color: Color::RGB(255, 255, 0),
        })?;
        current_y_right += line_height;

        for (name, percent) in sorted_metrics {
             // 4. Highlight expensive systems with warning colors.
             if percent < 1.0 { continue; }

             let color = if percent > 25.0 { Color::RGB(255, 50, 50) } // High Alert (Critical)
                         else if percent > 10.0 { Color::RGB(255, 165, 0) } // Warning (Heuristic)
                         else { Color::RGB(200, 200, 200) };
             
             let text = format!("{:<15} {:>5.1}%", name, percent);
             
             renderer.render_text(font_manager, TextRenderParams {
                text: &text,
                x: start_x_right,
                y: current_y_right,
                font_size: 24.0,
                scale: 1.0,
                color,
            })?;
            current_y_right += line_height;
        }

        Ok(())
    }
}
