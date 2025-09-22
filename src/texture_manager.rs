use sdl3::render::{Texture, TextureCreator};
use sdl3::video::WindowContext;

pub struct TextureManager<'a> {
    #[allow(dead_code)]
    texture_creator: TextureCreator<WindowContext>,
    virtual_canvas_texture: Option<Texture<'a>>,
}

impl<'a> TextureManager<'a> {
    pub fn new(texture_creator: TextureCreator<WindowContext>, virtual_width: u32, virtual_height: u32) -> Result<Self, String> {
        // This is an unsafe workaround for the self-referential struct problem.
        // The `Texture` type in `sdl3` borrows from `TextureCreator`.
        // If `TextureManager` owns both, it creates a self-referential struct
        // that Rust's borrow checker cannot safely verify.
        // We transmute the lifetime of `texture_creator` to `'static` to allow
        // `virtual_canvas_texture` to borrow from it for the lifetime of `TextureManager`.
        // This is safe because `texture_creator` is owned by `TextureManager` and will
        // be dropped when `TextureManager` is dropped.
        let static_texture_creator: &'static TextureCreator<WindowContext> = unsafe {
            std::mem::transmute(&texture_creator)
        };

        let virtual_canvas_texture = static_texture_creator
            .create_texture_target(None, virtual_width, virtual_height)
            .map_err(|e| e.to_string())?;

        Ok(Self {
            texture_creator,
            virtual_canvas_texture: Some(virtual_canvas_texture),
        })
    }

    pub fn virtual_canvas_texture(&self) -> &Texture<'a> {
        self.virtual_canvas_texture.as_ref().expect("Virtual canvas texture should be initialized")
    }

    pub fn virtual_canvas_texture_mut(&mut self) -> &mut Texture<'a> {
        self.virtual_canvas_texture.as_mut().expect("Virtual canvas texture should be initialized")
    }
}
