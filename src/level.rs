use crate::types::*;
use crate::util::*;
use byteorder::{LittleEndian, ReadBytesExt};
use std::collections::HashMap;
use std::{fs::File, io::Write};

const DIFF_BULLETS: u32 = 9;
const DIFF_WEAPONS: u32 = 11;
const DIFF_ENEMIES: u32 = 8;

const VERSION: u32 = 5;

pub struct GeneralInfo {
    pub comment: String, // max 19 characters + \0 termination
    pub time_limit: u32,
    pub enemy_table: [u32; DIFF_ENEMIES as usize],
}

pub struct Level {
    pub tiles: Tiles,
    pub p1_position: (u32, u32),
    pub p2_position: (u32, u32),
    pub scroll: (u32, u32),
    pub spotlights: HashMap<(u32, u32), u8>, // level coordinates: 0-9 intensity
    pub general_info: GeneralInfo,
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
            spotlights: HashMap::new(),
            general_info: GeneralInfo {
                comment: "Rust UTK editor".to_string(),
                time_limit: 45,
                enemy_table: [1, 0, 0, 0, 0, 1, 0, 0],
            },
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
                    }
                } else if x == LEVEL_SIZE_X - 1 {
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
                    }
                } else if x == LEVEL_SIZE_X - 1 {
                    Tile {
                        texture_type: TextureType::WALLS,
                        id: 16,
                        shadow: 0,
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
                    }
                } else if x == LEVEL_SIZE_X - 1 {
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
                });
            }
            tiles.push(row);
        }
        tiles
    }

    fn get_tile_index(&self, pointed_tile: u32) -> (usize, usize) {
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
            }
        } else {
            self.tiles[y][x].shadow = match selected_tile_id {
                Some(id) => id + 1,
                None => 0,
            };
        }
    }

    pub fn put_spotlight_to_level(&mut self, level_coordinates: &(u32, u32), spotlight: u8) {
        if spotlight < 10 {
            self.spotlights.insert(*level_coordinates, spotlight);
        }
    }

    pub fn get_spotlight_from_level(&self, level_coordinates: &(u32, u32)) -> u8 {
        *self.spotlights.get(level_coordinates).unwrap()
    }

    pub fn delete_spotlight_if_near(&mut self, level_coordinates: &(u32, u32)) {
        let mut to_be_removed = Vec::new();
        {
            let mut distances: Vec<_> = self
                .spotlights
                .iter()
                .map(|(spotlight_coordinates, &spotlight)| {
                    let distance =
                        get_distance_between_points(level_coordinates, spotlight_coordinates);
                    (spotlight_coordinates, spotlight, distance)
                })
                .collect();
            distances.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());
            for spotlight in distances {
                if get_spotlight_render_radius(&spotlight.1) as f64
                    >= spotlight.2 * RENDER_MULTIPLIER as f64
                {
                    to_be_removed.push(*spotlight.0);
                }
            }
        }
        for key in to_be_removed {
            self.spotlights.remove(&key);
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

        file.write_all(&(self.spotlights.len() as u32).to_le_bytes())
            .expect("Failed to write spot amount");

        for (coordinates, spotlight) in &self.spotlights {
            file.write_all(&coordinates.0.to_le_bytes())
                .expect("Failed to write spotlight x position");
            file.write_all(&coordinates.1.to_le_bytes())
                .expect("Failed to write spotlight y position");
            file.write_all(&(*spotlight as u32).to_le_bytes())
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

        file.write_all(&self.general_info.comment.as_bytes())
            .expect("Failed to write comment");
        for _ in 0..20 - self.general_info.comment.len() {
            file.write_all(b"\0")
                .expect("Failed to write comment padding");
        }
        file.write_all(&self.general_info.time_limit.to_le_bytes())
            .expect("Failed to write time limit");
        for enemy_amount in self.general_info.enemy_table {
            file.write_all(&enemy_amount.to_le_bytes())
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
        self.spotlights.clear();

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
                });
            }
            tiles.push(row);
        }
        self.tiles = tiles;

        self.p1_position.0 = file.read_u32::<LittleEndian>()?;
        self.p1_position.1 = file.read_u32::<LittleEndian>()?;
        self.p2_position.0 = file.read_u32::<LittleEndian>()?;
        self.p2_position.1 = file.read_u32::<LittleEndian>()?;

        let spotlight_amount = file.read_u32::<LittleEndian>()?;

        for _ in 0..spotlight_amount {
            let spotlight_x = file.read_u32::<LittleEndian>()?;
            let spotlight_y = file.read_u32::<LittleEndian>()?;
            self.spotlights.insert(
                (spotlight_x, spotlight_y),
                file.read_u32::<LittleEndian>()? as u8,
            );
        }

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
