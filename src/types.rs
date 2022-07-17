#[derive(Clone, Copy, PartialEq)]
pub enum TextureType {
    FLOOR = 0,
    WALLS,
    SHADOW,
}

#[derive(Clone, Copy)]
pub struct Tile {
    pub(crate) texture_type: TextureType,
    pub(crate) id: u32,
    pub(crate) shadow: u32,
}

pub type Tiles = [[Tile; 16]; 12];

pub enum NextMode {
    Editor,
    TileSelect,
    Help,
    Quit,
}
