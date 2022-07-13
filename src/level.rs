use std::{fs::File, io::Write};

use crate::util::Tile;

const DIFF_BULLETS: u32 = 9;
const DIFF_WEAPONS: u32 = 11;
const DIFF_ENEMIES: u32 = 8;

const VERSION: u32 = 5;

pub fn serialize(filename: &str, level: [[Tile; 16]; 12]) -> std::io::Result<()> {
    let mut file = File::create(filename)?;

    file.write_all(&VERSION.to_le_bytes())
        .expect("Failed to write version");
    file.write_all(&(level[0].len() as u32).to_le_bytes())
        .expect("Failed to write x size");
    file.write_all(&(level.len() as u32).to_le_bytes())
        .expect("Failed to write y size");
    for y in 0..(level.len()) {
        for x in 0..level[0].len() {
            file.write_all(&(level[y][x].texture_type as u32).to_le_bytes())
                .expect("Failed to write block type");
            file.write_all(&(level[y][x].id as u32).to_le_bytes())
                .expect("Failed to write block num");
            file.write_all(&0u32.to_le_bytes())
                .expect("Failed to write block num");
        }
    }

    file.write_all(&(1u32).to_le_bytes())
        .expect("Failed to write p1 start x");
    file.write_all(&(1u32).to_le_bytes())
        .expect("Failed to write p1 start y");
    file.write_all(&(2u32).to_le_bytes())
        .expect("Failed to write p2 start x");
    file.write_all(&(2u32).to_le_bytes())
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

    let comment = "Rust UTK editor     ";
    file.write_all(&comment.as_bytes())
        .expect("Failed to write comment");
    file.write_all(&(120u32).to_le_bytes())
        .expect("Failed to write time limit");
    file.write_all(&(4u32 * DIFF_ENEMIES).to_le_bytes())
        .expect("Failed to write normal game enemies");
    file.write_all(&(4u32 * DIFF_WEAPONS).to_le_bytes())
        .expect("Failed to write normal game weapons");
    file.write_all(&(4u32 * DIFF_BULLETS).to_le_bytes())
        .expect("Failed to write normal game bullets");
    file.write_all(&(0u32).to_le_bytes())
        .expect("Failed to write normal game energy crates");
    file.write_all(&(4u32 * DIFF_ENEMIES).to_le_bytes())
        .expect("Failed to write multiplayer game enemies");
    file.write_all(&(4u32 * DIFF_WEAPONS).to_le_bytes())
        .expect("Failed to write multiplayer game weapons");
    file.write_all(&(4u32 * DIFF_BULLETS).to_le_bytes())
        .expect("Failed to write multiplayer game bullets");
    file.write_all(&(0u32).to_le_bytes())
        .expect("Failed to write multiplayer game energy crates");
    file.write_all(&(0u32).to_le_bytes())
        .expect("Failed to write normal game crate amount");
    // TODO: Write normal game crates
    //  fread( normal_crate_info, sizeof(Crate_info) * normal_crate_amount, 1, dat );
    file.write_all(&(0u32).to_le_bytes())
        .expect("Failed to write multiplayer game crate amount");
    // TODO: Write deathmatch game crates
    //  fread( deathmatch_crate_info, sizeof(Crate_info) * deathmatch_crate_amount, 1, dat );

    Ok(())
}
