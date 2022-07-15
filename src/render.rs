use crate::types::*;
use crate::util::*;
use crate::Level;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::render::Texture;
use sdl2::render::TextureCreator;
use sdl2::render::TextureQuery;
use sdl2::ttf::Font;
use sdl2::video::Window;
use sdl2::video::WindowContext;

const TEXT_SIZE_DIVIDER: u32 = 4;

pub fn highlight_selected_tile(canvas: &mut Canvas<Window>, id: u32) {
    canvas.set_draw_color(Color::from((255, 255, 255)));

    let (x_logical, y_logical) = get_tile_coordinates(id);
    let x = x_logical * RENDER_MULTIPLIER;
    let y = y_logical * RENDER_MULTIPLIER;

    draw_line(canvas, x, y, x, y + RENDER_SIZE - 1);
    draw_line(canvas, x, y, x + RENDER_SIZE - 1, y);
    draw_line(
        canvas,
        x + RENDER_SIZE - 1,
        y,
        x + RENDER_SIZE - 1,
        y + RENDER_SIZE - 1,
    );
    draw_line(
        canvas,
        x,
        y + RENDER_SIZE - 1,
        x + RENDER_SIZE - 1,
        y + RENDER_SIZE - 1,
    );
}

pub fn draw_line(canvas: &mut Canvas<Window>, x0: u32, y0: u32, x1: u32, y1: u32) {
    let x0_signed = x0 as i32;
    let y0_signed = y0 as i32;
    let x1_signed = x1 as i32;
    let y1_signed = y1 as i32;

    canvas
        .draw_line(
            Point::from((x0_signed, y0_signed)),
            Point::from((x1_signed, y1_signed)),
        )
        .unwrap();
}

pub fn get_font_texture<'a>(
    texture_creator: &'a TextureCreator<WindowContext>,
    font: &Font,
    text: &str,
) -> Texture<'a> {
    let surface = font
        .render(text)
        .blended(Color::RGBA(255, 0, 0, 255))
        .map_err(|e| e.to_string())
        .unwrap();
    texture_creator
        .create_texture_from_surface(&surface)
        .map_err(|e| e.to_string())
        .unwrap()
}

pub fn render_text_texture(canvas: &mut Canvas<Window>, texture: &Texture, x: u32, y: u32) {
    let TextureQuery { width, height, .. } = texture.query();
    let dst = Rect::new(
        x as i32,
        y as i32,
        width / TEXT_SIZE_DIVIDER,
        height / TEXT_SIZE_DIVIDER,
    );
    canvas.copy(&texture, None, dst).unwrap();
}

pub fn render_level(
    canvas: &mut Canvas<Window>,
    level: &Level,
    texture_floor: &Texture,
    texture_walls: &Texture,
) {
    for y in 0..level.tiles.len() {
        for x in 0..level.tiles[0].len() {
            let src = get_block(level.tiles[y][x].id);
            let dst = Rect::new(
                (x * RENDER_SIZE as usize).try_into().unwrap(),
                (y * RENDER_SIZE as usize).try_into().unwrap(),
                RENDER_SIZE,
                RENDER_SIZE,
            );
            let texture = match level.tiles[y][x].texture_type {
                TextureType::FLOOR => texture_floor,
                TextureType::WALLS => texture_walls,
            };
            canvas.copy(&texture, src, dst).unwrap();
        }
    }
}
