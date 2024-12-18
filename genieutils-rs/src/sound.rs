use binrw::binrw;

use crate::common::DebugString;

#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};

#[binrw]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
struct SoundItem {
    filename: DebugString,
    resource_id: i32,
    probability: i16,
    civilization: i16,
    icon_set: i16,
}

#[binrw]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Sound {
    id: i16,
    play_delay: i16,
    #[br(temp)]
    #[bw(try_calc = items.len().try_into())]
    item_size: i16,
    cache_time: i32,
    total_probability: i16,
    #[br(count = item_size)]
    items: Vec<SoundItem>,
}
