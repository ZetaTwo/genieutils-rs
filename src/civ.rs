use binrw::binrw;
use binrw::helpers::args_iter_with;
use binrw::BinRead;

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

    #[br(count = units_size)]
    unit_pointers: Vec<i32>,

    #[br(parse_with = args_iter_with(&unit_pointers,
    |reader, endian, &pointer| {
        if pointer == 0 {
            Ok(None)
        } else {
            <Unit as BinRead>::read_options(reader, endian, (version,)).map(|x|Some(x))
        }
    }
    ))]
    units: Vec<Option<Unit>>,
}
