use binrw::binrw;

#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};

pub const TILE_TYPE_COUNT: usize = 19;
pub const TERRAIN_COUNT: usize = 200;
pub const TERRAIN_UNITS_SIZE: usize = 30;

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Copy)]
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

impl PartialEq<u8> for UnitType {
    fn eq(&self, other: &u8) -> bool {
        (*self as u8).eq(other)
    }
}

impl PartialEq<UnitType> for u8 {
    fn eq(&self, other: &UnitType) -> bool {
        self.eq(&(*other as u8))
    }
}

impl PartialOrd<u8> for UnitType {
    fn partial_cmp(&self, other: &u8) -> Option<std::cmp::Ordering> {
        Some((*self as u8).cmp(other))
    }
}

impl PartialOrd<UnitType> for u8 {
    fn partial_cmp(&self, other: &UnitType) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&(*other as u8)))
    }
}

#[binrw]
#[brw(little)]
#[br(assert(temp_size == 0x0A60, "DebugString temp_size invalid: {}", temp_size))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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

//#[cfg(test)]
/*impl std::fmt::Display for DebugString {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "DebugString: {}", self.int_str)
    }
}*/
/*#[cfg(test)]
impl std::fmt::Debug for DebugString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DebugString: {}", self.int_str)
    }
}*/
