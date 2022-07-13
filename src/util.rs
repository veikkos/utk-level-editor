use sdl2::{
    pixels::Color,
    rect::{Point, Rect},
    render::Canvas,
    video,
};

pub const TILE_SIZE: u32 = 20;
pub const RENDER_MULTIPLIER: u32 = 2;
pub const RENDER_SIZE: u32 = TILE_SIZE * RENDER_MULTIPLIER;

#[derive(Clone, Copy, PartialEq)]
pub enum TextureType {
    FLOOR = 0,
    WALLS,
}

#[derive(Clone, Copy)]
pub struct Tile {
    pub(crate) texture_type: TextureType,
    pub(crate) id: u32,
}

pub fn get_tile_coordinates(id: u32) -> (u32, u32) {
    let x = id * TILE_SIZE % 320;
    let y = id * TILE_SIZE / 320 * TILE_SIZE;
    (x, y)
}

pub fn get_block(id: u32) -> Rect {
    let (x, y) = get_tile_coordinates(id);
    Rect::new(x as i32, y as i32, TILE_SIZE, TILE_SIZE)
}

pub fn highlight_selected_tile(id: u32, canvas: &mut Canvas<video::Window>) {
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

pub fn draw_line(canvas: &mut Canvas<video::Window>, x0: u32, y0: u32, x1: u32, y1: u32) {
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

pub fn get_tile_id_from_coordinate(x: u32, y: u32) -> u32 {
    let x_logical = x / RENDER_MULTIPLIER;
    let y_logical = y / RENDER_MULTIPLIER;
    x_logical / TILE_SIZE + y_logical / TILE_SIZE * 16
}

pub fn put_tile_to_level(
    pointed_tile: u32,
    level: &mut [[Tile; 16]; 12],
    selected_tile_id: u32,
    selected_texture: &TextureType,
) {
    let x = pointed_tile as usize % level[0].len();
    let y = pointed_tile as usize / level[0].len();
    level[y][x] = Tile {
        texture_type: *selected_texture,
        id: selected_tile_id,
    };
}
