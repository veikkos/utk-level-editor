use crate::Level;
use crate::TextureType;
use sdl2::render::Canvas;
use sdl2::render::Texture;
use sdl2::render::TextureCreator;
use sdl2::ttf::Font;
use sdl2::video::Window;
use sdl2::video::WindowContext;
use sdl2::Sdl;

pub struct Textures<'a> {
    pub floor: Texture<'a>,
    pub walls: Texture<'a>,
    pub shadows: Texture<'a>,
}

pub struct Context<'a> {
    pub sdl: Sdl,
    pub canvas: Canvas<Window>,
    pub texture_creator: &'a TextureCreator<WindowContext>,
    pub font: Font<'a, 'a>,
    pub textures: Textures<'a>,
    pub level: Level,
    pub selected_tile_id: u32,
    pub texture_type_selected: TextureType,
    pub mouse: (u32, u32),
}