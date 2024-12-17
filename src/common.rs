use binrw::binrw;

pub const TILE_TYPE_COUNT: usize = 19;
pub const TERRAIN_COUNT: usize = 200;
pub const TERRAIN_UNITS_SIZE: usize = 30;

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub enum UnitType {
    EyeCandy = 10,
    Trees = 15,
    Flag = 20,
    DeadFish = 30,
    Bird = 40,
    Combatant = 50,
    Projectile = 60,
    Creatable = 70,
    Building = 80,
    AoeTrees = 90,
}

impl TryFrom<u8> for UnitType {
    type Error = std::io::Error;

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            x if x == Self::EyeCandy as u8 => Ok(Self::EyeCandy),
            x if x == Self::Trees as u8 => Ok(Self::Trees),
            x if x == Self::Flag as u8 => Ok(Self::Flag),
            x if x == Self::DeadFish as u8 => Ok(Self::DeadFish),
            x if x == Self::Bird as u8 => Ok(Self::Bird),
            x if x == Self::Combatant as u8 => Ok(Self::Combatant),
            x if x == Self::Projectile as u8 => Ok(Self::Projectile),
            x if x == Self::Creatable as u8 => Ok(Self::Creatable),
            x if x == Self::Building as u8 => Ok(Self::Building),
            x if x == Self::AoeTrees as u8 => Ok(Self::AoeTrees),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "invalid unit type",
            )),
        }
    }
}

#[binrw]
#[br(assert(temp_size == 0x0A60, "DebugString temp_size invalid: {}", temp_size))]
pub struct DebugString {
    #[br(temp)]
    #[bw(calc = 0x0A60)]
    temp_size: u16,

    size: u16,

    #[br(count = size)]
    #[br(try_map = |x: Vec<u8>| String::from_utf8(x))]
    #[bw(map = |x: &String| x.as_bytes())]
    int_str: String,
}
