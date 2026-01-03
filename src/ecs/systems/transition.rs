//! # Concept: Visual Transitions
//! 
//! This module manages fullscreen cinematic transitions (Iris In/Out).
//! It tracks transition progress over time and provides the rendering 
//! routine for the shutter geometry that masks the world.

use crate::ecs::event::{EventStartTransition, EventTransitionComplete, TransitionType};
use crate::ecs::systems::{SystemContext, RenderContext};
use crate::renderer::Renderer;
use sdl3::pixels::Color;

/// A system that manages the state and logic of screen transitions.
pub struct SystemTransition {
    state: TransitionState,
    timer: f32,
    duration: f32,
    center: Option<(i32, i32)>,
}

#[derive(PartialEq)]
enum TransitionState {
    Idle,
    Playing(TransitionType),
    Hold(TransitionType),
}

impl SystemTransition {
    pub fn new() -> Self {
        Self {
            state: TransitionState::Idle,
            timer: 0.0,
            duration: 1.0,
            center: None,
        }
    }

    /// Advances transition timers and publishes completion facts when finished.
    ///
    /// ⚠️ **Hotpath**: Called 120x per second.
    ///
    /// # Side Effects
    /// * Publishes [crate::ecs::event::EventTransitionComplete] when duration is reached.
    pub fn update(&mut self, world: &mut crate::ecs::world::World, context: &mut SystemContext<'_>) {
        // 1. Monitor the event bus for new transition requests.
        for event in world.event_bus.read::<EventStartTransition>() {
            println!("[SystemTransition] Event received: {:?}. Duration: {}", event.transition_type, event.duration);
            self.state = TransitionState::Playing(event.transition_type);
            self.duration = event.duration;
            self.timer = 0.0;
            self.center = event.center;
            world.transition_finished = false;
        }

        // 2. Advance progress if a transition is currently playing.
        if let TransitionState::Playing(transition_type) = self.state {
            self.timer += context.delta_time;
            
            if self.timer >= self.duration {
                // 3. Handle transition completion: publish fact and update world flag.
                println!("[SystemTransition] Transition Finished!");
                world.event_bus.publish(EventTransitionComplete { transition_type });
                world.transition_finished = true;
                
                // 4. Enter 'Hold' state if the shutter is closed (IrisOut) to keep the screen black.
                if transition_type == TransitionType::IrisOut {
                    self.state = TransitionState::Hold(TransitionType::IrisOut);
                } else {
                    self.state = TransitionState::Idle;
                }
            }
        }
    }

    /// Renders the black shutter rectangles based on the current transition progress.
    ///
    /// ⚠️ **Hotpath**: Called every frame.
    pub fn draw(&self, renderer: &mut Renderer, _context: &RenderContext<'_>) -> Result<(), String> {
        let (screen_w, screen_h) = renderer.output_size();

        // 1. Handle the static black screen for 'Hold' state.
        if let TransitionState::Hold(TransitionType::IrisOut) = self.state {
            renderer.fill_rect(
                &sdl3::rect::Rect::new(0, 0, screen_w, screen_h),
                Color::RGB(0, 0, 0)
            )?;
            return Ok(());
        }

        // 2. Calculate and render the dynamic shutter geometry.
        if let TransitionState::Playing(transition_type) = self.state {
            let progress = (self.timer / self.duration).clamp(0.0, 1.0);
            
            let (cx_int, cy_int) = self.center.unwrap_or(((screen_w / 2) as i32, (screen_h / 2) as i32));
            let cx = cx_int as f32;
            let cy = cy_int as f32;
            let sw = screen_w as f32;
            let sh = screen_h as f32;

            // 3. Calculate current edge positions for the four shutter doors (top, bottom, left, right).
            let (curr_top_h, curr_bottom_y, curr_left_w, curr_right_x) = match transition_type {
                TransitionType::IrisOut => {
                    // Shutters growing from edges toward the center.
                    (
                        cy * progress,              // Top height
                        sh - (sh - cy) * progress,  // Bottom Y
                        cx * progress,              // Left width
                        sw - (sw - cx) * progress   // Right X
                    )
                },
                TransitionType::IrisIn => {
                    // Shutters shrinking from center toward the edges.
                    (
                        cy * (1.0 - progress),              // Top height
                        cy + (sh - cy) * progress,          // Bottom Y
                        cx * (1.0 - progress),              // Left width
                        cx + (sw - cx) * progress           // Right X
                    )
                }
            };

            // 4. Render the 4 rectangular masks.
            // Top Door
            if curr_top_h > 0.0 {
                renderer.fill_rect(&sdl3::rect::Rect::new(0, 0, screen_w, curr_top_h.ceil() as u32), Color::RGB(0, 0, 0))?;
            }
            // Bottom Door
            if curr_bottom_y < sh {
                renderer.fill_rect(&sdl3::rect::Rect::new(0, curr_bottom_y.floor() as i32, screen_w, (sh - curr_bottom_y.floor()) as u32), Color::RGB(0, 0, 0))?;
            }
            // Left Door
            if curr_left_w > 0.0 {
                renderer.fill_rect(&sdl3::rect::Rect::new(0, 0, curr_left_w.ceil() as u32, screen_h), Color::RGB(0, 0, 0))?;
            }
            // Right Door
            if curr_right_x < sw {
                renderer.fill_rect(&sdl3::rect::Rect::new(curr_right_x.floor() as i32, 0, (sw - curr_right_x.floor()) as u32, screen_h), Color::RGB(0, 0, 0))?;
            }
        }
        Ok(())
    }
}

impl Default for SystemTransition {
    fn default() -> Self {
        Self::new()
    }
}
