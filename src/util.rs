use sdl2::rect::Rect;

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

pub fn init_empty_level(level: &mut [[Tile; 16]; 12]) {
    for x in 0..level[0].len() {
        level[0][x] = if x == 0 {
            Tile {
                texture_type: TextureType::WALLS,
                id: 0,
            }
        } else if x == level[0].len() - 1 {
            Tile {
                texture_type: TextureType::WALLS,
                id: 2,
            }
        } else {
            Tile {
                texture_type: TextureType::WALLS,
                id: 1,
            }
        }
    }
    for y in 1..(level.len() - 1) {
        for x in 0..level[0].len() {
            level[y][x] = if x == 0 {
                Tile {
                    texture_type: TextureType::WALLS,
                    id: 16,
                }
            } else if x == level[0].len() - 1 {
                Tile {
                    texture_type: TextureType::WALLS,
                    id: 16,
                }
            } else {
                Tile {
                    texture_type: TextureType::FLOOR,
                    id: 0,
                }
            }
        }
    }
    for x in 0..level[0].len() {
        level[level.len() - 1][x] = if x == 0 {
            Tile {
                texture_type: TextureType::WALLS,
                id: 32,
            }
        } else if x == level[0].len() - 1 {
            Tile {
                texture_type: TextureType::WALLS,
                id: 18,
            }
        } else {
            Tile {
                texture_type: TextureType::WALLS,
                id: 1,
            }
        }
    }
}
