use binrw::binrw;

use crate::common::DebugString;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

#[binrw]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "pyo3", derive(IntoPyObject, FromPyObject))]
pub struct SoundItem {
    pub filename: DebugString,
    pub resource_id: i32,
    pub probability: i16,
    pub civilization: i16,
    pub icon_set: i16,
}

#[binrw]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "pyo3", derive(IntoPyObject, FromPyObject))]
pub struct Sound {
    pub id: i16,
    pub play_delay: i16,
    #[br(temp)]
    #[bw(try_calc = items.len().try_into())]
    item_size: i16,
    pub cache_time: i32,
    pub total_probability: i16,
    #[br(count = item_size)]
    pub items: Vec<SoundItem>,
}
