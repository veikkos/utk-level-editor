use sdl2::rect::Rect;

pub const TILE_SIZE: u32 = 20;
pub const RENDER_MULTIPLIER: u32 = 2;
pub const RENDER_SIZE: u32 = TILE_SIZE * RENDER_MULTIPLIER;
pub const RESOLUTION_X: u32 = 640;
pub const RESOLUTION_Y: u32 = 480;
pub const TILES_X_PER_SCREEN: u32 = RESOLUTION_X / RENDER_SIZE;
pub const TILES_Y_PER_SCREEN: u32 = RESOLUTION_Y / RENDER_SIZE;

pub fn get_tile_coordinates(id: u32) -> (u32, u32) {
    let x = id * TILE_SIZE % 320;
    let y = id * TILE_SIZE / 320 * TILE_SIZE;
    (x, y)
}

pub fn get_block(id: u32) -> Rect {
    let (x, y) = get_tile_coordinates(id);
    Rect::new(x as i32, y as i32, TILE_SIZE, TILE_SIZE)
}

pub fn get_logical_coordinates(x: u32, y: u32, scroll: Option<(u32, u32)>) -> (u32, u32) {
    let scroll = scroll.unwrap_or((0, 0));
    (
        x / RENDER_MULTIPLIER / TILE_SIZE + scroll.0,
        y / RENDER_MULTIPLIER / TILE_SIZE + scroll.1,
    )
}

pub fn get_tile_id_from_coordinate(
    x: u32,
    y: u32,
    x_blocks: u32,
    scroll: Option<(u32, u32)>,
) -> u32 {
    let (x_logical, y_logical) = get_logical_coordinates(x, y, scroll);
    x_logical + y_logical * x_blocks
}

pub fn get_scroll_corrected_indexes(
    scroll: (u32, u32),
    x_index: u32,
    y_index: u32,
) -> (usize, usize) {
    ((x_index + scroll.0) as usize, (y_index + scroll.1) as usize)
}
