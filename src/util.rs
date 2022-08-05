use crate::types::Trigonometry;
use sdl2::rect::Rect;
use sdl2::render::{Texture, TextureQuery};
use std::cmp;

pub const TILE_SIZE: u32 = 20;
pub const RENDER_MULTIPLIER: u32 = 2;
pub const RENDER_SIZE: u32 = TILE_SIZE * RENDER_MULTIPLIER;
pub const RESOLUTION_X: u32 = 640;
pub const RESOLUTION_Y: u32 = 480;
pub const TILES_X_PER_SCREEN: u32 = RESOLUTION_X / RENDER_SIZE;
pub const TILES_Y_PER_SCREEN: u32 = RESOLUTION_Y / RENDER_SIZE;
pub const TITLE_POSITION: (u32, u32) = (20, 10);
pub const BOTTOM_TEXT_POSITION: (u32, u32) = (TITLE_POSITION.0, RESOLUTION_Y - 26);

pub fn get_tile_coordinates(id: u32, width: u32) -> (u32, u32) {
    let x = id * TILE_SIZE % width;
    let y = id * TILE_SIZE / width * TILE_SIZE;
    (x, y)
}

pub fn get_block(id: u32, width: u32) -> Rect {
    let (x, y) = get_tile_coordinates(id, width);
    Rect::new(x as i32, y as i32, TILE_SIZE, TILE_SIZE)
}

pub fn get_logical_coordinates(x: u32, y: u32, scroll: Option<(u32, u32)>) -> (u32, u32) {
    let scroll = scroll.unwrap_or((0, 0));
    (
        x / RENDER_MULTIPLIER / TILE_SIZE + scroll.0,
        y / RENDER_MULTIPLIER / TILE_SIZE + scroll.1,
    )
}

pub fn get_tile_id_from_coordinates(
    coordinates: &(u32, u32),
    x_blocks: u32,
    scroll: Option<(u32, u32)>,
) -> u32 {
    let (x_logical, y_logical) = get_logical_coordinates(coordinates.0, coordinates.1, scroll);
    x_logical + y_logical * x_blocks
}

pub fn get_scroll_corrected_indexes(
    scroll: (u32, u32),
    x_index: u32,
    y_index: u32,
) -> (usize, usize) {
    ((x_index + scroll.0) as usize, (y_index + scroll.1) as usize)
}

pub fn get_absolute_coordinates_from_logical(x: u32, y: u32) -> (i32, i32) {
    (
        (x * RENDER_SIZE).try_into().unwrap(),
        (y * RENDER_SIZE).try_into().unwrap(),
    )
}

pub fn get_level_coordinates_from_screen_coordinates(
    coordinates: &(u32, u32),
    scroll: &(u32, u32),
) -> (u32, u32) {
    (
        coordinates.0 / RENDER_MULTIPLIER + scroll.0 * TILE_SIZE,
        coordinates.1 / RENDER_MULTIPLIER + scroll.1 * TILE_SIZE,
    )
}

pub fn get_screen_coordinates_from_level_coordinates(
    coordinates: &(u32, u32),
    scroll: &(u32, u32),
) -> (i32, i32) {
    (
        (coordinates.0 * RENDER_MULTIPLIER) as i32 - (scroll.0 * RENDER_SIZE) as i32,
        (coordinates.1 * RENDER_MULTIPLIER) as i32 - (scroll.1 * RENDER_SIZE) as i32,
    )
}

pub fn get_distance_between_points(p0: &(u32, u32), p1: &(u32, u32)) -> f64 {
    let x0 = p0.0 as i32;
    let x1 = p1.0 as i32;
    let y0 = p0.1 as i32;
    let y1 = p1.1 as i32;
    (((x1 - x0) * (x1 - x0) + (y1 - y0) * (y1 - y0)) as f64).sqrt()
}

pub fn get_spotlight_render_radius(spotlight: &u8) -> u32 {
    *spotlight as u32 * 5 + 5
}

pub fn get_steam_render_radius() -> u32 {
    5
}

pub fn get_crate_render_size() -> u32 {
    28
}

pub fn check_box_click(
    point_position: &(u32, u32),
    box_position: &(u32, u32),
    box_size: u32,
) -> bool {
    point_position.0 >= box_position.0
        && point_position.0 < box_position.0 + box_size
        && point_position.1 >= box_position.1
        && point_position.1 < box_position.1 + box_size
}

pub fn get_selected_level_tiles(
    p0: &(u32, u32),
    p1: &(u32, u32),
    x_blocks: u32,
    scroll: Option<(u32, u32)>,
) -> Vec<u32> {
    let tile_ids = (
        get_tile_id_from_coordinates(
            &(cmp::min(p0.0, p1.0), cmp::min(p0.1, p1.1)),
            x_blocks,
            scroll,
        ),
        get_tile_id_from_coordinates(
            &(cmp::max(p0.0, p1.0), cmp::max(p0.1, p1.1)),
            x_blocks,
            scroll,
        ),
    );
    let x_diff = (tile_ids.1 - tile_ids.0) % x_blocks + 1;
    let y_diff = (tile_ids.1 - tile_ids.0) / x_blocks + 1;
    let mut lines: Vec<u32> = Vec::new();
    for y in 0..y_diff {
        let mut line: Vec<u32> =
            (tile_ids.0 + y * x_blocks..tile_ids.0 + x_diff + y * x_blocks).collect();
        lines.append(&mut line);
    }
    lines
}

pub fn limit_screen_coordinates_to_window(coordinates: &(u32, u32)) -> (u32, u32) {
    limit_coordinates(coordinates, &(RESOLUTION_X, RESOLUTION_Y))
}

pub fn limit_coordinates(coordinates: &(u32, u32), limit: &(u32, u32)) -> (u32, u32) {
    (
        std::cmp::min(coordinates.0, limit.0 - 1),
        std::cmp::min(coordinates.1, limit.1 - 1),
    )
}

pub fn get_number_of_tiles_in_texture(texture: &Texture) -> u32 {
    let TextureQuery { width, height, .. } = texture.query();
    width / TILE_SIZE * height / TILE_SIZE
}

impl Trigonometry {
    pub fn new() -> Self {
        Trigonometry {
            sin: (0..360)
                .map(|x| ((x as f32).to_radians()).sin())
                .collect::<Vec<f32>>()
                .try_into()
                .unwrap(),
            cos: (0..360)
                .map(|x| ((x as f32).to_radians()).cos())
                .collect::<Vec<f32>>()
                .try_into()
                .unwrap(),
        }
    }
}
