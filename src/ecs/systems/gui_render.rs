//! # Concept: Interface Visualization (HUD)
//! 
//! This module is responsible for rendering the persistent gameplay UI. 
//! It draws the Heads-Up Display (HUD) and meta-screens like Game Over, 
//! ensuring the player has clear feedback on their health and progress.

use crate::ecs::systems::RenderContext;
use crate::renderer::{Renderer, TextRenderParams};
use crate::texture_manager::TextureManager;
use crate::font_manager::FontManager;
use sdl3::rect::Rect;
use sdl3::pixels::Color;

/// A system that renders hearts, coin counters, and interface overlays.
pub struct SystemGUIRender;

impl SystemGUIRender {

    /// Renders the persistent gameplay HUD elements to the screen.

    ///

    /// ⚠️ **Hotpath**: Called every frame at monitor refresh rate.

    pub fn update(

        &mut self,

        renderer: &mut Renderer,

        texture_manager: &TextureManager,

        font_manager: &FontManager,

        world: &crate::ecs::world::World,

        context: &RenderContext<'_>,

    ) -> Result<(), String> {

        

        // 1. Render Player Health (Hearts).

        if let Some(heart_texture) = texture_manager.get("heart") {

            let heart_size = 64;

            let padding = 10;

            let start_pos = context.game_config.ui.hearts_pos;

            

            for i in 0..world.ui_state.display_lives {

                let dest_rect = Rect::new(

                    start_pos.x + (i as i32 * (heart_size + padding)),

                    start_pos.y,

                    heart_size as u32,

                    heart_size as u32,

                );

                renderer.copy(heart_texture, None, Some(dest_rect))?;

            }

        }



        // 2. Render Gold Coin Counter.

        let coin_pos = context.game_config.ui.coins_pos;

        

        // Reuse the visual definition from the gold coin prefab for consistency.

        if let Some(anim_config) = context.game_config.animation.get("gold_coin_spin")

            && let Some(texture) = texture_manager.get(&anim_config.texture)

        {

            // Draw the representative icon for the coin.

            let src_rect = Rect::new(

                anim_config.start_x,

                anim_config.start_y,

                anim_config.frame_width,

                anim_config.frame_height,

            );

                

                let icon_scale = crate::config::RENDER_SCALE_FACTOR; 

                let dest_w = (anim_config.frame_width as f32 * icon_scale) as u32;

                let dest_h = (anim_config.frame_height as f32 * icon_scale) as u32;

                

                let dest_rect = Rect::new(

                    coin_pos.x,

                    coin_pos.y,

                    dest_w,

                    dest_h,

                );

                

                renderer.copy(texture, Some(src_rect), Some(dest_rect))?;



                // Draw the count text with a simple shadow for legibility against bright backgrounds.

                let text = format!("x {:02}", world.ui_state.display_coin_count);

                

                // Drop Shadow

                renderer.render_text(font_manager, TextRenderParams {

                    text: &text,

                    x: coin_pos.x + dest_w as i32 + 10 + 2,

                    y: coin_pos.y + (dest_h as i32 / 2) - 8 + 2,

                    font_size: 32.0,

                    scale: 1.0,

                    color: Color::RGB(0, 0, 0),

                })?;



                // Foreground Text

                renderer.render_text(font_manager, TextRenderParams {

                    text: &text,

                    x: coin_pos.x + dest_w as i32 + 10,

                    y: coin_pos.y + (dest_h as i32 / 2) - 8,

                    font_size: 32.0,

                    scale: 1.0,

                    color: Color::RGB(255, 255, 255),

                })?;

            }



        Ok(())

    }



    /// Renders the full-screen Game Over overlay sequence.

    ///

    /// ⚠️ **Hotpath**: Called every frame when in Game Over state.

    pub fn render_game_over(

        &mut self,

        renderer: &mut Renderer,

        texture_manager: &TextureManager,

        context: &RenderContext<'_>,

    ) -> Result<(), String> {

        // 1. Draw a semi-transparent black scrim to dim the game world.

        let (w, h) = renderer.output_size();

        let screen_rect = Rect::new(0, 0, w, h);

        renderer.fill_rect(&screen_rect, sdl3::pixels::Color::RGBA(0, 0, 0, 150))?;



        // 2. Center and render the primary 'Game Over' graphic.

        let game_over_rect = Rect::new(

            (w as i32 - 1920) / 2,

            (h as i32 - 1080) / 2,

            1920,

            1080,

        );

        

        if let Some(texture) = texture_manager.get(&context.game_config.gameplay.game_over_texture) {

             renderer.copy(texture, None, Some(game_over_rect))?;

        }

       

        Ok(())

    }

}
