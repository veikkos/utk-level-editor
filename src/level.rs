use crate::types::*;
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
        let mut level = Level {
            tiles: [[Tile {
                texture_type: TextureType::FLOOR,
                id: 0,
                shadow: 0,
            }; 16]; 12],
            p1_position: (1u32, 1u32),
            p2_position: (1u32, 3u32),
        };
        level.init_default_level();
        level
    }

    pub fn init_default_level(&mut self) {
        for x in 0..self.tiles[0].len() {
            self.tiles[0][x] = if x == 0 {
                Tile {
                    texture_type: TextureType::WALLS,
                    id: 0,
                    shadow: 0,
                }
            } else if x == self.tiles[0].len() - 1 {
                Tile {
                    texture_type: TextureType::WALLS,
                    id: 2,
                    shadow: 0,
                }
            } else {
                Tile {
                    texture_type: TextureType::WALLS,
                    id: 1,
                    shadow: 0,
                }
            }
        }
        for y in 1..(self.tiles.len() - 1) {
            for x in 0..self.tiles[0].len() {
                self.tiles[y][x] = if x == 0 {
                    Tile {
                        texture_type: TextureType::WALLS,
                        id: 16,
                        shadow: 0,
                    }
                } else if x == self.tiles[0].len() - 1 {
                    Tile {
                        texture_type: TextureType::WALLS,
                        id: 16,
                        shadow: 0,
                    }
                } else {
                    Tile {
                        texture_type: TextureType::FLOOR,
                        id: 0,
                        shadow: if y == 1 || x == self.tiles[0].len() - 2 {
                            1
                        } else {
                            0
                        },
                    }
                }
            }
        }
        for x in 0..self.tiles[0].len() {
            self.tiles[self.tiles.len() - 1][x] = if x == 0 {
                Tile {
                    texture_type: TextureType::WALLS,
                    id: 32,
                    shadow: 0,
                }
            } else if x == self.tiles[0].len() - 1 {
                Tile {
                    texture_type: TextureType::WALLS,
                    id: 18,
                    shadow: 0,
                }
            } else {
                Tile {
                    texture_type: TextureType::WALLS,
                    id: 1,
                    shadow: 0,
                }
            }
        }
    }

    pub fn put_tile_to_level(
        &mut self,
        pointed_tile: u32,
        selected_tile_id: Option<u32>,
        selected_texture: &TextureType,
    ) {
        let x = pointed_tile as usize % self.tiles[0].len();
        let y = pointed_tile as usize / self.tiles[0].len();
        if *selected_texture != TextureType::SHADOW {
            self.tiles[y][x] = Tile {
                texture_type: *selected_texture,
                id: selected_tile_id.unwrap(),
                shadow: self.tiles[y][x].shadow,
            }
        } else {
            self.tiles[y][x].shadow = match selected_tile_id {
                Some(id) => id + 1,
                None => 0,
            };
        }
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
        file.write_all(&(0u32).to_le_bytes())
            .expect("Failed to write spot amount");

        // TODO: Write spots
        // for ( a = 0; a < Spot_amount; a ++  )
        // {
        //     fread( &spot_light[a].x, 4, 1, dat );
        //     fread( &spot_light[a].y, 4, 1, dat );
        //     fread( &spot_light[a].size, 4, 1, dat );
        // }

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
        let mut file = File::open(filename)?;
        let version: u32 = file.read_u32::<LittleEndian>()?;

        if version != VERSION {
            return Err(DeserializationError::ContentError(
                FileTypeError::InvalidVersion,
            ));
        }

        let x_size: u32 = file.read_u32::<LittleEndian>()?;
        if x_size != self.tiles[0].len() as u32 {
            return Err(DeserializationError::ContentError(
                FileTypeError::InvalidLevelSize,
            ));
        }

        let y_size: u32 = file.read_u32::<LittleEndian>()?;
        if y_size != self.tiles.len() as u32 {
            return Err(DeserializationError::ContentError(
                FileTypeError::InvalidLevelSize,
            ));
        }

        for y in 0..(self.tiles.len()) {
            for x in 0..self.tiles[0].len() {
                self.tiles[y][x].texture_type =
                    TextureType::from_u32(file.read_u32::<LittleEndian>()?);
                self.tiles[y][x].id = file.read_u32::<LittleEndian>()?;
                self.tiles[y][x].shadow = file.read_u32::<LittleEndian>()?;
            }
        }

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
