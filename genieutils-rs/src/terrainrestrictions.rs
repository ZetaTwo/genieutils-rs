use binrw::binrw;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

#[binrw]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "pyo3", derive(IntoPyObject, FromPyObject))]
pub struct TerrainPassGraphic {
    pub exit_tile_sprite_id: i32,
    pub enter_tile_sprite_id: i32,
    pub walk_tile_sprite_id: i32,
    pub walk_sprite_rate: i32,
}

#[binrw]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "pyo3", derive(IntoPyObject, FromPyObject))]
#[br(import(terrain_count: usize))]
#[bw(assert(passable_buildable_dmg_multiplier.len() == terrain_pass_graphics.len(), "terrain restriciton lists lengths unmatched: {} != {}", passable_buildable_dmg_multiplier.len(), terrain_pass_graphics.len()))]
pub struct TerrainRestriction {
    #[br(count = terrain_count)]
    pub passable_buildable_dmg_multiplier: Vec<f32>,
    #[br(count = terrain_count)]
    pub terrain_pass_graphics: Vec<TerrainPassGraphic>,
}
