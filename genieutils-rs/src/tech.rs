use binrw::binrw;

use crate::common::DebugString;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

#[binrw]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "pyo3", derive(IntoPyObject, FromPyObject))]
struct ResearchResourceCost {
    r#type: i16,
    amount: i16,
    flag: u8,
}

#[binrw]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "pyo3", derive(IntoPyObject, FromPyObject))]
pub struct Tech {
    required_techs: (i16, i16, i16, i16, i16, i16),
    resource_costs: (
        ResearchResourceCost,
        ResearchResourceCost,
        ResearchResourceCost,
    ),
    required_tech_count: i16,
    civ: i16,
    full_tech_mode: i16,
    research_location: i16,
    language_dll_name: i32,
    language_dll_description: i32,
    research_time: i16,
    effect_id: i16,
    r#type: i16,
    icon_id: i16,
    button_id: u8,
    language_dll_help: i32,
    language_dll_tech_tree: i32,
    hot_key: i32,
    name: DebugString,
    repeatable: u8,
}
