use crate::types::Trigonometry;
use crate::Graphics;
use sdl2::rect::Rect;
use sdl2::render::{Texture, TextureQuery};
use std::cmp;

pub const TITLE_POSITION: (u32, u32) = (20, 10);

pub fn get_bottom_text_position(resolution_y: u32) -> (u32, u32) {
    (TITLE_POSITION.0, resolution_y - 26)
}

pub fn get_tile_coordinates(id: u32, width: u32, tile_size: u32) -> (u32, u32) {
    let x = id * tile_size % width;
    let y = id * tile_size / width * tile_size;
    (x, y)
}

pub fn get_block(id: u32, width: u32, tile_size: u32) -> Rect {
    let (x, y) = get_tile_coordinates(id, width, tile_size);
    Rect::new(x as i32, y as i32, tile_size, tile_size)
}

pub fn get_logical_coordinates(
    graphics: &Graphics,
    x: u32,
    y: u32,
    scroll: Option<(u32, u32)>,
) -> (u32, u32) {
    let render_multiplier = graphics.render_multiplier;
    let tile_size = graphics.tile_size;
    let scroll = scroll.unwrap_or((0, 0));
    (
        x / render_multiplier / tile_size + scroll.0,
        y / render_multiplier / tile_size + scroll.1,
    )
}

pub fn get_tile_id_from_coordinates(
    graphics: &Graphics,
    coordinates: &(u32, u32),
    x_blocks: u32,
    scroll: Option<(u32, u32)>,
) -> u32 {
    let (x_logical, y_logical) =
        get_logical_coordinates(graphics, coordinates.0, coordinates.1, scroll);
    x_logical + y_logical * x_blocks
}

pub fn get_scroll_corrected_indexes(
    scroll: (u32, u32),
    x_index: u32,
    y_index: u32,
) -> (usize, usize) {
    ((x_index + scroll.0) as usize, (y_index + scroll.1) as usize)
}

pub fn get_absolute_coordinates_from_logical(x: u32, y: u32, render_size: u32) -> (i32, i32) {
    (
        (x * render_size).try_into().unwrap(),
        (y * render_size).try_into().unwrap(),
    )
}

pub fn get_level_coordinates_from_screen_coordinates(
    graphics: &Graphics,
    coordinates: &(u32, u32),
    scroll: &(u32, u32),
) -> (u32, u32) {
    let render_multiplier = graphics.render_multiplier;
    let tile_size = graphics.tile_size;
    (
        coordinates.0 / render_multiplier + scroll.0 * tile_size,
        coordinates.1 / render_multiplier + scroll.1 * tile_size,
    )
}

pub fn get_screen_coordinates_from_level_coordinates(
    graphics: &Graphics,
    coordinates: &(u32, u32),
    scroll: &(u32, u32),
) -> (i32, i32) {
    let render_multiplier = graphics.render_multiplier;
    let render_size = graphics.get_render_size();
    (
        (coordinates.0 * render_multiplier) as i32 - (scroll.0 * render_size) as i32,
        (coordinates.1 * render_multiplier) as i32 - (scroll.1 * render_size) as i32,
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
    graphics: &Graphics,
    p0: &(u32, u32),
    p1: &(u32, u32),
    x_blocks: u32,
    scroll: Option<(u32, u32)>,
) -> Vec<u32> {
    let tile_ids = (
        get_tile_id_from_coordinates(
            graphics,
            &(cmp::min(p0.0, p1.0), cmp::min(p0.1, p1.1)),
            x_blocks,
            scroll,
        ),
        get_tile_id_from_coordinates(
            graphics,
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

pub fn limit_coordinates(coordinates: &(u32, u32), limit: &(u32, u32)) -> (u32, u32) {
    (
        std::cmp::min(coordinates.0, limit.0 - 1),
        std::cmp::min(coordinates.1, limit.1 - 1),
    )
}

pub fn get_number_of_tiles_in_texture(texture: &Texture, tile_size: u32) -> u32 {
    let TextureQuery { width, height, .. } = texture.query();
    width / tile_size * height / tile_size
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
