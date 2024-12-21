use binrw::binrw;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

#[binrw]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "pyo3", derive(IntoPyObject, FromPyObject))]
pub struct PlayerColour {
    pub id: i32,
    pub player_color_base: i32,
    pub unit_outline_color: i32,
    pub unit_selection_color_1: i32,
    pub unit_selection_color_2: i32,
    pub minimap_color: i32,
    pub minimap_color_2: i32,
    pub minimap_color_3: i32,
    pub statistics_text: i32,
}
