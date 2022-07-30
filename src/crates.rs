#[derive(Clone, Copy)]
pub enum CrateClass {
    Weapon = 0,
    Bullet = 1,
    Energy = 2,
}

impl CrateClass {
    pub fn from_u32(value: u32) -> CrateClass {
        match value {
            0 => CrateClass::Weapon,
            1 => CrateClass::Bullet,
            2 => CrateClass::Energy,
            _ => panic!("Unknown value: {}", value),
        }
    }
}

pub type Crates<'a> = [std::vec::Vec<&'a str>; 3];

pub fn get_crates<'a>() -> Crates<'a> {
    [
        [
            "PISTOL",
            "SHOTGUN",
            "UZI",
            "AUTO RIFLE",
            "GRENADE LAUNCHER",
            "AUTO GRENADIER",
            "HEAVY LAUNCHER",
            "AUTO SHOTGUN",
            "C4-ACTIVATOR",
            "FLAME THROWER",
            "MINE DROPPER",
        ]
        .to_vec(),
        [
            "9MM BULLETS (50)",
            "12MM BULLETS (50)",
            "SHOTGUN SHELLS (20)",
            "LIGHT GRENADES (15)",
            "MEDIUM GRENADES (10)",
            "HEAVY GRENADES (5)",
            "C4-EXPLOSIVES (5)",
            "GAS (50)",
            "MINES (5)",
        ]
        .to_vec(),
        ["ENERGY"].to_vec(),
    ]
}
