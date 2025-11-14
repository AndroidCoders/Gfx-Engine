use image::{Rgba, RgbaImage};
use rusttype::{point, Font, Scale};

fn main() {
    // Create a new image with a black background
    let mut image = RgbaImage::new(800, 600);
    for pixel in image.pixels_mut() {
        *pixel = Rgba([0, 0, 0, 255]);
    }

    // Load a font
    let font_data = include_bytes!("../assets/fonts/LiberationMono-Regular.ttf");
    let font = Font::try_from_bytes(font_data).expect("Error constructing Font");

    // Set the text properties
    let scale = Scale::uniform(64.0);
    let color = Rgba([255, 255, 255, 255]);
    let text = "Game Over";

    // Get the text size
    let (text_width, text_height) = {
        let v_metrics = font.v_metrics(scale);
        let glyphs: Vec<_> = font.layout(text, scale, point(0.0, 0.0)).collect();
        let width = glyphs
            .iter()
            .rev()
            .map(|g| g.position().x as f32 + g.unpositioned().h_metrics().advance_width)
            .next()
            .unwrap_or(0.0)
            .ceil() as u32;
        let height = (v_metrics.ascent - v_metrics.descent).ceil() as u32;
        (width, height)
    };

    // Calculate the position to center the text
    let x = (image.width() - text_width) / 2;
    let y = (image.height() - text_height) / 2;

    // Draw the glyphs onto the image
    let glyphs: Vec<_> = font.layout(text, scale, point(x as f32, y as f32)).collect();
    for g in glyphs {
        if let Some(bb) = g.pixel_bounding_box() {
            g.draw(|gx, gy, gv| {
                let gx = gx as i32 + bb.min.x;
                let gy = gy as i32 + bb.min.y;

                let image_x = gx as u32;
                let image_y = gy as u32;

                if image_x < image.width() && image_y < image.height() {
                    let pixel = image.get_pixel_mut(image_x, image_y);
                    let new_pixel = Rgba([
                        (pixel[0] as f32 * (1.0 - gv) + color[0] as f32 * gv) as u8,
                        (pixel[1] as f32 * (1.0 - gv) + color[1] as f32 * gv) as u8,
                        (pixel[2] as f32 * (1.0 - gv) + color[2] as f32 * gv) as u8,
                        255,
                    ]);
                    *pixel = new_pixel;
                }
            });
        }
    }

    // Save the image
    image.save("assets/graphics/game_over.png").unwrap();
}