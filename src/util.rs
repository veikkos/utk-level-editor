use sdl2::rect::Rect;

pub const TILE_SIZE: u32 = 20;
pub const RENDER_MULTIPLIER: u32 = 2;
pub const RENDER_SIZE: u32 = TILE_SIZE * RENDER_MULTIPLIER;

pub fn get_tile_coordinates(id: u32) -> (u32, u32) {
    let x = id * TILE_SIZE % 320;
    let y = id * TILE_SIZE / 320 * TILE_SIZE;
    (x, y)
}

pub fn get_block(id: u32) -> Rect {
    let (x, y) = get_tile_coordinates(id);
    Rect::new(x as i32, y as i32, TILE_SIZE, TILE_SIZE)
}

pub fn get_tile_id_from_coordinate(x: u32, y: u32) -> u32 {
    let x_logical = x / RENDER_MULTIPLIER;
    let y_logical = y / RENDER_MULTIPLIER;
    x_logical / TILE_SIZE + y_logical / TILE_SIZE * 16
}
