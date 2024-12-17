use binrw::binrw;

use crate::common::DebugString;
use crate::unit::Unit;
use crate::versions::Version;

#[binrw]
#[br(import(version: Version))]
#[bw(import(version: Version))]
pub struct Civ {
    player_type: u8,
    name: DebugString,

    #[br(temp)]
    #[bw(try_calc = resources.len().try_into())]
    resources_size: i16,

    tech_tree_id: i16,
    team_bonus_id: i16,
    #[br(count = resources_size)]
    resources: Vec<f32>,
    icon_set: u8,

    #[br(temp)]
    #[bw(try_calc = units.len().try_into())]
    units_size: i16,

    // TODO: unit_pointers
    #[br(
        count = units_size,
        args { inner: (version,)  }
    )]
    units: Vec<Option<Unit>>,
}
