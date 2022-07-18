use crate::types::*;
use crate::util::*;
use byteorder::{LittleEndian, ReadBytesExt};
use std::{fs::File, io::Write};

const DIFF_BULLETS: u32 = 9;
const DIFF_WEAPONS: u32 = 11;
const DIFF_ENEMIES: u32 = 8;

const VERSION: u32 = 5;

pub struct Level {
    pub tiles: Tiles,
    pub p1_position: (u32, u32),
    pub p2_position: (u32, u32),
    pub scroll: (u32, u32),
}

#[derive(Debug)]
pub enum FileTypeError {
    InvalidVersion,
    InvalidLevelSize,
}

#[derive(Debug)]
pub enum DeserializationError {
    IOError(std::io::Error),
    ContentError(FileTypeError),
}

impl From<std::io::Error> for DeserializationError {
    fn from(e: std::io::Error) -> Self {
        DeserializationError::IOError(e)
    }
}

impl From<FileTypeError> for DeserializationError {
    fn from(e: FileTypeError) -> Self {
        DeserializationError::ContentError(e)
    }
}

impl Level {
    pub fn get_default_level() -> Level {
        Level {
            tiles: Level::init_default_level(),
            p1_position: (1, 1),
            p2_position: (1, 3),
            scroll: (0, 0),
        }
    }

    fn init_default_level() -> Tiles {
        const LEVEL_SIZE_X: u8 = 32;
        const LEVEL_SIZE_Y: u8 = 22;

        let mut tiles = Vec::new();

        // First row ...
        {
            let mut row = Vec::new();
            for x in 0..LEVEL_SIZE_X {
                row.push(if x == 0 {
                    Tile {
                        texture_type: TextureType::WALLS,
                        id: 0,
                        shadow: 0,
                        spotlight: 0,
                    }
                } else if x == LEVEL_SIZE_X - 1 {
                    Tile {
                        texture_type: TextureType::WALLS,
                        id: 2,
                        shadow: 0,
                        spotlight: 0,
                    }
                } else {
                    Tile {
                        texture_type: TextureType::WALLS,
                        id: 1,
                        shadow: 0,
                        spotlight: 0,
                    }
                });
            }
            tiles.push(row);
        }

        // .. all but final row ...
        for y in 1..LEVEL_SIZE_Y - 1 {
            let mut row = Vec::new();

            for x in 0..LEVEL_SIZE_X {
                row.push(if x == 0 {
                    Tile {
                        texture_type: TextureType::WALLS,
                        id: 16,
                        shadow: 0,
                        spotlight: 0,
                    }
                } else if x == LEVEL_SIZE_X - 1 {
                    Tile {
                        texture_type: TextureType::WALLS,
                        id: 16,
                        shadow: 0,
                        spotlight: 0,
                    }
                } else {
                    Tile {
                        texture_type: TextureType::FLOOR,
                        id: 0,
                        shadow: if y == 1 || x == LEVEL_SIZE_X - 2 {
                            1
                        } else {
                            0
                        },
                        spotlight: 0,
                    }
                });
            }
            tiles.push(row);
        }

        // ... and final row!
        {
            let mut row = Vec::new();
            for x in 0..LEVEL_SIZE_X {
                row.push(if x == 0 {
                    Tile {
                        texture_type: TextureType::WALLS,
                        id: 32,
                        shadow: 0,
                        spotlight: 0,
                    }
                } else if x == LEVEL_SIZE_X - 1 {
                    Tile {
                        texture_type: TextureType::WALLS,
                        id: 18,
                        shadow: 0,
                        spotlight: 0,
                    }
                } else {
                    Tile {
                        texture_type: TextureType::WALLS,
                        id: 1,
                        shadow: 0,
                        spotlight: 0,
                    }
                });
            }
            tiles.push(row);
        }
        tiles
    }

    fn get_tile_index(&mut self, pointed_tile: u32) -> (usize, usize) {
        (
            pointed_tile as usize % self.tiles[0].len(),
            pointed_tile as usize / self.tiles[0].len(),
        )
    }
    pub fn put_tile_to_level(
        &mut self,
        pointed_tile: u32,
        selected_tile_id: Option<u32>,
        selected_texture: &TextureType,
    ) {
        let (x, y) = self.get_tile_index(pointed_tile);
        if *selected_texture != TextureType::SHADOW {
            self.tiles[y][x] = Tile {
                texture_type: *selected_texture,
                id: selected_tile_id.unwrap(),
                shadow: self.tiles[y][x].shadow,
                spotlight: self.tiles[y][x].spotlight,
            }
        } else {
            self.tiles[y][x].shadow = match selected_tile_id {
                Some(id) => id + 1,
                None => 0,
            };
        }
    }

    pub fn put_spotlight_to_level(&mut self, pointed_tile: u32, spotlight: u8) {
        let (x, y) = self.get_tile_index(pointed_tile);
        if spotlight <= 10 {
            self.tiles[y][x].spotlight = spotlight;
        }
    }

    pub fn get_spotlight_from_level(&mut self, pointed_tile: u32) -> u8 {
        let (x, y) = self.get_tile_index(pointed_tile);
        self.tiles[y][x].spotlight
    }

    pub fn serialize(&self, filename: &str) -> std::io::Result<()> {
        let mut file = File::create(filename)?;

        file.write_all(&VERSION.to_le_bytes())
            .expect("Failed to write version");
        file.write_all(&(self.tiles[0].len() as u32).to_le_bytes())
            .expect("Failed to write x size");
        file.write_all(&(self.tiles.len() as u32).to_le_bytes())
            .expect("Failed to write y size");
        for y in 0..(self.tiles.len()) {
            for x in 0..self.tiles[0].len() {
                file.write_all(&(self.tiles[y][x].texture_type as u32).to_le_bytes())
                    .expect("Failed to write block type");
                file.write_all(&self.tiles[y][x].id.to_le_bytes())
                    .expect("Failed to write block num");
                file.write_all(&self.tiles[y][x].shadow.to_le_bytes())
                    .expect("Failed to write block shadow");
            }
        }

        file.write_all(&(self.p1_position.0).to_le_bytes())
            .expect("Failed to write p1 start x");
        file.write_all(&(self.p1_position.1).to_le_bytes())
            .expect("Failed to write p1 start y");
        file.write_all(&(self.p2_position.0).to_le_bytes())
            .expect("Failed to write p2 start x");
        file.write_all(&(self.p2_position.1).to_le_bytes())
            .expect("Failed to write p2 start y");

        let mut spots = Vec::new();
        for y in 0..(self.tiles.len()) {
            for x in 0..self.tiles[0].len() {
                let spotlight = self.tiles[y][x].spotlight;
                if spotlight > 0 {
                    spots.push((
                        x as u32 * TILE_SIZE + TILE_SIZE / 2,
                        y as u32 * TILE_SIZE + TILE_SIZE / 2,
                        (spotlight - 1) as u32,
                    ));
                }
            }
        }

        file.write_all(&(spots.len() as u32).to_le_bytes())
            .expect("Failed to write spot amount");

        for spot in spots {
            file.write_all(&spot.0.to_le_bytes())
                .expect("Failed to write spotlight x position");
            file.write_all(&spot.1.to_le_bytes())
                .expect("Failed to write spotlight x position");
            file.write_all(&spot.2.to_le_bytes())
                .expect("Failed to write spotlight intensity");
        }

        file.write_all(&(0u32).to_le_bytes())
            .expect("Failed to write steam amount");

        // TODO: Write steams
        // for ( a = 0; a < Steam_amount; a ++  )
        // {
        //     fread( &steam[a].x, 4, 1, dat );
        //     fread( &steam[a].y, 4, 1, dat );
        //     fread( &steam[a].angle, 4, 1, dat );
        //     fread( &steam[a].speed, 4, 1, dat );
        // }

        let comment = "Rust UTK editor\0\0\0\0\0";
        file.write_all(&comment.as_bytes())
            .expect("Failed to write comment");
        file.write_all(&(45u32).to_le_bytes())
            .expect("Failed to write time limit");
        for x in 0..DIFF_ENEMIES {
            let amount = if x == 0 { 1u32 } else { 0u32 };
            file.write_all(&(amount).to_le_bytes())
                .expect("Failed to write normal game enemies");
        }
        for x in 0..DIFF_WEAPONS {
            let amount = if x == 0 { 1u32 } else { 0u32 };
            file.write_all(&(amount).to_le_bytes())
                .expect("Failed to write normal game weapons");
        }
        for x in 0..DIFF_BULLETS {
            let amount = if x == 0 { 1u32 } else { 0u32 };
            file.write_all(&(amount).to_le_bytes())
                .expect("Failed to write normal game bullets");
        }
        file.write_all(&(1u32).to_le_bytes())
            .expect("Failed to write normal game energy crates");
        for x in 0..DIFF_WEAPONS {
            let amount = if x == 0 { 1u32 } else { 0u32 };
            file.write_all(&(amount).to_le_bytes())
                .expect("Failed to write multiplayer game weapons");
        }
        for x in 0..DIFF_BULLETS {
            let amount = if x == 0 { 1u32 } else { 0u32 };
            file.write_all(&(amount).to_le_bytes())
                .expect("Failed to write multiplayer game bullets");
        }
        file.write_all(&(1u32).to_le_bytes())
            .expect("Failed to write multiplayer game energy crates");
        file.write_all(&(0u32).to_le_bytes())
            .expect("Failed to write normal game crate amount");
        // TODO: Write normal game crates
        // fread( normal_crate_info, sizeof(Crate_info) * normal_crate_amount, 1, dat );
        file.write_all(&(0u32).to_le_bytes())
            .expect("Failed to write deathmatch game crate amount");
        // TODO: Write deathmatch game crates
        // fread( deathmatch_crate_info, sizeof(Crate_info) * deathmatch_crate_amount, 1, dat );

        Ok(())
    }

    pub fn deserialize(&mut self, filename: &str) -> Result<(), DeserializationError> {
        self.scroll = (0, 0);

        let mut file = File::open(filename)?;
        let version: u32 = file.read_u32::<LittleEndian>()?;

        if version != VERSION {
            return Err(DeserializationError::ContentError(
                FileTypeError::InvalidVersion,
            ));
        }

        let x_size: u32 = file.read_u32::<LittleEndian>()?;
        if x_size < TILES_X_PER_SCREEN {
            return Err(DeserializationError::ContentError(
                FileTypeError::InvalidLevelSize,
            ));
        }

        let y_size: u32 = file.read_u32::<LittleEndian>()?;
        if y_size < TILES_Y_PER_SCREEN {
            return Err(DeserializationError::ContentError(
                FileTypeError::InvalidLevelSize,
            ));
        }

        let mut tiles = Vec::new();
        for _ in 0..y_size {
            let mut row = Vec::new();
            for _ in 0..x_size {
                row.push(Tile {
                    texture_type: TextureType::from_u32(file.read_u32::<LittleEndian>()?),
                    id: file.read_u32::<LittleEndian>()?,
                    shadow: file.read_u32::<LittleEndian>()?,
                    spotlight: 0,
                });
            }
            tiles.push(row);
        }
        self.tiles = tiles;

        self.p1_position.0 = file.read_u32::<LittleEndian>()?;
        self.p1_position.1 = file.read_u32::<LittleEndian>()?;
        self.p2_position.0 = file.read_u32::<LittleEndian>()?;
        self.p2_position.1 = file.read_u32::<LittleEndian>()?;
        // file.write_all(&(0u32).to_le_bytes())
        //     .expect("Failed to write spot amount");

        // // TODO: Write spots
        // // for ( a = 0; a < Spot_amount; a ++  )
        // // {
        // //     fread( &spot_light[a].x, 4, 1, dat );
        // //     fread( &spot_light[a].y, 4, 1, dat );
        // //     fread( &spot_light[a].size, 4, 1, dat );
        // // }

        // file.write_all(&(0u32).to_le_bytes())
        //     .expect("Failed to write steam amount");

        // // TODO: Write steams
        // // for ( a = 0; a < Steam_amount; a ++  )
        // // {
        // //     fread( &steam[a].x, 4, 1, dat );
        // //     fread( &steam[a].y, 4, 1, dat );
        // //     fread( &steam[a].angle, 4, 1, dat );
        // //     fread( &steam[a].speed, 4, 1, dat );
        // // }

        // let comment = "Rust UTK editor\0\0\0\0\0";
        // file.write_all(&comment.as_bytes())
        //     .expect("Failed to write comment");
        // file.write_all(&(45u32).to_le_bytes())
        //     .expect("Failed to write time limit");
        // for x in 0..DIFF_ENEMIES {
        //     let amount = if x == 0 { 1u32 } else { 0u32 };
        //     file.write_all(&(amount).to_le_bytes())
        //         .expect("Failed to write normal game enemies");
        // }
        // for x in 0..DIFF_WEAPONS {
        //     let amount = if x == 0 { 1u32 } else { 0u32 };
        //     file.write_all(&(amount).to_le_bytes())
        //         .expect("Failed to write normal game weapons");
        // }
        // for x in 0..DIFF_BULLETS {
        //     let amount = if x == 0 { 1u32 } else { 0u32 };
        //     file.write_all(&(amount).to_le_bytes())
        //         .expect("Failed to write normal game bullets");
        // }
        // file.write_all(&(1u32).to_le_bytes())
        //     .expect("Failed to write normal game energy crates");
        // for x in 0..DIFF_WEAPONS {
        //     let amount = if x == 0 { 1u32 } else { 0u32 };
        //     file.write_all(&(amount).to_le_bytes())
        //         .expect("Failed to write multiplayer game weapons");
        // }
        // for x in 0..DIFF_BULLETS {
        //     let amount = if x == 0 { 1u32 } else { 0u32 };
        //     file.write_all(&(amount).to_le_bytes())
        //         .expect("Failed to write multiplayer game bullets");
        // }
        // file.write_all(&(1u32).to_le_bytes())
        //     .expect("Failed to write multiplayer game energy crates");
        // file.write_all(&(0u32).to_le_bytes())
        //     .expect("Failed to write normal game crate amount");
        // // TODO: Write normal game crates
        // // fread( normal_crate_info, sizeof(Crate_info) * normal_crate_amount, 1, dat );
        // file.write_all(&(0u32).to_le_bytes())
        //     .expect("Failed to write deathmatch game crate amount");
        // // TODO: Write deathmatch game crates
        // // fread( deathmatch_crate_info, sizeof(Crate_info) * deathmatch_crate_amount, 1, dat );

        Ok(())
    }
}
