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
            "pistol",
            "shotgun",
            "uzi",
            "auto rifle",
            "grenade launcher",
            "auto grenadier",
            "heavy launcher",
            "auto shotgun",
            "c4-activator",
            "flame thrower",
            "mine dropper",
        ]
        .to_vec(),
        [
            "9mm bullets (50)",
            "12mm bullets (50)",
            "shotgun shells (20)",
            "light grenades (15)",
            "medium grenades (10)",
            "heavy grenades (5)",
            "c4-explosives (5)",
            "gas (50)",
            "mines (5)",
        ]
        .to_vec(),
        ["energy"].to_vec(),
    ]
}
